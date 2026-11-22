"""🌊️ Independent oracle of the flow geometry of a 3D model: builds a synthetic flow field (a warm rising stream, a straight ambient stream, a cool sinking arc and five puffs, far enough apart that no scale thins them), and derives from the documented rules alone what the model must carry at every scale: per stream the runs of equal temperature-excess bin, the stations each stream tube keeps (Ramer–Douglas–Peucker in numpy at 0.08 mm of paper), the radius of every station (half the paper width of the local speed, capped at 0.012 of the extent), the arrow heads, the vapour and fog puffs with their bins, radii and sphere level, and the volume and area of the unit icospheres from trimesh.

Run `UV_PROJECT_ENVIRONMENT=<machine cache>/staffage-oracles/geometry uv run --project 🧪️tests/🧰️support/🐍️oracles --locked --only-group geometry python -I 🐍️.py [--write]` (the `geometry` oracle set: numpy, trimesh). Without `--write` it compares the fixture's `expected` with its own result and exits non-zero on a difference.
"""
import json
import math
import sys
from pathlib import Path

import numpy as np
import trimesh

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🌊️flow-model" / "🔣️.json"
RULES = {"streamMinMm": 0.09, "streamMaxMm": 0.3, "headLengthMm": 1.1, "headWidthMm": 0.75, "puffMinMm": 0.3, "puffMaxMm": 14, "vapourThresholdGKg": 0.3, "bins": 9, "spacingMm": 2.2, "chordMm": 0.08, "tubeShare": 0.012, "vapourBins": 4, "fogBins": 3, "puffTriangles": [80, 320], "puffFineMm": 2, "sharedExcessK": 5, "sharedVapourGKg": 20, "sharedFogGKg": 2}
SCALES = [50, 200, 500]
EXTENT_M = 1.73


def r6(v):
    return round(float(v), 6)


def half_up(x):
    return int(math.floor(x + 0.5))


def stream(role, emitter, points, speed, excess, vapour):
    points = [[r6(c) for c in p] for p in points]
    speed = [r6(s) for s in speed]
    travel = [0.0]
    for i in range(1, len(points)):
        ds = float(np.linalg.norm(np.subtract(points[i], points[i - 1])))
        travel.append(travel[-1] + 2 * ds / (speed[i] + speed[i - 1]))
    as_vec = [{"x": p[0], "y": p[1], "z": p[2]} for p in points]
    return {"seed": as_vec[0], "role": role, "emitter": emitter, "points": as_vec, "travelS": [r6(t) for t in travel], "speedMS": speed, "excessK": [r6(e) for e in excess], "vapourExcessKgPerKg": [r6(v) for v in vapour], "condensateKgPerKg": [0.0] * len(points)}


def field():
    n = range(80)
    warm = stream("source", "body-plume", [[0.15 * math.sin(0.08 * i) * i / 79, 0.3 + 0.03 * i, 0.1 * (1 - math.cos(0.05 * i))] for i in n], [0.1 + 0.5 * math.exp(-i / 40) for i in n], [6.2 * math.exp(-i / 30) for i in n], [0.004 * math.exp(-i / 20) for i in n])
    m = range(81)
    ambient = stream("ambient", None, [[-2 + 0.05 * i, 1.0, 3.0] for i in m], [0.2] * 81, [0.0] * 81, [0.0] * 81)
    k = range(61)
    cool = stream("entrained", "body-plume", [[math.cos(i * math.pi / 120), 0.2 + math.sin(i * math.pi / 120), -3.0] for i in k], [0.05 + 0.002 * i for i in k], [-3.1 * i / 60 for i in k], [0.0] * 61)
    puff = lambda at, radius, excess, vapour, condensate: {"at": {"x": at[0], "y": at[1], "z": at[2]}, "radiusM": radius, "excessK": excess, "vapourExcessKgPerKg": vapour, "condensateKgPerKg": condensate, "carries": "vapour" if vapour > 0 else "airflow"}
    puffs = [puff([0.0, 3.2, 0.0], 0.25, 1.0, 0.012, 0.0), puff([1.6, 2.0, 0.0], 0.02, 3.0, 0.002, 0.0), puff([-1.7, 1.2, 1.5], 0.12, 0.5, 0.018, 0.0015), puff([2.0, 0.5, -1.5], 0.004, 0.2, 0.0001, 0.0002), puff([-2.0, 2.5, -1.5], 0.1, 4.0, 0.0, 0.0)]
    return {"timeS": 14, "streams": [warm, ambient, cool], "puffs": puffs, "paths": [], "obstacles": [], "ambient": {"airTempC": 20, "relativeHumidity": 0.5, "dewPointC": 9.3, "windMS": 0.2, "windDirectionDeg": 0, "pressurePa": 101325}, "model": {"stepM": 0.03, "periodS": 1.5, "exposureS": 0.4}}


