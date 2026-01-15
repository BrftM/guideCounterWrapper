use extendr_api::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

struct Line {
    text: String,
}

fn spawn_reader<R: std::io::Read + Send + 'static>(reader: R, tx: mpsc::Sender<Line>) {
    thread::spawn(move || {
        let buf = BufReader::new(reader);
        for line in buf.lines().flatten() {
            let _ = tx.send(Line { text: line });
        }
    });
}

fn emit_message(line: &str) {
    let _ = extendr_api::call!("message", line);
}

/// Exposes the guidecounter_count_internal functionality as a Rust function callable from R.
/// 
/// @param input A character vector of input file paths.
/// @param library A string specifying the library name or path to use.
/// @param offset_min_fraction A numeric value specifying the minimum offset threshold.
/// @param output A string specifying the output file path.
/// @return A character string indicating success or error message.
#[extendr]
fn guidecounter_count_internal(
    input: Vec<String>,
    library: String,
    offset_min_fraction: f64,
    output: String,
    exact_match: bool
) -> i32 {
    let mut command = Command::new("guide-counter");
    command.arg("count");

    for input_path in input {
        command.arg("--input").arg(input_path);
    }

    command.arg("--offset-min-fraction").arg(offset_min_fraction.to_string())
           .arg("--library").arg(library)
           .arg("--output").arg(output);

    if exact_match {
        command.arg("--exact-match");
    }

    let mut child = match command.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(_e) => return 1,
    };

    let (tx, rx) = mpsc::channel();
    if let Some(stdout) = child.stdout.take() {
        spawn_reader(stdout, tx.clone());
    }
    if let Some(stderr) = child.stderr.take() {
        spawn_reader(stderr, tx.clone());
    }
    drop(tx);

    while let Ok(line) = rx.recv() {
        emit_message(&line.text);
    }

    match child.wait() {
        Ok(status) => status.code().unwrap_or(1),
        Err(_e) => 1,
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count_internal;
}
