"""⚡️ The EnergyPlus ORACLE for the `s.energy.model` epJSON io case — the second, translator-free route.

WHAT MAKES THIS AN ORACLE. Nothing here imports, reads or transliterates the Rust codec it judges.
It receives the subject's produced BYTES verbatim (`@oracle-input-subject-raw`) and asks two
third parties about them:

1. `jsonschema` 4.26.0, a third-party draft-07 validator, against EnergyPlus's OWN
   `Energy+.schema.epJSON` — the 858-object-type schema shipped inside the sha256-verified
   OpenStudio 3.11.0 archive `oracle-setup` provisions. Every violation is reported with its JSON
   path; there is no hand-written "looks about right" check anywhere in this file.
2. EnergyPlus 25.2.0 itself, run as `energyplus -a -w <epw> -d <dir> <file.epJSON>` on those exact
   bytes. No honeybee, no OpenStudio, no IDF: this is the whole point of the case, because the
   sibling case's honeybee route is worth +5.7…+8.1 % of annual cooling all by itself
   (`📓️w2-oracle-toolchain.md`), and only a translator-free run can separate that from physics.

DEPENDENCIES. Only the Python standard library plus `jsonschema`. The results come out of
EnergyPlus's own `eplusout.sql` through stdlib `sqlite3` and out of `eplusout.eio` by reading the
`Zone Information` record EnergyPlus writes about the document it was given — deliberately NOT
through ladybug's SQL reader, both because the oracle host's interpreter cannot install the
compiled OpenStudio wheel (`📓️w2-oracle-toolchain.md` §2) and because reading EnergyPlus's own
tables directly is one fewer library between the claim and the answer.

⚠️ NO SKIP CHANNEL. "A missing registration, a panic and an error are all results, never a silent
skip." A host that has not run `nx run @semio-tech/energy-oracle-py:oracle-setup` fails here with a
message naming the absent binary and that target, rather than reporting a green that means nothing.

@see ../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/ENERGY-PLUGIN-END-TO-END/📓️w6-epjson-io.md
"""

from __future__ import annotations

# region 🔖️Imports
import json
import math
import os
import shutil
import sqlite3
import subprocess
from pathlib import Path

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Cases
#: ⚡️ Mirrors the feature's `Examples` tables; see the Rust adapter for why these ten and not fourteen.
VALIDATED_CASES = ["600", "600FF", "610", "620", "640", "900", "900FF", "910", "920", "940"]
SIMULATED_CASES = ["600", "600FF", "900", "900FF"]
ACCOUNTED_CASES = ["600", "900"]

#: 📏️ Both sides are EnergyPlus 25.2.0 on the same simple glazing, so the documented glazing offset
#: is common to both and cannot appear here as a difference.
ANNUAL_TOLERANCE = 0.03
FREE_FLOAT_TOLERANCE_K = 0.5

FACTS_SCHEMA = "semio.energy.epjson-facts/1"
ACCOUNTING_SCHEMA = "semio.energy.epjson-accounting/1"
ROUTE_SCHEMA = "semio.energy.epjson-route/1"

_HEATING_OUTPUT = "Zone Ideal Loads Supply Air Total Heating Energy"
_COOLING_OUTPUT = "Zone Ideal Loads Supply Air Total Cooling Energy"
_TEMPERATURE_OUTPUT = "Zone Mean Air Temperature"
_JOULES_PER_KWH = 3.6e6
HOURS_PER_YEAR = 8760
# endregion 🔖️Cases


# region 🔖️Toolchain
def _openstudio_root(ctx: Context) -> Path:
    """🗂️ The extracted, sha256-verified OpenStudio tree, honouring the oracle package's own env hook."""
    declared = os.environ.get("SEMIO_ORACLE_OPENSTUDIO_ROOT")
    if declared:
        return Path(declared)
    cache = Path(ctx.repo_root) / ".🧬semio" / "🦑️repo" / "⚡️cache" / "oracles"
    candidates = sorted(cache.glob("openstudio-*")) if cache.is_dir() else []
    if not candidates:
        raise AssertionError(
            "no provisioned OpenStudio/EnergyPlus toolchain under %s. This case runs EnergyPlus itself; "
            "run `nx run @semio-tech/energy-oracle-py:oracle-setup` (or the 🔮️oracle🔋️energy⚙️setup launch entry) "
            "to download, sha256-verify and extract it. There is no skip channel in this host, so an "
            "unprovisioned machine reports this error rather than a meaningless green." % cache
        )
    return candidates[-1]


