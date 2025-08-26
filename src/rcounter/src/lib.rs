use extendr_api::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;


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
) -> Result<String> {
    let mut command = Command::new("guide-counter");
    command.arg("count");

    for input_path in input {
                command.arg("--input").arg(input_path);
    }

    command.arg("--offset-min-fraction").arg(offset_min_fraction.to_string())
            .arg("--library").arg(library)
            .arg("--output").arg(output);

    // Enable streaming of stdout/stderr
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let mut child = command.spawn()
    .map_err(|e| Error::from(format!("Failed to execute 'guide-counter': {e}")))?;

    // stdout thread
    let stdout = child.stdout.take();
    let stdout_handle = thread::spawn(move || {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines() {
                if let Ok(l) = line {
                    rprintln!("{}", l);
                }
            }
        }
    });

    // stderr thread
    let stderr = child.stderr.take();
    let stderr_handle = thread::spawn(move || {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines() {
                if let Ok(l) = line {
                    rprintln!("stderr: {}", l);
                }
            }
        }
    });

    // wait for process
    let status = child.wait()
        .map_err(|e| Error::from(format!("Failed to wait for 'guide-counter': {e}")))?;

    // wait for log threads to finish
    let _ = stdout_handle.join();
    let _ = stderr_handle.join();

    if status.success() {
        Ok("guide-counter completed successfully.".to_string())
    } else {
        Err(Error::from(format!("guide-counter failed: {:?}", status.code())))
    }
}

extendr_module! {
    mod guideCounterWrapper;
    fn guidecounter_count;
}