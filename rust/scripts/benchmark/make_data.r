set.seed(333)
X <- data.table::data.table(
    set = sample(1e5, 1e7, TRUE),
    element = sample(2e3, 1e7, TRUE),
    key = c("set", "element")
)
data.table::fwrite(X, "scripts/benchmark/data.csv")
