use extendr_api::prelude::*;
use std::process::{Command};

/// Function to call the fqtk CLI command for demux.
/// Exposes the demux functionality as a Rust function that can be called from R.
/// @export
#[extendr]
fn guidecounter_count(
    input: Vec<String>,               // List of input files (as strings, R will pass these as character vectors)
    library: String,                   // Path to the library (as a string, R will pass this as a character vector)
    offset_min_fraction: f64,          // Offset minimum fraction
    output: String                     // Output directory
) -> String {
    // Convert ininputputs and library to PathBuf

    // Build the `fqtk demux` command using the provided parameters
    let mut command = Command::new("guide-counter");
    command.arg("count");

    // Add inputs (path to input files)
    for input_path in input {
                command.arg("--input").arg(input_path);
    }

    // Add other parameters
    command.arg("--offset-min-fraction").arg(offset_min_fraction.to_string())
            .arg("--library").arg(library)
            .arg("--output").arg(output);

    // Execute the command
    match command.output() {
        Ok(output) => {
            if output.status.success() {
                "Demux operation completed successfully.".to_string()
            } else {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                format!("Demux failed: {}", err_msg)
            }
        }
        Err(e) => format!("Failed to execute command: {}", e),
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}