# Changelog

## 0.2.1 (2026-03-22)

- Fix CHANGELOG formatting

## 0.2.0 (2026-03-21)

- Add format_duration_iso8601() for ISO 8601 duration formatting (PT2H30M15S)
- Add parse_iso8601_duration() for parsing ISO 8601 duration strings
- Add format_duration_short() for abbreviated format (2h30m15s)
- Add #[must_use] attributes on all public functions

## 0.1.6 (2026-03-17)

- Add readme, rust-version, documentation to Cargo.toml
- Add Development section to README

## 0.1.5 (2026-03-16)

- Update install snippet to use full version

## 0.1.4 (2026-03-16)

- Add README badges
- Synchronize version across Cargo.toml, README, and CHANGELOG

## 0.1.0 (2026-03-15)

- Initial release
- `format_duration()` for compact formatting (e.g., "2h 30m 15s")
- `format_duration_verbose()` for verbose formatting (e.g., "2 hours, 30 minutes")
- `format_duration_precise()` with configurable unit count
- `parse_duration()` and `parse_duration_verbose()` for parsing
- Support for days, hours, minutes, seconds, and milliseconds
