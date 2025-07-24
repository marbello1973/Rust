mod parser;
mod test_generator;

use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "React Test Generator")]
#[command(about = "Genera pruebas unitarias automaticamente para proyectos React")]

struct Cli {
    #[arg(short, long)]
    project_path: String,

    #[arg(short, long, default_value = "./tests")]
    output_dir: String,
}

fn main() {
    
    let args = Cli::parse();
    let project_path = Path::new(&args.project_path);
    let output_dir = Path::new(&args.output_dir);

    if !output_dir.exists(){
        fs::create_dir_all(output_dir).expect("Unable to create output directory");
    }

    for entry in fs::read_dir(project_path).expect("Unable to read project directory"){
        let entry = entry.expect("unable to read directory entry");
        let path = entry.path();

        if path.extension().unwrap_or_default() == "jsx" {
            let file_content = fs::read_to_string(&path).expect("Unable to read file");
            let component_name = path.file_stem().unwrap().to_string_lossy();
            let props = parser::analyze_component(&file_content);
            test_generator::generate_test(&component_name, props, &args.output_dir);
        }
    }

    println!("Pruebas generadas en {}", args.output_dir);
}