def _energyplus_binary(ctx: Context) -> Path:
    binary = _openstudio_root(ctx) / "EnergyPlus" / "energyplus"
    if not binary.exists():
        raise AssertionError("no EnergyPlus binary at %s — run the oracle-setup target first" % binary)
    return binary


def _epjson_schema(ctx: Context) -> dict:
    """📐️ EnergyPlus's OWN schema, read from the verified archive rather than from anywhere else."""
    path = _openstudio_root(ctx) / "EnergyPlus" / "Energy+.schema.epJSON"
    if not path.exists():
        raise AssertionError("no Energy+.schema.epJSON at %s — run the oracle-setup target first" % path)
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def _validate(ctx: Context, document: dict) -> list:
    """🧾️ Every schema violation the third-party validator finds, as `<json path>: <message>`."""
    from jsonschema import validators

    schema = _epjson_schema(ctx)
    validator = validators.validator_for(schema)(schema)
    errors = sorted(validator.iter_errors(document), key=lambda error: list(error.absolute_path))
    return ["%s: %s" % ("/".join(str(part) for part in error.absolute_path) or "<root>", error.message) for error in errors]
# endregion 🔖️Toolchain


# region 🔖️Reading the document
def _round6(value):
    return round(value + 0.0, 6)


def _round2(value):
    return round(value + 0.0, 2)


def _group(document, kind):
    value = document.get(kind)
    return value if isinstance(value, dict) else {}


def _first(document, kind, field, fallback=0.0):
    for fields in _group(document, kind).values():
        value = fields.get(field)
        return value if isinstance(value, (int, float)) else fallback
    return fallback


def _vertices(fields):
    return [[vertex.get("vertex_x_coordinate", 0.0), vertex.get("vertex_y_coordinate", 0.0), vertex.get("vertex_z_coordinate", 0.0)] for vertex in fields.get("vertices", [])]


def _newell_area(vertices):
    """📐️ Planar polygon area as half the magnitude of the Newell vector."""
    if len(vertices) < 3:
        return 0.0
    accumulated = [0.0, 0.0, 0.0]
    for index, current in enumerate(vertices):
        following = vertices[(index + 1) % len(vertices)]
        accumulated[0] += (current[1] - following[1]) * (current[2] + following[2])
        accumulated[1] += (current[2] - following[2]) * (current[0] + following[0])
        accumulated[2] += (current[0] - following[0]) * (current[1] + following[1])
    return 0.5 * math.sqrt(sum(component * component for component in accumulated))


def _distance(a, b):
    return math.sqrt(sum((a[axis] - b[axis]) ** 2 for axis in range(3)))


def _aperture_corners(fields):
    corners = []
    for index in range(1, 5):
        try:
            corners.append([fields["vertex_%d_x_coordinate" % index], fields["vertex_%d_y_coordinate" % index], fields["vertex_%d_z_coordinate" % index]])
        except KeyError:
            return None
    return corners


def _aperture_area(document):
    """📐️ Total glazed area from the aperture rectangles the document itself carries."""
    total = 0.0
    for fields in _group(document, "FenestrationSurface:Detailed").values():
        corners = _aperture_corners(fields)
        if corners is not None:
            total += _distance(corners[0], corners[1]) * _distance(corners[1], corners[2])
    return total


def _floor_area(document):
    return sum(_newell_area(_vertices(fields)) for fields in _group(document, "BuildingSurface:Detailed").values() if fields.get("surface_type") == "Floor")


def _run_period(document):
    for fields in _group(document, "RunPeriod").values():
        return "%d/%d-%d/%d" % (fields.get("begin_month", 0), fields.get("begin_day_of_month", 0), fields.get("end_month", 0), fields.get("end_day_of_month", 0))
    return ""


