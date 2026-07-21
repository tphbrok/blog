use std::{collections::HashMap, fs, path};

pub struct Post {
    pub metadata: HashMap<String, String>,
    pub lines: Vec<String>,
}

impl Post {
    pub fn from_path(path: path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines: Vec<String> = fs::read_to_string(path)?
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

        Ok(Post { lines, metadata })
    }
}
