"""🐍️ Third-party oracle adapter for `s.gis` tiled-map Web-Mercator projection and slippy-tile selection.

Re-derives every vector in the sibling `🧫️fixtures/🔣️vectors.json` directly from `mercantile`
(pure Python, zero runtime dependencies — a test-only dependency added to `pyproject.toml`'s `test`
group) and asserts agreement with the frozen fixture. This is the third-party half of the
differential pair; the Rust half lives at
`../../📦️packages/🦀️rust/tests/🗺️tiled_map_mercator_oracle.rs`, which reads the SAME fixture and
asserts this repository's own implementation agrees with it.

Run directly: `.venv/bin/python3 "🐍️.py"` (from this directory) or via `uv run python "🐍️.py"`
from the repo root. Exits non-zero on any disagreement.

@see ../🥒️.feature
@see ../🧫️fixtures/🔣️vectors.json
"""

from __future__ import annotations

import json
import math
import sys
from pathlib import Path

import mercantile

RE = 6378137.0  # WGS84 semi-major axis — the sphere radius mercantile's spherical mercator uses
WORLD_TOL = 1e-9
DEG_TOL = 1e-6


def world_xy(lon: float, lat: float) -> tuple[float, float]:
    """📐️ mercantile's EPSG:3857 meters, normalized by (R*pi) to match `WORLD_HALF = 1`."""
    lon_c = max(-180.0, min(180.0, lon))
    lat_c = max(-85.05112878, min(85.05112878, lat))
    x_m, y_m = mercantile.xy(lon_c, lat_c)
    return x_m / (RE * math.pi), y_m / (RE * math.pi)


def check_projection(fixture: dict, failures: list[str]) -> None:
    for entry in fixture["projection"]:
        x, y = world_xy(entry["lon"], entry["lat"])
        if abs(x - entry["worldX"]) >= WORLD_TOL or abs(y - entry["worldY"]) >= WORLD_TOL:
            failures.append(f"projection[{entry['id']}]: got ({x},{y}) want ({entry['worldX']},{entry['worldY']})")


def check_tile_numbering(fixture: dict, failures: list[str]) -> None:
    for entry in fixture["tileNumbering"]:
        t = mercantile.tile(entry["lon"], entry["lat"], entry["z"])
        if (t.x, t.y) != (entry["x"], entry["y"]):
            failures.append(f"tileNumbering[{entry['id']}]: got ({t.x},{t.y}) want ({entry['x']},{entry['y']})")


def check_tile_bounds(fixture: dict, failures: list[str]) -> None:
    for entry in fixture["tileBounds"]:
        b = mercantile.bounds(entry["x"], entry["y"], entry["z"])
        for field, got in (("west", b.west), ("south", b.south), ("east", b.east), ("north", b.north)):
            if abs(got - entry[field]) >= DEG_TOL:
                failures.append(f"tileBounds[{entry['id']}].{field}: got {got} want {entry[field]}")


def check_lod_bands(fixture: dict, failures: list[str]) -> None:
    """🌐️ Specification-vector check only — GIS_MAP_LOD_MAX_SPAN_DEG/TILE_Z are repository-owned,
    no third-party reference exists for them. This re-derives from the SAME constants recorded in
    `fixture["lodConstants"]`, which were transcribed by hand from
    `🦀️.rs:84,90` — a reviewer must re-check that transcription against the source, since
    nothing here can independently verify it."""
    max_span = fixture["lodConstants"]["maxSpanDeg"]
    tile_z = fixture["lodConstants"]["tileZ"]
    for entry in fixture["lodBands"]:
        idx = next((i for i, threshold in enumerate(max_span) if entry["spanDeg"] > threshold), len(max_span) - 1)
        if idx != entry["lodIndex"] or tile_z[idx] != entry["tileZ"]:
            failures.append(f"lodBands[spanDeg={entry['spanDeg']}]: got (idx={idx}, z={tile_z[idx]}) want (idx={entry['lodIndex']}, z={entry['tileZ']})")


def main() -> int:
    fixture_path = Path(__file__).resolve().parent / "🧫️fixtures" / "🔣️vectors.json"
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    check_projection(fixture, failures)
    check_tile_numbering(fixture, failures)
    check_tile_bounds(fixture, failures)
    check_lod_bands(fixture, failures)
    if failures:
        print(f"FAIL ({len(failures)} disagreement(s) between mercantile and the frozen fixture):", file=sys.stderr)
        for line in failures:
            print(f"  - {line}", file=sys.stderr)
        return 1
    total = len(fixture["projection"]) + len(fixture["tileNumbering"]) + len(fixture["tileBounds"]) + len(fixture["lodBands"])
    print(f"PASS: mercantile {mercantile.__version__} agrees with all {total} frozen fixture vectors")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
