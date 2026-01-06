# frontmatter-parser

A fast CLI tool and library for extracting YAML frontmatter from markdown files.

## Installation

```bash
cargo install --path .
```

## CLI Usage

### Parse a single file

```bash
frontmatter-parser file post.md
```

Output:
```json
{
  "title": "My Blog Post",
  "author": "John Doe",
  "tags": ["rust", "cli"]
}
```

### Parse all files in a directory

```bash
frontmatter-parser dir ./posts
```

Output:
```json
[
  {
    "file": "posts/first.md",
    "frontmatter": { "title": "First Post" }
  },
  {
    "file": "posts/second.md",
    "frontmatter": { "title": "Second Post" }
  }
]
```

### Recursive directory scan

```bash
frontmatter-parser dir ./content --recursive
```

## Library Usage

```rust
use frontmatter_parser::{parse_file, parse_directory};

// Parse single file
let fm = parse_file("post.md")?;
println!("{}", fm.data["title"]);

// Parse directory
for result in parse_directory("posts/", true) {
    if let Ok(fm) = result {
        println!("{}: {:?}", fm.path.display(), fm.data);
    }
}
```

## License

MIT
