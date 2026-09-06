"""🏛️ The EnergyPlus / ANSI-ASHRAE-140 oracle role for the `s.energy.model` simulation case.

Two jobs, both reference-side, neither importing anything from the Rust it judges.

1. `case-parameters-<case>` — an INDEPENDENT re-derivation of every quantity ANSI/ASHRAE 140 §5.2
   states outright, read straight out of the committed `🔋️model.json`: surface areas from the raw
   `vertices_m` polygons by Newell's formula, air-to-air U-values from the layer thicknesses and
   conductivities plus the standard's own film resistances, glazing area, zone volume, infiltration
   rate and internal gain. It then asserts those against the values §5.2 publishes, so a case model
   that drifts away from the standard is caught before any simulation is run and without either
   side's simulator having an opinion. This is the part of the oracle that needs no EnergyPlus at
   all, and it is deliberately the first thing that runs.

2. `annual-energy-<case>` / `peak-load-<case>` / `free-float-<case>` / `hourly-temperature-<case>` —
   the committed `🔮️energyplus.json` for that case, read literally. Producing that document is the
   honeybee-energy → honeybee-openstudio → OpenStudio → EnergyPlus pipeline's job, run out of band
   against the SAME committed `🔋️model.json`; this adapter never re-authors a case from the
   standard and never runs a simulator of its own, so an agreement here is an agreement about
   physics rather than about two readings of §5.2.

⚠️ A case whose `🔮️energyplus.json` has not been produced yet raises. The host has no skip channel,
and answering with an empty or synthesized document would manufacture the one outcome worse than a
red: a green that means nothing.
"""

from __future__ import annotations

# region 🔖️Imports
import json
import math

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Standard
#: 📏️ Film resistances [m²·K/W] the air-to-air U-values in §5.2 are quoted against.
R_FILM_INTERIOR = 0.13
R_FILM_EXTERIOR = 0.04

#: 🏛️ What ANSI/ASHRAE 140 §5.2 states directly for the base geometry, shared by every case.
FLOOR_AREA_M2 = 48.0
ZONE_VOLUME_M3 = 129.6
INFILTRATION_ACH = 0.5
INTERNAL_GAIN_W = 200.0
GLAZING_AREA_M2 = 12.0
WINDOW_U_W_M2K = 3.0
GROUND_TEMPERATURE_C = 10.0

#: 🏛️ The published air-to-air U-values [W/(m²·K)], lightweight (600) and high-mass (900) sets.
LIGHT_U = {"wall": 0.514, "roof": 0.318, "floor": 0.039}
HEAVY_U = {"wall": 0.512, "roof": 0.318, "floor": 0.039}

#: 🏛️ Cases whose walls and floor are the high-mass set.
HEAVY_CASES = {"900", "900FF", "910", "920", "930", "940", "950"}

#: 🏛️ Cases with no HVAC at all.
FREE_FLOAT_CASES = {"600FF", "900FF"}

#: 🏛️ The conditioned cases with a committed EnergyPlus reference. 630/930 and 650/950 are absent
#: on purpose — the oracle has not translated fins or night ventilation, and pretending otherwise
#: would register scenarios that can only fail for want of a reference.
CONDITIONED_CASES = ["600", "610", "620", "640", "900", "910", "920", "940"]
PEAK_CASES = ["600", "900"]
HOURLY_CASES = ["600", "600FF", "900FF"]
#: 🏛️ Every case whose MODEL is committed — the parameter cross-check needs no simulator, so it
#: covers the four cases the oracle cannot yet run as well.
ALL_CASES = ["600", "600FF", "610", "620", "630", "640", "650", "900", "900FF", "910", "920", "930", "940", "950"]
# endregion 🔖️Standard


# region 🔖️Geometry
def _newell_normal(vertices):
    """📐️ Outward normal of a planar polygon by Newell's formula, independent of vertex count."""
    nx = ny = nz = 0.0
    count = len(vertices)
    for index in range(count):
        x0, y0, z0 = vertices[index]
        x1, y1, z1 = vertices[(index + 1) % count]
        nx += (y0 - y1) * (z0 + z1)
        ny += (z0 - z1) * (x0 + x1)
        nz += (x0 - x1) * (y0 + y1)
    return (nx, ny, nz)