def document_facts(case, document):
    """📋️ The oracle's own reading of the exported document — same fields, derived independently."""
    counts = {
        "materials": float(len(_group(document, "Material")) + len(_group(document, "Material:NoMass"))),
        "constructions": float(len(_group(document, "Construction"))),
        "glazing": float(len(_group(document, "WindowMaterial:SimpleGlazingSystem"))),
        "schedules": float(len(_group(document, "Schedule:Constant")) + len(_group(document, "Schedule:Compact"))),
        "infiltration": float(len(_group(document, "ZoneInfiltration:DesignFlowRate"))),
        "idealLoads": float(len(_group(document, "ZoneHVAC:IdealLoadsAirSystem"))),
        "thermostats": float(len(_group(document, "ZoneControl:Thermostat"))),
        "outputVariables": float(len(_group(document, "Output:Variable"))),
    }
    version = ""
    for fields in _group(document, "Version").values():
        version = str(fields.get("version_identifier", ""))
        break
    return {
        "schema": FACTS_SCHEMA,
        "case": case,
        "epJsonVersion": version,
        "objectTypes": sorted(document.keys()),
        "zoneNames": sorted(_group(document, "Zone").keys()),
        "surfaceNames": sorted(_group(document, "BuildingSurface:Detailed").keys()),
        "apertureNames": sorted(_group(document, "FenestrationSurface:Detailed").keys()),
        "counts": counts,
        "zoneVolumeM3": _round6(_first(document, "Zone", "volume")),
        "totalApertureAreaM2": _round6(_aperture_area(document)),
        "windowUFactor": _round6(_first(document, "WindowMaterial:SimpleGlazingSystem", "u_factor")),
        "windowShgc": _round6(_first(document, "WindowMaterial:SimpleGlazingSystem", "solar_heat_gain_coefficient")),
        "infiltrationAch": _round6(_first(document, "ZoneInfiltration:DesignFlowRate", "air_changes_per_hour")),
        "runPeriod": _run_period(document),
    }
# endregion 🔖️Reading the document


# region 🔖️Accounting
def _expected_diagnostic_codes(model):
    """🧾️ The refusals a faithful epJSON writer OWES on this model, re-derived from the model alone.

    This is a second reading of the same rule the export leaf states in its own module docstring —
    written from the semio schema, not from the Rust — so "the export reported everything it could
    not carry" is checked against an independent list rather than against the subject's own word.
    """
    codes = set()
    schedules = model.get("schedules") or {}
    if schedules.get("weekly"):
        codes.add("epjson.schedule.weekly-unsupported")
    if schedules.get("annual"):
        codes.add("epjson.schedule.annual-unsupported")
    if schedules.get("time_series"):
        codes.add("epjson.schedule.time-series-unsupported")
    for entry in model.get("infiltrations", []):
        if entry.get("method") in ("EffectiveLeakageArea", "WindAndStack"):
            codes.add("epjson.infiltration.method-unsupported")
    for entry in model.get("constructions", []):
        if len(entry.get("layer_material_ids", [])) > 10:
            codes.add("epjson.construction.too-many-layers")
    for entry in model.get("fenestrations", []):
        if entry.get("frame_conductance_w_k", 0.0) > 0.0 or entry.get("divider_conductance_w_k", 0.0) > 0.0:
            codes.add("epjson.fenestration.frame-dropped")
    for entry in model.get("surfaces", []):
        if entry.get("multiplier", 1) > 1:
            codes.add("epjson.surface.multiplier-dropped")
    for entry in model.get("thermostats", []):
        if entry.get("heating_throttle_range_k", 0.0) != 0.0 or entry.get("cooling_throttle_range_k", 0.0) != 0.0:
            codes.add("epjson.thermostat.throttle-range-dropped")
    for entry in model.get("ideal_loads", []):
        if entry.get("outdoor_air_per_person_m3_s", 0.0) > 0.0 or entry.get("outdoor_air_per_area_m3_s_m2", 0.0) > 0.0:
            codes.add("epjson.ideal-loads.outdoor-air-dropped")
    return sorted(codes)


