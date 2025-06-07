from set_cover import greedy_set_cover

# Example sets
sets = {"A": [1, 2, 3], "B": [1, 2], "C": [2]}

# Try both algorithms
result_0 = greedy_set_cover(sets, 0)  # HashSet-based
result_1 = greedy_set_cover(sets, 1)  # BitVec-based

print("Algorithm 0 result:", result_0)
print("Algorithm 1 result:", result_1)
