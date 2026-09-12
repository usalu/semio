"""🔮️ The third-party physics reference for `s.energy.model` — Honeybee → OpenStudio → EnergyPlus.

WHAT THIS IS. `✏️s/🔌️plugins/🔋️energy` ships its own building-energy solver (~19k lines of Rust under
`🔨️modules/⚡️simulation/⚙️engine`). Nothing in this file imports, reads or transliterates that Rust.
It reads the same INPUT — a semio `Model` serialised with the engine's own `ToValue` wire shape —
translates it into a Honeybee model, lets NREL's OpenStudio SDK translate that to an EnergyPlus IDF,
runs EnergyPlus 25.2.0 on it, and reports the annual/peak/hourly numbers in the shape both producers
agreed on (`📓️bestest-contract.md`). EnergyPlus is the reference implementation ANSI/ASHRAE Standard
140 itself is used to qualify, which is what makes it a real oracle for a heat-balance solver rather
than a second opinion.

TWO PRODUCERS, ON PURPOSE. `translate` runs whatever `🔋️model.json` the engine side authored, so a
disagreement is attributable to the physics. `native` builds ASHRAE 140 §5.2 cases from the
standard's own geometry and layer tables instead, so a disagreement between `native` and `translate`
on the same case number is attributable to the TRANSLATION. Running both is the only way to tell
those two failure modes apart, and the two paths deliberately share only the simulation and
result-reading code below the `_simulate` boundary.

WHERE THE ASHRAE 140 NUMBERS COME FROM. The case constructions, glazing, gains, infiltration and
control setpoints in `_CASES` are transcribed by hand from NREL's own BESTEST-GSR encoding of
Standard 140 §5.2 (`shared_resources/bestest_resources.osm`,
`shared_resources/Bestest_Geo_South_12_0_0.osm` and `measures/…envelope…/measure.rb`, fetched
2026-09-06 from https://github.com/NREL/BESTEST-GSR) rather than paraphrased from the standard's
copyrighted text. The weather year is that repository's own `725650TYCST.epw`, committed once at
`../../../../🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🌦️denver-tmy/🌦️.epw`.

TWO THINGS THE SEMIO SCHEMA DOES NOT SAY, DECLARED RATHER THAN GUESSED SILENTLY:

* `Fenestration` carries an AREA and no geometry, so a translated aperture's rectangle is not
  determined by the input. `_add_apertures` therefore places `n` equal rectangles of the given total
  area with a fixed, documented rule — width/height `_APERTURE_ASPECT`, sill `_APERTURE_SILL_M`, one
  window centred per equal horizontal bay — which happens to reproduce ASHRAE 140's own two 3.0 m ×
  2.0 m south windows exactly for the 600 series. Override with `--aperture-aspect`/`--sill-height`.
* `Construction.layer_material_ids` has no stated order. This file reads it OUTSIDE→INSIDE (the
  EnergyPlus convention); `--layer-order inside-out` reverses it. Note that the engine's own
  `precompute` takes a surface's solar absorptance from the LAST id in the list, which is only the
  exterior layer under the opposite reading — a real ambiguity for the engine side to settle.

LICENSES. EnergyPlus and the OpenStudio SDK are BSD-3-Clause. `ladybug-*`/`honeybee-*` are
AGPL-3.0-only and are used here strictly as an out-of-process validation reference: nothing in this
package is imported by, linked into, or reachable from any shipped semio artifact.

@see ../🛠️toolchain/🔣️.json — the pinned, sha256-verified toolchain this module runs against.
@see ../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️bestest-contract.md
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import sys
import tempfile
from pathlib import Path

RESULT_SCHEMA = "semio.energy.bestest-results/1"
ENERGYPLUS_VERSION = "25.2.0"
PRODUCER_VIA = "honeybee-energy 1.123.32 → honeybee-openstudio 0.7.2 → OpenStudio 3.11.0"
HOURS_PER_YEAR = 8760

_APERTURE_ASPECT = 1.5
_APERTURE_SILL_M = 0.2
_ROUGHNESS = "Rough"

_HEATING_OUTPUT = "Zone Ideal Loads Supply Air Total Heating Energy"
_COOLING_OUTPUT = "Zone Ideal Loads Supply Air Total Cooling Energy"
_TEMPERATURE_OUTPUT = "Zone Mean Air Temperature"
_SOLAR_OUTPUT = "Surface Window Transmitted Solar Radiation Energy"


def _oracle_root() -> Path:
    """🗂️ The extracted, sha256-verified OpenStudio tree `📜️script.ts setup` provisioned."""
    root = os.environ.get("SEMIO_ORACLE_OPENSTUDIO_ROOT")
    if not root:
        raise SystemExit("SEMIO_ORACLE_OPENSTUDIO_ROOT is unset — run the oracle-setup target first")
    path = Path(root)
    if not (path / "EnergyPlus" / "energyplus").exists():
        raise SystemExit(f"no EnergyPlus binary under {path} — run the oracle-setup target first")
    return path


def _bind_toolchain() -> Path:
    """🔗️ Points honeybee at the repository cache instead of `~/ladybug_tools` or `/Applications`.

    `honeybee_energy.config.Folders` has no environment-variable hook, so the documented way to
    redirect it is to assign the two properties before any run function is imported or called.
    """
    root = _oracle_root()
    from honeybee_energy.config import folders as hbe_folders

    hbe_folders.energyplus_path = str(root / "EnergyPlus")
    hbe_folders.openstudio_path = str(root / "bin")
    return root


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _weather_label(epw: Path) -> str:
    parts = epw.resolve().parts
    if "🧫️fixtures" in parts:
        return "/".join(parts[parts.index("🧫️fixtures"):])
    return epw.name


# ── ASHRAE 140 §5.2 case data (hand-transcribed from NREL/BESTEST-GSR, see module docstring) ──

_MATERIALS = {
    "WOOD_SIDING": (0.009, 0.14, 530.0, 900.0),
    "FIBERGLASS_QUILT_WALL": (0.066, 0.04, 12.0, 840.0),
    "PLASTERBOARD_WALL": (0.012, 0.16, 950.0, 840.0),
    "ROOF_DECK": (0.019, 0.14, 530.0, 900.0),
    "FIBERGLASS_QUILT_ROOF": (0.1118, 0.04, 12.0, 840.0),
    "PLASTERBOARD_ROOF": (0.01, 0.16, 950.0, 840.0),
    "TIMBER_FLOORING": (0.025, 0.14, 650.0, 1200.0),
    "CONCRETE_BLOCK": (0.1, 0.51, 1400.0, 1000.0),
    "FOAM_INSULATION": (0.0615, 0.04, 10.0, 1400.0),
    "CONCRETE_SLAB": (0.08, 1.13, 1400.0, 1000.0),
}
_NO_MASS = {"R25_INSULATION_LT": 25.075, "R25_INSULATION_HW": 25.175}
_LT_LAYERS = {
    "wall": ["WOOD_SIDING", "FIBERGLASS_QUILT_WALL", "PLASTERBOARD_WALL"],
    "roof": ["ROOF_DECK", "FIBERGLASS_QUILT_ROOF", "PLASTERBOARD_ROOF"],
    "floor": ["R25_INSULATION_LT", "TIMBER_FLOORING"],
}
_HW_LAYERS = {
    "wall": ["WOOD_SIDING", "FOAM_INSULATION", "CONCRETE_BLOCK"],
    "roof": ["ROOF_DECK", "FIBERGLASS_QUILT_ROOF", "PLASTERBOARD_ROOF"],
    "floor": ["R25_INSULATION_HW", "CONCRETE_SLAB"],
}
_GLASS = (0.003048, 0.834, 0.075, 0.91325, 0.082, 0.0, 0.84, 0.84, 1.0)
_GAS = (0.012, "Air")

_WIDTH_M, _DEPTH_M, _HEIGHT_M = 8.0, 6.0, 2.7
_GAIN_W = 200.0
_GAIN_RADIANT_FRACTION = 0.6
_INFILTRATION_ACH = 0.5

_CASES = {
    "600": {"mass": "LT", "glazing": {180.0: 12.0}, "heating_c": 20.0, "cooling_c": 27.0},
    "600FF": {"mass": "LT", "glazing": {180.0: 12.0}, "heating_c": None, "cooling_c": None},
    "610": {"mass": "LT", "glazing": {180.0: 12.0}, "heating_c": 20.0, "cooling_c": 27.0, "overhang_m": 1.0},
    "620": {"mass": "LT", "glazing": {90.0: 6.0, 270.0: 6.0}, "heating_c": 20.0, "cooling_c": 27.0},
    "640": {"mass": "LT", "glazing": {180.0: 12.0}, "heating_c": "setback", "cooling_c": 27.0},
    "900": {"mass": "HW", "glazing": {180.0: 12.0}, "heating_c": 20.0, "cooling_c": 27.0},
    "900FF": {"mass": "HW", "glazing": {180.0: 12.0}, "heating_c": None, "cooling_c": None},
    "910": {"mass": "HW", "glazing": {180.0: 12.0}, "heating_c": 20.0, "cooling_c": 27.0, "overhang_m": 1.0},
    "920": {"mass": "HW", "glazing": {90.0: 6.0, 270.0: 6.0}, "heating_c": 20.0, "cooling_c": 27.0},
    "940": {"mass": "HW", "glazing": {180.0: 12.0}, "heating_c": "setback", "cooling_c": 27.0},
}
_SETBACK_TIMESTEP = 6
_SETBACK_VALUES = [10.0] * 42 + [11.67, 13.33, 15.0, 16.67, 18.33] + [20.0] * 91 + [10.0] * 6


def _energy_material(identifier, spec):
    from honeybee_energy.material.opaque import EnergyMaterial

    thickness, conductivity, density, specific_heat = spec
    return EnergyMaterial(identifier, thickness, conductivity, density, specific_heat, _ROUGHNESS, 0.9, 0.6, 0.6)


def _no_mass_material(identifier, r_value):
    from honeybee_energy.material.opaque import EnergyMaterialNoMass

    return EnergyMaterialNoMass(identifier, r_value, _ROUGHNESS, 0.9, 0.6, 0.6)


def _bestest_layer(identifier):
    if identifier in _NO_MASS:
        return _no_mass_material(identifier, _NO_MASS[identifier])
    return _energy_material(identifier, _MATERIALS[identifier])


def _bestest_construction_set(mass):
    """🧱️ The `BESTEST LT` / `BESTEST HW` construction set, layers ordered outside→inside."""
    from honeybee_energy.construction.opaque import OpaqueConstruction
    from honeybee_energy.construction.window import WindowConstruction
    from honeybee_energy.constructionset import ConstructionSet
    from honeybee_energy.material.gas import EnergyWindowMaterialGas
    from honeybee_energy.material.glazing import EnergyWindowMaterialGlazing

    layers = _LT_LAYERS if mass == "LT" else _HW_LAYERS
    glass = EnergyWindowMaterialGlazing(f"BESTEST_GLASS_{mass}", *_GLASS)
    gas = EnergyWindowMaterialGas(f"BESTEST_AIR_GAP_{mass}", *_GAS)
    construction_set = ConstructionSet(f"BESTEST_{mass}")
    construction_set.wall_set.exterior_construction = OpaqueConstruction(f"BESTEST_{mass}_WALL", [_bestest_layer(name) for name in layers["wall"]])
    construction_set.roof_ceiling_set.exterior_construction = OpaqueConstruction(f"BESTEST_{mass}_ROOF", [_bestest_layer(name) for name in layers["roof"]])
    construction_set.floor_set.exterior_construction = OpaqueConstruction(f"BESTEST_{mass}_FLOOR", [_bestest_layer(name) for name in layers["floor"]])
    construction_set.aperture_set.window_construction = WindowConstruction(f"BESTEST_{mass}_WINDOW", [glass, gas, glass])
    return construction_set


def _wall_by_azimuth(room, azimuth):
    for face in room.faces:
        if face.type.name == "Wall" and abs(face.horizontal_orientation() - azimuth) % 360.0 < 1.0:
            return face
    raise SystemExit(f"no wall facing {azimuth}° on room {room.identifier}")


def _add_apertures(face, total_area, count, aspect, sill):
    """🪟️ Places `count` equal rectangles totalling `total_area` on `face` by the documented rule."""
    if total_area <= 0.0 or count <= 0:
        return
    face_width = face.geometry.max.distance_to_point(face.geometry.min)
    width = max(face.geometry.boundary_polygon2d.max.x - face.geometry.boundary_polygon2d.min.x, 1e-6)
    height = max(face.geometry.boundary_polygon2d.max.y - face.geometry.boundary_polygon2d.min.y, 1e-6)
    each = total_area / count
    aperture_height = min((each / aspect) ** 0.5, 0.95 * (height - sill))
    aperture_width = min(each / aperture_height, 0.95 * width / count)
    aperture_height = each / aperture_width
    del face_width
    face.apertures_by_ratio_rectangle(total_area / face.area, aperture_height, sill, width / count, 0.0, 0.01)


def _constant_schedule(identifier, value, limit_name):
    from honeybee_energy.lib.scheduletypelimits import schedule_type_limit_by_identifier
    from honeybee_energy.schedule.ruleset import ScheduleRuleset

    return ScheduleRuleset.from_constant_value(identifier, value, schedule_type_limit_by_identifier(limit_name))


def _daily_schedule(identifier, values, limit_name, timestep=1):
    """📅️ A schedule repeating one day, `values` given at `timestep` steps per hour."""
    from honeybee_energy.lib.scheduletypelimits import schedule_type_limit_by_identifier
    from honeybee_energy.schedule.ruleset import ScheduleRuleset

    return ScheduleRuleset.from_daily_values(identifier, values, timestep, schedule_type_limit_by_identifier(limit_name))


def _ideal_air_system(identifier, heating_c, cooling_c):
    """❄️ Unlimited-capacity ideal loads with humidity control off, per ASHRAE 140 §5.2."""
    from honeybee.altnumber import no_limit
    from honeybee_energy.hvac.idealair import IdealAirSystem

    system = IdealAirSystem(identifier)
    system.economizer_type = "NoEconomizer"
    system.demand_controlled_ventilation = False
    system.sensible_heat_recovery = 0
    system.latent_heat_recovery = 0
    system.heating_limit = no_limit
    system.cooling_limit = no_limit
    system.heating_air_temperature = 50.0
    system.cooling_air_temperature = 13.0
    del heating_c, cooling_c
    return system


def _native_model(case):
    """🏛️ Builds one ASHRAE 140 §5.2 case directly from the standard's own geometry and layers."""
    from ladybug_geometry.geometry3d.pointvector import Point3D
    from honeybee.boundarycondition import Outdoors
    from honeybee.model import Model
    from honeybee.room import Room
    from honeybee_energy.load.equipment import ElectricEquipment
    from honeybee_energy.load.infiltration import Infiltration
    from honeybee_energy.load.setpoint import Setpoint
    from honeybee_energy.load.ventilation import Ventilation

    spec = _CASES.get(case)
    if spec is None:
        raise SystemExit(f"unknown ASHRAE 140 case {case!r}; known: {', '.join(sorted(_CASES))}")
    room = Room.from_box(f"ZONE_ONE", _WIDTH_M, _DEPTH_M, _HEIGHT_M, 0, Point3D(0, 0, 0))
    for face in room.faces:
        face.boundary_condition = Outdoors(True, True)
    room[0].boundary_condition = Outdoors(False, False, 0)
    for azimuth, area in spec["glazing"].items():
        wall = _wall_by_azimuth(room, azimuth)
        _add_apertures(wall, area, 2 if area > 6.0 else 1, _APERTURE_ASPECT, _APERTURE_SILL_M)
        if spec.get("overhang_m"):
            wall.overhang(spec["overhang_m"], 0, False, 0.01)
    always_on = _constant_schedule("BESTEST_ALWAYS_ON", 1.0, "Fractional")
    volume = _WIDTH_M * _DEPTH_M * _HEIGHT_M
    exterior_area = sum(face.area for face in room.faces)
    room.properties.energy.construction_set = _bestest_construction_set(spec["mass"])
    room.properties.energy.electric_equipment = ElectricEquipment("BESTEST_INTERNAL_GAIN", _GAIN_W / (_WIDTH_M * _DEPTH_M), always_on, _GAIN_RADIANT_FRACTION, 0.0, 0.0)
    room.properties.energy.infiltration = Infiltration("BESTEST_INFILTRATION", _INFILTRATION_ACH * volume / 3600.0 / exterior_area, always_on)
    room.properties.energy.ventilation = Ventilation("BESTEST_NO_OUTDOOR_AIR", 0.0, 0.0, 0.0, 0.0)
    if spec["heating_c"] is not None:
        heating = _daily_schedule("BESTEST_HEATING_SETBACK", _SETBACK_VALUES, "Temperature", _SETBACK_TIMESTEP) if spec["heating_c"] == "setback" else _constant_schedule("BESTEST_HEATING_SETPOINT", spec["heating_c"], "Temperature")
        cooling = _constant_schedule("BESTEST_COOLING_SETPOINT", spec["cooling_c"], "Temperature")
        room.properties.energy.setpoint = Setpoint("BESTEST_SETPOINT", heating, cooling)
        room.properties.energy.hvac = _ideal_air_system("BESTEST_IDEAL_LOADS", spec["heating_c"], spec["cooling_c"])
    return Model(f"BESTEST_{case}", [room], units="Meters", tolerance=0.01, angle_tolerance=1.0)


