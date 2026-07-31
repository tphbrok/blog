use std::{collections::HashMap, fs, path};

use crate::template::wrap_in_template;

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
        let mut lines: Vec<String> = fs::read_to_string(path.clone())?
            .split("\n")
            .filter(|line| line.len() > 0)
            .map(|line| line.to_string())
            .collect();

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

        let mut formatted_lines = self
            .lines
            .iter()
            .map(|line| {
                let mut line = line.clone();

                line = replace_styling_with_tags(line, "**", "b");
                line = replace_styling_with_tags(line, "_", "i");

                let should_close_ul = !line.starts_with("- ") && currently_in_list;

                if line.starts_with("# ") {
                    line = line.replace("# ", "");
                    line.insert_str(0, "<h1 class=\"title\">");
                    line.push_str("</h1>");

                    let metadata_category = &self.metadata.iter().find(|item| item.0 == "category");

                    if let Some(metadata_category) = metadata_category {
                        line.push_str("<ul class=\"categories\">Categories: ");

                        metadata_category
                            .1
                            .split(",")
                            .map(|category| category.trim())
                            .for_each(|category| {
                                line.push_str("<li>");
                                line.push_str(category);
                                line.push_str("</li>");
                            });

                        line.push_str("</ul>");
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
                } else {
                    line.insert_str(0, "<p>");
                    line.push_str("</p>");

                    if should_close_ul {
                        line.insert_str(0, "</ul>");
                    }
                }

                line
            })
            .collect::<Vec<String>>()
            .join("\n");

        formatted_lines.insert_str(0, "<article>");
        formatted_lines.push_str("</article>");

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
