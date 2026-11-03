"""🚰️ Oracle of the water-bearing regions: scipy `solve_ivp` (LSODA) of the nonlinear lumped balances with CoolProp air properties and the Churchill-Chu natural convection of the transient oracle.

- `hydronic` plate: `C dT/dt = (T_water - T)/R - q_out(T)` with the circuit water interpolated from the supply/return rows at the plate's place along the flow; the surface never exceeds the supply.
- `coupled` wall: water node `C_w dT_w/dt = P/A_w - q_out_w(T_w) - (A_s/A_w) k (T_w - T_s)` with the boiling cap at 100 degC, wall node `C_s dT_s/dt = k (T_w - T_s) - q_out_s(T_s)`, `k = f / (R_layer + R_film)`.

Run `uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --group test python -I 🐍️.py [--write]`.
"""
import importlib.util
import json
import sys
from pathlib import Path

from scipy.constants import Stefan_Boltzmann as SIGMA
from scipy.integrate import solve_ivp
from scipy.optimize import brentq

HERE = Path(__file__).resolve()
FIXTURE = HERE.parents[2] / "🧫️fixtures" / "🚰️hydronic-coupling" / "🔣️.json"
_spec = importlib.util.spec_from_file_location("transient_oracle", HERE.parents[1] / "⏳️thermal-transient" / "🐍️.py")
transient = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(transient)

K = 273.15
ENV = {"airTempC": 21, "meanRadiantTempC": 21, "relativeHumidity": 0.5, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 0.1, "pressurePa": 101325}
ROWS = [{"v": 0.515, "supplyTempC": 55, "returnTempC": 45}, {"v": 0.748, "supplyTempC": 65, "returnTempC": 55}, {"v": 1, "supplyTempC": 75, "returnTempC": 65}]
PLATE = {"area": 0.2, "length": 0.6, "eps": 0.9, "capacity": 22000.0, "film": 0.003}
PLATE_CASES = [("upper-rated", 1.0, 1 / 6), ("lower-rated", 1.0, 5 / 6), ("middle-comfort", 0.748, 0.5), ("middle-night", 0.515, 0.5), ("middle-quarter", 0.25, 0.5)]
PLATE_TIMES = [30, 120, 300, 900]
COUPLED = {
    "water": {"area": 0.012, "length": 0.12, "eps": 0.96, "capacity": 348800.0, "heatW": 1955.0},
    "wall": {"area": 0.11, "length": 0.19, "eps": 0.3, "capacity": 3500.0, "layer": 4e-05, "film": 0.0025, "wet": 0.6},
    "timesS": [60, 120, 180, 190, 200, 240, 300],
    "pointC": 100.0,
}


def out(ts, length, eps):
    hc = transient.convection(ts, ENV["airTempC"], ENV["pressurePa"], ENV["windMS"], length)
    return eps * SIGMA * ((ts + K) ** 4 - (ENV["meanRadiantTempC"] + K) ** 4) + hc * (ts - ENV["airTempC"])


def circuit(level, position):
    v = max(0.0, level)
    air = ENV["airTempC"]
    if v < ROWS[0]["v"]:
        w = v / ROWS[0]["v"]
        supply, back = air + (ROWS[0]["supplyTempC"] - air) * w, air + (ROWS[0]["returnTempC"] - air) * w
    elif v >= ROWS[-1]["v"]:
        supply, back = ROWS[-1]["supplyTempC"], ROWS[-1]["returnTempC"]
    else:
        k = next(i for i in range(len(ROWS) - 1) if ROWS[i]["v"] <= v < ROWS[i + 1]["v"])
        lo, hi = ROWS[k], ROWS[k + 1]
        w = (v - lo["v"]) / (hi["v"] - lo["v"])
        supply = lo["supplyTempC"] + (hi["supplyTempC"] - lo["supplyTempC"]) * w
        back = lo["returnTempC"] + (hi["returnTempC"] - lo["returnTempC"]) * w
    return supply - (supply - back) * position, supply


def plate_case(name, level, position):
    water, supply = circuit(level, position)
    rate = lambda t, y: [((water - y[0]) / PLATE["film"] - out(y[0], PLATE["length"], PLATE["eps"])) / PLATE["capacity"]]
    steady = brentq(lambda ts: rate(0, [ts])[0], ENV["airTempC"] - 5, water + 1, xtol=1e-12)
    sol = solve_ivp(rate, (0, PLATE_TIMES[-1]), [ENV["airTempC"]], method="LSODA", rtol=1e-10, atol=1e-10, t_eval=PLATE_TIMES)
    assert steady < water <= supply
    return {"id": name, "level": level, "flowPosition": position, "waterC": water, "supplyC": supply, "steadyC": steady, "responseC": [float(x) for x in sol.y[0]]}


