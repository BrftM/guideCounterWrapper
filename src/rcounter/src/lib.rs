use extendr_api::prelude::*;
use std::process::{Command, Stdio};

/// Exposes the guidecounter_count_internal functionality as a Rust function callable from R.
/// 
/// @param input A character vector of input file paths.
/// @param library A string specifying the library name or path to use.
/// @param offset_min_fraction A numeric value specifying the minimum offset threshold.
/// @param output A string specifying the output file path.
/// @return A character string indicating success or error message.
/// @export
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

    command.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(_e) => 1,
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count_internal;
}
