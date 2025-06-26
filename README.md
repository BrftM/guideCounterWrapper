# counter
R package wrapper around the command-line tool `guide-counter`, developed by Fulcrum Genomics.

This wrapper is based on the original `guide-counter` tool, which is developed by Tim Fennell and Nils Homer.

### Citation

If you use `guide-counter` or this wrapper in your work, please cite the original tool as follows:

- Tim Fennell, & Nils Homer. (2022). *fulcrumgenomics/guide-counter: v0.1.3 (v01.3)*. Zenodo. [https://doi.org/10.5281/zenodo.6375792](https://doi.org/10.5281/zenodo.6375792)

### Installation

You'll need to install Rust on your system if you don't have it already. Then you can install directly from GitHub using the `remotes` package in R.

```r
# Installation of remotes
if (!requireNamespace("remotes", quietly = TRUE)) {
  install.packages("remotes")
}

# Installation from GitHub in the same way as pepitope
remotes::install_github("BrftM/guidecounter.wrapper")
```
### Example Table (lib.tsv)

| oligo_id | barcode          | gene     |
|----------|------------------|----------|
| oligo_1  | AACAACAACACC     | oligo_1  |
| oligo_2  | AACAACAACGGT     | oligo_2  |
| oligo_3  | AACAACACAAGC     | oligo_3  |
| oligo_4  | AACAACACCTCA     | oligo_4  |
| oligo_5  | AACAACAGAGTC     | oligo_5  |
| oligo_6  | AACAACAGCCGA     | oligo_6  |
| oligo_7  | AACAACATGGAC     | oligo_7  |
| oligo_8  | AACAACATTCGC     | oligo_8  |
| oligo_9  | AACAACCACTGA     | oligo_9  |

---

Once the `guidecounter_count` function is exposed to R, you can call it directly within R:

```r
# Load the package
library(guidecounter.wrapper)

# Define the input parameters
input  <- c("C:/Users/.../input.fq.gz")
offset_min_fraction <- 0.2
lib = "lib.tsv"
output  <- "output_folder/"

guidecounter_count(
  input = input,
  library = lib,
  offset_min_fraction = offset_min_fraction,
  output = output
)
```
This README provides all the necessary instructions and examples for installing and using the `guidecounter.wrapper` package in R.