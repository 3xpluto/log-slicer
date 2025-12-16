# log-slicer
[![CI](https://github.com/3xpluto/log-slicer/actions/workflows/ci.yml/badge.svg)](https://github.com/3xpluto/log-slicer/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

A fast, streaming CLI to **filter**, **search**, and **reshape** logs — works with plain text or NDJSON.
Think: `grep + jq`, but optimized for day-to-day debugging.

- Stream-friendly (stdin or files)
- Text + JSON (NDJSON) support
- Field-based filtering with dotted paths (`request.id`)
- Output modes: raw line, extracted field, or NDJSON with selected fields
- Optional stats (counts + top values)

---

## Install

### From source (recommended for now)
```bash
cargo install --path .
```

Or just run locally:
```bash
cargo run -- --help
```

---

## Quick start

### Read from stdin (text)
```bash
cat app.log | log-slicer --contains "ERROR"
```

### Read files (text)
```bash
log-slicer ./logs/app.log ./logs/worker.log --regex "(timeout|deadline)"
```

### JSON filtering (NDJSON)
```bash
log-slicer app.ndjson --input json --field level --equals error
log-slicer app.ndjson --input json --field request.id --regex "deadbeef"
```

### Print only a field value
```bash
log-slicer app.ndjson --input json --field message --output field
```

### Select fields and emit NDJSON
```bash
log-slicer app.ndjson --input json --select timestamp,level,message,request.id --output ndjson
```

### Head / Tail
```bash
log-slicer app.ndjson --input json --field level --equals error --head 10
log-slicer app.ndjson --input json --regex "timeout" --tail 50
```

---

## Usage

```bash
log-slicer --help
```

### Input modes

- `--input auto` (default): attempts JSON parse when a line looks like JSON (`{` or `[`), otherwise treats as text
- `--input text`: never parse JSON
- `--input json`: parse every line as JSON (NDJSON)

### Filtering options

Filters apply to:
- the **whole line** in text mode, OR
- the **selected field** if `--field` is provided (JSON mode)

Available filters:
- `--contains <SUBSTR>`
- `--regex <PATTERN>`
- `--field <DOTTED.PATH>` (JSON)
- `--equals <STRING>` (works for either whole-line target or selected field)

### Field paths (JSON)

Use dotted paths like:
- `level`
- `request.id`
- arrays are supported with numeric indices: `items.0.name`

### Output modes

- `--output plain` (default): prints the full raw line
- `--output field`: prints only the extracted field value (requires `--field`)
- `--output ndjson`:
  - if `--select` is set, emits an object containing only those fields
  - otherwise prints the original JSON line (if parsed), or wraps text as `{"line": "..."}`

#### `--select` details
`--select` takes comma-separated dotted paths. Keys in output are normalized by replacing `.` with `_`:

Example:
```bash
--select request.id,timestamp
```

becomes:
```json
{"request_id":"...","timestamp":"..."}
```

---

## Stats

Enable stats with:
```bash
--stats
```

- Stats are printed to **stderr**
- Matching lines still print to stdout (so the tool remains pipe-friendly)

Example:
```bash
cat examples/app.ndjson | log-slicer --input json --stats --field level
```

### PowerShell tip (stats only)
PowerShell shows stdout + stderr together. If you want **only stats**:

```powershell
cat examples/app.ndjson | cargo run -- --input json --stats --field level 1>$null
```

Or split them:
```powershell
cat examples/app.ndjson | cargo run -- --input json --stats --field level 1>out.ndjson 2>stats.txt
```

---

## Time slicing

Filter records by timestamp range (RFC3339):

- `--since <RFC3339>` keep records with timestamp >= since
- `--until <RFC3339>` keep records with timestamp <= until
- `--time-field <path>` dotted JSON path for timestamps (default: `timestamp`)

Example:
```bash
log-slicer app.ndjson --input json \
  --since 2025-12-15T09:00:01Z \
  --until 2025-12-15T09:00:02Z
```

If time slicing is enabled, records without a parseable timestamp are excluded.

---

## Examples

Using the included demo file:
```bash
cat examples/app.ndjson | log-slicer --input json --field level --equals error
cat examples/app.ndjson | log-slicer --input json --field message --output field
cat examples/app.ndjson | log-slicer --input json --select timestamp,level,message,request.id --output ndjson
cat examples/app.ndjson | log-slicer --input json --stats --field level
```

---

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

GitHub Actions CI runs fmt + clippy + tests on push/PR.

---

## Roadmap (nice upgrades)

- `--stats-only` / `--quiet` (don’t print matches)
- `--follow` tail -f mode (stream files as they grow)
- Better numeric comparisons: `--gt/--lt` for JSON numbers
- Output formats: `--output tsv/csv`
- Benchmarks + large-log performance notes

---

## License
MIT
