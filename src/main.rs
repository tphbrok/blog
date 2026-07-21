use std::fs;

use crate::post::Post;

mod post;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for blog in fs::read_dir("blogs")? {
        let blog = Post::from_path(blog?.path())?;

        dbg!(blog.metadata);
        dbg!(blog.lines);
    }

    Ok(())
}
