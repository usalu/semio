"""🫁️ Independent oracle of the respiration model: scipy quad integrates the half-sine flow of every scenario pattern (inhaled and exhaled volume = tidal volume), CoolProp gives the respiratory heat of the exhaled state, and the ASHRAE regression gives the numbers the thermal solver uses.

Run `uv run --with scipy --with CoolProp --with numpy python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

from CoolProp.CoolProp import HAPropsSI, PropsSI
from scipy.integrate import quad

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "🧫️fixtures" / "🫁️respiration" / "🔣️.json"
SCENARIOS = ROOT / "🧫️fixtures" / "💨️transport" / "🔣️.json"


def half_sine_volume_l(vt_l, duration_s):
    flow = lambda t: math.pi * vt_l / (2 * duration_s) * math.sin(math.pi * t / duration_s)
    return quad(flow, 0, duration_s, epsabs=1e-13, epsrel=1e-13)[0]


def heat(pattern, resp, env):
    p = env["pressurePa"]
    ta, rh = env["airTempC"] + 273.15, env["relativeHumidity"]
    te = resp["exhaledTempC"] + 273.15
    wa = HAPropsSI("W", "T", ta, "P", p, "R", rh)
    we = HAPropsSI("W", "T", te, "P", p, "R", resp["exhaledRelativeHumidity"])
    ha = HAPropsSI("H", "T", ta, "P", p, "W", wa)
    he = HAPropsSI("H", "T", te, "P", p, "W", we)
    v_dry = HAPropsSI("Vda", "T", te, "P", p, "W", we)
    mdot = pattern["rateBpm"] / 60 * pattern["tidalVolumeL"] / 1000 / v_dry
    hfg = PropsSI("H", "T", te, "Q", 1, "Water") - PropsSI("H", "T", te, "Q", 0, "Water")
    latent = mdot * (we - wa) * hfg
    return {"dryAirKgS": mdot, "sensibleW": mdot * (he - ha) - latent, "latentW": latent, "vapourPa": rh * PropsSI("P", "T", ta, "Q", 1, "Water")}


def compute(fixture):
    scenarios = json.loads(SCENARIOS.read_text(encoding="utf-8"))
    resp = scenarios["assets"]["human"]["thermal"]["respiration"]
    env = fixture["heat"]["environment"]
    out = {}
    for pattern in resp["patterns"]:
        period = 60 / pattern["rateBpm"]
        h = heat(pattern, resp, env)
        m, area = fixture["heat"]["metWM2"], fixture["heat"]["areaM2"]
        pa_kpa = h["vapourPa"] / 1000
        out[pattern["id"]] = {
            "inhaleL": half_sine_volume_l(pattern["tidalVolumeL"], period * pattern["inhaleFraction"]),
            "exhaleL": half_sine_volume_l(pattern["tidalVolumeL"], period * pattern["exhaleFraction"]),
            "coolprop": h,
            "ashrae": {"sensibleW": 0.0014 * m * (34 - env["airTempC"]) * area, "latentW": 0.0173 * m * (5.87 - pa_kpa) * area},
        }
    return out


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for key, value in computed.items():
            assert abs(value["exhaleL"] - fixture["expected"][key]["exhaleL"]) < 1e-12, "volume drift"
    for key, value in computed.items():
        print("respiration:", key, {k: round(v, 6) for k, v in value["coolprop"].items()}, {k: round(v, 3) for k, v in value["ashrae"].items()}, round(value["exhaleL"], 9))
