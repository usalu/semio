"""🧪️ Shared oracle helpers of the structural solver cases: python forward kinematics, segment masses, fixture IO.

Every case script builds its complete language-agnostic fixture (model, scenarios, oracle values), computes the oracle values with a
third-party library (anastruct, numpy) and either writes `🧫️fixtures/<case>/🔣️.json` (`--write`) or checks the stored file (`--check`, default).

Sign map from anastruct to the staffage conventions: reaction = -node result, M_staffage = -M_anastruct (anastruct hogging is positive),
V_staffage = -Q_anastruct, w_staffage = -uy_anastruct (anastruct y displacement is positive downward).
"""
import json
import math
import sys
from importlib.metadata import version
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RTOL = 1e-9


def fixture_path(case):
    return ROOT / "🧫️fixtures" / case / "🔣️.json"


def forward(doc, scenario=None):
    """🦴️ World origin and angle of every bone from the rest transforms plus the scenario overrides."""
    over = (scenario or {}).get("bones", {})
    out = {}
    for b in doc["bones"]:
        o = over.get(b["id"], {})
        x, y, a = o.get("x", b["x"]), o.get("y", b["y"]), o.get("angleDeg", b["angleDeg"])
        if b["parent"] is None:
            ang = math.radians(doc["root"]["angleDeg"])
            out[b["id"]] = (doc["root"]["x"], doc["root"]["y"], ang)
        else:
            px, py, pa = out[b["parent"]]
            out[b["id"]] = (px + math.cos(pa) * x - math.sin(pa) * y, py + math.sin(pa) * x + math.cos(pa) * y, pa + math.radians(a))
    return out


def point(doc, frame, bone, s, y=0.0):
    ox, oy, a = frame[bone]
    return (ox + math.cos(a) * s - math.sin(a) * y, oy + math.sin(a) * s + math.cos(a) * y)


def masses(doc, frame):
    """⚖️ One (bone, x, y, kg) per segment at its centre of mass; uniform masses sit at mid-length."""
    out = []
    for b in doc["bones"]:
        m = b.get("mass")
        if not m:
            continue
        s = b["lengthM"] / 2 if m["distribution"] == "uniform" else m["comX"]
        x, y = point(doc, frame, b["id"], s, m["comY"])
        out.append((b["id"], x, y, m["kg"]))
    return out


def run(case, build, compute):
    doc = build()
    values = compute(doc)
    doc["oracle"] = {"tool": values.pop("_tool"), "version": values.pop("_version"), "values": values}
    mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
    path = fixture_path(case)
    if mode == "--write":
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(doc, ensure_ascii=False, indent=1) + "\n", encoding="utf8")
        print("wrote", path)
        return
    stored = json.loads(path.read_text(encoding="utf8"))
    if not close(stored, doc):
        print("MISMATCH", case)
        sys.exit(1)
    print("OK", case, doc["oracle"]["tool"], doc["oracle"]["version"])


def close(a, b):
    if isinstance(a, dict):
        return a.keys() == b.keys() and all(close(a[k], b[k]) for k in a)
    if isinstance(a, list):
        return len(a) == len(b) and all(close(x, y) for x, y in zip(a, b))
    if isinstance(a, (int, float)) and not isinstance(a, bool):
        return abs(a - b) <= RTOL * max(1.0, abs(a), abs(b))
    return a == b


def anastruct_version():
    return version("anastruct")


def numpy_version():
    return version("numpy")
