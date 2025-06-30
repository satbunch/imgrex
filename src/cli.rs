use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long)]
    pub config: Option<String>,

    #[arg(long, default_value = ".")]
    pub input_root: String,

    #[arg(long, default_value = "^[0-9]{4}$")]
    pub dir_pattern: String,

    #[arg(long)]
    pub mapping: Option<String>,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}