def _unaccounted(model, epjson_text):
    """🧾️ Model entity names that appear nowhere in the exported document."""
    missing = []
    for collection, kind in (("zones", "zone"), ("surfaces", "surface"), ("fenestrations", "fenestration"), ("materials", "material"), ("constructions", "construction")):
        for entity in model.get(collection, []):
            name = str(entity.get("name", "")).strip()
            if name and ('"%s' % name) not in epjson_text:
                missing.append("%s %s" % (kind, name))
    return sorted(missing)
# endregion 🔖️Accounting


# region 🔖️Running EnergyPlus
def _zone_information(work_dir: Path):
    """🏢️ What EnergyPlus itself says about the building it was handed, from its own `eplusout.eio`.

    The `! <Zone Information>` header names the columns, so the values are read BY NAME rather than
    by a hard-coded column index that a future EnergyPlus could quietly shift.
    """
    eio = work_dir / "eplusout.eio"
    if not eio.exists():
        raise AssertionError("EnergyPlus wrote no eplusout.eio in %s" % work_dir)
    header = None
    for line in eio.read_text(errors="replace").splitlines():
        stripped = line.strip()
        if stripped.startswith("! <Zone Information>"):
            header = [part.strip() for part in stripped[len("! <Zone Information>,"):].split(",")]
            continue
        if header is not None and stripped.startswith("Zone Information,"):
            values = [part.strip() for part in stripped[len("Zone Information,"):].split(",")]
            row = dict(zip(header, values))
            pick = lambda prefix: next((value for key, value in row.items() if key.startswith(prefix)), None)
            return {
                "zoneVolumeM3": _round2(float(pick("Volume"))),
                "floorAreaM2": _round2(float(pick("Floor Area"))),
                "exteriorWindowAreaM2": _round2(float(pick("Exterior Window Area"))),
                "surfaces": float(pick("Number of Surfaces")),
                "subSurfaces": float(pick("Number of SubSurfaces")),
            }
    raise AssertionError("no `Zone Information` record in %s — EnergyPlus did not get as far as describing the building" % eio)


def _hourly(sql_path: Path, variable: str):
    """📈️ The summed hourly series for one output variable, straight out of EnergyPlus's own tables.

    Warm-up rows and design-day environments are excluded the way EnergyPlus itself distinguishes
    them, so an 8760-long weather run comes back 8760 long.
    """
    connection = sqlite3.connect("file:%s?mode=ro" % sql_path, uri=True)
    try:
        rows = connection.execute(
            """
            SELECT t.TimeIndex, SUM(d.Value)
            FROM ReportData d
            JOIN ReportDataDictionary k ON k.ReportDataDictionaryIndex = d.ReportDataDictionaryIndex
            JOIN Time t ON t.TimeIndex = d.TimeIndex
            WHERE k.Name = ? AND k.ReportingFrequency = 'Hourly' AND t.IntervalType = 1
              AND (t.WarmupFlag = 0 OR t.WarmupFlag IS NULL)
            GROUP BY t.TimeIndex ORDER BY t.TimeIndex
            """,
            (variable,),
        ).fetchall()
    finally:
        connection.close()
    return [value for _, value in rows]


def _zone_count(sql_path: Path, variable: str) -> int:
    connection = sqlite3.connect("file:%s?mode=ro" % sql_path, uri=True)
    try:
        return connection.execute("SELECT COUNT(*) FROM ReportDataDictionary WHERE Name = ? AND ReportingFrequency = 'Hourly'", (variable,)).fetchone()[0] or 1
    finally:
        connection.close()


