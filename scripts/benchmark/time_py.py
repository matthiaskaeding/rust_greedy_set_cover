import polars as pl
from setcover import setcover
import time


def verify_cover(sets, solution):
    # Get all elements from original sets
    all_elements = set()
    for s in sets.values():
        all_elements.update(s)

    # Get all elements from solution sets
    covered_elements = set()
    for set_id in solution:
        covered_elements.update(sets[set_id])

    # Check if all elements are covered
    return all_elements.issubset(covered_elements)


df = pl.read_csv("scripts/benchmark/data.csv")
assert df.height == int(1e7)
sets = {}
for s, df_ in df.group_by("set"):
    sets[s[0]] = df_.get_column("element").to_list()

des_len = 100
print("-Results python" + "-" * (des_len - len("-Results python")))

start = time.time()
res = setcover(sets, "greedy-0")
end = time.time()

print("greedy-0")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")
assert verify_cover(sets, res)
del res

start = time.time()
res = setcover(sets, "greedy-1")
end = time.time()

assert verify_cover(sets, res)
print("greedy-1")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")
