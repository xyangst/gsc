mod analyse;
pub mod error;
use analyse::Analysis;
use clap::Parser;
pub use error::{Error, Result};
use ignore::Walk;
use std::{collections::HashMap, path::Path};
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    #[arg(default_value = "./")]
    name: String,

    ///amount of file extensions to display
    #[arg(short, long, default_value_t = 6)]
    amount: usize,
}

fn main() {
    let cli = Cli::parse();
    let data = Data::get(&cli.name).unwrap();
    data.print_table(cli.amount);
}
#[derive(Debug, Default)]
struct Data {
    extension_counts: HashMap<String, Analysis>,
}
impl Data {
    pub fn print_table(&self, amount: usize) {
        // Collect the entries into a Vec and sort by source_lines in descending order
        let mut sorted_entries: Vec<(&String, &Analysis)> = self.extension_counts.iter().collect();
        sorted_entries.sort_by(|a, b| b.1.source_lines.cmp(&a.1.source_lines));

        // Print the top 5 extensions
        println!(
            "{:<10} {:>10} {:>10} {:>10} {:>10}",
            "Extension", "Source", "Comments", "Empty", "Total"
        );
        let header = "-".repeat(54);
        println!("{}", header);
        let mut total = Analysis {
            comments: 0,
            empty: 0,
            source_lines: 0,
        };
        for (ext, analysis) in sorted_entries.iter().take(amount) {
            let ext = if ext.is_empty() { "unknown" } else { ext };
            println!(
                "{:<10} {:>10} {:>10} {:>10} {:>10}",
                ext,
                analysis.source_lines,
                analysis.comments,
                analysis.empty,
                analysis.total()
            );
            total.comments += analysis.comments;
            total.empty += analysis.empty;
            total.source_lines += analysis.source_lines;
        }
        println!(
            "{:<10} {:>10} {:>10} {:>10} {:>10}",
            "Total",
            total.source_lines,
            total.comments,
            total.empty,
            total.total()
        );
    }
    fn get(path: &str) -> Result<Self> {
        let path = Path::new(&path);
        Walk::new(path)
            .filter_map(|v| v.ok())
            .filter(|v| {
                let path = v.path();
                !path.is_symlink() && path.is_file()
            })
            .try_fold(Self::default(), |mut a, v| {
                let res = Analysis::file(v.path());
                if let Err(Error::IO(_)) = res {
                    return Ok(a);
                }
                let (analysis, extension) = res?;

                a.extension_counts
                    .entry(extension)
                    .and_modify(|a| {
                        a.comments += analysis.comments;
                        a.empty += analysis.empty;
                        a.source_lines += analysis.source_lines;
                    })
                    .or_insert(analysis);
                Ok(a)
            })
    }
}
