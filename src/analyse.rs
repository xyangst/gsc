use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::Result;

#[derive(Debug)]
pub struct Analysis {
    pub comments: usize,
    pub empty: usize,
    pub source_lines: usize,
}

impl Analysis {
    pub fn total(&self) -> usize {
        self.comments + self.empty + self.source_lines
    }
    pub fn file(path: &Path) -> Result<(Analysis, String)> {
        let extension = path
            .extension()
            .and_then(|v| v.to_owned().into_string().ok())
            .unwrap_or("".into());
        let file_type = FileTypes::from_string(&extension);
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut summary = Analysis {
            comments: 0,
            empty: 0,
            source_lines: 0,
        };

        for line in reader.lines() {
            //ignore non utf8 lines
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                summary.empty += 1;
            } else if is_commment_line(trimmed_line, &file_type) {
                summary.comments += 1;
            } else {
                summary.source_lines += 1;
            }
        }

        // Ok((code_lines, empty_lines, comment_lines))
        Ok((summary, extension))
    }
}

fn is_commment_line(line: &str, filetype: &FileTypes) -> bool {
    let Some(comment_sign) = filetype.get() else {
        return false;
    };
    line.starts_with(&comment_sign)
}
enum FileTypes {
    Javascript,
    TypeScript,
    Svelte,
    Rust,
    Toml,
    Json,
    Python,
    Unknown,
}
impl FileTypes {
    fn from_string(extension: &str) -> Self {
        use FileTypes::*;
        match extension {
            "svelte" => Svelte,
            "js" => Javascript,
            "ts" => TypeScript,
            "rs" => Rust,
            "toml" => Toml,
            "json" => Json,
            "py" => Python,
            _ => Unknown,
        }
    }
    fn get(&self) -> Option<String> {
        use FileTypes::*;
        match self {
            Javascript | TypeScript | Rust => Some("//".into()),
            Python | Toml => Some("#".into()),
            _ => None,
        }
    }
}
