
use anyhow::{Context, Result};
use clap::Parser;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn do_hard_work() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn barra_progreso() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    barra_progreso();

    Ok(())
}


