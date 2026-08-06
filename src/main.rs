mod pages;
mod post;
mod template;

use std::fs;

use crate::{
    pages::{posts::generate_posts_page, root::generate_root_page},
    post::Post,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut posts: Vec<Post> = vec![];

    fs::remove_dir_all("site")?;
    fs::create_dir("site")?;

    for post in fs::read_dir("posts")? {
        let post = Post::from_path(post?.path().clone())?;

        let output_path = post.render_to_file()?;

        posts.push(post);

        println!("Wrote to {}", output_path);
    }

    posts.sort_by_key(|post| post.date.clone());
    posts.reverse();

    fs::write("site/index.html", generate_root_page(&posts))?;
    fs::write("site/posts.html", generate_posts_page(&posts))?;

    Ok(())
}
