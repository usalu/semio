"""⏳️ Independent oracle of the transient response: scipy `solve_ivp` (LSODA, rtol 1e-10) of the nonlinear lumped energy balance `C dT/dt = q_in + α G f − ε σ (T⁴ − T_env⁴) − h(T) (T − T_air)`
for a sunlit passive plate and a plate heated from inside, with CoolProp air properties and the Churchill–Chu / laminar flat plate mixed convection.

Run `uv run --with scipy --with CoolProp python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

from CoolProp.CoolProp import PropsSI
from scipy.constants import Stefan_Boltzmann as SIGMA, g as GRAVITY
from scipy.integrate import solve_ivp
from scipy.optimize import brentq

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "⏳️thermal-transient" / "🔣️.json"
ENV = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 5, "groundTempC": 22, "relativeHumidity": 0.5, "windMS": 2, "windDirectionDeg": 0, "solarWM2": 800, "diffuseFraction": 0.1, "pressurePa": 101325}
PLATES = [
    {"id": "sunlit-plate", "model": "passive", "capacity": 8000.0, "area": 1.0, "length": 0.5, "eps": 0.9, "alpha": 0.7, "heatW": 0.0, "sky": 0.5},
    {"id": "heated-plate", "model": "operating", "capacity": 20000.0, "area": 0.5, "length": 0.4, "eps": 0.8, "alpha": 0.0, "heatW": 400.0, "sky": 0.3},
]
TIMES = [30, 120, 300, 600, 1200, 2400, 3600]


def kelvin(c):
    return c + 273.15


def convection(ts, ta, p, speed, length):
    tf = (ts + ta) / 2 + 273.15
    k, mu, rho, pr, cp = (PropsSI(x, "T", tf, "P", p, "Air") for x in ("L", "V", "D", "Prandtl", "C"))
    nu, alpha = mu / rho, k / (rho * cp)
    ra = GRAVITY * abs(ts - ta) * length**3 / (tf * nu * alpha)
    psi = (1 + (0.492 / pr) ** (9 / 16)) ** (8 / 27)
    natural = 0.0 if ra <= 0 else (0.825 + 0.387 * ra ** (1 / 6) / psi) ** 2 * k / length
    re = speed * length / nu
    forced = 0.0 if re <= 0 else (0.664 * math.sqrt(re) if re < 5e5 else 0.037 * re**0.8 - 871) * pr ** (1 / 3) * k / length
    return (natural**3 + forced**3) ** (1 / 3)


def environment_c(plate):
    sky = plate["sky"]
    ground = (1 - sky) * 0.5
    rest = 1 - sky - ground
    return (sky * kelvin(ENV["skyTempC"]) ** 4 + ground * kelvin(ENV["groundTempC"]) ** 4 + rest * kelvin(ENV["meanRadiantTempC"]) ** 4) ** 0.25 - 273.15


def heat_in(plate, on):
    solar = plate["alpha"] * ENV["solarWM2"] * 0.5 * (1.0 if on else ENV["diffuseFraction"]) if plate["model"] == "passive" else 0.0
    return solar + (plate["heatW"] * (1.0 if on else 0.0) / plate["area"] if plate["model"] == "operating" else 0.0)


def rate(plate, on):
    tenv = environment_c(plate)

    def f(t, y):
        ts = y[0]
        out = plate["eps"] * SIGMA * (kelvin(ts) ** 4 - kelvin(tenv) ** 4) + convection(ts, ENV["airTempC"], ENV["pressurePa"], ENV["windMS"], plate["length"]) * (ts - ENV["airTempC"])
        return [(heat_in(plate, on) - out) / plate["capacity"]]

    return f


def steady(plate, on):
    f = rate(plate, on)
    return brentq(lambda ts: f(0, [ts])[0], -30, 400, xtol=1e-12)


def compute(plate):
    t0 = steady(plate, False)
    sol = solve_ivp(rate(plate, True), (0, TIMES[-1]), [t0], method="LSODA", rtol=1e-10, atol=1e-10, t_eval=TIMES)
    return {"initialC": t0, "finalSteadyC": steady(plate, True), "responseC": [float(x) for x in sol.y[0]]}


def build():
    return {
        "id": "thermal-transient",
        "oracle": "scipy solve_ivp (LSODA) of the nonlinear lumped balance, CoolProp air properties",
        "tolerances": {"responseK": 0.25, "steadyK": 0.05},
        "timesS": TIMES,
        "asset": {
            "dimensions": {"heightM": 1, "massKg": 0},
            "materials": [{"id": f"m-{p['id']}", "title": {"en": p["id"], "de": p["id"]}, "emissivity": p["eps"], "solarAbsorptivity": p["alpha"], "source": "illustrative"} for p in PLATES],
            "thermal": {
                "environment": ENV,
                "passengers": [],
                "operating": [{"id": "power", "title": {"en": "Power", "de": "Leistung"}, "levels": [{"v": 0, "title": {"en": "off", "de": "aus"}}, {"v": 1, "title": {"en": "rated", "de": "Nennlast"}}]}],
                "emitters": [],
                "assumptions": [],
                "regions": [
                    {"id": p["id"], "title": {"en": p["id"], "de": p["id"]}, "kind": "surface" if p["model"] == "passive" else "heater", "model": p["model"], "material": f"m-{p['id']}", "areaM2": p["area"], "characteristicLengthM": p["length"], "geometry": "vertical-plate", "skyViewFactor": p["sky"], "layers": [], "heatCapacityJM2K": p["capacity"], **({"operating": {"key": "power", "heatW": p["heatW"]}} if p["model"] == "operating" else {}), "source": "illustrative"}
                    for p in PLATES
                ],
            },
        },
        "expected": {p["id"]: compute(p) for p in PLATES},
    }


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        stored = json.loads(FIXTURE.read_text(encoding="utf-8"))
        for pid, e in document["expected"].items():
            assert abs(e["finalSteadyC"] - stored["expected"][pid]["finalSteadyC"]) < 1e-6, pid
    print(json.dumps({k: {"initial": round(v["initialC"], 2), "steady": round(v["finalSteadyC"], 2), "response": [round(x, 2) for x in v["responseC"]]} for k, v in document["expected"].items()}))
