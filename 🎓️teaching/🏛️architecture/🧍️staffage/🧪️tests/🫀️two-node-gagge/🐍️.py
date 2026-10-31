"""🫀️ Independent oracle of the two-node thermoregulation: pythermalcomfort `two_nodes_gagge` (Gagge, Stolwijk & Nishi 1971; ASHRAE 55 implementation), 60 minutes from the neutral state.

The fixture builds a one-region trunk body from the same inputs the library takes (clothing insulation `0.155 clo`, area factor `1 + 0.15 clo`, vapour resistance `f_cl · r_clo / (0.0165 i_cl)`
with the library's permeation efficiency 0.45, radiation `ε·f_eff = 0.95 · 0.73`, skin and core heat capacities of 0.97 Wh/(kg K), respiratory ventilation from the library's dry loss
`0.0014 M (34 − t_a)`), so the comparison isolates the controller and the heat balance. Run `uv run --with pythermalcomfort python -I 🐍️.py [--write]`.
"""
import json
import sys
from pathlib import Path

from pythermalcomfort.models import two_nodes_gagge

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🫀️two-node-gagge" / "🔣️.json"
AREA = 1.8258
MASS = 70.0
MET_FACTOR = 58.2
CASES = [
    {"id": "neutral", "tdb": 24.0, "v": 0.1, "rh": 50.0, "met": 1.0, "clo": 0.6},
    {"id": "warm", "tdb": 28.0, "v": 0.1, "rh": 50.0, "met": 1.0, "clo": 0.5},
    {"id": "hot-sweating", "tdb": 35.0, "v": 0.2, "rh": 50.0, "met": 1.2, "clo": 0.4},
    {"id": "cool", "tdb": 18.0, "v": 0.1, "rh": 50.0, "met": 1.0, "clo": 1.0},
    {"id": "cold-windy", "tdb": 10.0, "v": 1.0, "rh": 60.0, "met": 1.2, "clo": 1.5},
    {"id": "walking", "tdb": 24.0, "v": 0.1, "rh": 50.0, "met": 2.0, "clo": 0.6},
    {"id": "humid-working", "tdb": 30.0, "v": 0.2, "rh": 80.0, "met": 1.5, "clo": 0.5},
]


def environment(case):
    return {"airTempC": case["tdb"], "meanRadiantTempC": case["tdb"], "relativeHumidity": case["rh"] / 100, "windMS": case["v"], "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325}


def asset(case):
    clo = case["clo"]
    r_clo = 0.155 * clo
    f_cl = 1 + 0.15 * clo
    layers = []
    if clo > 0:
        layers = [{"id": "clothing", "title": {"en": "Clothing ensemble", "de": "Bekleidungsensemble"}, "kind": "clothing", "material": "clothing", "resistanceM2KW": r_clo * f_cl, "vapourResistanceM2PaW": f_cl * r_clo / (0.0165 * 0.45), "source": "literature"}]
    metabolic = case["met"] * MET_FACTOR * AREA
    flow_m3s = 0.0014 * case["met"] * MET_FACTOR * AREA / (1.2 * 1006)
    rate = 15.0
    return {
        "dimensions": {"heightM": 1.75, "massKg": MASS},
        "materials": [{"id": "clothing", "title": {"en": "Clothing, effective radiation", "de": "Bekleidung, effektive Strahlung"}, "emissivity": 0.95 * 0.73, "solarAbsorptivity": 0.5, "source": "literature"}],
        "thermal": {
            "environment": environment(case),
            "baseMet": 1,
            "body": {"species": "human", "massKg": MASS, "skinAreaM2": AREA, "basalMetabolismW": 0, "metWM2": MET_FACTOR, "coreSetPointC": 36.8, "skinSetPointC": 33.7, "sweating": True, "panting": False, "maxSkinBloodFlowLM2H": 90, "minSkinBloodFlowLM2H": 0.5, "source": "literature"},
            "respiration": {
                "noseEmitter": "nose",
                "exhaledTempC": 34,
                "exhaledRelativeHumidity": 0.95,
                "patterns": [{"id": "p", "title": {"en": "Matched to the library", "de": "An die Bibliothek angepasst"}, "rateBpm": rate, "tidalVolumeL": flow_m3s * 1000 * 60 / rate, "inhaleFraction": 0.4, "pauseAfterInhaleFraction": 0.1, "exhaleFraction": 0.4, "pauseAfterExhaleFraction": 0.1, "route": "nose", "source": "illustrative"}],
                "motion": [],
                "source": "illustrative",
            },
            "passengers": [],
            "operating": [],
            "emitters": [],
            "assumptions": [],
            "regions": [{"id": "body", "title": {"en": "Whole body", "de": "Ganzer Körper"}, "kind": "clothing", "model": "body", "material": "clothing", "areaM2": AREA * f_cl, "characteristicLengthM": 0.2, "geometry": "cylinder", "skyViewFactor": 0.5, "layers": layers, "heatCapacityJM2K": 0.97 * 3600 * 0.1 * MASS / (AREA * f_cl), "body": {"segment": "trunk", "metabolicShare": 1, "skinFraction": 1}, "source": "literature"}],
        },
        "_metabolicW": metabolic,
    }


def compute(case):
    r = two_nodes_gagge(tdb=case["tdb"], tr=case["tdb"], v=case["v"], rh=case["rh"], met=case["met"], clo=case["clo"], round_output=False)
    keys = ["t_core", "t_skin", "w", "m_bl", "e_skin", "q_sensible", "q_res", "e_max", "m_rsw", "set"]
    return {k: float(getattr(r, k)) for k in keys}


def build():
    out = []
    for case in CASES:
        a = asset(case)
        a.pop("_metabolicW")
        out.append({"id": case["id"], "inputs": case, "asset": a, "state": {"timeS": 3600, "thermal": {"activityMet": case["met"], "externalWorkW": 0, "operating": {}, "regions": {"body": {"exposedFraction": 1, "sunlitFraction": 0}}, "emitters": {}, "breathing": "p"}, "anchors": {}, "bones": {}}, "expected": compute(case)})
    return out


if __name__ == "__main__":
    cases = build()
    document = {
        "id": "two-node-gagge",
        "oracle": "pythermalcomfort 4.x two_nodes_gagge (Gagge, Stolwijk & Nishi 1971), 60 min from neutral, round_output=False",
        "tolerances": {"coreK": 0.15, "skinK": 1.0, "wettedness": 0.05, "bloodFlowRelative": 0.2, "bloodFlowAbsolute": 2.0, "respiratoryWM2": 1.0, "sensibleWM2": 6.0, "evaporationWM2": 3.0},
        "cases": cases,
    }
    if "--write" in sys.argv:
        FIXTURE.write_text(json.dumps(document, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        stored = json.loads(FIXTURE.read_text(encoding="utf-8"))
        for case, ref in zip(cases, stored["cases"]):
            for key, value in case["expected"].items():
                assert abs(value - ref["expected"][key]) < 1e-6, (case["id"], key, value, ref["expected"][key])
    for case in cases:
        print(case["id"], {k: round(v, 3) for k, v in case["expected"].items()})
