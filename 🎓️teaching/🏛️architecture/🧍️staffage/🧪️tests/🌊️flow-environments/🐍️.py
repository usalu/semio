"""🌤️ Independent oracle of how the flow view answers the environment: for every reference environment preset of the catalog the dew point (CoolProp humid air, cross-checked with psychrolib), whether exhaled air of the fixture (34 °C, 95 % relative humidity) fogs when it mixes with that air (the adiabatic mixing line by dry-air mass fraction against CoolProp's saturation humidity ratio), and which way a body of the fixture's surface temperature sends its convection plume (Newton's law of cooling: up when warmer than the air, down when colder, none when equal).

Run `uv run --no-project --with CoolProp==8.0.0 --with psychrolib==2.5.0 --with numpy python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

import numpy as np
import psychrolib
from CoolProp.CoolProp import HAPropsSI

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "🧫️fixtures" / "🌊️flow-environments" / "🔣️.json"
PRESETS = ROOT / "🗂️catalog" / "🌤️environments" / "🔣️.json"
K = 273.15


def fog(exhaled, air_c, rh, p):
    """Largest liquid water (kg per kg dry air, before the latent heat is returned) along the mixing line: positive where the mixture holds more water than saturated air of its temperature."""
    w_e = HAPropsSI("W", "T", exhaled["tempC"] + K, "P", p, "R", exhaled["relativeHumidity"])
    h_e = HAPropsSI("H", "T", exhaled["tempC"] + K, "P", p, "W", w_e)
    w_a = HAPropsSI("W", "T", air_c + K, "P", p, "R", max(rh, 1e-6))
    h_a = HAPropsSI("H", "T", air_c + K, "P", p, "W", w_a)
    worst = -np.inf
    for f in np.linspace(0.005, 0.995, 199):
        w = f * w_e + (1 - f) * w_a
        t = HAPropsSI("T", "H", f * h_e + (1 - f) * h_a, "P", p, "W", w)
        worst = max(worst, w - HAPropsSI("W", "T", t, "P", p, "R", 1.0))
    return float(worst)


def compute(fixture):
    psychrolib.SetUnitSystem(psychrolib.SI)
    presets = json.loads(PRESETS.read_text(encoding="utf-8"))["presets"]
    rows = []
    for preset in presets:
        e = preset["environment"]
        dew = HAPropsSI("D", "T", e["airTempC"] + K, "P", e["pressurePa"], "R", e["relativeHumidity"]) - K
        excess = fog(fixture["exhaled"], e["airTempC"], e["relativeHumidity"], e["pressurePa"])
        heat = fixture["body"]["conductanceWK"] * (fixture["body"]["surfaceTempC"] - e["airTempC"])
        rows.append({"preset": preset["id"], "airTempC": e["airTempC"], "relativeHumidity": e["relativeHumidity"], "dewPointC": float(dew), "dewPointPsychrolibC": float(psychrolib.GetTDewPointFromRelHum(e["airTempC"], e["relativeHumidity"])), "supersaturationKgPerKg": excess, "breathFogs": bool(excess > 0), "decisive": bool(abs(excess) > fixture["decisiveKgPerKg"]), "plumeHeatW": float(heat), "plume": int(np.sign(heat))})
    return rows


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        print(json.dumps(computed))
        sys.exit(0 if computed == fixture.get("expected") else 1)
