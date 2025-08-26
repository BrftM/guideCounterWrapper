use extendr_api::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

/// Exposes the guidecounter_count functionality as a Rust function callable from R.
///
/// @param input A character vector of input file paths.
/// @param library A string specifying the library name or path to use.
/// @param offset_min_fraction A numeric value specifying the minimum offset threshold.
/// @param output_path A string specifying the output file path.
/// @return A character string indicating success; otherwise throws an R error.
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

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .map_err(|e| Error::from(format!("Failed to execute 'guide-counter': {e}")))?;

    // Spawn thread for stdout
    let mut stdout = child.stdout.take().unwrap();
    let stdout_handle = thread::spawn(move || {
        let reader = BufReader::new(&mut stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                // Immediately print to R console
                rprintln!("{}", l);
            }
        }
    });

    // Spawn thread for stderr
    let mut stderr = child.stderr.take().unwrap();
    let stderr_handle = thread::spawn(move || {
        let reader = BufReader::new(&mut stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                rprintln!("stderr: {}", l);
            }
        }
    });

    // Wait for process to exit
    let status = child
        .wait()
        .map_err(|e| Error::from(format!("Failed to wait for 'guide-counter': {e}")))?;

    // Join threads (ensure logs are flushed)
    let _ = stdout_handle.join();
    let _ = stderr_handle.join();

    if status.success() {
        Ok("guide-counter completed successfully.".to_string())
    } else {
        Err(Error::from(format!(
            "guide-counter failed with exit code {:?}",
            status.code()
        )))
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}
