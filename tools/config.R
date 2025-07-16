# Note: Any variables prefixed with `.` are used for text
# replacement in the Makevars.in and Makevars.win.in

# Check the packages MSRV first
source("tools/msrv.R")

# Check DEBUG and NOT_CRAN environment variables
env_debug <- Sys.getenv("DEBUG")
env_not_cran <- Sys.getenv("NOT_CRAN")

# Check if the vendored zip file exists
vendor_exists <- file.exists("src/rcounter/vendor.tar.xz")

is_not_cran <- env_not_cran != ""
is_debug <- env_debug != ""

if (is_debug) {
  # If we have DEBUG then we set not cran to TRUE
  # CRAN is always a release build
  is_not_cran <- TRUE
  message("Creating DEBUG build.")
}

if (!is_not_cran) {
  message("Building for CRAN.")
}

# We set CRAN flags only if NOT_CRAN is empty and if
# the vendored crates are present.
.cran_flags <- ifelse(
  vendor_exists,
  "-j 2 --offline",  # Enable offline builds if vendored crates exist
  ""
)

# When DEBUG environment variable is present we use `--debug` build
.profile <- ifelse(is_debug, "", "--release")
.clean_targets <- ifelse(is_debug, "", "$(TARGET_DIR)")

# We specify this target when building for webR
webr_target <- "wasm32-unknown-emscripten"

# Here we check if the platform we are building for is webR
is_wasm <- identical(R.version$platform, webr_target)

# Print to terminal to inform we are building for webR
if (is_wasm) {
  message("Building for WebR")
}

# We check if we are making a debug build or not
# If so, the LIBDIR environment variable becomes:
# LIBDIR = $(TARGET_DIR)/{wasm32-unknown-emscripten}/debug
# This will be used to fill out the LIBDIR env var for Makevars.in
target_libpath <- if (is_wasm) "wasm32-unknown-emscripten" else NULL
cfg <- if (is_debug) "debug" else "release"

# Used to replace @LIBDIR@
.libdir <- paste(c(target_libpath, cfg), collapse = "/")

# Use this to replace @TARGET@
# We specify the target _only_ on webR
# There may be use cases later where this can be adapted or expanded
.target <- ifelse(is_wasm, paste0("--target=", webr_target), "")

# Read in the Makevars.in file checking
is_windows <- .Platform[["OS.type"]] == "windows"

# If Windows, we replace in the Makevars.win.in
mv_fp <- ifelse(
  is_windows,
  "src/Makevars.win.in",
  "src/Makevars.in"
)

# Set the output file
mv_ofp <- ifelse(
  is_windows,
  "src/Makevars.win",
  "src/Makevars"
)

# Delete the existing Makevars{.win}
if (file.exists(mv_ofp)) {
  message("Cleaning previous `", mv_ofp, "`.")
  invisible(file.remove(mv_ofp))
}

# Read as a single string
mv_txt <- readLines(mv_fp)

# Replace placeholder values
new_txt <- gsub("@CRAN_FLAGS@", .cran_flags, mv_txt) |>
  gsub("@PROFILE@", .profile, x = _) |>
  gsub("@CLEAN_TARGET@", .clean_targets, x = _) |>
  gsub("@LIBDIR@", .libdir, x = _) |>
  gsub("@TARGET@", .target, x = _)

message("Writing `", mv_ofp, "`.")
con <- file(mv_ofp, open = "wb")
writeLines(new_txt, con, sep = "\n")
close(con)

message("`tools/config.R` has finished.")
