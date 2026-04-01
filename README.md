# rs-duration-fmt

[![CI](https://github.com/philiprehberger/rs-duration-fmt/actions/workflows/ci.yml/badge.svg)](https://github.com/philiprehberger/rs-duration-fmt/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/philiprehberger-duration-fmt.svg)](https://crates.io/crates/philiprehberger-duration-fmt)
[![Last updated](https://img.shields.io/github/last-commit/philiprehberger/rs-duration-fmt)](https://github.com/philiprehberger/rs-duration-fmt/commits/main)

Human-readable duration formatting and parsing

## Installation

```toml
[dependencies]
philiprehberger-duration-fmt = "0.2.2"
```

## Usage

```rust
use philiprehberger_duration_fmt::{format_duration, parse_duration};
use std::time::Duration;

// Format a duration
let d = Duration::from_secs(9015);
assert_eq!(format_duration(d), "2h 30m 15s");

// Parse a duration string
let d = parse_duration("2h 30m").unwrap();
assert_eq!(d, Duration::from_secs(9000));

// Verbose formatting
use philiprehberger_duration_fmt::format_duration_verbose;
assert_eq!(format_duration_verbose(d), "2 hours, 30 minutes");

// Limit precision
use philiprehberger_duration_fmt::format_duration_precise;
let d = Duration::from_secs(90061);
assert_eq!(format_duration_precise(d, 2), "1d 1h");
```

### ISO 8601

```rust
use philiprehberger_duration_fmt::{format_duration_iso8601, parse_iso8601_duration};
use std::time::Duration;

let d = Duration::from_secs(9015);
assert_eq!(format_duration_iso8601(d), "PT2H30M15S");

let parsed = parse_iso8601_duration("PT2H30M15S").unwrap();
assert_eq!(parsed, d);
```

### Short format

```rust
use philiprehberger_duration_fmt::format_duration_short;
use std::time::Duration;

assert_eq!(format_duration_short(Duration::from_secs(9015)), "2h30m15s");
```

## API

| Function | Description |
|----------|-------------|
| `format_duration(d)` | Compact format: "2h 30m 15s" |
| `format_duration_verbose(d)` | Verbose format: "2 hours, 30 minutes, 15 seconds" |
| `format_duration_precise(d, n)` | Show only top N units |
| `parse_duration(s)` | Parse compact: "2h30m", "500ms" |
| `parse_duration_verbose(s)` | Parse verbose: "2 hours 30 minutes" |
| `format_duration_iso8601(d)` | ISO 8601 format: "PT2H30M15S" |
| `parse_iso8601_duration(s)` | Parse ISO 8601 duration strings |
| `format_duration_short(d)` | Abbreviated format: "2h30m15s" |

## Development

```bash
cargo test
cargo clippy -- -D warnings
```

## Support

If you find this project useful:

⭐ [Star the repo](https://github.com/philiprehberger/rs-duration-fmt)

🐛 [Report issues](https://github.com/philiprehberger/rs-duration-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Abug)

💡 [Suggest features](https://github.com/philiprehberger/rs-duration-fmt/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)

❤️ [Sponsor development](https://github.com/sponsors/philiprehberger)

🌐 [All Open Source Projects](https://philiprehberger.com/open-source-packages)

💻 [GitHub Profile](https://github.com/philiprehberger)

🔗 [LinkedIn Profile](https://www.linkedin.com/in/philiprehberger)

## License

[MIT](LICENSE)
