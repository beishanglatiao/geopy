"""
Zonal histogram tool: for each polygon, count integer pixel values within it.
Returns list[dict] directly from Rust core.
"""

from geopy._core import zonal_histogram_py, count_patches_py


def zonal_histogram(
    raster,
    layer,
    max_value_limit=None,
) -> list[dict]:
    """Compute zonal histogram for each feature.

    For each feature, counts how many pixels of each integer value
    fall within the feature boundary.

    Args:
        raster: Path to input raster or Raster object
        layer: Path to input vector or Layer object
        max_value_limit: Optional upper limit for pixel values to count

    Returns:
        list[dict]: Each row as {"ID": int, "VALUE_0": int, "VALUE_1": int, ...}
    """
    return zonal_histogram_py(raster, layer, max_value_limit)


def count_patches(raster) -> dict:
    """Count connected components per pixel value in a single-band integer raster.

    Args:
        raster: Path to input raster or Raster object

    Returns:
        dict: {pixel_value: number_of_patches}
    """
    return count_patches_py(raster)
