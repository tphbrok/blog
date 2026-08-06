use std::fs;

pub fn wrap_in_template(content: String, title: String) -> String {
    let template = fs::read_to_string("src/template.html").expect("Failed to read template");

    let result = template
        .replace("{{content}}", content.as_str())
        .replace("{{title}}", format!("{} - tphbrok.me", title).as_str());

    result
}
