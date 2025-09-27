# md compiler

A compiler for MarkDown files to PDF written in Rust.

## Requirements
1. `typst` cli on the system
2. `cargo` and `rust`

## Installation
1. Clone the repository
```sh
git clone https://github.com/DobbiKov/md-compiler.git
```

2. Enter to the directory
```sh
cd md-compiler
```

3. Install the CLI globally
```sh
cargo install --path .
```

## Usage
The simplest usage:
```sh
md-compiler --input <your_input_md_file>
```

If you want to specify the output file:
```sh
md-compiler --input <your_input_md_file> --output <your_output_pdf_file>
```

## Idea
We create a document that uses
[cmarker](https://typst.app/universe/package/cmarker/) typst extension to use
markdown document in the typst ones. Then we compile such a typst document.

It maybe inefficient but seemed easy and interesting for me.

## Future work
In the future, we could add a command `--style` that would use different styles
(LaTeX, custom ones) to use typst to style documents differently.