# ── semio `Model` → honeybee translation ──

_SURFACE_TYPES = {
    "ExteriorWall": "wall",
    "InteriorWall": "wall",
    "Roof": "roof_ceiling",
    "Ceiling": "roof_ceiling",
    "Floor": "floor",
    "Interzone": "wall",
    "Adiabatic": "wall",
    "Ground": "floor",
}


def _face_type(surface_class):
    """🧭️ Maps a semio `SurfaceClass` onto one of honeybee's four face-type singletons."""
    from honeybee.facetype import face_types

    return getattr(face_types, _SURFACE_TYPES[surface_class])


def _translated_material(entry):
    """🧱️ One semio `Material` as a honeybee layer; ρ·cp = 0 becomes a massless resistance layer.

    ASHRAE 140 tabulates its floor insulation with zero density and zero specific heat, which
    EnergyPlus rejects on a `Material` object and expresses as `Material:NoMass` instead — the same
    substitution NREL's own BESTEST-GSR encoding makes.
    """
    thickness = float(entry["thickness_m"])
    conductivity = float(entry["conductivity_w_m_k"])
    density = float(entry["density_kg_m3"])
    specific_heat = float(entry["specific_heat_j_kg_k"])
    identifier = f"SEMIO_MATERIAL_{entry['id']}"
    if density * specific_heat <= 0.0:
        return _no_mass_material(identifier, thickness / conductivity)
    material = _energy_material(identifier, (thickness, conductivity, density, specific_heat))
    material.thermal_absorptance = float(entry["thermal_absorptance"])
    material.solar_absorptance = float(entry["solar_absorptance"])
    material.visible_absorptance = float(entry["visible_absorptance"])
    return material


