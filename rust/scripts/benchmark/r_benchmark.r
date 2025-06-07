df <- data.table::fread("scripts/benchmark/data.csv")
# Makes some data in R
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
res <- RcppGreedySetCover::greedySetCover(df)

print(tictoc::toc())
stopifnot(setequal(res$element, df$element))
n_sets <- length(unique((res$set)))
print(paste("Number of sets found:", n_sets))
