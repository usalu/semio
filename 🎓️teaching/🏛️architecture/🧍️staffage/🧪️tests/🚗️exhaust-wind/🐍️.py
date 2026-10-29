"""🚗️ Independent oracle of the exhaust plume in calm air and in ±3 m/s wind: re-integrates the documented jet ODE with scipy's RK45 (rtol 1e-10) and stores the reference polyline.

Run `uv run --with scipy python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy.integrate import solve_ivp

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🚗️exhaust-wind" / "🔣️.json"
G = 9.80665


def psat(tc):
    return 611.21 * math.exp((18.678 - tc / 234.5) * (tc / (257.14 + tc)))


def spec_hum(e, p):
    return 0.622 * e / (p - 0.378 * e)


def plume(fixture, wind_ms, wind_dir_deg=0.0):
    env = fixture["asset"]["thermal"]["environment"]
    em = fixture["asset"]["thermal"]["emitters"][0]
    anchor = fixture["state"]["anchors"][em["anchor"]]
    bone = fixture["state"]["bones"][anchor["bone"]]
    strength = fixture["state"]["thermal"]["emitters"][em["id"]]
    ta = env["airTempC"]
    qa = spec_hum(env["relativeHumidity"] * psat(ta), env["pressurePa"])
    d, u0, dt0 = em["diameterM"], em["velocityMS"] * strength, em["temperatureC"] - ta
    q0 = em.get("vapourKgPerKg", qa)
    puff = em.get("periodS", 0) > 0 and em.get("dutyTo", 1) - em.get("dutyFrom", 0) < 1
    phi = math.radians(wind_dir_deg)
    wind = np.array([wind_ms * math.cos(phi), wind_ms * math.sin(phi)])
    cross = float(np.hypot(*wind))
    direction = math.radians(bone["angleDeg"] + em["directionDeg"])
    src = np.array([anchor["x"], anchor["y"]])
    vapour = em["carries"] == "vapour"

    def dil(ell, t):
        x = ell + cross * t
        return max(1.0, (x / (4 * d)) ** 3) if puff else max(1.0, x / (5 * d))

    def rel(t, ell):
        uj = u0 * min(1.0, (4 * d / max(ell, 1e-12)) ** 3) if puff else u0 * min(1.0, 6.2 * d / max(ell, 1e-12))
        vb = G * dt0 / (ta + 273.15) * t / dil(ell, t)
        return uj * math.cos(direction), uj * math.sin(direction) + vb

    def rhs(t, y):
        ux, uy = rel(t, y[2])
        return [wind[0] + ux, wind[1] + uy, math.hypot(ux, uy)]

    needed = max(abs(dt0) / 0.2 if abs(dt0) >= 0.2 else 1.0, 50.0 if abs(q0 - qa) > 0 else 1.0)
    thr = 0.005 if vapour else 0.05

    def slow(t, y):
        return math.hypot(*rel(t, y[2])) - thr

    def mixed(t, y):
        return needed - dil(y[2], t)

    def reach(t, y):
        return 10.0 - math.hypot(y[0] - src[0], y[1] - src[1])

    events = [slow, mixed, reach]
    for e in events:
        e.terminal = True
    sol = solve_ivp(rhs, (0, 60), [src[0], src[1], 0.0], method="RK45", rtol=1e-10, atol=1e-12, events=events, dense_output=True)
    t_end = sol.t[-1]
    ts = np.linspace(0, t_end, 400)
    ref = sol.sol(ts)[:2].T
    length = float(np.hypot(*np.diff(ref, axis=0).T).sum())
    tail = solve_ivp(rhs, (t_end, t_end * 1.15 + 0.05), sol.y[:, -1], method="RK45", rtol=1e-10, atol=1e-12, dense_output=True)
    ext = tail.sol(np.linspace(t_end, tail.t[-1], 40))[:2].T
    reference = np.vstack([ref, ext[1:]])
    return {"reference": [[float(a), float(b)] for a, b in reference], "endpoint": [float(ref[-1][0]), float(ref[-1][1])], "length": length}


CASES = {"calm": (0.0, 0.0), "wind-right": (3.0, 0.0), "wind-left": (3.0, 180.0)}


def compute(fixture):
    return {name: plume(fixture, w, deg) for name, (w, deg) in CASES.items()}


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = {"cases": computed, "environments": {k: {"windMS": w, "windDirectionDeg": deg} for k, (w, deg) in CASES.items()}}
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for name, c in computed.items():
            assert np.allclose(c["endpoint"], fixture["expected"]["cases"][name]["endpoint"], atol=1e-6), name
    for name, c in computed.items():
        print("exhaust-wind:", name, c["endpoint"], round(c["length"], 3))