def _run(ctx: Context, case: str, epjson_bytes: bytes, epw: str) -> Path:
    """▶️ `energyplus -a -w <epw> -d <dir> <file.epJSON>` — nothing between the bytes and the solver.

    ⚠️ `-r` is deliberately absent: the OpenStudio bundle ships no `ReadVarsESO`, so it always fails
    (`📓️w2-oracle-toolchain.md` §7). The results come from `Output:SQLite`, which the codec writes.
    ⚠️ The work directory is emptied first: EnergyPlus reuses whatever it finds, and a stale
    `eplusout.sql` next to a failed run reads exactly like a successful one.
    """
    work = Path(ctx.work_dir) / ("epjson-%s" % case)
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir(parents=True, exist_ok=True)
    document = work / ("semio-%s.epJSON" % case)
    document.write_bytes(epjson_bytes)
    completed = subprocess.run([str(_energyplus_binary(ctx)), "-a", "-w", epw, "-d", str(work), str(document)], capture_output=True, text=True)
    sql = work / "eplusout.sql"
    if completed.returncode != 0 or not sql.exists():
        errors = work / "eplusout.err"
        tail = errors.read_text(errors="replace")[-4000:] if errors.exists() else "(no eplusout.err)"
        raise AssertionError("case %s: EnergyPlus exited %d without a usable eplusout.sql\n%s\n%s\n%s" % (case, completed.returncode, completed.stdout[-2000:], completed.stderr[-2000:], tail))
    return work


def _results(case: str, work: Path, free_float: bool):
    """📊️ The contract's `semio.energy.bestest-results/1` shape, read out of EnergyPlus's own SQLite."""
    sql = work / "eplusout.sql"
    temperature = _hourly(sql, _TEMPERATURE_OUTPUT)
    if len(temperature) != HOURS_PER_YEAR:
        raise AssertionError("case %s: expected %d hourly zone temperatures, EnergyPlus reported %d" % (case, HOURS_PER_YEAR, len(temperature)))
    zones = _zone_count(sql, _TEMPERATURE_OUTPUT) or 1
    temperature = [value / zones for value in temperature]
    document = {"schema": "semio.energy.bestest-results/1", "case": case, "producer": {"name": "energyplus", "version": "25.2.0", "via": "semio epJSON codec → EnergyPlus (no translator)"}, "timestepMinutes": 60, "annual": None, "peak": None, "freeFloat": None, "hourly": {"zoneAirTemperatureC": [round(value, 4) for value in temperature]}}
    if free_float:
        minimum, maximum = min(temperature), max(temperature)
        document["freeFloat"] = {"minC": round(minimum, 4), "minHour": temperature.index(minimum), "maxC": round(maximum, 4), "maxHour": temperature.index(maximum), "meanC": round(sum(temperature) / len(temperature), 4)}
    else:
        heating = [value / _JOULES_PER_KWH for value in _hourly(sql, _HEATING_OUTPUT)] or [0.0] * HOURS_PER_YEAR
        cooling = [value / _JOULES_PER_KWH for value in _hourly(sql, _COOLING_OUTPUT)] or [0.0] * HOURS_PER_YEAR
        document["annual"] = {"heatingKwh": round(sum(heating), 4), "coolingKwh": round(sum(cooling), 4)}
        document["peak"] = {"heatingKw": round(max(heating), 4), "heatingHour": heating.index(max(heating)), "coolingKw": round(max(cooling), 4), "coolingHour": cooling.index(max(cooling))}
    return document
# endregion 🔖️Running EnergyPlus


# region 🔖️Handlers
def _subject_document(ctx: Context):
    """📥️ The subject's produced bytes, and the document they parse into."""
    raw = ctx.subject_raw_bytes("rust")
    try:
        return raw, json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, ValueError) as error:
        raise AssertionError("the subject's produced bytes are not a JSON document: %s" % error) from error


def _model(ctx: Context, case: str):
    return json.loads(ctx.fixture_bytes("asset://🧫️fixtures/🏛️bestest-%s/🔋️model.json" % case).decode("utf-8"))


def _reference(ctx: Context, case: str):
    uri = "asset://🧫️fixtures/🏛️bestest-%s/🔮️energyplus.json" % case
    try:
        raw = ctx.fixture_bytes(uri)
    except Exception as error:  # noqa: BLE001 — the message is the whole point
        raise AssertionError("case %s: no committed EnergyPlus reference at %s (%s); this oracle will not synthesize one" % (case, uri, error)) from error
    return json.loads(raw.decode("utf-8"))


