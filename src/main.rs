mod post;

use std::fs;

use crate::post::Post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut slugs: Vec<String> = vec![];

    for post in fs::read_dir("posts")? {
        let post = Post::from_path(post?.path().clone())?;

        let output_path = post.render_to_file()?;

        slugs.push(post.slug);

        println!("Wrote to {}", output_path);
    }

    Ok(())
}
