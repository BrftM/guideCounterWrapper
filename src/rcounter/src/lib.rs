use extendr_api::prelude::*;
use std::process::Command;

/// Exposes the guidecounter_count functionality as a Rust function callable from R.
///
/// @param input A character vector of input file paths.
/// @param library A string specifying the library name or path to use.
/// @param offset_min_fraction A numeric value specifying the minimum offset threshold.
/// @param output_path A string specifying the output file path.
/// @return A character string on success; otherwise throws an R error.
/// @export
#[extendr]
fn guidecounter_count(
    input: Vec<String>,
    library: String,
    offset_min_fraction: f64,
    output_path: String,
) -> Result<String> {
    let mut command = Command::new("guide-counter");
    command.arg("count");

    for input_path in input {
        command.arg("--input").arg(input_path);
    }

    command
        .arg("--offset-min-fraction").arg(offset_min_fraction.to_string())
        .arg("--library").arg(library)
        .arg("--output").arg(output_path);

    match command.output() {
        Ok(out) => {
            if out.status.success() {
                Ok("Demux operation completed successfully.".to_string())
            } else {
                let code = out.status.code().unwrap_or(-1);
                let err_msg = String::from_utf8_lossy(&out.stderr).to_string();
                Err(Error::from(format!(
                    "guide-counter failed (exit code {}): {}",
                    code, err_msg
                )))
            }
        }
        Err(e) => Err(Error::from(format!("Failed to execute 'guide-counter': {e}"))),
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}
