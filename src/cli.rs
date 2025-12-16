use clap::{Parser, ValueEnum};

/// A fast, streaming log slicer for text and JSON logs.
#[derive(Parser, Debug, Clone)]
#[command(name = "log-slicer", version, about, long_about = None)]
pub struct Args {
    /// Input paths. If none provided, reads from STDIN.
    pub paths: Vec<std::path::PathBuf>,

    /// Input format.
    #[arg(long, value_enum, default_value_t = Input::Auto)]
    pub input: Input,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Output::Plain)]
    pub output: Output,

    /// Substring filter applied to the whole line (text) or to the chosen field (JSON).
    #[arg(long)]
    pub contains: Option<String>,

    /// Regex filter applied to the whole line (text) or to the chosen field (JSON).
    #[arg(short, long)]
    pub regex: Option<String>,

    /// For JSON logs, read this dotted path (e.g. "level" or "request.id").
    #[arg(long)]
    pub field: Option<String>,

    /// For JSON logs: require field equals this (string compare).
    #[arg(long)]
    pub equals: Option<String>,

    /// Keep only records with timestamp >= this (RFC3339).
    #[arg(long)]
    pub since: Option<String>,

    /// Keep only records with timestamp <= this (RFC3339).
    #[arg(long)]
    pub until: Option<String>,

    /// Dotted JSON path for timestamps used by --since/--until (default: "timestamp").
    #[arg(long, default_value = "timestamp")]
    pub time_field: String,

    /// Keep only the first N matching lines.
    #[arg(long)]
    pub head: Option<usize>,

    /// Keep only the last N matching lines (buffers matches in memory).
    #[arg(long)]
    pub tail: Option<usize>,

    /// Emit simple statistics to stderr (counts, and optionally top values if --field is set).
    #[arg(long)]
    pub stats: bool,

    /// For JSON output modes: select fields to output (comma-separated dotted paths).
    #[arg(long)]
    pub select: Option<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Input {
    Auto,
    Text,
    Json,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, ValueEnum)]
pub enum Output {
    /// Print the whole raw line
    Plain,
    /// Print extracted field or selected fields as JSON per line (NDJSON)
    Ndjson,
    /// Print extracted field only (string) per line
    Field,
}