def _infiltration_per_exterior_area(entry, room):
    """💨️ One semio `Infiltration` as honeybee's single flow-per-exterior-area number.

    `ScheduledAch` is converted against the zone volume and the same exterior area EnergyPlus itself
    uses for `Flow/ExteriorArea` — the gross area of every surface whose outside boundary is the
    external environment, which is what `Zone Information` reports back. The wind/stack methods carry
    no honeybee equivalent and are refused rather than silently approximated.
    """
    method = entry.get("method", "PerExteriorArea")
    if method == "PerExteriorArea":
        return float(entry["flow_per_exterior_area_m3_s_m2"])
    if method == "ScheduledAch":
        exterior_area = sum(face.area for face in room.faces if face.boundary_condition.name == "Outdoors")
        if exterior_area <= 0.0:
            raise SystemExit(f"infiltration {entry['id']} uses ScheduledAch but zone {room.identifier} has no exterior surface")
        return float(entry["design_flow_ach"]) * room.volume / 3600.0 / exterior_area
    raise SystemExit(f"infiltration {entry['id']} uses method {method!r}, which has no honeybee equivalent")


def _schedule_library(schedules, schedule_kind):
    """📅️ Resolves a semio `ScheduleSet` into honeybee rulesets keyed by `ScheduleId`."""
    library = {}
    for entry in schedules.get("constants", []) or []:
        library[int(entry["id"])] = ("constant", float(entry["value"]))
    for entry in schedules.get("daily", []) or []:
        library[int(entry["id"])] = ("daily", [float(value) for value in entry["hourly_values"]])
    unsupported = [key for key in ("weekly", "annual", "time_series") if schedules.get(key)]
    if unsupported:
        raise SystemExit(f"the oracle translates constant and daily schedules only; model uses {', '.join(unsupported)}")
    resolved = {}
    for schedule_id, (kind, value) in library.items():
        limit = schedule_kind.get(schedule_id, "Fractional")
        identifier = f"SEMIO_SCHEDULE_{schedule_id}_{limit.upper()}"
        resolved[schedule_id] = _constant_schedule(identifier, value, limit) if kind == "constant" else _daily_schedule(identifier, value, limit)
    return resolved


