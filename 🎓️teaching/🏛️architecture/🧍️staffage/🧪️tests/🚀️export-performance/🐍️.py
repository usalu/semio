"""🚀 Independent oracle of the exact Euclidean distance transform used by the print-suitability measure: scipy `ndimage.distance_transform_edt` on explicit and seeded pseudo-random images.

`--widths FILE` prints the shapely width (opening, the criterion of the measure) of every feature-width shape of a JSON array. The fixture key `distance` holds, per case, the image (`#` is a source cell), the squared distance of every cell to the nearest source computed by scipy (rounded to integers, exact for squared pixel distances) and
`null` rows for an image without a source (the transform reports 1e20 there). Run `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`.

@see ../../🔨️modules/🖨️representation/📐️measure/🟦️.ts — squaredDistanceTransform
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.ndimage.distance_transform_edt.html
"""
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy.ndimage import distance_transform_edt
from shapely.geometry import Polygon

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🚀️export-performance" / "🔣️.json"


def seeded(seed, nx, ny, density):
    state = seed
    rows = []
    for _ in range(ny):
        row = ""
        for _ in range(nx):
            state = (state * 1103515245 + 12345) % 2147483648
            row += "#" if (state >> 8) % 1000 < density else "."
        rows.append(row)
    return rows


IMAGES = {
    "single-source": ["........", "........", "...#....", "........", "........"],
    "two-sources": ["#.......", "........", "........", "........", ".......#"],
    "row-and-column": ["...#....", "...#....", "########", "...#....", "...#...."],
    "diagonal": ["#.......", ".#......", "..#.....", "...#....", "....#...", ".....#..", "......#.", ".......#"],
    "corner": [".......", ".......", ".......", ".......", "......#"],
    "no-source": ["....", "....", "...."],
    "sparse-seeded-31x23": seeded(7, 31, 23, 15),
    "dense-seeded-17x29": seeded(11, 17, 29, 450),
    "single-column-seeded": seeded(5, 1, 20, 100),
    "single-row-seeded": seeded(3, 25, 1, 150),
}


def squared(rows):
    source = np.array([[c == "#" for c in row] for row in rows])
    if not source.any():
        return None
    distance = distance_transform_edt(~source)
    return np.rint(distance * distance).astype(int).tolist()


def polygon(rings):
    return Polygon([(p["x"], p["y"]) for p in rings[0]], [[(p["x"], p["y"]) for p in ring] for ring in rings[1:]])


def erosion(shape):
    low, high = 0.0, max(shape.bounds[2] - shape.bounds[0], shape.bounds[3] - shape.bounds[1])
    for _ in range(40):
        mid = (low + high) / 2
        if shape.buffer(-mid / 2, join_style="mitre").is_empty:
            high = mid
        else:
            low = mid
    return low


def opening(shape, min_feature):
    tolerance = max(0.01 * shape.area, (min_feature / 2) ** 2)

    def thin(width):
        opened = shape.buffer(-width / 2, join_style="round").buffer(width / 2, join_style="round")
        return shape.difference(opened).area > tolerance + 4 * (1 - math.pi / 4) * (width / 2) ** 2

    low, high = 0.0, erosion(shape)
    if not thin(high):
        return high
    for _ in range(40):
        mid = (low + high) / 2
        if thin(mid):
            high = mid
        else:
            low = mid
    return high


def widths(path):
    shapes = json.loads(Path(path).read_text(encoding="utf-8"))
    return {s["name"]: opening(polygon(s["rings"]), s["minFeatureMm"]) for s in shapes}


def main():
    if len(sys.argv) == 3 and sys.argv[1] == "--widths":
        print(json.dumps(widths(sys.argv[2])))
        return
    cases = [{"name": name, "image": rows, "squared": squared(rows)} for name, rows in IMAGES.items()]
    if "--write" in sys.argv:
        fixture = json.loads(FIXTURE.read_text(encoding="utf-8")) if FIXTURE.exists() else {}
        fixture["distance"] = cases
        FIXTURE.parent.mkdir(parents=True, exist_ok=True)
        FIXTURE.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"{len(cases)} distance cases")


if __name__ == "__main__":
    main()
