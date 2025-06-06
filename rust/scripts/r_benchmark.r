# Makes some data in R
set.seed(333)
X <- data.table::data.table(
    set = sample(1e5, 1e7, TRUE),
    element = sample(2e3, 1e7, TRUE),
    key = c("set", "element")
)
data.table::fwrite(X, "data.csv")
# Input is in long format
# head(X)
#>    set element
#> 1:   1      12
#> 2:   1      19
#> 3:   1      32
#> 4:   1      45
#> 5:   1      51
#> 6:   1      62
# Run set cover
tictoc::tic()
res <- RcppGreedySetCover::greedySetCover(X)

print(tictoc::toc())
stopifnot(setequal(res$element, X$element))
n_sets <- length(unique((res$set)))
print(paste("Number of sets found:", n_sets))