def _polygon_area_m2(vertices):
    """📐️ Planar polygon area as half the magnitude of the Newell vector."""
    nx, ny, nz = _newell_normal(vertices)
    return 0.5 * math.sqrt(nx * nx + ny * ny + nz * nz)


def _orientation(vertices):
    """🧭️ Tilt from horizontal and azimuth clockwise from north."""
    nx, ny, nz = _newell_normal(vertices)
    length = math.sqrt(nx * nx + ny * ny + nz * nz)
    if length <= 0.0:
        return (0.0, 0.0)
    nx, ny, nz = nx / length, ny / length, nz / length
    tilt = math.degrees(math.acos(max(-1.0, min(1.0, nz))))
    azimuth = math.degrees(math.atan2(nx, ny)) % 360.0
    return (tilt, azimuth)
# endregion 🔖️Geometry


# region 🔖️Derivation
def _by_id(items):
    return {item["id"]: item for item in items}


def _u_value(model, construction_id):
    """🔥️ Air-to-air U-value [W/(m²·K)] from the layer stack plus both film resistances."""
    constructions = _by_id(model["constructions"])
    materials = _by_id(model["materials"])
    construction = constructions.get(construction_id)
    resistance = R_FILM_INTERIOR + R_FILM_EXTERIOR
    if construction is not None:
        for layer_id in construction["layer_material_ids"]:
            layer = materials[layer_id]
            resistance += layer["thickness_m"] / layer["conductivity_w_m_k"]
    return 1.0 / resistance


def derive_parameters(case, model):
    """📐️ Everything §5.2 states, derived from the committed model and nothing else."""
    surfaces = []
    for surface in model["surfaces"]:
        gross = _polygon_area_m2(surface["vertices_m"])
        glazed = sum(window["area_m2"] for window in model["fenestrations"] if window["surface_id"] == surface["id"])
        tilt, azimuth = _orientation(surface["vertices_m"])
        surfaces.append(
            {
                "name": surface["name"],
                "grossAreaM2": gross,
                "netOpaqueAreaM2": gross - glazed,
                "uValueWM2K": _u_value(model, surface["construction_id"]),
                "tiltDeg": tilt,
                "azimuthDeg": azimuth,
            }
        )
    windows = [
        {
            "name": window["name"],
            "areaM2": window["area_m2"],
            "uValueWM2K": window["u_value_w_m2k"],
            "shgc": window["shgc"],
            "overhangDepthM": window["overhang_depth_m"],
            "finDepthM": window["fin_depth_m"],
        }
        for window in model["fenestrations"]
    ]
    zone = model["zones"][0]
    infiltration = model["infiltrations"][0] if model["infiltrations"] else {"design_flow_ach": 0.0}
    return {
        "schema": "semio.energy.bestest-parameters/1",
        "case": case,
        "zoneVolumeM3": zone["volume_m3"],
        "floorAreaM2": FLOOR_AREA_M2,
        "infiltrationAch": infiltration["design_flow_ach"],
        "internalGainW": sum(gain["watts_per_area"] * FLOOR_AREA_M2 for gain in model["equipment"]),
        "conditioned": bool(model["ideal_loads"]),
        "groundTemperatureC": model["ground_temperature"]["building_surface_c"][0],
        "surfaces": surfaces,
        "windows": windows,
    }


def _close(actual, expected, tolerance, what):
    assert abs(actual - expected) <= tolerance, f"{what}: derived {actual!r}, ANSI/ASHRAE 140 §5.2 states {expected!r}"