def _schedule_kinds(model):
    """🏷️ Which honeybee type limit each referenced `ScheduleId` needs, from its referrer."""
    kinds = {}
    for thermostat in model.get("thermostats", []) or []:
        kinds[int(thermostat["heating_setpoint_schedule_id"])] = "Temperature"
        kinds[int(thermostat["cooling_setpoint_schedule_id"])] = "Temperature"
    for collection in ("people", "lighting", "equipment", "infiltrations", "mechanical_ventilations"):
        for entry in model.get(collection, []) or []:
            kinds.setdefault(int(entry["schedule_id"]), "Fractional")
    return kinds


def _boundary_condition(surface, face_by_surface_id):
    from honeybee.boundarycondition import Ground, Outdoors, boundary_conditions

    condition = surface["outside_boundary_condition"]
    if isinstance(condition, dict):
        other = face_by_surface_id.get(int(condition["Interzone"]))
        return ("interzone", other)
    if condition == "OutdoorAir":
        return ("bc", Outdoors(bool(surface["sun_exposed"]), bool(surface["wind_exposed"])))
    if condition == "Ground":
        return ("bc", Ground())
    return ("bc", boundary_conditions.adiabatic)


def _translated_model(document, layer_order, aspect, sill):
    """🔁️ Turns a semio `Model` (plus its `ScheduleSet`) into an equivalent honeybee `Model`."""
    from ladybug_geometry.geometry3d.face import Face3D
    from ladybug_geometry.geometry3d.pointvector import Point3D
    from honeybee.face import Face
    from honeybee.model import Model
    from honeybee.room import Room
    from honeybee.shade import Shade
    from honeybee_energy.construction.opaque import OpaqueConstruction
    from honeybee_energy.construction.window import WindowConstruction
    from honeybee_energy.load.equipment import ElectricEquipment
    from honeybee_energy.load.infiltration import Infiltration
    from honeybee_energy.load.lighting import Lighting
    from honeybee_energy.load.people import People
    from honeybee_energy.load.setpoint import Setpoint
    from honeybee_energy.load.ventilation import Ventilation
    from honeybee_energy.material.glazing import EnergyWindowMaterialSimpleGlazSys

    model = document["model"] if "model" in document and "zones" in document.get("model", {}) else document
    schedules = model.get("schedules") or document.get("schedules") or {}
    resolved_schedules = _schedule_library(schedules, _schedule_kinds(model))
    always_on = _constant_schedule("SEMIO_ALWAYS_ON", 1.0, "Fractional")

    materials = {int(entry["id"]): _translated_material(entry) for entry in model.get("materials", [])}
    constructions = {}
    for entry in model.get("constructions", []):
        layer_ids = [int(value) for value in entry["layer_material_ids"]]
        ordered = layer_ids if layer_order == "outside-in" else list(reversed(layer_ids))
        constructions[int(entry["id"])] = OpaqueConstruction(f"SEMIO_CONSTRUCTION_{entry['id']}", [materials[value] for value in ordered])

    fenestrations = {}
    for entry in model.get("fenestrations", []):
        fenestrations.setdefault(int(entry["surface_id"]), []).append(entry)

    rooms = []
    face_by_surface_id = {}
    for zone in model.get("zones", []):
        zone_id = int(zone["id"])
        faces = []
        for surface in model.get("surfaces", []):
            if int(surface["zone_id"]) != zone_id:
                continue
            geometry = Face3D([Point3D(*vertex) for vertex in surface["vertices_m"]])
            face = Face(f"SEMIO_SURFACE_{surface['id']}", geometry, _face_type(surface["class"]))
            face.properties.energy.construction = constructions[int(surface["construction_id"])]
            faces.append(face)
            face_by_surface_id[int(surface["id"])] = (face, surface)
        room = Room(f"SEMIO_ZONE_{zone_id}", faces, 0.01, 1.0)
        room.multiplier = int(zone["multiplier"])
        rooms.append(room)

    for surface_id, (face, surface) in face_by_surface_id.items():
        kind, value = _boundary_condition(surface, {key: pair[0] for key, pair in face_by_surface_id.items()})
        if kind == "bc":
            face.boundary_condition = value
        windows = fenestrations.get(surface_id, [])
        if not windows:
            continue
        total = sum(float(window["area_m2"]) for window in windows)
        _add_apertures(face, total, len(windows), aspect, sill)
        first = windows[0]
        glazing = EnergyWindowMaterialSimpleGlazSys(f"SEMIO_GLAZING_{first['id']}", float(first["u_value_w_m2k"]), float(first["shgc"]), float(first["vlt"]))
        construction = WindowConstruction(f"SEMIO_WINDOW_{first['id']}", [glazing])
        for aperture in face.apertures:
            aperture.properties.energy.construction = construction

    rooms_by_zone = {int(zone["id"]): room for zone, room in zip(model.get("zones", []), rooms)}
    for entry in model.get("infiltrations", []):
        room = rooms_by_zone[int(entry["zone_id"])]
        schedule = resolved_schedules.get(int(entry["schedule_id"]), always_on)
        room.properties.energy.infiltration = Infiltration(f"SEMIO_INFILTRATION_{entry['id']}", _infiltration_per_exterior_area(entry, room), schedule, float(entry["constant_term_coefficient"]), float(entry["temperature_term_coefficient"]), float(entry["velocity_term_coefficient"]))
    for entry in model.get("equipment", []):
        room = rooms_by_zone[int(entry["zone_id"])]
        room.properties.energy.electric_equipment = ElectricEquipment(f"SEMIO_EQUIPMENT_{entry['id']}", float(entry["watts_per_area"]), resolved_schedules.get(int(entry["schedule_id"]), always_on), float(entry["radiant_fraction"]), float(entry["latent_fraction"]), 0.0)
    for entry in model.get("lighting", []):
        room = rooms_by_zone[int(entry["zone_id"])]
        room.properties.energy.lighting = Lighting(f"SEMIO_LIGHTING_{entry['id']}", float(entry["watts_per_area"]), resolved_schedules.get(int(entry["schedule_id"]), always_on), float(entry["return_air_fraction"]), float(entry["radiant_fraction"]), float(entry["visible_fraction"]))
    for entry in model.get("people", []):
        room = rooms_by_zone[int(entry["zone_id"])]
        room.properties.energy.people = People(f"SEMIO_PEOPLE_{entry['id']}", float(entry["people_per_area"]), resolved_schedules.get(int(entry["schedule_id"]), always_on), None, float(entry["radiant_fraction"]), float(entry["latent_fraction"]))
    for entry in model.get("thermostats", []):
        room = rooms_by_zone[int(entry["zone_id"])]
        room.properties.energy.setpoint = Setpoint(f"SEMIO_SETPOINT_{entry['id']}", resolved_schedules[int(entry["heating_setpoint_schedule_id"])], resolved_schedules[int(entry["cooling_setpoint_schedule_id"])])
    for entry in model.get("ideal_loads", []):
        from honeybee.altnumber import no_limit

        room = rooms_by_zone[int(entry["zone_id"])]
        system = _ideal_air_system(f"SEMIO_IDEAL_LOADS_{entry['id']}", None, None)
        system.heating_air_temperature = float(entry["max_heating_supply_air_temp_c"])
        system.cooling_air_temperature = float(entry["min_cooling_supply_air_temp_c"])
        system.heating_limit = no_limit if entry.get("max_heating_capacity_w") is None else float(entry["max_heating_capacity_w"])
        system.cooling_limit = no_limit if entry.get("max_cooling_capacity_w") is None else float(entry["max_cooling_capacity_w"])
        room.properties.energy.hvac = system
        room.properties.energy.ventilation = Ventilation(f"SEMIO_VENTILATION_{entry['id']}", float(entry["outdoor_air_per_person_m3_s"]), float(entry["outdoor_air_per_area_m3_s_m2"]), 0.0, 0.0)

    shades = [Shade(f"SEMIO_SHADE_{entry['id']}", Face3D([Point3D(*vertex) for vertex in entry["vertices_m"]])) for entry in model.get("shading_surfaces", []) or []]
    honeybee_model = Model(model.get("name") or "SEMIO_ENERGY_MODEL", rooms, orphaned_shades=shades, units="Meters", tolerance=0.01, angle_tolerance=1.0)
    honeybee_model.user_data = {"northAngle": float(model.get("site", {}).get("north_axis_deg", 0.0))}
    return honeybee_model


