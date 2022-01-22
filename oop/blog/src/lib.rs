// Docs: https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#implementing-an-object-oriented-design-pattern

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content
            .push_str(self.state.as_ref().unwrap().add_text(text));
    }

    pub fn content(&self) -> &str {
        // as_ref, do not want to take ownership -> Returns Option<&Box<dyn State>>
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn add_text<'a>(&self, text: &'a str) -> &'a str {
        &text
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approved: false })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {
    // Require two approvals
    approved: bool,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approved: false })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        match self.approved {
            false => Box::new(PendingReview { approved: true }),
            true => Box::new(Published {}),
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // Lifetime of content is bond to post
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_states() {
        let mut post = Post::new();

        post.add_text("Hello, here is some text. ");
        assert_eq!("", post.content());

        post.request_review();
        post.add_text("Attempting to add text in Pending Review State. ");
        assert_eq!("", post.content());

        post.reject();
        post.add_text("Adding text in Draft State.");
        assert_eq!("", post.content());

        post.request_review();
        post.approve();
        assert_eq!("", post.content());

        post.approve();
        post.add_text("Attempting to add text in Published State.");
        assert_eq!(
            "Hello, here is some text. Adding text in Draft State.",
            post.content()
        );
    }
}
