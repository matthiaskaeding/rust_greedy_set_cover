import polars as pl
from setcover import setcover
import time

df = pl.read_csv("scripts/benchmark/data.csv")
assert df.height == int(1e7)
sets = {}
for s, df_ in df.group_by("set"):
    sets[s[0]] = df_.get_column("element").to_list()


start = time.time()
res = setcover(sets, "greedy-0")
end = time.time()

des_len = 100
print("-Results python" + "-" * (des_len - len("-Results python")))
print("greedy-0")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")

start = time.time()
res = setcover(sets, "greedy-1")
end = time.time()

print("greedy-1")
print(f"Cover: {len(res)} sets")
print(f"Time:  {end - start:.1f} seconds")
