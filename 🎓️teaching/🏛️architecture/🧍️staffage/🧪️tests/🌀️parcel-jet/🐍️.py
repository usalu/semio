"""🌀️ Independent oracle of the parcel model: re-integrates the documented parcel ODE (slice jet/plume entrainment, momentum and enthalpy/total-water mixing, buoyancy with liquid loading, saturation adjustment) with scipy's RK45 (rtol 1e-10) and an independent psychrometric implementation (scipy brentq).

Run `uv run --with scipy --with CoolProp --with numpy python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy.integrate import solve_ivp
from scipy.optimize import brentq

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "🧫️fixtures" / "🌀️parcel-jet" / "🔣️.json"
SCENARIOS = ROOT / "🧫️fixtures" / "💨️transport" / "🔣️.json"
G, CPA, CPV, CPW, L0, RD, EPS = 9.80665, 1006.0, 1860.0, 4186.0, 2.501e6, 287.05, 0.622
STEPS = 120


def psat(t):
    return 611.21 * math.exp((18.678 - t / 234.5) * t / (257.14 + t))


def ws(t, p):
    e = psat(t)
    return math.inf if e >= p else EPS * e / (p - e)


def enth(t, wv, wl):
    return CPA * t + wv * (L0 + CPV * t) + wl * CPW * t


def moist(h, wt, p):
    tu = (h - wt * L0) / (CPA + wt * CPV)
    if wt <= ws(tu, p):
        return tu, wt, 0.0
    hi = brentq(lambda x: psat(x) - p, 50, 150)
    dew = brentq(lambda x: ws(x, p) - wt if x < hi else 1e9, tu - 1e-9, hi - 1e-9, xtol=1e-13, rtol=1e-14)
    t = brentq(lambda x: enth(x, ws(x, p), wt - ws(x, p)) - h, tu - 1e-9, dew, xtol=1e-13, rtol=1e-14)
    return t, ws(t, p), wt - ws(t, p)


def vol(t, wv, p):
    return RD * (t + 273.15) * (1 + wv / EPS) / p


def describe(s, amb, tau, sigma, ap, aj):
    x, y, ux, uy, md, h, wt = s
    t, wv, wl = moist(h, wt, amb["p"])
    v = vol(t, wv, amb["p"])
    volume = md * v
    speed = math.hypot(ux, uy)
    ue = math.hypot(speed, sigma)
    b = math.sqrt(volume / (math.pi * ue * tau))
    buoy = G * ((1 + amb["w"]) * v / amb["v"] - (1 + wt)) / (1 + wt)
    ri = abs(buoy) * b / ue**2
    alpha = aj + (ap - aj) * min(1.0, ri / (8 * ap / 5))
    dmd = 2 * alpha * ue * volume / b / amb["v"]
    return t, wv, wl, b, buoy, dmd


def compute(fixture):
    scenarios = json.loads(SCENARIOS.read_text(encoding="utf-8"))
    env = scenarios["environments"]["calm-cold"]
    p, ta, rh = env["pressurePa"], env["airTempC"], env["relativeHumidity"]
    wa = EPS * rh * psat(ta) / (p - rh * psat(ta))
    amb = {"p": p, "w": wa, "v": vol(ta, wa, p), "h": enth(ta, wa, 0.0)}
    sigma = max(fixture["turbulence"]["intensity"] * env["windMS"], fixture["turbulence"]["floorMS"])
    aj, ap = fixture["alphaJet"], fixture["alphaPlume"]
    base = scenarios["assets"]["machine"]["thermal"]["emitters"][0]
    anchor = scenarios["assets"]["machine"]["anchors"][0]
    kin = scenarios["kinematics"]["machine"]
    out = {}
    for case in fixture["cases"]:
        em = {**base, **case["emitter"]}
        step_count = 8
        tau = step_count / STEPS
        t_birth = (4) / STEPS
        tc = ta + em["temperatureRiseK"]
        wv0 = wa if "vapourKgPerKg" not in em else em["vapourKgPerKg"] / (1 - em["vapourKgPerKg"])
        wv = wa + (wv0 - wa)
        q = em["volumeFlowM3S"]
        area = math.pi * em["diameterM"] ** 2 / 4
        speed0 = q * tau / tau / area
        direction = math.radians(em["directionDeg"])
        s0 = [kin["rootX"]["x0"] + kin["pivotX"] + anchor["x"], kin["pivotY"] + anchor["y"], speed0 * math.cos(direction), speed0 * math.sin(direction), q * tau / vol(tc, wv, p), enth(tc, wv, 0.0), wv]

        def rhs(_t, s):
            t, wvv, wl, b, buoy, dmd = describe(s, amb, tau, sigma, ap, aj)
            rate = dmd / s[4]
            mom = dmd * (1 + wa) / (s[4] * (1 + s[6]))
            return [s[2], s[3], -mom * s[2], buoy - mom * s[3], dmd, rate * (amb["h"] - s[5]), rate * (wa - s[6])]

        ages = case["ages"]
        sol = solve_ivp(rhs, (0, max(ages)), s0, method="RK45", rtol=1e-10, atol=1e-13, t_eval=ages)
        rows = []
        for age, s in zip(ages, sol.y.T):
            t, wvv, wl, b, _, _ = describe(s, amb, tau, sigma, ap, aj)
            q_parcel = wvv / (1 + wvv)
            rows.append({"ageS": age, "x": s[0], "y": s[1], "ux": s[2], "uy": s[3], "radiusM": b, "excessK": t - ta, "vapourExcessKgPerKg": q_parcel - wa / (1 + wa), "condensateKgPerKg": wl / (1 + s[6])})
        out[case["id"]] = rows
    return out


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = json.loads(json.dumps(computed))
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for key, rows in computed.items():
            assert np.allclose([r["x"] for r in rows], [r["x"] for r in fixture["expected"][key]], atol=1e-9), "drift " + key
    for key, rows in computed.items():
        print("parcel-jet:", key, [(r["ageS"], round(r["x"], 4), round(r["radiusM"], 4), round(r["excessK"], 3), round(r["condensateKgPerKg"] * 1000, 4)) for r in rows])
