"""🍃️ Independent oracle of leaf transpiration: scipy brentq on the leaf energy balance (absorbed sunlight, two-faced longwave exchange, mixed natural/forced convection,
Jarvis-type stomatal control, Penman–Monteith series conductance) with CoolProp saturation pressure and CoolProp air conductivity, viscosity and density at the film temperature, plus
the closed-form Penman–Monteith solution of the linearised balance (Monteith & Unsworth: isothermal net radiation, radiative conductance added to the heat conductance); it does not apply where dew forms (`null`),
because condensation bypasses the stomata.

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
PR, SC, CP = 0.71, 0.6, 1006.0
LEAVES = [
    {"id": "c3-leaf", "photosynthesis": "c3", "gmax": 0.008, "lai": 3.0, "alpha": 0.5, "eps": 0.97, "length": 0.08},
    {"id": "c4-leaf", "photosynthesis": "c4", "gmax": 0.010, "lai": 3.0, "alpha": 0.5, "eps": 0.97, "length": 0.25},
    {"id": "cam-leaf", "photosynthesis": "cam", "gmax": 0.004, "lai": 2.0, "alpha": 0.6, "eps": 0.97, "length": 0.3},
]
CASES = [
    {"id": "hot-sunny-dry", "env": {"airTempC": 32, "meanRadiantTempC": 32, "skyTempC": 21, "groundTempC": 40, "relativeHumidity": 0.35, "windMS": 1.5, "windDirectionDeg": 0, "solarWM2": 850, "diffuseFraction": 0.12, "pressurePa": 101325}, "sunlit": 1.0},
    {"id": "shaded-humid", "env": {"airTempC": 22, "meanRadiantTempC": 22, "skyTempC": 12, "groundTempC": 22, "relativeHumidity": 0.85, "windMS": 0.5, "windDirectionDeg": 0, "solarWM2": 500, "diffuseFraction": 0.3, "pressurePa": 101325}, "sunlit": 0.0},
    {"id": "clear-night", "env": {"airTempC": 15, "meanRadiantTempC": 15, "skyTempC": -6, "groundTempC": 14, "relativeHumidity": 0.7, "windMS": 0.3, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}, "sunlit": 0.0},
    {"id": "windy-cool-sun", "env": {"airTempC": 12, "meanRadiantTempC": 12, "skyTempC": -5, "groundTempC": 14, "relativeHumidity": 0.6, "windMS": 6, "windDirectionDeg": 0, "solarWM2": 700, "diffuseFraction": 0.2, "pressurePa": 101325}, "sunlit": 1.0},
]


def psat(tc):
    return PropsSI("P", "T", tc + 273.15, "Q", 0, "Water")


def stomata(leaf, env, sunlit):
    q = env["solarWM2"] * 0.5 * (sunlit + env["diffuseFraction"] * (1 - sunlit))
    light = min(1.0, q / (q + 100.0) * (1 + 100.0 / 500.0))
    if leaf["photosynthesis"] == "cam":
        return leaf["gmax"] * (0.05 + 0.95 * (1 - light))
    d0 = 3.0 if leaf["photosynthesis"] == "c4" else 2.0
    deficit = psat(env["airTempC"]) * (1 - env["relativeHumidity"]) / 1000
    vpd = min(1.0, (1 + 0.5 / d0) / (1 + deficit / d0))
    return leaf["gmax"] * max(0.02, light * vpd)


def radiant_environment(env, sky_view=0.7, ground_share=0.5):
    ground = (1 - sky_view) * ground_share
    surroundings = 1 - sky_view - ground
    t = lambda c: c + 273.15
    return (sky_view * t(env["skyTempC"]) ** 4 + ground * t(env["groundTempC"]) ** 4 + surroundings * t(env["meanRadiantTempC"]) ** 4) ** 0.25 - 273.15


def convection(ts, ta, p, speed, length):
    tf = (ts + ta) / 2 + 273.15
    k, mu, rho, pr = (PropsSI(x, "T", tf, "P", p, "Air") for x in ("L", "V", "D", "Prandtl"))
    nu, alpha = mu / rho, k / (rho * PropsSI("C", "T", tf, "P", p, "Air"))
    dt = ts - ta
    ra = GRAVITY * abs(dt) * length**3 / (tf * nu * alpha)
    psi = (1 + (0.492 / pr) ** (9 / 16)) ** (8 / 27)
    natural = 0.0 if ra <= 0 else (0.825 + 0.387 * ra ** (1 / 6) / psi) ** 2 * k / length
    re = speed * length / nu
    forced = 0.0 if re <= 0 else (0.664 * math.sqrt(re) if re < 5e5 else 0.037 * re**0.8 - 871) * pr ** (1 / 3) * k / length
    return (natural**3 + forced**3) ** (1 / 3)


def solve(leaf, env, sunlit):
    ta, p, v = env["airTempC"], env["pressurePa"], env["windMS"]
    pa = env["relativeHumidity"] * psat(ta)
    qa = 0.622 * pa / (p - 0.378 * pa)
    rho = (p - pa) / (287.058 * (ta + 273.15)) + pa / (461.495 * (ta + 273.15))
    tenv = radiant_environment(env)
    shortwave = leaf["alpha"] * env["solarWM2"] * 0.5 * (sunlit + env["diffuseFraction"] * (1 - sunlit))
    gs = stomata(leaf, env, sunlit)

    def parts(ts):
        h = convection(ts, ta, p, v, leaf["length"])
        radiative = 2 * leaf["eps"] * SIGMA * ((ts + 273.15) ** 4 - (tenv + 273.15) ** 4)
        sensible = 2 * h * (ts - ta)
        gb = h / (rho * CP) * (PR / SC) ** (2 / 3)
        es = psat(ts)
        qs = 0.622 * es / (p - 0.378 * es)
        g = gb if qs < qa else gs * gb / (gs + gb)
        latent = (2.501e6 - 2361 * ts) * rho * g * (qs - qa)
        return radiative, sensible, latent, h, g

    def balance(ts):
        r, s, l, *_ = parts(ts)
        return shortwave - r - s - l

    ts = brentq(balance, ta - 25, ta + 45, xtol=1e-12)
    radiative, sensible, latent, h, g = parts(ts)
    return {"surfaceTempC": ts, "netRadiativeWM2": radiative, "sensibleWM2": sensible, "latentWM2": latent, "absorbedSolarWM2": shortwave, "stomatalMS": gs, "convectionCoefficientWM2K": 2 * h, "moistureGM2H": latent / (2.501e6 - 2361 * ts) * 3.6e6, "pmLinearised": None if latent < 0 else linearised(leaf, env, shortwave, gs, h, rho, tenv)}


def linearised(leaf, env, shortwave, gs, h, rho, tenv):
    ta, p = env["airTempC"], env["pressurePa"]
    pa = env["relativeHumidity"] * psat(ta)
    eps = leaf["eps"]
    s = (psat(ta + 0.005) - psat(ta - 0.005)) / 0.01
    lam = 2.501e6 - 2361 * ta
    gamma = CP * p / (0.622 * lam)
    iso = shortwave - 2 * eps * SIGMA * ((ta + 273.15) ** 4 - (tenv + 273.15) ** 4)
    g_h = 2 * h / (rho * CP)
    g_r = 2 * 4 * eps * SIGMA * (ta + 273.15) ** 3 / (rho * CP)
    gb = h / (rho * CP) * (PR / SC) ** (2 / 3)
    g_v = gs * gb / (gs + gb)
    deficit = psat(ta) - pa
    return (s * iso + rho * CP * (g_h + g_r) * deficit) / (s + gamma * (1 + (g_h + g_r) / g_v))


def asset():
    return {
        "dimensions": {"heightM": 1, "massKg": 0},
        "materials": [{"id": f"leaf-{l['id']}", "title": {"en": l["id"], "de": l["id"]}, "emissivity": l["eps"], "solarAbsorptivity": l["alpha"], "source": "illustrative"} for l in LEAVES],
        "thermal": {
            "environment": CASES[0]["env"],
            "passengers": [],
            "operating": [],
            "emitters": [],
            "assumptions": [],
            "regions": [
                {"id": l["id"], "title": {"en": l["id"], "de": l["id"]}, "kind": "foliage", "model": "leaf", "material": f"leaf-{l['id']}", "areaM2": 1.0, "characteristicLengthM": l["length"], "geometry": "vertical-plate", "skyViewFactor": 0.7, "layers": [], "heatCapacityJM2K": 800, "leaf": {"stomatalConductanceMaxMS": l["gmax"], "leafAreaIndex": l["lai"], "photosynthesis": l["photosynthesis"]}, "source": "illustrative"}
                for l in LEAVES
            ],
        },
    }


def build():
    cases = []
    for case in CASES:
        cases.append({"id": case["id"], "environment": case["env"], "sunlit": case["sunlit"], "expected": {l["id"]: solve(l, case["env"], case["sunlit"]) for l in LEAVES}})
    return {
        "id": "leaf-transpiration",
        "oracle": "scipy brentq energy balance with CoolProp (IAPWS-95 saturation pressure, air properties); closed-form Penman-Monteith of the linearised balance",
        "tolerances": {"surfaceTempK": 0.2, "fluxRelative": 0.03, "fluxAbsoluteWM2": 2.0, "pmRelative": 0.15, "pmAbsoluteWM2": 3.0},
        "asset": asset(),
        "cases": cases,
    }


if __name__ == "__main__":
    document = build()
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        stored = json.loads(FIXTURE.read_text(encoding="utf-8"))
        for case, ref in zip(document["cases"], stored["cases"]):
            for lid, values in case["expected"].items():
                assert abs(values["surfaceTempC"] - ref["expected"][lid]["surfaceTempC"]) < 1e-6, (case["id"], lid)
    for case in document["cases"]:
        for lid, v in case["expected"].items():
            print(case["id"], lid, {k: (None if x is None else round(x, 3)) for k, x in v.items()})
