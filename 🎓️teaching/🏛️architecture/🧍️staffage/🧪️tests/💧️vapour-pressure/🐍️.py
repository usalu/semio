"""💧️ Independent oracle of the vapour pressure case: IAPWS-95 saturation pressure and humid-air humidity ratio from CoolProp.

The saturation pressures are cross-checked against psychrolib (ASHRAE Handbook, Hyland-Wexler) within 0.3 %.

Run `uv run --with CoolProp --with psychrolib python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

from CoolProp.CoolProp import PropsSI
from CoolProp.HumidAirProp import HAPropsSI
import psychrolib

psychrolib.SetUnitSystem(psychrolib.SI)

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "💧️vapour-pressure" / "🔣️.json"


def compute(fixture):
    h = fixture["humidity"]
    saturation = [{"tempC": t, "pascal": PropsSI("P", "T", t + 273.15, "Q", 0, "Water")} for t in fixture["temperaturesC"]]
    specific = []
    for t in h["temperaturesC"]:
        w = HAPropsSI("W", "T", t + 273.15, "P", h["pressurePa"], "R", h["relativeHumidity"])
        specific.append({"tempC": t, "specificHumidity": w / (1 + w), "vapourPressurePa": h["relativeHumidity"] * PropsSI("P", "T", t + 273.15, "Q", 0, "Water")})
    for row in saturation:
        assert abs(psychrolib.GetSatVapPres(row["tempC"]) / row["pascal"] - 1) < 0.003, row
    return {"saturation": saturation, "specific": specific}


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for a, b in zip(computed["saturation"], fixture["expected"]["saturation"]):
            assert abs(a["pascal"] - b["pascal"]) <= 1e-9 * a["pascal"], a
    print(json.dumps(computed))
