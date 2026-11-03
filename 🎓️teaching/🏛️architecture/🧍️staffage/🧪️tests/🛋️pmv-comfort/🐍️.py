"""🛋️ Independent oracle of the ISO 7730 comfort model: pythermalcomfort `pmv_ppd_iso` (model `7730-2005`, inputs unlimited so the extrapolation is compared too).

Each case lists the six ISO parameters; the library's `pmvPpd` and the fixtures' `bodyComfort` of the Gagge two-node asset (same clothing, same environment) must reproduce PMV, PPD and the sensation class.
Run `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

from pythermalcomfort.models import pmv_ppd_iso

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🛋️pmv-comfort" / "🔣️.json"
CASES = [
    {"id": "neutral-office", "tdb": 24.0, "tr": 24.0, "v": 0.1, "rh": 50.0, "met": 1.2, "clo": 0.6},
    {"id": "iso-example-b", "tdb": 22.0, "tr": 22.0, "v": 0.1, "rh": 60.0, "met": 1.2, "clo": 0.5},
    {"id": "warm-summer-clothes", "tdb": 28.0, "tr": 28.0, "v": 0.2, "rh": 50.0, "met": 1.0, "clo": 0.4},
    {"id": "cool-winter-indoor", "tdb": 18.0, "tr": 18.0, "v": 0.1, "rh": 40.0, "met": 1.0, "clo": 1.0},
    {"id": "cold-radiant-wall", "tdb": 21.0, "tr": 14.0, "v": 0.1, "rh": 50.0, "met": 1.2, "clo": 0.8},
    {"id": "hot-radiant-sun", "tdb": 26.0, "tr": 38.0, "v": 0.1, "rh": 50.0, "met": 1.2, "clo": 0.5},
    {"id": "walking-indoors", "tdb": 22.0, "tr": 22.0, "v": 0.3, "rh": 50.0, "met": 2.0, "clo": 0.7},
    {"id": "light-work-fan", "tdb": 27.0, "tr": 27.0, "v": 0.6, "rh": 60.0, "met": 1.6, "clo": 0.5},
    {"id": "sleeping-bedding", "tdb": 17.0, "tr": 17.0, "v": 0.05, "rh": 50.0, "met": 0.9, "clo": 2.0},
    {"id": "outdoor-cold-extrapolated", "tdb": 5.0, "tr": 5.0, "v": 1.0, "rh": 70.0, "met": 1.4, "clo": 1.5},
    {"id": "outdoor-hot-extrapolated", "tdb": 36.0, "tr": 40.0, "v": 0.5, "rh": 40.0, "met": 1.8, "clo": 0.3},
]


def compute(case):
    r = pmv_ppd_iso(tdb=case["tdb"], tr=case["tr"], vr=case["v"], rh=case["rh"], met=case["met"], clo=case["clo"], model="7730-2005", limit_inputs=False, round_output=False)
    return {"pmv": float(r.pmv), "ppd": float(r.ppd), "tsv": str(r.tsv)}


def build():
    return [{"id": c["id"], "inputs": c, "expected": compute(c)} for c in CASES]


if __name__ == "__main__":
    cases = build()
    document = {
        "id": "pmv-comfort",
        "oracle": "pythermalcomfort pmv_ppd_iso(model='7730-2005', limit_inputs=False, round_output=False); vr is the relative air speed",
        "tolerances": {"pmv": 0.01, "ppdPercent": 0.1},
        "cases": cases,
    }
    if "--write" in sys.argv:
        FIXTURE.parent.mkdir(parents=True, exist_ok=True)
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        stored = json.loads(FIXTURE.read_text(encoding="utf-8"))
        for case, ref in zip(cases, stored["cases"]):
            assert abs(case["expected"]["pmv"] - ref["expected"]["pmv"]) < 1e-6, case["id"]
            assert abs(case["expected"]["ppd"] - ref["expected"]["ppd"]) < 1e-6, case["id"]
    for case in cases:
        print(case["id"], {k: (round(v, 3) if isinstance(v, float) else v) for k, v in case["expected"].items()})