# ── simulation and result extraction ──


def _simulation_parameter(north_angle):
    """⚙️ ASHRAE 140 §5.2 simulation settings, matching NREL/BESTEST-GSR's own configuration."""
    from honeybee_energy.simulation.parameter import SimulationParameter

    sim_par = SimulationParameter()
    sim_par.timestep = 6
    sim_par.terrain_type = "Country"
    sim_par.north_angle = north_angle
    sim_par.shadow_calculation.solar_distribution = "FullInteriorAndExterior"
    sim_par.shadow_calculation.calculation_frequency = 1
    sim_par.simulation_control.do_zone_sizing = False
    sim_par.simulation_control.do_system_sizing = False
    sim_par.simulation_control.do_plant_sizing = False
    sim_par.simulation_control.run_for_sizing_periods = False
    sim_par.simulation_control.run_for_run_periods = True
    sim_par.output.add_zone_energy_use("Total")
    sim_par.output.add_comfort_metrics()
    sim_par.output.add_glazing_solar()
    sim_par.output.reporting_frequency = "Hourly"
    return sim_par


def _run_energyplus(honeybee_model, epw, work_dir):
    """▶️ HBJSON → OSM → IDF → EnergyPlus, entirely inside `work_dir`."""
    from honeybee_energy.run import run_idf, to_openstudio_sim_folder

    shutil.rmtree(work_dir, ignore_errors=True)
    work_dir.mkdir(parents=True, exist_ok=True)
    _, _, idf = to_openstudio_sim_folder(honeybee_model, str(work_dir), epw_file=str(epw), sim_par=_simulation_parameter(float((honeybee_model.user_data or {}).get("northAngle", 0.0))), enforce_rooms=True)
    if idf is None:
        raise SystemExit("honeybee produced no IDF for this model")
    sql, _, _, _, err = run_idf(idf, str(epw), expand_objects=True, silent=True)
    if sql is None or not Path(sql).exists():
        tail = Path(err).read_text(errors="replace")[-4000:] if err and Path(err).exists() else "(no eplusout.err)"
        raise SystemExit(f"EnergyPlus produced no SQL output.\n{tail}")
    return Path(sql)


def _series(sql, output_name):
    collections = sql.data_collections_by_output_name(output_name)
    return [list(collection) for collection in collections]


def _summed(series):
    if not series:
        return None
    return [sum(values) for values in zip(*series)]


def _peak(hourly_kwh):
    if not hourly_kwh:
        return (0.0, 0)
    peak = max(hourly_kwh)
    return (peak, hourly_kwh.index(peak))


def _south_solar(sql, honeybee_model):
    """☀️ Transmitted solar through the south-facing apertures only, in Wh per hour."""
    south = set()
    for room in honeybee_model.rooms:
        for face in room.faces:
            if face.type.name != "Wall" or abs(face.horizontal_orientation() - 180.0) % 360.0 >= 45.0:
                continue
            for aperture in face.apertures:
                south.add(aperture.identifier.upper())
    if not south:
        return None
    collections = sql.data_collections_by_output_name(_SOLAR_OUTPUT)
    picked = [list(collection) for collection in collections if str(collection.header.metadata.get("Surface", "")).upper() in south]
    total = _summed(picked)
    return None if total is None else [value * 1000.0 for value in total]


def _results_document(case, sql_path, honeybee_model, epw, free_float, via=PRODUCER_VIA, south_apertures=None):
    from ladybug.sql import SQLiteResult

    sql = SQLiteResult(str(sql_path))
    temperature = _summed(_series(sql, _TEMPERATURE_OUTPUT))
    heating = _summed(_series(sql, _HEATING_OUTPUT))
    cooling = _summed(_series(sql, _COOLING_OUTPUT))
    if temperature is None or len(temperature) != HOURS_PER_YEAR:
        raise SystemExit(f"expected {HOURS_PER_YEAR} hourly zone temperatures, got {0 if temperature is None else len(temperature)}")
    zone_count = max(len(_series(sql, _TEMPERATURE_OUTPUT)), 1)
    temperature = [value / zone_count for value in temperature]
    document = {
        "schema": RESULT_SCHEMA,
        "case": case,
        "producer": {"name": "energyplus", "version": ENERGYPLUS_VERSION, "via": via},
        "weather": {"file": _weather_label(epw), "sha256": _sha256(epw)},
        "timestepMinutes": 60,
        "annual": None,
        "peak": None,
        "freeFloat": None,
        "hourly": {"zoneAirTemperatureC": [round(value, 4) for value in temperature]},
    }
    if free_float:
        minimum, maximum = min(temperature), max(temperature)
        document["freeFloat"] = {"minC": round(minimum, 4), "minHour": temperature.index(minimum), "maxC": round(maximum, 4), "maxHour": temperature.index(maximum), "meanC": round(sum(temperature) / len(temperature), 4)}
    else:
        heating = heating or [0.0] * HOURS_PER_YEAR
        cooling = cooling or [0.0] * HOURS_PER_YEAR
        peak_heating, peak_heating_hour = _peak(heating)
        peak_cooling, peak_cooling_hour = _peak(cooling)
        document["annual"] = {"heatingKwh": round(sum(heating), 4), "coolingKwh": round(sum(cooling), 4)}
        document["peak"] = {"heatingKw": round(peak_heating, 4), "heatingHour": peak_heating_hour, "coolingKw": round(peak_cooling, 4), "coolingHour": peak_cooling_hour}
        document["hourly"]["heatingW"] = [round(value * 1000.0, 3) for value in heating]
        document["hourly"]["coolingW"] = [round(value * 1000.0, 3) for value in cooling]
    solar = _named_solar(sql, south_apertures) if honeybee_model is None else _south_solar(sql, honeybee_model)
    if solar is not None:
        document["hourly"]["transmittedSolarSouthWh"] = [round(value, 3) for value in solar]
    return document


