use rusty_blog::Post;

fn main() {
    // DraftPost
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // PendingReviewPost
    let post = post.request_review();

    // Post (for production)
    let post = post.approve().approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
