use rusty_blog::{Post, TypeOr};

fn main() {
    // DraftPost
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    // PendingReviewPost
    let post = post.request_review();

    // Require two approvals
    let mut post = post.approve();

    if let TypeOr::PendingReviewPost(value) = post {
        post = value.approve();
    };

    if let TypeOr::Post(value) = post {
        assert_eq!("I ate a salad for lunch today", value.content());
    };
}
