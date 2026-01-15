#' Run guide-counter count via the internal Rust wrapper
#'
#' @inheritParams guidecounter_count_internal
#' @param verbose Logical; emit progress messages from guide-counter.
#' @export
guidecounter_count <- function(input, library, offset_min_fraction, output, exact_match, verbose = TRUE) {
  call_fun <- function() {
    guidecounter_count_internal(
      input = path.expand(input),
      library = path.expand(library),
      offset_min_fraction = offset_min_fraction,
      output = path.expand(output),
      exact_match = exact_match
    )
  }

  status <- if (isTRUE(verbose)) call_fun() else suppressMessages(call_fun())
  if (status != 0L) {
    stop(paste("guide-counter count failed with status", status))
  }
}
