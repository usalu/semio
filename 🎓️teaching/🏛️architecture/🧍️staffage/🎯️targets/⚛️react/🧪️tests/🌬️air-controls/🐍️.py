"""🧭️ Oracle of the wind dial: numpy places a wind (the bearing the air moves towards, 0° ahead (+x), 90° to the asset's left (+z), and its speed) on the dial of each planar viewpoint and solves the dial point back to the wind; it also names the 45° sector of the bearing.

The dial is the ground plane as the viewpoint shows it: paper x and paper up of the library's planar cameras, the axis along the line of sight foreshortened by `depth` with "away from the viewer" up. The radius is the square root of the speed over the rim speed.
Run with `--write` to regenerate `🧫️fixtures/🧭️wind-dial/🔣️.json`; without it the script checks the fixture and exits non-zero on a difference.

@see ./🟦️.tsx — the TypeScript test that reads the same fixture
"""
import json
import sys
from pathlib import Path

import numpy as np

FIXTURE = Path(__file__).resolve().parents[4] / "🧫️fixtures" / "🧭️wind-dial" / "🔣️.json"
DEPTH = 0.42
RIM_MS = 20.0
SECTORS = ["ahead", "aheadLeft", "left", "behindLeft", "behind", "behindRight", "right", "aheadRight"]
# columns: where the unit vectors ahead (+x) and left (+z) land on the dial (x right, y down)
BASES = {
    "elevation": np.array([[1.0, 0.0], [0.0, -DEPTH]]),  # seen from the right, looking along +z: left is away, drawn up
    "front": np.array([[0.0, 1.0], [DEPTH, 0.0]]),  # seen from ahead, looking along -x: ahead is towards the viewer, drawn down; left is paper x
    "plan": np.array([[1.0, 0.0], [0.0, -1.0]]),  # seen from above: ahead is paper x, left is paper up
}
WINDS = [(0, 1.0), (90, 1.5), (180, 0.2), (270, 5.0), (45, 20.0), (135, 0.5), (200, 3.2), (315, 12.0), (359, 7.3), (10, 0.1)]


def cases():
    for view, basis in BASES.items():
        for bearing, speed in WINDS:
            phi = np.deg2rad(bearing)
            ground = np.sqrt(speed / RIM_MS) * np.array([np.cos(phi), np.sin(phi)])
            point = basis @ ground
            back = np.linalg.solve(basis, point)
            radius = float(np.hypot(*back))
            yield {
                "view": view,
                "bearingDeg": bearing,
                "speedMS": speed,
                "point": [round(float(point[0]), 12), round(float(point[1]), 12)],
                "solvedBearingDeg": int(np.rint(np.rad2deg(np.arctan2(back[1], back[0])))) % 360,
                "solvedSpeedMS": round(radius * radius * RIM_MS, 1),
                "sector": SECTORS[int(np.floor(bearing / 45 + 0.5)) % 8],
            }


def main() -> int:
    document = {"depth": DEPTH, "rimMS": RIM_MS, "sectors": SECTORS, "cases": list(cases())}
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
        return 0
    stored = json.loads(FIXTURE.read_text(encoding="utf8"))
    if stored != document:
        print("wind dial fixture differs from the numpy oracle", file=sys.stderr)
        return 1
    print(f"wind dial fixture agrees with numpy: {len(document['cases'])} cases")
    return 0


if __name__ == "__main__":
    sys.exit(main())
