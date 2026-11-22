"""🌊️ Independent oracle of the flow field: builds the fixture's plume and jet from the classical similarity solutions (Morton, Taylor & Turner top-hat plume; top-hat round jet), re-implements the documented field (wind, Gaussian mean flow of the slices, entrainment sinks with ground images and far-group aggregation) with numpy, integrates the continuous entrainment line sink with scipy `quad`, the volume flux of the Gaussian profile with numpy, the streams with scipy `solve_ivp` (DOP853) in arc length and the advection of a particle in time.

Run `UV_PROJECT_ENVIRONMENT=<machine cache>/staffage-oracles/geometry uv run --project 🧪️tests/🧰️support/🐍️oracles --locked --only-group geometry python -I 🐍️.py [--write]` (the `geometry` oracle set: numpy, scipy).
"""
import json
import math
import sys
from pathlib import Path

import numpy as np
from scipy.integrate import quad, solve_ivp

FIXTURE = Path(__file__).resolve().parents[2] / "🧫️fixtures" / "🌊️flow-field" / "🔣️.json"
G = 9.80665
ALPHA_JET, ALPHA_PLUME = 0.076, 0.116
RI_PLUME = 8 * ALPHA_PLUME / 5
CUT, GROUP, FAR, PASSIVE = 3.5, 12, 6.0, 0.01
STEPS = {"breath": 4, "plume": 12, "transpiration": 60}


def tau_of(kind):
    return STEPS.get(kind, 8) / 120


def plume_parcels(case, ambient):
    """Top-hat MTT plume of buoyancy flux F0 above a point source at the ground: b = 6a/5 z, w = 5/(6a) (9 a F0 / (10 pi))^(1/3) z^(-1/3), g' = 5 F0 / (6 a pi) (9 a F0 / (10 pi))^(-1/3) z^(-5/3)."""
    a, f0, tau = ALPHA_PLUME, case["buoyancyFluxM4S3"], tau_of("plume")
    k = (9 * a * f0 / (10 * math.pi)) ** (1 / 3)
    out, z, age, index = [], case["fromM"], 0.0, 0
    while z <= case["toM"]:
        w = 5 / (6 * a) * k * z ** (-1 / 3)
        b = 6 * a / 5 * z
        reduced = 5 * f0 / (6 * a * math.pi) / k * z ** (-5 / 3)
        out.append({"id": f"{case['emitter']}#{index}", "emitter": case["emitter"], "bornS": 0, "ageS": round(age, 9), "at": {"x": 0, "y": round(z, 9), "z": 0}, "velocity": {"x": 0, "y": round(w, 12), "z": 0}, "radiusM": round(b, 12), "excessK": round(reduced * (ambient["airTempC"] + 273.15) / G, 12), "vapourExcessKgPerKg": 0, "condensateKgPerKg": 0, "carries": "airflow"})
        step = math.hypot(w, ambient["sigmaMS"]) * tau
        z += step
        age += step / w
        index += 1
    return out


def jet_parcels(case, ambient):
    """Top-hat round jet: b = r0 + 2 a x, u = U0 r0 / b, excess ~ r0 / b."""
    a, r0, u0, tau = ALPHA_JET, case["radiusM"], case["speedMS"], tau_of("fan")
    out, x, age, index = [], case["fromM"], 0.0, 0
    while x <= case["toM"]:
        b = r0 + 2 * a * x
        u = u0 * r0 / b
        out.append({"id": f"{case['emitter']}#{index}", "emitter": case["emitter"], "bornS": 0, "ageS": round(age, 9), "at": {"x": round(x, 9), "y": case["heightM"], "z": 0}, "velocity": {"x": round(u, 12), "y": 0, "z": 0}, "radiusM": round(b, 12), "excessK": round(case["excessK"] * r0 / b, 12), "vapourExcessKgPerKg": round(case["vapourExcessKgPerKg"] * r0 / b, 15), "condensateKgPerKg": 0, "carries": "airflow"})
        step = math.hypot(u, ambient["sigmaMS"]) * tau
        x += step
        age += step / u
        index += 1
    return out


