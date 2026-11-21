# /// script
# requires-python = ">=3.11"
# dependencies = ["pyproj", "shapely"]
# ///
"""📐️ Check projected polar overlay geometry against PROJ and GEOS."""

import json
import math
from pathlib import Path

from pyproj import Proj
from shapely.geometry import LineString, Point, box

fixture = json.loads(Path(__file__).with_name("🧊️polar.json").read_text(encoding="utf-8"))
west, south, east, north = fixture["bounds"]
lon, lat = fixture["origin"]
raster = box(west, south, east, north)
projection = Proj(proj="eqc", lat_ts=lat, lat_0=lat, lon_0=lon, R=111_320 * 180 / math.pi)


def project(coordinate):
    longitude, latitude = coordinate
    column = (longitude - west) / (east - west) * 32
    row = (latitude - south) / (north - south) * 32
    height = fixture["heightAtSouthWest"] + column * fixture["heightStepEast"] + row * fixture["heightStepNorth"]
    return (*projection(longitude, latitude), height * fixture["exaggeration"])


def near(first, second):
    assert all(math.isclose(a, b, rel_tol=0, abs_tol=1e-7) for a, b in zip(first, second, strict=True)), (first, second)


overlays, expected = fixture["overlays"], fixture["expected"]
visible = [marker for marker in overlays["positions"] if raster.covers(Point(marker["coordinates"]))]
assert [marker["id"] for marker in visible] == [marker["id"] for marker in expected["positions"]]
for marker, desired in zip(visible, expected["positions"], strict=True):
    near(project(marker["coordinates"]), desired["point"])
clipped = raster.intersection(LineString(overlays["routes"][0]["coordinates"]))
for actual, desired in zip(clipped.coords, expected["route"], strict=True):
    near(project(actual), desired)
near(project(overlays["regions"][0]["rings"][0][0]), expected["regionStart"])
