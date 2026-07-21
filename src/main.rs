mod post;

use std::fs;

use crate::post::Post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for post in fs::read_dir("posts")? {
        let post = Post::from_path(post?.path())?;

        dbg!(post.metadata);
        dbg!(post.lines);
    }

    Ok(())
}
