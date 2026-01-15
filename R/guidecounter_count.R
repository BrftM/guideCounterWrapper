#' Run guide-counter count via the internal Rust wrapper
#'
#' @inheritParams guidecounter_count_internal
#'
#' @export
guidecounter_count <- function(input, library, offset_min_fraction, output, exact_match) {
  status <- guidecounter_count_internal(
    input = path.expand(input),
    library = path.expand(library),
    offset_min_fraction = offset_min_fraction,
    output = path.expand(output),
    exact_match = exact_match
  )

  if (status != 0) {
    stop(sprintf("guide-counter failed with status: %d", status))
  }
}
