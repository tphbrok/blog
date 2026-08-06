use crate::{post::Post, template::wrap_in_template};

pub fn generate_root_page(posts: &Vec<Post>) -> String {
    let mut content = String::new();

    content.push_str(
        "<h1 class=\"name\">Thomas Brok</h1><p>I'm a Dutch developer, powered by Axxes.<br><br>I spend my professional time writing TypeScript and deploying to AWS, with coding agents as my sidekicks. I spend my free time writing plain text and Rust <i>without</i> AI (because I'm in it for learning and general enjoyment of programming).<br><br>Whenever I'm not spending time with my family and friends, I produce music, play videogames and read paper books.");

    content.push_str("<h1>Latest posts</h1><ul class=\"posts-list\">");

    posts.iter().take(5).for_each(|post| {
        content.push_str(
            format!(
                "<li><a href=\"{}.html\">{}</a><br>({})</li>",
                post.slug, post.title, post.date
            )
            .as_str(),
        );
    });

    content.push_str("</ul><a href=\"posts\">View all posts &rarr;</a>");

    wrap_in_template(content, String::from("Home"))
}
