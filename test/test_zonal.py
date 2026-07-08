"""Test new zonal_histogram API returning list[dict]."""
import polars as pl
from geopy import zonal_histogram, Raster, Layer

r = Raster.read(r"test\data\slope_int.tif")
l = Layer.read(r"test\data\县级边界.shp")
x =r"test\data\zonal_hist.csv"

import time

t0 = time.time()
result = zonal_histogram(r, l, max_value_limit=10)
df = pl.DataFrame(result)
df.write_csv(x)

elapsed = time.time() - t0

print(f"Elapsed time: {elapsed:.4f} seconds")