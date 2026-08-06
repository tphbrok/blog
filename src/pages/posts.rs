use crate::{post::Post, template::wrap_in_template};

pub fn generate_posts_page(posts: &Vec<Post>) -> String {
    let mut content = String::new();

    content.push_str("<h1>Posts</h1>");

    let posts_per_year =
        posts.chunk_by(|a, b| a.date.split("-").take(1).next() == b.date.split("-").take(1).next());

    posts_per_year.for_each(|posts| {
        let year = posts[0].date.split_at(4).0;

        content.push_str(
            format!(
                "<h2 class=\"posts-year\">{}</h2><ul class=\"posts-list\">",
                year
            )
            .as_str(),
        );

        posts.iter().for_each(|post| {
            content.push_str(
                format!(
                    "<li><a href=\"{}.html\">{}</a><br>({})</li>",
                    post.slug, post.title, post.date
                )
                .as_str(),
            )
        });

        content.push_str("</ul>");
    });

    wrap_in_template(content, String::from("Posts"))
}
