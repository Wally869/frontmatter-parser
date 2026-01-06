pub mod error;
pub mod parser;

pub use error::{FrontmatterError, Result};
pub use parser::{Frontmatter, parse_content, parse_directory, parse_file};
