use std::fs;

use crate::post::Post;

mod post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for post in fs::read_dir("posts")? {
        let post = Post::from_path(post?.path())?;

        dbg!(post.metadata);
        dbg!(post.lines);
    }

    Ok(())
}
