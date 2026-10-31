"""🌡️ Independent oracle of the radiation exchange: emitted flux εσT⁴, net longwave exchange with sky, ground and surroundings (view-factor weighted) and the secant coefficient,
with the CODATA Stefan–Boltzmann constant of scipy and numpy.

Run `uv run --with scipy python -I 🐍️.py [--write]`; without `--write` the fixture's `expected` block is verified, with it the fixture is regenerated.
"""
import json
import sys
from pathlib import Path

import numpy as np
from scipy.constants import Stefan_Boltzmann as SIGMA, convert_temperature

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🌡️blackbody-exchange" / "🔣️.json"
ENV = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": -10, "groundTempC": 30, "relativeHumidity": 0.5, "windMS": 0, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}
REGIONS = [
    {"id": "warm-plate", "geometry": "horizontal-plate-up", "sky": 0.8, "tempC": 33, "area": 1.0, "eps": 0.95},
    {"id": "cold-wall", "geometry": "vertical-plate", "sky": 0.3, "tempC": 5, "area": 2.0, "eps": 0.95},
    {"id": "underside", "geometry": "horizontal-plate-down", "sky": 0.0, "tempC": 25, "area": 0.5, "eps": 0.9},
    {"id": "equilibrium", "geometry": "sphere", "sky": 0.5, "tempC": 20, "area": 1.0, "eps": 0.95},
]


def kelvin(c):
    return convert_temperature(c, "C", "K")


def views(geometry, sky):
    share = {"horizontal-plate-down": 1.0, "horizontal-plate-up": 0.0}.get(geometry, 0.5)
    ground = (1 - sky) * share
    return {"sky": sky, "ground": ground, "surroundings": 1 - sky - ground}


def compute(region):
    ts = kelvin(region["tempC"])
    temps = {"sky": kelvin(ENV["skyTempC"]), "ground": kelvin(ENV["groundTempC"]), "surroundings": kelvin(ENV["meanRadiantTempC"])}
    f = views(region["geometry"], region["sky"])
    split = {k: float(region["eps"] * SIGMA * f[k] * (ts**4 - temps[k] ** 4)) for k in f}
    net = sum(split.values())
    teff = sum(f[k] * temps[k] ** 4 for k in f) ** 0.25
    delta = ts - teff
    coefficient = 4 * region["eps"] * SIGMA * ts**3 if np.isclose(delta, 0, atol=1e-9) else net / delta
    return {"emittedWM2": float(region["eps"] * SIGMA * ts**4), "netWM2": float(net), "netW": float(net * region["area"]), "splitWM2": split, "coefficientWM2K": float(coefficient)}


def build():
    return {
        "id": "blackbody-exchange",
        "oracle": "scipy.constants.Stefan_Boltzmann (CODATA 2018), numpy",
        "tolerances": {"relative": 0.002, "absoluteWM2": 0.01},
        "asset": {
            "dimensions": {"heightM": 1, "massKg": 0},
            "materials": [{"id": f"eps-{r['eps']}", "title": {"en": f"Grey body, emissivity {r['eps']}", "de": f"Grauer Körper, Emissionsgrad {r['eps']}"}, "emissivity": r["eps"], "source": "illustrative"} for r in {x["eps"]: x for x in REGIONS}.values()],
            "thermal": {
                "environment": ENV,
                "passengers": [],
                "operating": [],
                "emitters": [],
                "assumptions": [],
                "regions": [
                    {"id": r["id"], "title": {"en": r["id"], "de": r["id"]}, "kind": "surface", "model": "fixed", "material": f"eps-{r['eps']}", "areaM2": r["area"], "characteristicLengthM": 0.5, "geometry": r["geometry"], "skyViewFactor": r["sky"], "layers": [], "heatCapacityJM2K": 1000, "fixed": {"surfaceTempC": r["tempC"]}, "source": "illustrative"}
                    for r in REGIONS
                ],
            },
        },
        "state": {"timeS": 0, "thermal": {"activityMet": 0, "externalWorkW": 0, "operating": {}, "regions": {r["id"]: {"exposedFraction": 1, "sunlitFraction": 0} for r in REGIONS}, "emitters": {}, "breathing": None}, "anchors": {}, "bones": {}},
        "expected": {r["id"]: compute(r) for r in REGIONS},
    }


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        stored = json.loads(FIXTURE.read_text(encoding="utf-8"))
        for rid, values in document["expected"].items():
            assert abs(values["netWM2"] - stored["expected"][rid]["netWM2"]) < 1e-9, rid
    print(json.dumps(document["expected"], indent=1))
