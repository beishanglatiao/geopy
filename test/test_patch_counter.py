"""Test patch_counter: count connected components per pixel value."""

import time
from geopy import count_patches, Raster

# slope_int.tif is a single-band integer raster
raster_path = r"C:\Users\Huang\Documents\geopy\test\data\slope_int.tif"

r = Raster.read(raster_path)
print(f"Raster: {r.rows}x{r.cols}, bands={r.bands}, nodata={r.nodata}")
print(f"Data type: {r.data_type}")

t0 = time.time()
result = count_patches(r)
print(result)
elapsed = time.time() - t0
print(elapsed)
