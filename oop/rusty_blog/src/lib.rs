// Docs: https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types

// Better than object oriented implementation, the transformations between states
// are no longer encapsulated entirely within the Post implementation
// However, our gain is that invalid states are now impossible because of the type system
// and type checking that happens at compile time (no trait objects)
// This ensures that certain bugs, such as display of the content of an
// unpublished post, will be discovered berfore the make it to production

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    // This way no instance of Post can be created, hast to be DraftPost first
    // (because content is private)
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // Now Draft Posts have not way to display there content and therefore cannot be displayed in production
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        // Consume self to 'destroy' Draft Post State
        PendingReviewPost {
            content: self.content,
            approved: false,
        }
    }
}

pub enum TypeOr<PendingReviewPost, Post> {
    PendingReviewPost(PendingReviewPost),
    Post(Post),
}

pub struct PendingReviewPost {
    content: String,
    approved: bool,
}

impl PendingReviewPost {
    pub fn approve(self) -> TypeOr<PendingReviewPost, Post> {
        match self.approved {
            false => TypeOr::PendingReviewPost(PendingReviewPost {
                content: self.content,
                approved: true,
            }),
            true => TypeOr::Post(Post {
                content: self.content,
            }),
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
