"""🍃️ Independent oracle of leaf transpiration and a passive sunlit panel: scipy brentq on the energy balance with CoolProp
saturation pressure and CoolProp air conductivity, viscosity and density at the film temperature.

Run `uv run --with scipy --with CoolProp python -I 🐍️.py [--write]`.
"""
import json
import math
import sys
from pathlib import Path

from CoolProp.CoolProp import PropsSI
from scipy.constants import Stefan_Boltzmann as SIGMA, g as GRAVITY
from scipy.optimize import brentq

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🍃️leaf-transpiration" / "🔣️.json"
PR, SC = 0.71, 0.6


def psat(tc):
    return PropsSI("P", "T", tc + 273.15, "Q", 0, "Water")


def qsat(tc, p):
    e = psat(tc)
    return 0.622 * e / (p - 0.378 * e)


def balance(region, material, env, solar_factor):
    ta, tm, p, v = env["airTempC"], env["meanRadiantTempC"], env["pressurePa"], env["windMS"]
    pa = env["relativeHumidity"] * psat(ta)
    qa = 0.622 * pa / (p - 0.378 * pa)
    rho = (p - pa) / (287.058 * (ta + 273.15)) + pa / (461.495 * (ta + 273.15))
    length = region["characteristicLengthM"]
    absorbed = material.get("solarAbsorptivity", 0) * env["solarWM2"] * solar_factor

    def hc_of(ts):
        tf = (ts + ta) / 2 + 273.15
        k, mu, rho_f = (PropsSI(x, "T", tf, "P", p, "Air") for x in ("L", "V", "D"))
        nu = mu / rho_f
        dt = abs(ts - ta)
        ra = GRAVITY * dt * length**3 * PR / (tf * nu**2)
        nat = 0.0 if dt == 0 else (1.42 * (dt / length) ** 0.25 if ra < 1e9 else 1.31 * dt ** (1 / 3))
        re = v * length / nu
        nu_f = 0.0 if re == 0 else (0.664 * math.sqrt(re) if re < 5e5 else 0.037 * re**0.8 - 871) * PR ** (1 / 3)
        return max(nat, nu_f * k / length)

    def latent(ts, hc):
        if region["model"] != "leaf":
            return 0.0
        gb = hc / (rho * 1006) * (PR / SC) ** (1 / 3)
        gs = region["stomatalConductanceMS"]
        lam = 2.501e6 - 2361 * ts
        return max(0.0, lam * rho * gs * gb / (gs + gb) * (qsat(ts, p) - qa))

    def parts(ts):
        hc = hc_of(ts)
        return material["emissivity"] * SIGMA * ((ts + 273.15) ** 4 - (tm + 273.15) ** 4), hc * (ts - ta), latent(ts, hc), hc

    def g(ts):
        qr, qc, qe, _ = parts(ts)
        return absorbed - qr - qc - qe

    ts = brentq(g, min(ta, tm) - 15, max(ta, tm) + 120, xtol=1e-12)
    qr, qc, qe, hc = parts(ts)
    return {"surfaceTempC": ts, "radiantFluxWM2": qr, "convectiveFluxWM2": qc, "latentFluxWM2": qe, "netFluxWM2": qr + qc + qe - absorbed, "convectionCoefficientWM2K": hc}


def compute(fixture):
    mats = {m["id"]: m for m in fixture["asset"]["materials"]}
    factor = {"foliage": 0.5, "surface": 1.0}
    return {
        name: {r["id"]: balance(r, mats[r["material"]], env, factor[r["kind"]]) for r in fixture["asset"]["thermal"]["regions"]}
        for name, env in fixture["environments"].items()
    }


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for name, regs in computed.items():
            for rid, vals in regs.items():
                assert abs(vals["surfaceTempC"] - fixture["expected"][name][rid]["surfaceTempC"]) < 1e-6
    print(json.dumps(computed, indent=1))
