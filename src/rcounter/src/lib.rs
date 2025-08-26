use extendr_api::prelude::*;
use std::process::Command;

#[extendr]
fn guidecounter_count(
    input: Vec<String>,
    library: String,
    offset_min_fraction: f64,
    output: String,
) -> RobjResult<String> {
    let mut command = Command::new("guide-counter");
    command.arg("count");

    for input_path in input {
        command.arg("--input").arg(input_path);
    }

    command
        .arg("--offset-min-fraction")
        .arg(offset_min_fraction.to_string())
        .arg("--library")
        .arg(library)
        .arg("--output")
        .arg(output);

    match command.output() {
        Ok(out) => {
            if out.status.success() {
                Ok("Demux operation completed successfully.".to_string())
            } else {
                let code = out.status.code().unwrap_or(-1);
                let err_msg = String::from_utf8_lossy(&out.stderr).to_string();
                Err(Error::from(format!(
                    "Demux failed (exit code {}): {}",
                    code, err_msg
                )))
            }
        }
        Err(e) => Err(Error::from(format!("Failed to execute command: {e}"))),
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}
