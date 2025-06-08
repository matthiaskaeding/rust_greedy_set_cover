set.seed(333)

# Parse command line arguments
args <- commandArgs(trailingOnly = TRUE)

n_sets <- if (length(args) >= 1) as.numeric(args[1]) else 1e5
n_elements <- if (length(args) >= 2) as.numeric(args[2]) else 2e3
n_rows <- if (length(args) >= 3) as.numeric(args[3]) else 1e7

df <- data.table::data.table(
    set = sample(n_sets, n_rows, TRUE),
    element = sample(n_elements, n_rows, TRUE),
    key = c("set", "element")
)
data.table::fwrite(df, "scripts/benchmark/data.csv")
