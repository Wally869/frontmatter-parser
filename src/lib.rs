pub mod error;
pub mod parser;

pub use error::{FrontmatterError, Result};
pub use parser::{parse_content, parse_directory, parse_file, Frontmatter};