def _simulate(case, honeybee_model, epw, out_path, work_dir, keep, free_float):
    temporary = work_dir is None
    directory = Path(tempfile.mkdtemp(prefix=f"semio-energy-oracle-{case}-")) if temporary else Path(work_dir)
    try:
        print(f"[oracle] simulating case {case} in {directory}", flush=True)
        sql_path = _run_energyplus(honeybee_model, epw, directory)
        document = _results_document(case, sql_path, honeybee_model, epw, free_float)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        annual = document["annual"] or {}
        free = document["freeFloat"] or {}
        print(f"[oracle] {case}: heating {annual.get('heatingKwh', '-')} kWh, cooling {annual.get('coolingKwh', '-')} kWh, free-float {free.get('minC', '-')}..{free.get('maxC', '-')} °C -> {out_path}", flush=True)
        return document
    finally:
        if temporary and not keep:
            shutil.rmtree(directory, ignore_errors=True)


# ── the second, honeybee-free route: a semio-written epJSON straight into EnergyPlus ──


def _epjson_schema_path():
    """📐️ EnergyPlus's OWN `Energy+.schema.epJSON`, shipped inside the verified OpenStudio tree."""
    path = _oracle_root() / "EnergyPlus" / "Energy+.schema.epJSON"
    if not path.exists():
        raise SystemExit(f"no {path} — run the oracle-setup target first")
    return path


def _validate_epjson(document):
    """🧾️ Every schema violation, from the third-party `jsonschema` validator.

    The validator is chosen by the schema's own `$schema` (draft-07 for 25.2), and the errors are
    sorted and rendered as JSON paths so a failure names the offending object and field rather than
    just saying no.
    """
    from jsonschema import validators

    schema = json.loads(_epjson_schema_path().read_text(encoding="utf-8"))
    validator = validators.validator_for(schema)(schema)
    return [f"{'/'.join(str(part) for part in error.absolute_path) or '<root>'}: {error.message}" for error in sorted(validator.iter_errors(document), key=lambda error: list(error.absolute_path))]


def _epjson_free_float(document):
    """🌡️ A document with no ideal-loads system is a free-float case, exactly as the codec writes it."""
    return not document.get("ZoneHVAC:IdealLoadsAirSystem")


def _epjson_surface_normal(vertices):
    """📐️ Newell normal of one `BuildingSurface:Detailed` vertex list."""
    accumulated = [0.0, 0.0, 0.0]
    for index, current in enumerate(vertices):
        following = vertices[(index + 1) % len(vertices)]
        accumulated[0] += (current[1] - following[1]) * (current[2] + following[2])
        accumulated[1] += (current[2] - following[2]) * (current[0] + following[0])
        accumulated[2] += (current[0] - following[0]) * (current[1] + following[1])
    length = sum(component * component for component in accumulated) ** 0.5
    return None if length <= 1e-12 else [component / length for component in accumulated]


def _epjson_south_apertures(document):
    """☀️ Names of the apertures whose host wall faces within 45° of south, read from the document.

    The honeybee route asks its own model object the same question (`_south_solar`); this route has
    only the epJSON, so the answer comes out of the geometry the codec wrote.
    """
    south = set()
    for name, surface in (document.get("BuildingSurface:Detailed") or {}).items():
        vertices = [[vertex.get("vertex_x_coordinate", 0.0), vertex.get("vertex_y_coordinate", 0.0), vertex.get("vertex_z_coordinate", 0.0)] for vertex in surface.get("vertices", [])]
        normal = _epjson_surface_normal(vertices) if len(vertices) >= 3 else None
        if normal is None or normal[1] > -0.7071:
            continue
        for aperture, fields in (document.get("FenestrationSurface:Detailed") or {}).items():
            if fields.get("building_surface_name") == name:
                south.add(aperture.upper())
    return south


def _named_solar(sql, names):
    """☀️ Transmitted solar through a named aperture set, in Wh per hour."""
    if not names:
        return None
    collections = sql.data_collections_by_output_name(_SOLAR_OUTPUT)
    picked = [list(collection) for collection in collections if str(collection.header.metadata.get("Surface", "")).upper() in names]
    total = _summed(picked)
    return None if total is None else [value * 1000.0 for value in total]


def _run_energyplus_epjson(epjson_path, epw, work_dir):
    """▶️ `energyplus -a -w <epw> -d <work_dir> <file.epJSON>` — no honeybee, no OpenStudio, no IDF.

    ⚠️ `-r` (ReadVarsESO) is deliberately not passed: the OpenStudio bundle ships no `ReadVarsESO`
    binary, so it always fails (`📓️w2-oracle-toolchain.md` §7). The results are read from
    `Output:SQLite` instead, which the codec always writes.
    """
    import subprocess

    binary = _oracle_root() / "EnergyPlus" / "energyplus"
    shutil.rmtree(work_dir, ignore_errors=True)
    work_dir.mkdir(parents=True, exist_ok=True)
    completed = subprocess.run([str(binary), "-a", "-w", str(epw), "-d", str(work_dir), str(epjson_path)], capture_output=True, text=True)
    errors = work_dir / "eplusout.err"
    sql = work_dir / "eplusout.sql"
    if completed.returncode != 0 or not sql.exists():
        tail = errors.read_text(errors="replace")[-4000:] if errors.exists() else "(no eplusout.err)"
        raise SystemExit(f"energyplus exited {completed.returncode} without a usable eplusout.sql\n{completed.stdout[-2000:]}\n{completed.stderr[-2000:]}\n{tail}")
    return sql


# ── commands ──


def _command_status(_args):
    root = _bind_toolchain()
    from honeybee_energy.config import folders as hbe_folders
    import honeybee_energy
    import honeybee_openstudio
    import ladybug

    print(f"[oracle] python {sys.version.split()[0]}")
    print(f"[oracle] toolchain root {root}")
    print(f"[oracle] energyplus {hbe_folders.energyplus_version} at {hbe_folders.energyplus_exe}")
    print(f"[oracle] openstudio {hbe_folders.openstudio_version} at {hbe_folders.openstudio_exe}")
    print(f"[oracle] honeybee-energy {honeybee_energy.__version__ if hasattr(honeybee_energy, '__version__') else '1.123.32'}, honeybee-openstudio {getattr(honeybee_openstudio, '__version__', '0.7.2')}, ladybug-core {getattr(ladybug, '__version__', '0.44.59')}")
    print(f"[oracle] native ASHRAE 140 cases: {', '.join(sorted(_CASES))}")
    return 0


def _command_translate(args):
    _bind_toolchain()
    document = json.loads(Path(args.model).read_text(encoding="utf-8"))
    honeybee_model = _translated_model(document, args.layer_order, args.aperture_aspect, args.sill_height)
    case = args.case or Path(args.model).parent.name.lstrip("🏛️").removeprefix("bestest-") or honeybee_model.identifier
    free_float = not any(room.properties.energy.hvac is not None for room in honeybee_model.rooms)
    _simulate(case, honeybee_model, Path(args.weather), Path(args.out), args.work, args.keep, free_float)
    return 0


