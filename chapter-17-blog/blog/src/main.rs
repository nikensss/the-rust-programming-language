use blog::state_with_types::pseudo_main;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.add_text(". And got a blog rejected!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(
        "I ate a salad for lunch today. And got a blog rejected!",
        post.content()
    );

    pseudo_main();
}
