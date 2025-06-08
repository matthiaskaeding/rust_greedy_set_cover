import time
import polars as pl
from setcover import setcover


def verify_cover(sets, cover):
    """Verify that the cover actually covers all elements."""
    covered = set()
    for set_name in cover:
        covered.update(sets[set_name])

    universe = set()
    for elements in sets.values():
        universe.update(elements)

    return covered == universe


df = pl.read_csv("scripts/benchmark/data.csv")
assert df.height == int(1e7)
sets = {}
for s, df_ in df.group_by("set"):
    sets[s[0]] = df_.get_column("element").to_list()

des_len = 100
print("-Results python" + "-" * (des_len - len("-Results python")))

start = time.time()
res = setcover(sets, "greedy-standard")
end = time.time()

print("greedy-standard")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")
assert verify_cover(sets, res)
del res

start = time.time()
res = setcover(sets, "greedy-bitvec")
end = time.time()

assert verify_cover(sets, res)
print("greedy-bitvec")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")