class Field:
    """The documented field of one case, written from the module text, not from the product code."""

    def __init__(self, fixture, case):
        amb = fixture["ambient"]
        phi = math.radians(amb["windDirectionDeg"])
        self.wind = np.array([amb["windMS"] * math.cos(phi), 0.0, amb["windMS"] * math.sin(phi)])
        kinds = {e["id"]: e["kind"] for e in fixture["emitters"]}
        self.plumes = []
        streams = {}
        for p in case["parcels"]:
            streams.setdefault(p["id"].rsplit("#", 1)[0], []).append(p)
        for parcels in streams.values():
            rows = []
            for p in parcels:
                v = np.array([p["velocity"]["x"], p["velocity"]["y"], p["velocity"]["z"]])
                rel = v - self.wind
                speed = float(np.linalg.norm(rel))
                ue = math.hypot(speed, amb["sigmaMS"])
                b = max(p["radiusM"], 1e-3)
                buoyancy = G * (p["excessK"] / (amb["airTempC"] + 273.15) + 0.608 * p["vapourExcessKgPerKg"] - p["condensateKgPerKg"])
                ri = abs(buoyancy) * p["radiusM"] / ue**2
                alpha = ALPHA_JET + (ALPHA_PLUME - ALPHA_JET) * min(1.0, ri / RI_PLUME)
                length = ue * tau_of(kinds[p["emitter"]])
                axis = float(np.linalg.norm(v))
                passive = speed < PASSIVE or axis == 0
                rows.append({"at": np.array([p["at"]["x"], p["at"]["y"], p["at"]["z"]]), "u": rel, "d": np.array([0.0, 1.0, 0.0]) if passive else v / axis, "b": b, "h": max(length, b / 2), "q": 2 * math.pi * alpha * b * length * speed, "t": p["excessK"], "wv": p["vapourExcessKgPerKg"], "wl": p["condensateKgPerKg"], "passive": passive})
            groups = []
            for i in range(0, len(rows), GROUP):
                chunk = rows[i : i + GROUP]
                q = sum(r["q"] for r in chunk)
                centre = sum(r["q"] * r["at"] for r in chunk) / q if q > 0 else np.zeros(3)
                radius = max(float(np.linalg.norm(r["at"] - centre)) + r["b"] for r in chunk)
                reach = [CUT * max(r["b"], r["h"]) for r in chunk]
                lo = np.min([r["at"] - e for r, e in zip(chunk, reach)], axis=0)
                hi = np.max([r["at"] + e for r, e in zip(chunk, reach)], axis=0)
                groups.append({"rows": chunk, "q": q, "centre": centre, "far2": (FAR * radius) ** 2, "lo": lo, "hi": hi})
            self.plumes.append(groups)

    @staticmethod
    def sink(point, at, q, floor):
        out = np.zeros(3)
        for mirror in (1.0, -1.0):
            d = point - np.array([at[0], mirror * at[1], at[2]])
            r = max(float(np.linalg.norm(d)), floor)
            out += -q * d / (4 * math.pi * r**3)
        return out

    def sample(self, point):
        point = np.asarray(point, dtype=float)
        v = self.wind.copy()
        scalars = np.zeros(3)
        for groups in self.plumes:
            a, s, c = 0.0, np.zeros(3), np.zeros(3)
            for g in groups:
                if g["q"] > 0:
                    d = point - g["centre"]
                    if float(d @ d) > g["far2"]:
                        v += self.sink(point, g["centre"], g["q"], 0.0)
                    else:
                        for r in g["rows"]:
                            if r["q"] != 0:
                                v += self.sink(point, r["at"], r["q"], r["b"])
                if np.any(point < g["lo"]) or np.any(point > g["hi"]):
                    continue
                for r in g["rows"]:
                    d = point - r["at"]
                    r2 = float(d @ d)
                    if r["passive"]:
                        if r2 > (CUT * r["b"]) ** 2:
                            continue
                        w, profile = math.exp(-2 * r2 / r["b"] ** 2), 1.0
                    else:
                        along = float(d @ r["d"])
                        across2 = max(0.0, r2 - along**2)
                        if abs(along) > CUT * r["h"] or across2 > (CUT * r["b"]) ** 2:
                            continue
                        w, profile = math.exp(-(along**2) / (2 * r["h"] ** 2)), 2 * math.exp(-2 * across2 / r["b"] ** 2)
                    a += w
                    s += w * profile * r["u"]
                    c += w * profile * np.array([r["t"], r["wv"], r["wl"]])
            if a > 0:
                gain = min(1.0, a) / a
                v += gain * s
                scalars += gain * c
        return v, scalars


