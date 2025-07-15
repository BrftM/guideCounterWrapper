The package uses Rust via `{extendr}` and includes vendored crates in `src/rcounter/vendor.tar.xz` to ensure offline compilation.  
Some of the original vendored crates (e.g. `lz4-sys`) contain GNU-style Makefiles that are not used by R.  
The compressed archive ensures these files are excluded from R CMD check while preserving reproducibility.
