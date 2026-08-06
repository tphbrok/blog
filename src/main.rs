mod post;
mod template;

use std::fs;

use crate::{post::Post, template::wrap_in_template};

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

    let mut homepage_content = String::new();

    homepage_content.push_str(
        "<h1 class=\"name\">Thomas Brok</h1><p>I'm a Dutch developer, powered by Axxes.<br><br>I spend my professional time writing TypeScript and deploying to AWS, with coding agents as my sidekicks. I spend my free time writing plain text and Rust <i>without</i> AI (because I'm in it for learning and general enjoyment of programming).<br><br>Whenever I'm not spending time with my family and friends, I produce music, play videogames and read paper books.");

    homepage_content.push_str("<h1>Latest posts</h1><ul class=\"posts-list\">");

    posts.sort_by_key(|post| post.date.clone());
    posts.reverse();

    posts.iter().take(5).for_each(|post| {
        homepage_content.push_str(
            format!(
                "<li><a href=\"{}.html\">{}</a><br>({})</li>",
                post.slug, post.title, post.date
            )
            .as_str(),
        );
    });

    homepage_content.push_str("</ul><a href=\"posts\">View all posts &rarr;</a>");

    let homepage_output = wrap_in_template(homepage_content, String::from("Home"));
    let homepage_output_path = "site/index.html";

    fs::write(homepage_output_path, homepage_output)?;

    let mut posts_content = String::new();

    posts_content.push_str("<h1>Posts</h1>");

    let posts_per_year =
        posts.chunk_by(|a, b| a.date.split("-").take(1).next() == b.date.split("-").take(1).next());

    posts_per_year.for_each(|posts| {
        let year = posts[0].date.split_at(4).0;

        posts_content.push_str(
            format!(
                "<h2 class=\"posts-year\">{}</h2><ul class=\"posts-list\">",
                year
            )
            .as_str(),
        );

        posts.iter().for_each(|post| {
            posts_content.push_str(
                format!(
                    "<li><a href=\"{}.html\">{}</a><br>({})</li>",
                    post.slug, post.title, post.date
                )
                .as_str(),
            )
        });

        posts_content.push_str("</ul>");
    });

    let posts_output = wrap_in_template(posts_content, String::from("Posts"));
    let posts_output_path = "site/posts.html";

    fs::write(posts_output_path, posts_output)?;

    Ok(())
}