def stations(points, start, end, tolerance):
    """Ramer–Douglas–Peucker over points[start..end]: the indices kept."""
    if end - start < 2:
        return [start, end]
    a, b = points[start], points[end]
    ab = b - a
    l2 = float(ab @ ab)
    inner = points[start + 1:end]
    t = np.clip(((inner - a) @ ab) / l2, 0, 1) if l2 > 0 else np.zeros(len(inner))
    d = np.linalg.norm(inner - (a + np.outer(t, ab)), axis=1)
    worst = int(np.argmax(d))
    if d[worst] <= tolerance:
        return [start, end]
    middle = start + 1 + worst
    return stations(points, start, middle, tolerance)[:-1] + stations(points, middle, end, tolerance)


def expected(flow):
    speed_max = max(0.05, max(s for st in flow["streams"] for s in st["speedMS"]))
    span = RULES["sharedExcessK"]
    bin_of = lambda k: half_up((RULES["bins"] - 1) * (min(span, max(-span, k)) + span) / (2 * span))
    width = lambda s: RULES["streamMinMm"] + (RULES["streamMaxMm"] - RULES["streamMinMm"]) * math.sqrt(min(1, max(0, s / speed_max)))
    out = []
    for scale in SCALES:
        mm = scale / 1000
        unit = min(mm, RULES["tubeShare"] * EXTENT_M / (RULES["streamMaxMm"] / 2))
        streams = []
        for index, st in enumerate(flow["streams"]):
            pts = np.array([[p["x"], p["y"], p["z"]] for p in st["points"]])
            bins = [bin_of(k) for k in st["excessK"]]
            tubes, start, i = [], 0, 1
            while i <= len(pts):
                if i < len(pts) and bins[i] == bins[start]:
                    i += 1
                    continue
                end = min(len(pts) - 1, i)
                if end > start:
                    kept = stations(pts, start, end, RULES["chordMm"] * mm)
                    tubes.append({"start": start, "bin": bins[start], "stations": kept, "radiiM": [width(st["speedMS"][j]) / 2 * unit for j in kept]})
                start = end
                if i >= len(pts) - 1:
                    break
                i += 1
            length = float(np.sum(np.linalg.norm(np.diff(pts, axis=0), axis=1)))
            count = max(1, min(4, int(math.floor(length / (12 * RULES["spacingMm"] * mm)))))
            arrows = []
            for k in range(1, count + 1):
                j = min(len(pts) - 1, max(1, half_up(k * (len(pts) - 1) / (count + 1))))
                d = pts[j] - pts[j - 1]
                arrows.append({"index": j, "bin": bins[j], "direction": (d / np.linalg.norm(d)).tolist()})
            streams.append({"stream": index, "tubes": tubes, "arrows": arrows})
        radius = lambda p: min(RULES["puffMaxMm"] * mm, max(RULES["puffMinMm"] * mm, p["radiusM"]))
        triangles = lambda p: RULES["puffTriangles"][0 if radius(p) < RULES["puffFineMm"] * mm else 1]
        vapour = [{"puff": i, "bin": min(RULES["vapourBins"] - 1, int(RULES["vapourBins"] * p["vapourExcessKgPerKg"] * 1000 / RULES["sharedVapourGKg"])), "radiusM": radius(p), "triangles": triangles(p)} for i, p in enumerate(flow["puffs"]) if p["vapourExcessKgPerKg"] * 1000 > RULES["vapourThresholdGKg"]]
        fog = [{"puff": i, "bin": min(RULES["fogBins"] - 1, int(RULES["fogBins"] * p["condensateKgPerKg"] * 1000 / RULES["sharedFogGKg"])), "radiusM": radius(p), "triangles": triangles(p)} for i, p in enumerate(flow["puffs"]) if p["condensateKgPerKg"] > 0]
        out.append({"scale": scale, "unitMPerMm": unit, "headLengthM": RULES["headLengthMm"] * unit, "headRadiusM": RULES["headWidthMm"] / 2 * unit, "streams": streams, "vapour": vapour, "fog": fog})
    spheres = {}
    for level, count in zip([1, 2], RULES["puffTriangles"]):
        ball = trimesh.creation.icosphere(subdivisions=level, radius=1.0)
        assert len(ball.faces) == count and ball.is_watertight
        spheres[str(count)] = {"vertices": len(ball.vertices), "volume": float(ball.volume), "area": float(ball.area)}
    return {"scales": out, "unitSpheres": spheres}