def coupled_case():
    w, s, p = COUPLED["water"], COUPLED["wall"], COUPLED["pointC"]
    k = s["wet"] / (s["layer"] + s["film"])
    ratio = s["area"] / w["area"]

    def rate(t, y):
        tw, ts = y
        draw = k * (tw - ts)
        dtw = (w["heatW"] / w["area"] - out(tw, w["length"], w["eps"]) - ratio * draw) / w["capacity"]
        dts = (draw - out(ts, s["length"], s["eps"])) / s["capacity"]
        return [dtw, dts]

    def reach(t, y):
        return y[0] - p

    reach.terminal, reach.direction = True, 1
    air = ENV["airTempC"]
    first = solve_ivp(rate, (0, 3000), [air, air], method="LSODA", rtol=1e-10, atol=1e-10, events=reach)
    t_cross = float(first.t_events[0][0])
    y_cross = first.y_events[0][0]
    hold = lambda t, y: [(k * (p - y[0]) - out(y[0], s["length"], s["eps"])) / s["capacity"]]
    rows = []
    for t in COUPLED["timesS"]:
        if t <= t_cross:
            tw, ts = solve_ivp(rate, (0, t), [air, air], method="LSODA", rtol=1e-10, atol=1e-10).y[:, -1]
        else:
            ts = solve_ivp(hold, (t_cross, t), [y_cross[1]], method="LSODA", rtol=1e-10, atol=1e-10).y[0][-1]
            tw = p
        rows.append({"waterC": float(tw), "wallC": float(ts)})
    wall_steady = brentq(lambda ts: hold(0, [ts])[0], air, p, xtol=1e-12)
    surplus = w["heatW"] - w["area"] * out(p, w["length"], w["eps"]) - s["area"] * k * (p - wall_steady)
    return {"crossingS": t_cross, "wallSteadyC": wall_steady, "plateauW": surplus, "rows": rows}


def material(mid, eps, phase=None):
    m = {"id": mid, "title": {"en": mid, "de": mid}, "emissivity": eps, "solarAbsorptivity": 0.1, "source": "literature"}
    if phase:
        m["phaseChange"] = phase
    return m


def region(rid, kind, model, mat, spec, **extra):
    return {"id": rid, "title": {"en": rid, "de": rid}, "kind": kind, "model": model, "material": mat, "areaM2": spec["area"], "characteristicLengthM": spec["length"], "geometry": "vertical-plate", "skyViewFactor": 0.5, "layers": extra.pop("layers", []), "heatCapacityJM2K": spec["capacity"], "source": "illustrative", **extra}


def asset(regions, materials, key):
    return {
        "dimensions": {"heightM": 1, "massKg": 0},
        "materials": materials,
        "thermal": {"environment": ENV, "passengers": [], "operating": [{"id": key, "title": {"en": key, "de": key}, "levels": [{"v": 0, "title": {"en": "off", "de": "aus"}}, {"v": 1, "title": {"en": "rated", "de": "Nennlast"}}]}], "emitters": [], "assumptions": [], "regions": regions},
    }


def build():
    plates = []
    for name, level, position in PLATE_CASES:
        plate = plate_case(name, level, position)
        plate["asset"] = asset([region("plate", "heater", "hydronic", "steel", PLATE, hydronic={"key": "heating", "rows": ROWS, "flowPosition": position, "filmM2KW": PLATE["film"]})], [material("steel", PLATE["eps"])], "heating")
        plates.append(plate)
    w, s = COUPLED["water"], COUPLED["wall"]
    layer = [{"id": "skin", "title": {"en": "skin", "de": "skin"}, "kind": "metal", "material": "wall", "resistanceM2KW": s["layer"], "source": "illustrative"}]
    wet = {"transition": "boiling", "pointC": COUPLED["pointC"], "latentHeatJKg": 2256470.0}
    coupled = asset(
        [region("water", "heater", "operating", "water", w, operating={"key": "power", "heatW": w["heatW"]}), region("wall", "surface", "coupled", "wall", s, layers=layer, coupled={"region": "water", "wetFraction": s["wet"], "filmM2KW": s["film"]})],
        [material("water", w["eps"], wet), material("wall", s["eps"])],
        "power",
    )
    return {
        "id": "hydronic-coupling",
        "oracle": "scipy solve_ivp (LSODA) of the nonlinear lumped balances of a circuit-heated plate and a water-coupled wall, CoolProp air properties",
        "tolerances": {"plateK": 0.05, "waterK": 0.6, "wallK": 0.6, "steadyK": 0.05, "plateauRel": 0.03, "crossingS": 6, "wetGapK": 6},
        "plateTimesS": PLATE_TIMES,
        "plates": plates,
        "coupled": {"timesS": COUPLED["timesS"], "pointC": COUPLED["pointC"], "wetFraction": s["wet"], "asset": coupled, "expected": coupled_case()},
    }


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.parent.mkdir(parents=True, exist_ok=True)
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")
    for p in document["plates"]:
        print(p["id"], round(p["waterC"], 2), round(p["steadyC"], 2), [round(x, 2) for x in p["responseC"]])
    e = document["coupled"]["expected"]
    print("coupled", round(e["crossingS"], 1), round(e["wallSteadyC"], 2), round(e["plateauW"], 1), [(round(r["waterC"], 2), round(r["wallC"], 2)) for r in e["rows"]])
