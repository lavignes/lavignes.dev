use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    process::ExitCode,
};

use clap::Parser;
use jotdown::{html::Renderer, Render};

#[derive(Parser)]
struct Args {
    /// Input file path.
    source: PathBuf,

    /// Output file path. [default: stdout]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    if let Err(e) = main_real() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn wrap_io_err<Msg: AsRef<str>>(e: io::Error, msg: Msg) -> io::Error {
    let msg = msg.as_ref();
    io::Error::new(e.kind(), format!("{msg}: {e}"))
}

fn main_real() -> io::Result<()> {
    let args = Args::parse();

    let mut output: Box<dyn Write> = if let Some(path) = args.output {
        let path = path.as_path();
        Box::new(
            File::options()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .map_err(|e| {
                    wrap_io_err(
                        e,
                        format!("could not open file for writing \"{}\"", path.display()),
                    )
                })?,
        )
    } else {
        Box::new(io::stdout())
    };

    let source = args.source.as_path();
    let mut input = File::open(source)
        .map_err(|e| wrap_io_err(e, format!("could not open file \"{}\"", source.display())))?;

    let mut string = String::new();
    input
        .read_to_string(&mut string)
        .map_err(|e| wrap_io_err(e, format!("could not read file \"{}\"", source.display())))?;

    let events = jotdown::Parser::new(&string);

    let html = Renderer::default();
    html.write(events, &mut output)
        .map_err(|e| wrap_io_err(e, "could not write output"))
}