def apart(flow):
    """The synthetic marks are farther apart than the coarsest spacing, so no scale thins them."""
    spacing = RULES["spacingMm"] * max(SCALES) / 1000
    centres = [np.array([p["at"]["x"], p["at"]["y"], p["at"]["z"]]) for p in flow["puffs"]]
    assert all(np.linalg.norm(a - b) > spacing for i, a in enumerate(centres) for b in centres[i + 1:])
    zs = [st["points"][0]["z"] for st in flow["streams"]]
    assert all(abs(a - b) > spacing + 0.5 for i, a in enumerate(zs) for b in zs[i + 1:])


def main():
    flow = field()
    apart(flow)
    want = expected(flow)
    if "--write" in sys.argv:
        document = {
            "id": "flow-model",
            "oracle": "numpy re-implementation of the documented rules (bins of the paper paint, Ramer-Douglas-Peucker stations, paper widths, arrow stations, puff classes) on a synthetic flow field; trimesh icosphere for the volume and area of the puff spheres; in the test three's GLTFLoader, the Khronos gltf-validator and three's Line3 for the GLB of the fixture assets",
            "frame": "a field point (x ahead, y up, z left) is (z, y, x) in the neutral frame of the model",
            "rules": RULES,
            "extentM": EXTENT_M,
            "theme": "light",
            "range": {"mode": "shared"},
            "tolerances": {"lengthM": 1e-9, "sphereRelative": 1e-9, "hugM": 1e-6},
            "assets": [
                {"id": "sample-figure-own-air", "fixture": "👤️sample-figure", "pose": "stand", "environment": None, "glbScales": [50, 200], "fog": False},
                {"id": "reference-walker-cold-humid", "fixture": "🚶️reference-walker", "pose": "stand", "environment": {"airTempC": -5, "meanRadiantTempC": -5, "skyTempC": -15, "groundTempC": -5, "relativeHumidity": 0.8, "windMS": 0.2, "windDirectionDeg": 180, "solarWM2": 0, "diffuseFraction": 0.2, "pressurePa": 101325}, "glbScales": [], "fog": True},
            ],
            "field": flow,
            "expected": want,
        }
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        print(f"wrote {FIXTURE}")
        return
    have = json.loads(FIXTURE.read_text(encoding="utf-8"))
    assert have["field"] == json.loads(json.dumps(flow)), "the fixture's field differs from the oracle's"
    assert json.loads(json.dumps(want)) == have["expected"], "the fixture's expected values differ from the oracle's"
    print("flow-model fixture matches the oracle")


main()
