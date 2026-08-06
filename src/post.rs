use regex::Regex;
use std::{collections::HashMap, fs, path};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct Post {
    pub date: String,
    pub lines: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub slug: String,
    pub title: String,
}

/**
 * TODO:
 * - Make process more forgiving (by not having 'expect's everywhere)
 */
impl Post {
    pub fn from_path(path: path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
        // .unwrap()
        // .replace_all(&line, "<a class=\"article-link\" href=\"$2\">$1</a>")
        // .to_string()

        let mut content = fs::read_to_string(path.clone())?;

        let code_block_regex = Regex::new(r"(?m)^```(?:\s*(\w+))?([\s\S]*?)^```$").unwrap();

        code_block_regex
            .captures_iter(content.clone().as_str())
            .for_each(|c| {
                let raw_code_block = c.get(0).unwrap().as_str();

                let syntax_set = SyntaxSet::load_defaults_newlines();
                let syntax = syntax_set.find_syntax_by_name("Rust").unwrap();
                let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                    syntax,
                    &syntax_set,
                    ClassStyle::Spaced,
                );

                for line in LinesWithEndings::from(raw_code_block) {
                    html_generator
                        .parse_html_for_line_which_includes_newline(line)
                        .unwrap();
                }
                let output_html = html_generator.finalize();

                content = content.replace(raw_code_block, output_html.as_str());
            });

        let mut lines: Vec<String> = content.split("\n").map(|line| line.to_string()).collect();

        let mut metadata: HashMap<String, String> = HashMap::new();

        let mut metadata_boundary_indices =
            lines.iter().enumerate().filter_map(|(position, line)| {
                if *line == "---" {
                    return Some(position);
                }

                None
            });

        let metadata_start_line_index = metadata_boundary_indices.next();

        if let Some(metadata_start_line_index) = metadata_start_line_index {
            let metadata_end_line_index = metadata_boundary_indices
                .next()
                .expect("Could not find metadata end (---)");

            for (line_index, line) in lines.iter().enumerate() {
                if line_index <= metadata_start_line_index {
                    continue;
                }

                // If the metadata closer (---) is reached, do not continue
                if *line == "---" {
                    break;
                }

                let (key, value) = line
                    .split_once(":")
                    .expect(format!("Badly formatted metadata: {}", line).as_str());

                metadata.insert(key.trim().to_string(), value.trim().to_string());
            }

            // Remove metadata lines
            lines = lines
                .split_at(metadata_end_line_index)
                .1
                .to_vec()
                .split_at_checked(1)
                .expect("Did not find content after metadata end")
                .1
                .to_vec();
        }

        let slug = path
            .clone()
            .file_name()
            .expect("Could not get filename from path")
            .to_string_lossy()
            .split(".")
            .take_while(|word| *word != "md")
            .collect::<Vec<&str>>()
            .join("");

        let date = metadata
            .get("date")
            .expect("Could not get date from metadata")
            .to_owned();

        let title = lines
            .iter()
            .find(|line| line.starts_with("# "))
            .expect("Could not find title line (containg '# ')")
            .replace("# ", "");

        Ok(Post {
            date,
            lines,
            metadata,
            slug,
            title,
        })
    }

    pub fn render_to_file(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output_path = format!("site/{}.html", self.slug);

        let title_line = self
            .lines
            .iter()
            .find(|line| line.starts_with("# "))
            .expect("Could not find title line (containg '# ')");

        let mut currently_in_list = false;
        let mut currently_in_code_block = false;

        let mut formatted_lines = self
            .lines
            .iter()
            .map(|line| {
                let mut line = line.clone();

                let should_close_ul = !line.starts_with("- ") && currently_in_list;

                if currently_in_code_block {
                    if line.ends_with("```</span>") {
                        currently_in_code_block = false;

                        return "</pre>".to_string();
                    }

                    return line;
                } else if line == "<span class=\"source rust\">```rust" {
                    currently_in_code_block = true;
                    return "<pre>".to_string();
                } else {
                    line = replace_styling_with_tags(line, "**", "b");
                    line = replace_styling_with_tags(line, "_", "i");
                    line = replace_styling_with_tags(line, "`", "code");
                    line = replace_links(line);
                }

                if line.starts_with("# ") {
                    line = line.replace("# ", "");
                    line.insert_str(0, "<h1 class=\"title\">");
                    line.push_str("</h1>");

                    if !self.metadata.is_empty() {
                        line.push_str("<section id=\"metadata\">");

                        let metadata_date = &self.metadata.iter().find(|item| item.0 == "date");

                        if let Some(metadata_date) = metadata_date {
                            line.push_str(
                                format!("<span>Published: {}</span>", metadata_date.1).as_str(),
                            );
                        }

                        let metadata_category =
                            &self.metadata.iter().find(|item| item.0 == "category");

                        if let Some(metadata_category) = metadata_category {
                            line.push_str("<span><ul class=\"categories\">Categories: ");

                            metadata_category
                                .1
                                .split(",")
                                .map(|category| category.trim())
                                .for_each(|category| {
                                    line.push_str("<li>");
                                    line.push_str(category);
                                    line.push_str("</li>");
                                });

                            line.push_str("</ul></span>");
                        }

                        line.push_str("</section>");
                    }
                } else if line.starts_with("- ") {
                    line = line.replace("- ", "");

                    line.insert_str(0, "<li>");
                    line.push_str("</li>");

                    if currently_in_list == false {
                        currently_in_list = true;

                        line.insert_str(0, "<ul class=\"posts-list\">");
                    }
                } else if line.starts_with("## ") {
                    line = line.replace("## ", "");
                    line.insert_str(0, "<h2>");
                    line.push_str("</h2>");
                } else if line.starts_with("### ") {
                    line = line.replace("### ", "");
                    line.insert_str(0, "<h3>");
                    line.push_str("</h3>");
                } else {
                    line.insert_str(0, "<p>");
                    line.push_str("</p>");

                    if should_close_ul {
                        line.insert_str(0, "</ul>");
                        currently_in_list = false;
                    }
                }

                line
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatted_lines.insert_str(0, "<article>");
        formatted_lines.push_str("</article>");
        formatted_lines = formatted_lines.replace("<p></p>", "");

        let output = wrap_in_template(formatted_lines, title_line.replace("# ", ""));

        fs::write(output_path.clone(), output)?;

        Ok(output_path)
    }
}

fn replace_styling_with_tags(line: String, source: &str, target: &str) -> String {
    let parts = line.split(source);
    let parts_len = parts.clone().collect::<Vec<&str>>().len();

    parts
        .enumerate()
        .map(|(index, part)| {
            if index == parts_len - 1 {
                format!("{}", part)
            } else if index % 2 == 0 {
                format!("{}<{}>", part, target)
            } else {
                format!("{}</{}>", part, target)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn replace_links(line: String) -> String {
    Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
        .unwrap()
        .replace_all(&line, "<a class=\"article-link\" href=\"$2\">$1</a>")
        .to_string()
}

fn wrap_in_template(content: String, title: String) -> String {
    let template = fs::read_to_string("src/template.html").expect("Failed to read template");

    template
        .replace("{{title}}", format!("{} - tphbrok.me", title).as_str())
        .replace("{{content}}", content.as_str())
}
