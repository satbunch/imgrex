mod cli;
mod scanner;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    println!("スキャン対象ルート: {}", args.input_root);
    println!("使用パターン: {}", args.dir_pattern);

    let target_dirs =
        scanner::find_target_dirs(std::path::Path::new(&args.input_root), &args.dir_pattern)?;

    println!("マッチしたディレクトリ一覧:");
    for dir in target_dirs {
        println!(" - {}", dir.display());
    }

    Ok(())
}
