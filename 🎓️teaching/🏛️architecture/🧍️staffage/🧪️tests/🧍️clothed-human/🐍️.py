"""🧍️ Independent oracle of the clothed human: scipy brentq on the ISO 7730 clothing balance, CoolProp saturation pressure, ASHRAE respiration,
and a PMV cross-check of the solved clothing temperature against pythermalcomfort.

Run `uv run --with scipy --with CoolProp --with pythermalcomfort python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

from CoolProp.CoolProp import PropsSI
from pythermalcomfort.models import pmv_ppd_iso
from scipy.constants import Stefan_Boltzmann as SIGMA
from scipy.optimize import brentq

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🧍️clothed-human" / "🔣️.json"


def psat(tc):
    return PropsSI("P", "T", tc + 273.15, "Q", 0, "Water")


def solve(region, eps, env, p0=101325.0):
    ta, tr, v = env["airTempC"], env["meanRadiantTempC"], env["windMS"]
    pa = env["relativeHumidity"] * psat(ta)
    w = region.get("wettedness", 0.06 if region["kind"] == "skin" else 0.0)

    def parts(ts):
        hc = max(2.38 * abs(ts - ta) ** 0.25, 12.1 * v**0.5)
        qr = eps * SIGMA * ((ts + 273.15) ** 4 - (tr + 273.15) ** 4)
        qe = max(0.0, w * 16.5 * hc * (p0 / env["pressurePa"]) * (psat(ts) - pa) / 1000)
        return qr, hc * (ts - ta), qe, hc

    core, r = region["coreTempC"], region["insulationM2KW"]

    def g(ts):
        qr, qc, qe, _ = parts(ts)
        return (core - ts) / r - qr - qc - qe

    ts = brentq(g, min(ta, tr, core) - 1, max(ta, tr, core) + 1, xtol=1e-12)
    qr, qc, qe, hc = parts(ts)
    return {"surfaceTempC": ts, "radiantFluxWM2": qr, "convectiveFluxWM2": qc, "latentFluxWM2": qe, "netFluxWM2": (core - ts) / r, "convectionCoefficientWM2K": hc}


def compute(fixture):
    env = fixture["asset"]["thermal"]["environment"]
    mats = {m["id"]: m for m in fixture["asset"]["materials"]}
    regions = {r["id"]: solve(r, mats[r["material"]]["emissivity"], env) for r in fixture["asset"]["thermal"]["regions"]}
    dims = fixture["asset"]["dimensions"]
    area = 0.202 * dims["massKg"] ** 0.425 * dims["heightM"] ** 0.725
    m = fixture["state"]["thermal"]["activityMet"] * 58.15
    pa_kpa = env["relativeHumidity"] * psat(env["airTempC"]) / 1000
    totals = {"emitterSensibleW": 0.0014 * m * (34 - env["airTempC"]) * area, "emitterLatentW": 0.0173 * m * (5.87 - pa_kpa) * area}
    return {"regions": regions, "totals": totals}


def pmv_check(fixture, expected):
    iso = fixture["iso7730"]
    env = fixture["asset"]["thermal"]["environment"]
    m = iso["met"] * 58.15
    ta, tr = iso["tdb"], iso["tr"]
    pa = iso["rh"] * (2.718281828459045 ** (16.6536 - 4030.183 / (ta + 235))) * 1000
    icl = 0.155 * iso["clo"]
    fcl = 1.05 + 0.645 * icl
    c = expected["regions"]["clothing"]
    hc, tcl = c["convectionCoefficientWM2K"], c["surfaceTempC"]
    loss = (
        m
        - 3.05e-3 * (5733 - 6.99 * m - pa)
        - 0.42 * (m - 58.15)
        - 1.7e-5 * m * (5867 - pa)
        - 0.0014 * m * (34 - ta)
        - 3.96e-8 * fcl * ((tcl + 273) ** 4 - (tr + 273) ** 4)
        - fcl * hc * (tcl - ta)
    )
    pmv = (0.303 * 2.718281828459045 ** (-0.036 * m) + 0.028) * loss
    ref = pmv_ppd_iso(tdb=ta, tr=tr, vr=iso["vr"], rh=iso["rh"] * 100, met=iso["met"], clo=iso["clo"], wme=0, model="7730-2005", limit_inputs=False, round_output=False)
    return float(pmv), float(ref.pmv)


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    computed = compute(fixture)
    mine, ref = pmv_check(fixture, computed)
    assert abs(mine - ref) <= fixture["tolerances"]["pmv"], (mine, ref)
    if "--write" in sys.argv:
        fixture["expected"] = {**computed, "pmv": ref}
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        for rid, vals in computed["regions"].items():
            assert abs(vals["surfaceTempC"] - fixture["expected"]["regions"][rid]["surfaceTempC"]) < 1e-6
    print(json.dumps(computed), "pmv(solved t_cl)", mine, "pythermalcomfort", ref)