def line_sink(case, point):
    """Radial inflow of the continuous plume: sinks of strength 2 pi b(z) a w(z) per metre of axis between the first and last slice, with their images, integrated by scipy quad."""
    a, f0 = ALPHA_PLUME, case["buoyancyFluxM4S3"]
    k = (9 * a * f0 / (10 * math.pi)) ** (1 / 3)
    r, y = math.hypot(point[0], point[2]), point[1]
    ys = [p["at"]["y"] for p in case["parcels"]]
    half = [math.hypot(p["velocity"]["y"], 0.03) * tau_of("plume") / 2 for p in (case["parcels"][0], case["parcels"][-1])]

    def strength(z):
        return 2 * math.pi * (6 * a / 5 * z) * a * (5 / (6 * a) * k * z ** (-1 / 3))

    def radial(z, mirror):
        d = math.hypot(r, y - mirror * z)
        return strength(z) * r / (4 * math.pi * d**3)

    return sum(quad(radial, ys[0] - half[0], ys[-1] + half[1], args=(m,), epsabs=1e-13, epsrel=1e-11)[0] for m in (1.0, -1.0))


def compute(fixture):
    out = {}
    for case in fixture["cases"]:
        field = Field(fixture, case)
        samples = []
        for at in case["points"]:
            v, s = field.sample(at)
            samples.append({"at": at, "u": v.tolist(), "inducedMS": float(np.linalg.norm(v - field.wind)), "excessK": float(s[0]), "vapourExcessKgPerKg": float(s[1])})
        result = {"samples": samples}
        if case["kind"] == "plume":
            result["lineSink"] = [{"at": at, "radialInflowMS": line_sink(case, at)} for at in case["entrainmentPoints"]]
            flux = []
            for z in case["fluxHeightsM"]:
                radii = np.linspace(0.0, 4 * 6 * ALPHA_PLUME / 5 * z, 4001)
                w = np.array([field.sample([r, z, 0.0])[0][1] for r in radii[::40]])
                rr = radii[::40]
                a, f0 = ALPHA_PLUME, case["buoyancyFluxM4S3"]
                k = (9 * a * f0 / (10 * math.pi)) ** (1 / 3)
                flux.append({"heightM": z, "sampledM3S": float(np.trapezoid(w * 2 * math.pi * rr, rr)), "similarityM3S": math.pi * (6 * a / 5 * z) ** 2 * 5 / (6 * a) * k * z ** (-1 / 3)})
            result["flux"] = flux
        streams = []
        step, count = fixture["stream"]["stepM"], fixture["stream"]["points"]
        for seed in case["seeds"]:
            def by_arc(_s, state):
                v, _ = field.sample(state[:3])
                speed = float(np.linalg.norm(v))
                return [*(v / speed), 1 / speed]

            arc = solve_ivp(by_arc, (0, step * count), [*seed, 0.0], method="DOP853", t_eval=np.arange(1, count + 1) * step, rtol=1e-10, atol=1e-12)
            def by_time(_t, state):
                return field.sample(state)[0]

            times = fixture["stream"]["advectS"]
            moved = solve_ivp(by_time, (0, times[-1]), seed, method="DOP853", t_eval=times, rtol=1e-10, atol=1e-12)
            streams.append({"seed": seed, "points": arc.y[:3].T.tolist(), "travelS": arc.y[3].tolist(), "advected": [{"timeS": t, "at": moved.y[:, i].tolist()} for i, t in enumerate(times)]})
        result["streams"] = streams
        out[case["id"]] = result
    return out


def generate(fixture):
    for case in fixture["cases"]:
        case["parcels"] = plume_parcels(case, fixture["ambient"]) if case["kind"] == "plume" else jet_parcels(case, fixture["ambient"])


if __name__ == "__main__":
    fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
    if "--write" in sys.argv:
        generate(fixture)
    computed = compute(fixture)
    if "--write" in sys.argv:
        fixture["expected"] = computed
        FIXTURE.write_text(json.dumps(fixture, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    else:
        worst = 0.0
        for case_id, result in computed.items():
            for mine, theirs in zip(result["samples"], fixture["expected"][case_id]["samples"]):
                worst = max(worst, *(abs(a - b) for a, b in zip(mine["u"], theirs["u"])))
        print(json.dumps({"worstVelocityDifferenceMS": worst}))
        sys.exit(0 if worst < 1e-9 else 1)