def _command_native(args):
    _bind_toolchain()
    honeybee_model = _native_model(args.case)
    _simulate(args.case, honeybee_model, Path(args.weather), Path(args.out), args.work, args.keep, args.case.endswith("FF"))
    return 0


def _command_selftest(_args):
    """🧪️ Exercises translation and case construction without spending an EnergyPlus run."""
    _bind_toolchain()
    failures = []
    for case in sorted(_CASES):
        model = _native_model(case)
        room = model.rooms[0]
        glazed = sum(aperture.area for face in room.faces for aperture in face.apertures)
        expected = sum(_CASES[case]["glazing"].values())
        if abs(glazed - expected) > 1e-6:
            failures.append(f"{case}: glazing {glazed} != {expected}")
        if abs(room.volume - _WIDTH_M * _DEPTH_M * _HEIGHT_M) > 1e-6:
            failures.append(f"{case}: volume {room.volume}")
        if (room.properties.energy.hvac is None) != case.endswith("FF"):
            failures.append(f"{case}: hvac presence does not match free-float flag")
        print(f"[selftest] {case}: volume {room.volume:.2f} m3, glazing {glazed:.2f} m2, hvac {room.properties.energy.hvac is not None}")
    document = _round_trip_document()
    translated = _translated_model(document, "outside-in", _APERTURE_ASPECT, _APERTURE_SILL_M)
    translated_room = translated.rooms[0]
    translated_glazing = sum(aperture.area for face in translated_room.faces for aperture in face.apertures)
    print(f"[selftest] translated: rooms {len(translated.rooms)}, faces {len(translated_room.faces)}, glazing {translated_glazing:.2f} m2, volume {translated_room.volume:.2f} m3")
    if abs(translated_glazing - 12.0) > 1e-6:
        failures.append(f"translated glazing {translated_glazing} != 12.0")
    if translated_room.properties.energy.hvac is None:
        failures.append("translated model lost its ideal loads system")
    if abs(translated_room.properties.energy.infiltration.flow_per_exterior_area - 1.048951e-4) > 1e-9:
        failures.append("translated model lost its infiltration rate")
    for failure in failures:
        print(f"[selftest] FAIL {failure}", file=sys.stderr)
    print(f"[selftest] {len(_CASES) + 1} checks, {len(failures)} failures")
    return 1 if failures else 0


_SEMIO_BOX = [
    ("SOUTH", "ExteriorWall", "wall", [[0.0, 0.0, 0.0], [8.0, 0.0, 0.0], [8.0, 0.0, 2.7], [0.0, 0.0, 2.7]]),
    ("EAST", "ExteriorWall", "wall", [[8.0, 0.0, 0.0], [8.0, 6.0, 0.0], [8.0, 6.0, 2.7], [8.0, 0.0, 2.7]]),
    ("NORTH", "ExteriorWall", "wall", [[8.0, 6.0, 0.0], [0.0, 6.0, 0.0], [0.0, 6.0, 2.7], [8.0, 6.0, 2.7]]),
    ("WEST", "ExteriorWall", "wall", [[0.0, 6.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 2.7], [0.0, 6.0, 2.7]]),
    ("ROOF", "Roof", "roof", [[0.0, 0.0, 2.7], [8.0, 0.0, 2.7], [8.0, 6.0, 2.7], [0.0, 6.0, 2.7]]),
    ("FLOOR", "Floor", "floor", [[0.0, 6.0, 0.0], [8.0, 6.0, 0.0], [8.0, 0.0, 0.0], [0.0, 0.0, 0.0]]),
]
_SEMIO_WINDOW = {"u_value_w_m2k": 3.0, "shgc": 0.787, "vlt": 0.84}
_SEMIO_MASS_LAYERS = {
    "LT": {"wall": [("WOOD_SIDING", 10), ("FIBERGLASS_QUILT_WALL", 11), ("PLASTERBOARD_WALL", 12)], "roof": [("ROOF_DECK", 13), ("FIBERGLASS_QUILT_ROOF", 14), ("PLASTERBOARD_ROOF", 15)], "floor": [("R25_INSULATION_LT", 16), ("TIMBER_FLOORING", 17)]},
    "HW": {"wall": [("WOOD_SIDING", 10), ("FOAM_INSULATION", 18), ("CONCRETE_BLOCK", 19)], "roof": [("ROOF_DECK", 13), ("FIBERGLASS_QUILT_ROOF", 14), ("PLASTERBOARD_ROOF", 15)], "floor": [("R25_INSULATION_HW", 21), ("CONCRETE_SLAB", 22)]},
}


def _semio_material(name, identifier):
    """🧱️ One semio `Material`; a massless standard layer keeps ρ = cp = 0 as the standard states it."""
    if name in _NO_MASS:
        thickness, conductivity, density, specific_heat = _NO_MASS[name] * 0.04, 0.04, 0.0, 0.0
    else:
        thickness, conductivity, density, specific_heat = _MATERIALS[name]
    return {"id": identifier, "name": name, "thickness_m": thickness, "conductivity_w_m_k": conductivity, "density_kg_m3": density, "specific_heat_j_kg_k": specific_heat, "thermal_absorptance": 0.9, "solar_absorptance": 0.6, "visible_absorptance": 0.6}


