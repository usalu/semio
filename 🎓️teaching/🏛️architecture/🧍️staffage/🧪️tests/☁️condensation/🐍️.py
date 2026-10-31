"""☁️ Independent oracle of the condensation layer: mixes saturated-ish exhaled air with ambient air by dry-air mass fraction using CoolProp humid-air enthalpy and humidity ratio, and finds the supersaturated mixture state (temperature, liquid water) by brentq on CoolProp's saturated humid air plus liquid-water enthalpy.

Run `uv run --with scipy --with CoolProp --with numpy python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

from CoolProp.CoolProp import HAPropsSI, PropsSI
from scipy.optimize import brentq

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "☁️condensation" / "🔣️.json"
K = 273.15


def liquid_h(t_k):
    return PropsSI("H", "T", t_k, "Q", 0, "Water") - PropsSI("H", "T", K + 0.01, "Q", 0, "Water")


def mix(fixture, ambient, f):
    p = fixture["pressurePa"]
    ex, am = fixture["exhaled"], ambient
    w_e = HAPropsSI("W", "T", ex["tempC"] + K, "P", p, "R", ex["relativeHumidity"])
    h_e = HAPropsSI("H", "T", ex["tempC"] + K, "P", p, "W", w_e)
    w_a = HAPropsSI("W", "T", am["tempC"] + K, "P", p, "R", am["relativeHumidity"])
    h_a = HAPropsSI("H", "T", am["tempC"] + K, "P", p, "W", w_a)
    wt = f * w_e + (1 - f) * w_a
    h = f * h_e + (1 - f) * h_a
    t_u = HAPropsSI("T", "H", h, "P", p, "W", wt)
    w_sat_u = HAPropsSI("W", "T", t_u, "P", p, "R", 1.0)
    rh_u = 2.0 if wt > w_sat_u else HAPropsSI("R", "T", t_u, "P", p, "W", wt)
    if rh_u <= 1:
        return {"tempC": t_u - K, "liquid": 0.0, "relativeHumidity": rh_u, "saturated": False}
    def residual(t):
        w_s = HAPropsSI("W", "T", t, "P", p, "R", 1.0)
        return HAPropsSI("H", "T", t, "P", p, "W", w_s) + (wt - w_s) * liquid_h(t) - h
    t_dew = HAPropsSI("T", "P", float(p), "W", float(wt), "R", 1.0)
    t = brentq(residual, t_u - 1e-6, t_dew + 1e-6, xtol=1e-12)
    w_s = HAPropsSI("W", "T", t, "P", p, "R", 1.0)
    return {"tempC": t - K, "liquid": wt - w_s, "relativeHumidity": 1.0, "saturated": True}


def compute(fixture):
    p = fixture["pressurePa"]
    out = {"mixing": [], "saturation": [], "ambient": []}
    for ambient in fixture["ambients"]:
        for f in fixture["fractions"]:
            out["mixing"].append({"ambientTempC": ambient["tempC"], "ambientRelativeHumidity": ambient["relativeHumidity"], "fraction": f, **mix(fixture, ambient, f)})
        w_a = HAPropsSI("W", "T", ambient["tempC"] + K, "P", p, "R", ambient["relativeHumidity"])
        out["ambient"].append({"tempC": ambient["tempC"], "humidityRatio": w_a, "enthalpy": HAPropsSI("H", "T", ambient["tempC"] + K, "P", p, "W", w_a)})
    for t in (0.5, 5, 10, 20, 34, 40):
        out["saturation"].append({"tempC": t, "humidityRatio": HAPropsSI("W", "T", t + K, "P", p, "R", 1.0), "enthalpy": HAPropsSI("H", "T", t + K, "P", p, "R", 1.0)})
    return out


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        assert all(abs(a["liquid"] - b["liquid"]) < 1e-9 for a, b in zip(computed["mixing"], fixture["expected"]["mixing"])), "drift"
    for row in computed["mixing"]:
        print("condensation:", row["ambientTempC"], row["fraction"], round(row["tempC"], 3), round(row["liquid"] * 1000, 4))