def assert_matches_the_standard(case, derived):
    """🏛️ The independent half: hold the derived quantities against §5.2's own published values."""
    _close(derived["zoneVolumeM3"], ZONE_VOLUME_M3, 1e-6, f"case {case} zone volume")
    _close(derived["floorAreaM2"], FLOOR_AREA_M2, 1e-6, f"case {case} floor area")
    _close(derived["infiltrationAch"], INFILTRATION_ACH, 1e-9, f"case {case} infiltration rate")
    _close(derived["internalGainW"], INTERNAL_GAIN_W, 1e-6, f"case {case} internal gain")
    _close(derived["groundTemperatureC"], GROUND_TEMPERATURE_C, 1e-9, f"case {case} ground temperature")
    assert derived["conditioned"] == (case not in FREE_FLOAT_CASES), f"case {case} conditioning does not match §5.2"

    published = HEAVY_U if case in HEAVY_CASES else LIGHT_U
    for surface in derived["surfaces"]:
        name = surface["name"].lower()
        key = "roof" if "roof" in name else "floor" if "floor" in name else "wall"
        _close(surface["uValueWM2K"], published[key], 0.01, f"case {case} {surface['name']} U-value")

    glazing = sum(window["areaM2"] for window in derived["windows"])
    _close(glazing, GLAZING_AREA_M2, 1e-6, f"case {case} total glazing area")
    for window in derived["windows"]:
        _close(window["uValueWM2K"], WINDOW_U_W_M2K, 1e-9, f"case {case} {window['name']} U-value")

    envelope = sum(surface["grossAreaM2"] for surface in derived["surfaces"])
    _close(envelope, 2 * 48.0 + 2 * 21.6 + 2 * 16.2, 1e-6, f"case {case} total envelope area")
# endregion 🔖️Derivation


# region 🔖️Handlers
def _model_uri(case):
    return f"asset://🧫️fixtures/🏛️bestest-{case}/🔋️model.json"


def _reference_uri(case):
    return f"asset://🧫️fixtures/🏛️bestest-{case}/🔮️energyplus.json"


def _parameters_for(case):
    def handler(ctx: Context) -> Outcome:
        model = json.loads(ctx.fixture_bytes(_model_uri(case)).decode("utf-8"))
        derived = derive_parameters(case, model)
        assert_matches_the_standard(case, derived)
        return Outcome(projection=derived, raw=json.dumps(derived, sort_keys=True, separators=(",", ":")).encode("utf-8"))

    return handler


def _reference_for(case):
    def handler(ctx: Context) -> Outcome:
        uri = _reference_uri(case)
        try:
            raw = ctx.fixture_bytes(uri)
        except Exception as error:  # noqa: BLE001 — the message is the whole point
            raise AssertionError(
                f"case {case}: no committed EnergyPlus reference at {uri} ({error}). "
                "Run the honeybee-energy → honeybee-openstudio → OpenStudio → EnergyPlus pipeline "
                "against the committed 🔋️model.json and commit its result document; this adapter "
                "will not synthesize one."
            ) from error
        document = json.loads(raw.decode("utf-8"))
        assert document.get("schema") == "semio.energy.bestest-results/1", f"case {case}: the committed reference declares schema {document.get('schema')!r}"
        assert document.get("case") == case, f"case {case}: the committed reference is for case {document.get('case')!r}"
        return Outcome(projection=document, raw=raw)

    return handler
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables.
    Oracle role only — registering these as subjects too would make the reference its own subject
    and manufacture a guaranteed-green self-comparison."""
    built = Adapter("python")
    for case in ALL_CASES:
        built = built.oracle(f"case-parameters-{case}", _parameters_for(case))
    for case in CONDITIONED_CASES:
        built = built.oracle(f"annual-energy-{case}", _reference_for(case))
    for case in PEAK_CASES:
        built = built.oracle(f"peak-load-{case}", _reference_for(case))
    for case in sorted(FREE_FLOAT_CASES):
        built = built.oracle(f"free-float-{case}", _reference_for(case))
    for case in HOURLY_CASES:
        built = built.oracle(f"hourly-temperature-{case}", _reference_for(case))
    return built
# endregion 🔖️Registration
