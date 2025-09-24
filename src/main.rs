use std::{
    io::Write,
    path::{Path, PathBuf},
};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: PathBuf,

    /// Number of times to greet
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let output_path = match args.output {
        None => get_output_file_from_input(args.input.as_path(), "pdf").expect("Couldn't generate an output path from the input path. Please, specify the output using --output option."),
        Some(v) => v,
    };

    let typst_path = get_output_file_from_input(args.input.as_path(), "typ")
        .unwrap_or(PathBuf::from("temp.typ"));

    // create intermediate typst file
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(&typst_path)
        .expect("Couldn't open the output file");
    write!(&mut f, "{}", generate_typst_file_contents(&args.input));

    // compile this typst file
    std::process::Command::new("typst")
        .arg("compile")
        .arg(typst_path.to_str().unwrap())
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Couldn't compile your markdown file!");

    // remove typst file
    std::fs::remove_file(&typst_path).expect("Couldn't remove intermediate file!");
}

#[derive(Debug)]
enum GenOutputNameError {
    ExtractFileName,
}

/// Generates content of the intermediate typst file
fn generate_typst_file_contents(md_file_path: &Path) -> String {
    format!(
        "#import \"@preview/cmarker:0.1.6\"\n
\n
#cmarker.render(read(\"{}\"))",
        md_file_path.to_str().unwrap()
    )
}

/// Takes a path to the input file and generates the name for the output file
fn get_output_file_from_input(input: &Path, file_ext: &str) -> Result<PathBuf, GenOutputNameError> {
    let name_os = input
        .file_stem()
        .ok_or(GenOutputNameError::ExtractFileName)?;

    let name_os = name_os.to_owned();
    let f_name = name_os
        .to_str()
        .ok_or(GenOutputNameError::ExtractFileName)?;

    let new_name = format!("{}.{}", f_name, file_ext);
    Ok(PathBuf::from(new_name))
}
