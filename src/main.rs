mod post;

use std::fs;

use crate::post::Post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for post in fs::read_dir("posts")? {
        let post = Post::from_path(post?.path().clone())?;

        let output_path = post.render_to_file()?;

        println!("Wrote to {}", output_path);
    }

    Ok(())
}
