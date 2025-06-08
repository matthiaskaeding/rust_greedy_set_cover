library(data.table)
library(RcppGreedySetCover)

# Time data loading
start_load <- Sys.time()
df <- fread("scripts/benchmark/data.csv")
load_time <- Sys.time() - start_load

# Time algorithm execution
start_algo <- Sys.time()
res <- greedySetCover(df)
algo_time <- Sys.time() - start_algo

# Print results in Python-like format
des_len <- 100
header <- "-Results R"
cat(header, strrep("-", des_len - nchar(header)), "\n", sep = "")
cat("greedy\n")
cat(sprintf("Cover: %d sets\n", length(unique(res$set))))
cat(sprintf("Time:  %.1f seconds\n", as.numeric(algo_time)))
