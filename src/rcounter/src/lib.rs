use extendr_api::prelude::*;
use std::process::{Command};

/// Exposes the guidecounter_count functionality as a Rust function callable from R.
/// 
/// @param input A character vector of input file paths.
/// @param library A string specifying the library name or path to use.
/// @param offset_min_fraction A numeric value specifying the minimum offset threshold.
/// @param output A string specifying the output file path.
/// @return A character string indicating success or error message.
/// @export
#[extendr]
fn guidecounter_count(
    input: Vec<String>,             
    library: String,                  
    offset_min_fraction: f64,          
    output: String                     
) -> String {
    let mut command = Command::new("guide-counter");
    command.arg("count");

    for input_path in input {
                command.arg("--input").arg(input_path);
    }

    command.arg("--offset-min-fraction").arg(offset_min_fraction.to_string())
            .arg("--library").arg(library)
            .arg("--output").arg(output);

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

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}