def _schema_validity(case):
    def handler(ctx: Context) -> Outcome:
        raw, document = _subject_document(ctx)
        violations = _validate(ctx, document)
        assert not violations, "case %s: the exported epJSON breaks EnergyPlus's own Energy+.schema.epJSON in %d place(s):\n%s" % (case, len(violations), "\n".join("  " + entry for entry in violations[:40]))
        facts = document_facts(case, document)
        return Outcome(projection=facts, raw=raw)

    return handler


def _nothing_dropped(case):
    def handler(ctx: Context) -> Outcome:
        raw, _ = _subject_document(ctx)
        model = _model(ctx, case)
        projection = {"schema": ACCOUNTING_SCHEMA, "case": case, "unaccountedEntities": _unaccounted(model, raw.decode("utf-8")), "diagnosticCodes": _expected_diagnostic_codes(model)}
        return Outcome(projection=projection, raw=raw)

    return handler


def _energyplus_run(case):
    def handler(ctx: Context) -> Outcome:
        raw, document = _subject_document(ctx)
        violations = _validate(ctx, document)
        assert not violations, "case %s: the exported epJSON breaks Energy+.schema.epJSON in %d place(s):\n%s" % (case, len(violations), "\n".join("  " + entry for entry in violations[:40]))
        epw = ctx.fixture("asset://🧫️fixtures/🌦️denver-tmy/🌦️.epw")
        free_float = not document.get("ZoneHVAC:IdealLoadsAirSystem")
        work = _run(ctx, case, raw, epw)
        measured = _results(case, work, free_float)
        with open(ctx.artifact("oracle", "⚡️energyplus-direct.json"), "w", encoding="utf-8") as handle:
            json.dump(measured, handle, indent=2, ensure_ascii=False)
        reference = _reference(ctx, case)
        deviations = []
        annual_ok = None
        free_ok = None
        if free_float:
            free_ok = True
            for metric in ("minC", "maxC", "meanC"):
                gap = abs(measured["freeFloat"][metric] - reference["freeFloat"][metric])
                if gap > FREE_FLOAT_TOLERANCE_K:
                    free_ok = False
                    deviations.append("free-float %s: direct %.4f, honeybee route %.4f, gap %.4f K against %.2f K" % (metric, measured["freeFloat"][metric], reference["freeFloat"][metric], gap, FREE_FLOAT_TOLERANCE_K))
        else:
            annual_ok = True
            for metric in ("heatingKwh", "coolingKwh"):
                expected = reference["annual"][metric]
                gap = abs(measured["annual"][metric] - expected) / abs(expected) if abs(expected) > 1e-9 else abs(measured["annual"][metric] - expected)
                if gap > ANNUAL_TOLERANCE:
                    annual_ok = False
                    deviations.append("annual %s: direct %.4f, honeybee route %.4f, deviation %.4f against %.4f" % (metric, measured["annual"][metric], expected, gap, ANNUAL_TOLERANCE))
        assert not deviations, "case %s: EnergyPlus run directly on the semio-written epJSON disagrees with the committed reference — %s" % (case, " | ".join(deviations))
        projection = {
            "schema": ROUTE_SCHEMA,
            "case": case,
            "epJsonSchemaViolations": float(len(violations)),
            "energyPlus": _zone_information(work),
            "annualWithinTolerance": annual_ok,
            "freeFloatWithinTolerance": free_ok,
            "annualToleranceRelative": ANNUAL_TOLERANCE,
            "freeFloatToleranceK": FREE_FLOAT_TOLERANCE_K,
        }
        return Outcome(projection=projection, raw=raw)

    return handler
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Oracle role only, by full expanded scenario id. The subject is the Rust codec; registering
    this file as a subject too would make EnergyPlus its own subject."""
    built = Adapter("python")
    for case in VALIDATED_CASES:
        built = built.oracle("schema-validity-%s" % case, _schema_validity(case))
    for case in ACCOUNTED_CASES:
        built = built.oracle("nothing-dropped-%s" % case, _nothing_dropped(case))
    for case in SIMULATED_CASES:
        built = built.oracle("energyplus-run-%s" % case, _energyplus_run(case))
    return built
# endregion 🔖️Registration
