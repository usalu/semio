"""🌡️ Independent oracle of the blackbody-exchange case: grey-body radiation with the CODATA Stefan–Boltzmann constant of scipy.

Run `uv run --with scipy python -I 🐍️.py [--write]`; without `--write` the fixture's `expected` block is verified, with it the block is regenerated.
"""
import json
import sys
from pathlib import Path

import numpy as np
from scipy.constants import Stefan_Boltzmann as SIGMA, convert_temperature

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🌡️blackbody-exchange" / "🔣️.json"


def compute(fixture):
    env = fixture["asset"]["thermal"]["environment"]
    eps = fixture["asset"]["materials"][0]["emissivity"]
    tmrt = convert_temperature(env["meanRadiantTempC"], "C", "K")
    out = {}
    for region in fixture["asset"]["thermal"]["regions"]:
        ts = convert_temperature(region["surfaceTempC"], "C", "K")
        q = eps * SIGMA * (ts**4 - tmrt**4)
        h = 4 * eps * SIGMA * ts**3 if np.isclose(ts, tmrt) else q / (ts - tmrt)
        out[region["id"]] = {"radiantFluxWM2": float(q), "radiationCoefficientWM2K": float(h), "radiantW": float(q * region["areaM2"])}
    return out


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    else:
        rel = fixture["tolerances"]["relative"]
        for rid, values in computed.items():
            for key, v in values.items():
                e = fixture["expected"][rid][key]
                assert abs(v - e) <= rel * abs(v) + 1e-9, (rid, key, v, e)
    print(json.dumps(computed, indent=1))
