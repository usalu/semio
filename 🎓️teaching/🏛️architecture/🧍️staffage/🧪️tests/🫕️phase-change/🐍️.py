"""🫕️ Oracle of the phase-change plateau: scipy `solve_ivp` (LSODA, terminal event at the phase-change point) of the nonlinear lumped balance of a region heated from inside, then the energy balance of the plateau.

Before the point: `C dT/dt = P/A - q_out(T)` with grey-body radiation, Churchill-Chu natural convection with CoolProp air properties and, for a wetted surface, the Lewis-relation evaporation against the
CoolProp (IAPWS-95) saturation pressure. At the point the temperature holds and the converted mass rate is `A (P/A - q_out(T_b)) / L`. The latent heat of boiling and the boiling point are CoolProp values at 1 atm.
The closure `P t = A C (T - T_0) + A integral(q_out) + L m` is asserted for every time.

Run `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`.
"""
import importlib.util
import json
import sys
from pathlib import Path

from CoolProp.CoolProp import PropsSI
from scipy.constants import Stefan_Boltzmann as SIGMA
from scipy.integrate import solve_ivp
from scipy.optimize import brentq

HERE = Path(__file__).resolve()
FIXTURE = HERE.parents[2] / "🧫️fixtures" / "🫕️phase-change" / "🔣️.json"
_spec = importlib.util.spec_from_file_location("transient_oracle", HERE.parents[1] / "⏳️thermal-transient" / "🐍️.py")
transient = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(transient)

ENV = {"airTempC": 20, "meanRadiantTempC": 20, "relativeHumidity": 0.5, "windMS": 0, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0.1, "pressurePa": 101325}
COLD = {**ENV, "airTempC": -5, "meanRadiantTempC": -5, "relativeHumidity": 0.7}
LEWIS = 16.5
KELVIN = 273.15
CASES = [
    {"id": "kettle-water", "transition": "boiling", "material": "water", "env": ENV, "kind": "water", "model": "operating", "wetted": True, "eps": 0.96, "area": 0.012, "length": 0.12, "capacity": 349000.0, "heatW": 1930.0, "pointC": 100.0, "timesS": [30, 90, 150, 180, 190, 200, 260, 400]},
    {"id": "ice-plate", "transition": "melting", "material": "ice", "env": COLD, "kind": "surface", "model": "operating", "wetted": False, "eps": 0.97, "area": 0.1, "length": 0.3, "capacity": 60000.0, "heatW": 150.0, "pointC": 0.0, "timesS": [60, 150, 250, 400, 800]},
]


def latent_heat(point_c):
    return float(PropsSI("H", "T", point_c + KELVIN, "Q", 1, "Water") - PropsSI("H", "T", point_c + KELVIN, "Q", 0, "Water"))


def saturation(t_c):
    return float(PropsSI("P", "T", t_c + KELVIN, "Q", 1, "Water"))


def outflow(case, t_c):
    env = case["env"]
    hc = transient.convection(t_c, env["airTempC"], env["pressurePa"], env["windMS"], case["length"])
    radiative = case["eps"] * SIGMA * ((t_c + KELVIN) ** 4 - (env["meanRadiantTempC"] + KELVIN) ** 4)
    sensible = hc * (t_c - env["airTempC"])
    latent = max(0.0, LEWIS * hc / 1000 * (101325 / env["pressurePa"]) * (saturation(t_c) - env["relativeHumidity"] * saturation(env["airTempC"]))) if case["wetted"] else 0.0
    return radiative + sensible + latent, latent


