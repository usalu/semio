"""🍃️ Independent oracle of the Penman–Monteith equation: FAO-56 example 18 (Uccle, 6 July, published ET0 = 3.88 mm/day) and further daily cases computed with pyet `pm_fao56`.

The fixture stores the inputs of the FAO-56 reference-crop form (r_a = 208/u2, r_s = 70 s/m, net radiation, mean saturation deficit) and the library result in mm/day.
Run `uv run --with pyet --with pandas python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

import pandas as pd
import pyet

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🍃️penman-monteith" / "🔣️.json"
CASES = [
    {"id": "fao56-example-18", "tmax": 21.5, "tmin": 12.3, "tmean": 16.9, "u2": 2.078, "rn": 13.28, "ea": 1.409, "elevation": 100.0, "published": 3.88},
    {"id": "hot-dry-windy", "tmax": 36.0, "tmin": 24.0, "tmean": 30.0, "u2": 3.5, "rn": 18.0, "ea": 1.2, "elevation": 50.0},
    {"id": "cool-humid-calm", "tmax": 11.0, "tmin": 5.0, "tmean": 8.0, "u2": 0.8, "rn": 4.0, "ea": 0.9, "elevation": 300.0},
]


def e0(t):
    return 0.6108 * math.exp(17.27 * t / (t + 237.3))


def library(case):
    idx = pd.date_range("2000-07-06", periods=1)
    series = lambda x: pd.Series([x], index=idx)
    et = pyet.pm_fao56(series(case["tmean"]), series(case["u2"]), rn=series(case["rn"]), g=series(0.0), ea=series(case["ea"]), tmax=series(case["tmax"]), tmin=series(case["tmin"]), elevation=case["elevation"], lat=0.88)
    return float(et.iloc[0])


def build():
    cases = []
    for c in CASES:
        c = dict(c)
        c["pyet"] = library(c)
        c["es"] = (e0(c["tmax"]) + e0(c["tmin"])) / 2
        cases.append(c)
    return {"id": "penman-monteith", "oracle": "pyet pm_fao56 (Allen et al. 1998, FAO-56 equation 6); published FAO-56 example 18", "tolerances": {"relative": 0.04, "publishedRelative": 0.03}, "cases": cases}


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    for c in document["cases"]:
        print(c["id"], c["pyet"], c.get("published"))
