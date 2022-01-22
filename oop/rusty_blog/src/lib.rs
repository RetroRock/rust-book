// Docs: https://doc.rust-lang.org/stable/book/ch17-03-oo-design-patterns.html#encoding-states-and-behavior-as-types

// Better than object oriendted implementation, the transformations between states
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

    pub fn request_review(self) -> PendingReviewOnePost {
        // Consume self to 'destroy' Draft Post State
        PendingReviewOnePost {
            content: self.content,
        }
    }
}

// TODO: Fix duplication
pub struct PendingReviewOnePost {
    content: String,
}

impl PendingReviewOnePost {
    // Reveal content only after it has been approved
    // Consume self to 'destroy' Pending Review State
    pub fn approve(self) -> PendingReviewTwoPost {
        PendingReviewTwoPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
pub struct PendingReviewTwoPost {
    content: String,
}

impl PendingReviewTwoPost {
    // Reveal content only after it has been approved
    // Consume self to 'destroy' Pending Review State
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