def compute(case):
    power = case["heatW"] / case["area"]
    cap, point = case["capacity"], case["pointC"]
    off = brentq(lambda t: -outflow(case, t)[0], -40, 90, xtol=1e-12)
    hfg = latent_heat(point) if case["transition"] == "boiling" else None
    latent_j = 333550.0 if case["transition"] == "melting" else hfg

    def rate(t, y):
        return [(power - outflow(case, y[0])[0]) / cap, outflow(case, y[0])[0]]

    def reach(t, y):
        return y[0] - point

    reach.terminal, reach.direction = True, 1
    cross = solve_ivp(rate, (0, 3600), [off, 0.0], method="LSODA", rtol=1e-10, atol=1e-10, events=reach)
    t_cross = float(cross.t_events[0][0])
    surplus = power - outflow(case, point)[0]
    rows = []
    for t in case["timesS"]:
        if t <= t_cross:
            sol = solve_ivp(rate, (0, t), [off, 0.0], method="LSODA", rtol=1e-10, atol=1e-10)
            temp, lost, kg = float(sol.y[0][-1]), float(sol.y[1][-1]), 0.0
        else:
            lost = float(cross.y_events[0][0][1]) + outflow(case, point)[0] * (t - t_cross)
            temp, kg = point, case["area"] * surplus * (t - t_cross) / latent_j
        closure = power * t - (cap * (temp - off) + lost + latent_j * kg / case["area"])
        assert abs(closure) < 1e-3 * power * t, (case["id"], t, closure)
        rows.append({"tempC": temp, "phaseKg": kg, "phaseW": case["area"] * surplus if t > t_cross else 0.0, "closureJM2": closure})
    plateau_latent = outflow(case, point)[1]
    return {"initialC": off, "crossingS": t_cross, "latentHeatJKg": latent_j, "plateauW": case["area"] * surplus, "plateauKgS": case["area"] * surplus / latent_j, "plateauLewisKgS": case["area"] * plateau_latent / (latent_heat(point) if case["wetted"] else 1.0) if case["wetted"] else 0.0, "rows": rows}


def region(case):
    return {
        "id": case["id"], "title": {"en": case["id"], "de": case["id"]}, "kind": case["kind"], "model": case["model"], "material": case["material"], "areaM2": case["area"], "characteristicLengthM": case["length"], "geometry": "vertical-plate",
        "skyViewFactor": 0.5, "layers": [], "heatCapacityJM2K": case["capacity"], "operating": {"key": "power", "heatW": case["heatW"]}, "source": "illustrative",
    }


def build():
    cases = []
    for case in CASES:
        expected = compute(case)
        cases.append({
            "id": case["id"], "transition": case["transition"], "pointC": case["pointC"], "timesS": case["timesS"],
            "asset": {
                "dimensions": {"heightM": 1, "massKg": 0},
                "materials": [{"id": case["material"], "title": {"en": case["material"], "de": case["material"]}, "emissivity": case["eps"], "solarAbsorptivity": 0.1, "phaseChange": {"transition": case["transition"], "pointC": case["pointC"], "latentHeatJKg": round(expected["latentHeatJKg"], 1)}, "source": "literature"}],
                "thermal": {
                    "environment": case["env"], "passengers": [],
                    "operating": [{"id": "power", "title": {"en": "Power", "de": "Leistung"}, "levels": [{"v": 0, "title": {"en": "off", "de": "aus"}}, {"v": 1, "title": {"en": "rated", "de": "Nennlast"}}]}],
                    "emitters": [], "assumptions": [], "regions": [region(case)],
                },
            },
            "expected": expected,
        })
    return {
        "id": "phase-change",
        "oracle": "scipy solve_ivp (LSODA, terminal event) of the nonlinear lumped balance with CoolProp air properties, IAPWS-95 saturation pressure and latent heat of boiling; energy-balance closure per time",
        "tolerances": {"tempK": 0.05, "massG": 0.1, "powerRel": 0.002, "closureRel": 0.001, "latentRel": 0.002, "pointK": 0.05},
        "coolprop": {"boilingPointC": float(PropsSI("T", "P", 101325, "Q", 0, "Water")) - KELVIN, "latentHeatBoilingJKg": latent_heat(float(PropsSI("T", "P", 101325, "Q", 0, "Water")) - KELVIN)},
        "cases": cases,
    }


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")
    for c in document["cases"]:
        e = c["expected"]
        print(c["id"], round(e["initialC"], 2), round(e["crossingS"], 2), round(e["plateauW"], 1), [(round(r["tempC"], 2), round(r["phaseKg"] * 1000, 2)) for r in e["rows"]])
    print(document["coolprop"])
