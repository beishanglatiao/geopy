"""
Vector spatial analysis tools: buffer, overlay, simplification, hull, dissolve, etc.
All functions operate on Layer objects or file paths.
"""

from geopy._core import (
    buffer_py,
    intersect_py,
    union_py,
    erase_py,
    sym_diff_py,
    simplify_py,
    convex_hull_py,
    concave_hull_py,
    dissolve_py,
    make_valid_py,
    reproject_vector_py,
    reproject_raster_py,
)


def buffer(layer, distance):
    """Buffer all geometries in a layer by a given distance.

    Args:
        layer: Path to input vector or Layer object
        distance: Buffer distance in CRS units

    Returns:
        Layer containing buffered polygons
    """
    return buffer_py(layer, distance)


def intersect(layer_a, layer_b):
    """Compute intersection of two polygon layers.

    For each polygon pair (A×B), returns the overlapping region.

    Args:
        layer_a: Path to first vector or Layer object
        layer_b: Path to second vector or Layer object

    Returns:
        Layer containing intersection polygons
    """
    return intersect_py(layer_a, layer_b)


def union(layer_a, layer_b):
    """Compute union of two polygon layers.

    Merges overlapping regions between polygon pairs.

    Args:
        layer_a: Path to first vector or Layer object
        layer_b: Path to second vector or Layer object

    Returns:
        Layer containing union polygons
    """
    return union_py(layer_a, layer_b)


def erase(layer_a, layer_b):
    """Erase (clip out) parts of layer_a covered by layer_b.

    Args:
        layer_a: Path to base vector or Layer object
        layer_b: Path to erase vector or Layer object

    Returns:
        Layer containing layer_a minus areas covered by layer_b
    """
    return erase_py(layer_a, layer_b)


def sym_diff(layer_a, layer_b):
    """Compute symmetric difference of two polygon layers.

    Returns areas covered by exactly one of the two inputs (A xor B).

    Args:
        layer_a: Path to first vector or Layer object
        layer_b: Path to second vector or Layer object

    Returns:
        Layer containing symmetric difference polygons
    """
    return sym_diff_py(layer_a, layer_b)


def simplify(layer, tolerance):
    """Simplify geometries using Ramer-Douglas-Peucker algorithm.

    Args:
        layer: Path to input vector or Layer object
        tolerance: Simplification tolerance (larger = more simplification)

    Returns:
        Layer with simplified geometries
    """
    return simplify_py(layer, tolerance)


def convex_hull(layer):
    """Compute convex hull for each geometry in a layer.

    Args:
        layer: Path to input vector or Layer object

    Returns:
        Layer containing convex hull geometries
    """
    return convex_hull_py(layer)


def concave_hull(layer, max_edge_length):
    """Compute concave hull for each geometry in a layer.

    Args:
        layer: Path to input vector or Layer object
        max_edge_length: Maximum edge length in the resulting hull

    Returns:
        Layer containing concave hull geometries
    """
    return concave_hull_py(layer, max_edge_length)


def dissolve(layer):
    """Dissolve (unary union) all polygons in a layer into a single polygon.

    Args:
        layer: Path to input vector or Layer object

    Returns:
        Layer with dissolved (merged) polygons
    """
    return dissolve_py(layer)


def make_valid(layer):
    """Repair invalid geometries in a layer.

    Fixes self-intersections, ring orientation, and other geometry issues.

    Args:
        layer: Path to input vector or Layer object

    Returns:
        Layer with repaired geometries
    """
    return make_valid_py(layer)


def reproject_vector(layer, target_epsg):
    """Reproject a vector layer to a target CRS.

    Args:
        layer: Path to input vector or Layer object
        target_epsg: Target EPSG code (e.g. 4326 for WGS84)

    Returns:
        Reprojected Layer
    """
    return reproject_vector_py(layer, target_epsg)


def reproject_raster(raster, target_epsg):
    """Reproject a raster to a target CRS.

    Uses bilinear resampling by default.

    Args:
        raster: Path to input raster or Raster object
        target_epsg: Target EPSG code (e.g. 4326 for WGS84)

    Returns:
        Reprojected Raster
    """
    return reproject_raster_py(raster, target_epsg)