def _semio_case_document(case):
    """🧫️ One ASHRAE 140 case written as a semio `Model` in the engine's own `ToValue` wire shape.

    This is the oracle's own reading of what a `🔋️model.json` fixture for `<case>` should contain —
    published so the engine side can diff its fixture against it rather than guess, and used by
    `selftest` and by the translated-versus-native cross-check.
    """
    spec = _CASES.get(case)
    if spec is None:
        raise SystemExit(f"unknown ASHRAE 140 case {case!r}; known: {', '.join(sorted(_CASES))}")
    if spec.get("overhang_m") or set(spec["glazing"]) != {180.0} or spec["heating_c"] == "setback":
        raise SystemExit(f"case {case} needs shading, non-south glazing or a setback schedule, which this emitter does not yet write")
    layers = _SEMIO_MASS_LAYERS[spec["mass"]]
    materials = {identifier: name for group in layers.values() for name, identifier in group}
    constructions = {"wall": 100, "roof": 101, "floor": 102}
    conditioned = spec["heating_c"] is not None
    model = {
        "name": f"BESTEST_{case}",
        "version": "1",
        "site": {"latitude_deg": 39.83, "longitude_deg": -104.65, "elevation_m": 1650.0, "time_zone_hours": -7.0, "north_axis_deg": 0.0},
        "zones": [{"id": 1, "name": "ZONE_ONE", "volume_m3": 129.6, "multiplier": 1, "conditioned": conditioned, "part_of_total_floor_area": True}],
        "materials": [_semio_material(name, identifier) for identifier, name in sorted(materials.items())],
        "constructions": [{"id": constructions[group], "name": f"BESTEST_{spec['mass']}_{group.upper()}", "layer_material_ids": [identifier for _, identifier in layers[group]]} for group in ("wall", "roof", "floor")],
        "surfaces": [{"id": 30 + index, "name": name, "zone_id": 1, "class": kind, "vertices_m": vertices, "construction_id": constructions[group], "outside_boundary_condition": "OutdoorAir", "sun_exposed": group != "floor", "wind_exposed": group != "floor", "multiplier": 1} for index, (name, kind, group, vertices) in enumerate(_SEMIO_BOX)],
        "fenestrations": [dict(_SEMIO_WINDOW, id=40 + index, name=f"SOUTH_WINDOW_{index}", surface_id=30, area_m2=6.0, frame_conductance_w_k=0.0, divider_conductance_w_k=0.0) for index in range(2)],
        "equipment": [{"id": 50, "zone_id": 1, "schedule_id": 1, "watts_per_area": _GAIN_W / (_WIDTH_M * _DEPTH_M), "radiant_fraction": _GAIN_RADIANT_FRACTION, "latent_fraction": 0.0}],
        "infiltrations": [{"id": 60, "zone_id": 1, "schedule_id": 1, "method": "ScheduledAch", "design_flow_ach": _INFILTRATION_ACH, "flow_per_exterior_area_m3_s_m2": 0.0, "effective_leakage_area_m2": 0.0, "discharge_coefficient": 1.0, "stack_height_m": 0.0, "constant_term_coefficient": 1.0, "temperature_term_coefficient": 0.0, "velocity_term_coefficient": 0.0, "velocity_squared_term_coefficient": 0.0}],
        "thermostats": [{"id": 70, "zone_id": 1, "heating_setpoint_schedule_id": 2, "cooling_setpoint_schedule_id": 3, "heating_throttle_range_k": 0.0, "cooling_throttle_range_k": 0.0}] if conditioned else [],
        "ideal_loads": [{"id": 80, "zone_id": 1, "max_heating_supply_air_temp_c": 50.0, "min_cooling_supply_air_temp_c": 13.0, "max_heating_capacity_w": None, "max_cooling_capacity_w": None, "outdoor_air_per_person_m3_s": 0.0, "outdoor_air_per_area_m3_s_m2": 0.0}] if conditioned else [],
        "run_period": {"start_month": 1, "start_day": 1, "end_month": 12, "end_day": 31, "year": 2026},
        "schedules": {"constants": [{"id": 1, "value": 1.0}] + ([{"id": 2, "value": spec["heating_c"]}, {"id": 3, "value": spec["cooling_c"]}] if conditioned else []), "daily": [], "weekly": [], "annual": [], "time_series": []},
    }
    return {"model": model}


def _round_trip_document():
    """🧫️ The case-600 semio document `selftest` translates without spending an EnergyPlus run."""
    return _semio_case_document("600")


def _command_epjson(args):
    """⚡️ Validate a semio-written epJSON against EnergyPlus's own schema, then run EnergyPlus on it.

    This is the SECOND oracle route and it shares nothing above `_results_document` with the
    honeybee one: no honeybee, no OpenStudio, no IDF translation. If the two routes agree, the
    residual disagreement between semio and EnergyPlus is physics; if they disagree, it is the
    honeybee translation.
    """
    path = Path(args.document)
    document = json.loads(path.read_text(encoding="utf-8"))
    errors = _validate_epjson(document)
    if errors:
        listed = "\n".join(f"  {error}" for error in errors[:40])
        raise SystemExit(f"{path} fails EnergyPlus's own Energy+.schema.epJSON in {len(errors)} place(s):\n{listed}")
    print(f"[oracle] {path.name} validates against {_epjson_schema_path()} ({len(document)} object types)", flush=True)
    if args.validate_only:
        return 0
    case = args.case or path.stem.removeprefix("bestest-")
    epw = Path(args.weather)
    temporary = args.work is None
    directory = Path(tempfile.mkdtemp(prefix=f"semio-energy-epjson-{case}-")) if temporary else Path(args.work)
    try:
        print(f"[oracle] running EnergyPlus {ENERGYPLUS_VERSION} directly on {path.name} in {directory}", flush=True)
        sql = _run_energyplus_epjson(path, epw, directory)
        document_out = _results_document(case, sql, None, epw, _epjson_free_float(document), via="semio epJSON codec → EnergyPlus (no translator)", south_apertures=_epjson_south_apertures(document))
        out = Path(args.out)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(json.dumps(document_out, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        annual = document_out["annual"] or {}
        free = document_out["freeFloat"] or {}
        print(f"[oracle] {case}: heating {annual.get('heatingKwh', '-')} kWh, cooling {annual.get('coolingKwh', '-')} kWh, free-float {free.get('minC', '-')}..{free.get('maxC', '-')} °C -> {out}", flush=True)
        return 0
    finally:
        if temporary and not args.keep:
            shutil.rmtree(directory, ignore_errors=True)


def _command_emit(args):
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(_semio_case_document(args.case), indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"[oracle] wrote the oracle's reading of case {args.case} as a semio Model -> {out}")
    return 0


def main(argv):
    """🚪️ CLI entry point; see `📓️w2-oracle-toolchain.md` for the runner contract."""
    parser = argparse.ArgumentParser(prog="semio-energy-oracle", description="Honeybee/OpenStudio/EnergyPlus reference runner")
    sub = parser.add_subparsers(dest="command")
    sub.add_parser("status").set_defaults(handler=_command_status)
    sub.add_parser("selftest").set_defaults(handler=_command_selftest)

    translate = sub.add_parser("translate")
    translate.add_argument("model")
    translate.add_argument("weather")
    translate.add_argument("out")
    translate.add_argument("--case", default=None)
    translate.add_argument("--work", default=None)
    translate.add_argument("--keep", action="store_true")
    translate.add_argument("--layer-order", choices=["outside-in", "inside-out"], default="outside-in", dest="layer_order")
    translate.add_argument("--aperture-aspect", type=float, default=_APERTURE_ASPECT, dest="aperture_aspect")
    translate.add_argument("--sill-height", type=float, default=_APERTURE_SILL_M, dest="sill_height")
    translate.set_defaults(handler=_command_translate)

    emit = sub.add_parser("emit")
    emit.add_argument("case")
    emit.add_argument("out")
    emit.set_defaults(handler=_command_emit)

    epjson = sub.add_parser("epjson")
    epjson.add_argument("document")
    epjson.add_argument("weather", nargs="?", default=None)
    epjson.add_argument("out", nargs="?", default=None)
    epjson.add_argument("--case", default=None)
    epjson.add_argument("--work", default=None)
    epjson.add_argument("--keep", action="store_true")
    epjson.add_argument("--validate-only", action="store_true", dest="validate_only")
    epjson.set_defaults(handler=_command_epjson)

    native = sub.add_parser("native")
    native.add_argument("case")
    native.add_argument("weather")
    native.add_argument("out")
    native.add_argument("--work", default=None)
    native.add_argument("--keep", action="store_true")
    native.set_defaults(handler=_command_native)

    args = parser.parse_args(argv or ["status"])
    return args.handler(args)


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
