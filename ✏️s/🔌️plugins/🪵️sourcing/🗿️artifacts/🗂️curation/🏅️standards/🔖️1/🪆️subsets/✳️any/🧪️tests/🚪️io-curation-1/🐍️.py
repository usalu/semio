#!/usr/bin/env python3
"""🚪️ Third-party ORACLE for the `s.sourcing.curation` io codecs: the SQLite snapshot, JSON, ZIP and UTF-8 text.

The subject (Rust and TypeScript, `🚪️io/`) projects a curation snapshot into the relational schema `🪶️sqlite/📸️snapshot/🗄️.sql`, and
declares json as an `Exact` bridge and zip/text as `Lossy` hops. This file reproduces the committed expectations of
`🧫️fixtures/🚪️io-curation-1/🔣️.json` with the Python standard library, which has never seen this repository:

* `sqlite3` loads the committed `🗄️.sql`, receives the snapshot rows projected independently here (IEEE-754 bits and class per scalar, ordinals
  per collection), passes `integrity_check` and `foreign_key_check`, matches the committed per-table row counts, restores the same snapshot,
  and reproduces the committed `📏️semantic-cells` row count and cell extent for every raw binary64/binary32 word;
* `json` round-trips the snapshot exactly, keeps member and array order, and refuses what RFC 8259 refuses;
* `zipfile` stores the JSON bytes (stored and deflated), lists the committed entry names, verifies CRC-32 and restores the same bytes;
* UTF-8 text reports the committed byte, scalar and UTF-16 lengths and refuses the committed malformed sequences;
* when `SEMIO_CURATION_IO_OUT` names a directory the Rust codec tests wrote (`archive.zip`, `snapshot.json`, `snapshot.txt`), those very bytes are re-read
  with `zipfile`, `json` and the utf-8 codec and held to the committed members and the kit snapshot; unset, that part is reported as skipped;
* the declared fidelity of each foreign hop in `🚪️io/🟦️.ts` equals the committed matrix.

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures>

@see ../../🚪️io/🪶️sqlite/📸️snapshot/🗄️.sql — the relational schema under test
@see ../../🚪️io/🟦️.ts — the declared io entries and their fidelity
"""

# region 🔖️Imports
import io
import json
import math
import os
import re
import sqlite3
import struct
import sys
import zipfile
import zlib
from pathlib import Path

# endregion 🔖️Imports


# region 🔖️Vocabulary
SUBSET = Path(__file__).resolve().parents[2]
"""📍️ The `✳️any` subset directory that owns this case."""

CELLS = SUBSET / "🚪️io" / "🪶️sqlite" / "📸️snapshot" / "🧪️tests" / "🧫️fixtures"
"""🧫️ The committed sqlite fixtures: the neutral snapshot and the semantic-cells law."""

DIMENSIONS = {"box": ("width", "height", "depth"), "frame": ("width", "height", "depth", "profile"), "slab": ("width", "depth", "thickness")}
"""📐️ The binary64 members of each dimensioned recipe, in column order."""

# endregion 🔖️Vocabulary


# region 🔖️Words
def class_of(value):
    """🏷️ The column `*_class` of a float."""
    return "nan" if math.isnan(value) else "positiveInfinity" if value == math.inf else "negativeInfinity" if value == -math.inf else "finite"


def binary64(value):
    """🔢️ The signed 64-bit word of a number (or of a raw `{bits}` word) and its REAL cell, NULL only for NaN since SQLite stores infinities."""
    if isinstance(value, dict):
        word = value["bits"]
    else:
        word = struct.unpack(">Q", struct.pack(">d", float(value)))[0]
    real = struct.unpack(">d", struct.pack(">Q", word))[0]
    return (None if math.isnan(real) else real), struct.unpack(">q", struct.pack(">Q", word))[0], class_of(real)


def binary32(value):
    """🔢️ The unsigned 32-bit word of a number (or of a raw `{bits}` word) and its REAL cell."""
    if isinstance(value, dict):
        word = value["bits"]
    else:
        word = struct.unpack(">I", struct.pack(">f", float(value)))[0]
    real = struct.unpack(">f", struct.pack(">I", word))[0]
    return (None if math.isnan(real) else real), word, class_of(real)


def real_of(row, name):
    """↩️ The float a table row's `<name>_bits` word stands for."""
    return struct.unpack(">d", struct.pack(">q", row[name + "_bits"]))[0]


# endregion 🔖️Words


# region 🔖️Sqlite
def project(snapshot):
    """🗄️ The snapshot as an in-memory database over the committed schema, rows inserted by this file alone."""
    database = sqlite3.connect(":memory:")
    database.execute("PRAGMA foreign_keys = ON")
    database.executescript((SUBSET / "🚪️io" / "🪶️sqlite" / "📸️snapshot" / "🗄️.sql").read_text(encoding="utf-8"))

    def insert(table, values):
        return database.execute("INSERT INTO %s VALUES (%s)" % (table, ",".join("?" * (len(values) + 1))), [None, *values]).lastrowid

    insert("curation_document", [])
    catalog = snapshot["catalog"]
    insert("curation_catalog", [1, catalog["childId"], catalog["target"]["artifactId"], *(catalog["target"]["dialect"][key] for key in ("artifactKind", "standard", "subset"))])
    for ordinal, extra in enumerate(snapshot["stockExtra"]):
        stock = insert("curation_stock_extra", [1, ordinal, extra["id"], extra["name"], extra["moduleId"], extra["availability"]])
        for position, segment in enumerate(extra["typologyPath"]):
            insert("curation_typology_segment", [stock, position, segment])
        recipe = extra["geometry"]
        geometry = insert("curation_geometry", [stock, recipe["kind"]])
        if recipe["kind"] in DIMENSIONS:
            cells = [cell for member in DIMENSIONS[recipe["kind"]] for cell in binary64(recipe[member])]
            insert("curation_" + recipe["kind"], [geometry, *cells[0::3], *(cell for at in range(len(DIMENSIONS[recipe["kind"]])) for cell in cells[3 * at + 1 : 3 * at + 3])])
        elif recipe["kind"] == "glb":
            real, bits, kind = binary64(recipe["extent"])
            insert("curation_glb", [geometry, recipe["url"], real, bits, kind])
        else:
            mesh = insert("curation_mesh", [geometry])
            for table, member in (("position", "positions"), ("normal", "normals")):
                for ordinal2, value in enumerate(recipe[member]):
                    insert("curation_mesh_" + table, [mesh, ordinal2, *binary32(value)])
            for ordinal2, value in enumerate(recipe["indices"]):
                insert("curation_mesh_index", [mesh, ordinal2, value])
    for ordinal, item in enumerate(snapshot["curated"]):
        insert("curation_curated", [1, ordinal, item["objectId"], item["count"]])
    database.commit()
    return database


def rows(database, table):
    """📖️ Every row of a table as a dict, in id order."""
    database.row_factory = sqlite3.Row
    found = [dict(row) for row in database.execute('SELECT * FROM "%s" ORDER BY id' % table)]
    database.row_factory = None
    return found


def restore(database):
    """↩️ The snapshot the database stands for, rebuilt from rows alone."""
    catalog = rows(database, "curation_catalog")[0]
    snapshot = {"catalog": {"childId": catalog["child_id"], "target": {"artifactId": catalog["artifact_id"], "dialect": {"artifactKind": catalog["artifact_kind"], "standard": catalog["standard"], "subset": catalog["subset"]}}}, "stockExtra": [], "curated": []}
    for extra in sorted(rows(database, "curation_stock_extra"), key=lambda row: row["ordinal"]):
        path = [row["segment"] for row in sorted(rows(database, "curation_typology_segment"), key=lambda row: row["ordinal"]) if row["stock_extra_id"] == extra["id"]]
        geometry = next(row for row in rows(database, "curation_geometry") if row["stock_extra_id"] == extra["id"])
        recipe = {"kind": geometry["kind"]}
        if geometry["kind"] in DIMENSIONS:
            row = next(row for row in rows(database, "curation_" + geometry["kind"]) if row["geometry_id"] == geometry["id"])
            recipe.update({member: real_of(row, member) for member in DIMENSIONS[geometry["kind"]]})
        elif geometry["kind"] == "glb":
            row = next(row for row in rows(database, "curation_glb") if row["geometry_id"] == geometry["id"])
            recipe.update({"url": row["url"], "extent": real_of(row, "extent")})
        else:
            mesh = next(row for row in rows(database, "curation_mesh") if row["geometry_id"] == geometry["id"])
            word = lambda table: [struct.unpack(">f", struct.pack(">I", row["value_bits"]))[0] for row in sorted(rows(database, table), key=lambda row: row["ordinal"]) if row["mesh_id"] == mesh["id"]]
            recipe.update({"positions": word("curation_mesh_position"), "normals": word("curation_mesh_normal"), "indices": [row["value"] for row in sorted(rows(database, "curation_mesh_index"), key=lambda row: row["ordinal"]) if row["mesh_id"] == mesh["id"]]})
        snapshot["stockExtra"].append({"id": extra["object_id"], "name": extra["name"], "moduleId": extra["module_id"], "typologyPath": path, "availability": extra["availability"], "geometry": recipe})
    snapshot["curated"] = [{"objectId": row["object_id"], "count": row["count"]} for row in sorted(rows(database, "curation_curated"), key=lambda row: row["ordinal"])]
    return snapshot


def extent_of(database):
    """📏️ The cell extent: UTF-8 bytes of text, eight bytes per other non-null cell, nothing for NULL."""
    total = 0
    for (name,) in database.execute("SELECT name FROM sqlite_schema WHERE type = 'table'").fetchall():
        for row in database.execute('SELECT * FROM "%s"' % name):
            total += sum(0 if cell is None else len(cell.encode("utf-8")) if isinstance(cell, str) else 8 for cell in row)
    return total


def word_applied(snapshot, case):
    """🔧️ The neutral snapshot with one raw word in every dimension (binary64) or every mesh scalar (binary32)."""
    edited = json.loads(json.dumps(snapshot))
    for extra in edited["stockExtra"]:
        recipe = extra["geometry"]
        if case["width"] == 32 and recipe["kind"] == "mesh":
            recipe["positions"] = [{"bits": int(case["hex"], 16)} for _ in recipe["positions"]]
            recipe["normals"] = [{"bits": int(case["hex"], 16)} for _ in recipe["normals"]]
        elif case["width"] == 64 and recipe["kind"] != "mesh":
            word = {"bits": int(case["hex"], 16)}
            for member in DIMENSIONS.get(recipe["kind"], ("extent",)):
                recipe[member] = word
    return edited


def check_sqlite(root, law, failures):
    """🪶️ Projects, queries and restores the real kit and the neutral snapshot, and re-measures every semantic-cells case."""
    kit = json.loads((root / law["sqlite"]["kit"]).read_text(encoding="utf-8"))
    database = project(kit)
    for problem, query in (("integrity_check", "PRAGMA integrity_check"), ("foreign_key_check", "PRAGMA foreign_key_check")):
        found = database.execute(query).fetchall()
        if found not in ([("ok",)], []):
            failures.append("kit %s: %r" % (problem, found))
    counts = {table: database.execute('SELECT COUNT(*) FROM "%s"' % table).fetchone()[0] for table in law["sqlite"]["kitRows"]}
    if counts != law["sqlite"]["kitRows"]:
        failures.append("kit row counts %r, committed %r" % (counts, law["sqlite"]["kitRows"]))
    if restore(database) != kit:
        failures.append("the kit snapshot did not come back from its own rows")
    ordered = [row[0] for row in database.execute("SELECT object_id FROM curation_curated ORDER BY ordinal")]
    if ordered != [item["objectId"] for item in kit["curated"]]:
        failures.append("curated order moved: %r" % ordered)
    cells = json.loads((CELLS / "📏️semantic-cells" / "🔣️.json").read_text(encoding="utf-8"))
    neutral = json.loads((CELLS / "🔣️.json").read_text(encoding="utf-8"))["snapshot"]
    for case in cells["cases"]:
        measured = project(word_applied(neutral, case))
        tables = measured.execute("SELECT COUNT(*) FROM sqlite_schema WHERE type = 'table'").fetchone()[0]
        total = sum(measured.execute('SELECT COUNT(*) FROM "%s"' % name).fetchone()[0] for (name,) in measured.execute("SELECT name FROM sqlite_schema WHERE type = 'table'").fetchall())
        if (tables, total, extent_of(measured)) != (cells["tableCount"], cells["rowCount"], case["bytes"]):
            failures.append("semantic-cells %s: tables, rows, bytes %r, committed %r" % (case["id"], (tables, total, extent_of(measured)), (cells["tableCount"], cells["rowCount"], case["bytes"])))
        if measured.execute("PRAGMA integrity_check").fetchone() != ("ok",) or measured.execute("PRAGMA foreign_key_check").fetchall():
            failures.append("semantic-cells %s: integrity" % case["id"])
    return len(cells["cases"])


# endregion 🔖️Sqlite


# region 🔖️Codecs
def check_json(root, law, failures):
    """🔣️ The `Exact` bridge: bytes parse to the same value, order survives, and RFC 8259 refusals hold."""
    kit = json.loads((root / law["sqlite"]["kit"]).read_text(encoding="utf-8"))
    data = json.dumps(kit, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    if json.loads(data.decode("utf-8")) != kit or list(json.loads(data)) != law["json"]["members"]:
        failures.append("json round trip or member order moved")
    if [item["objectId"] for item in json.loads(data)["curated"]] != [item["objectId"] for item in kit["curated"]] or len(kit["stockExtra"]) != law["json"]["stock"] or len(kit["curated"]) != law["json"]["curated"]:
        failures.append("json array order or sizes moved")
    for text in law["json"]["refused"]:
        try:
            json.loads(text, parse_constant=lambda name: (_ for _ in ()).throw(ValueError(name)))
        except ValueError:
            continue
        failures.append("json accepted %r" % text)
    return data


def check_zip(data, law, failures):
    """🗜️ Stores the JSON bytes under the committed entry name and restores them, with a valid CRC-32."""
    for method in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, "w", method) as archive:
            archive.writestr(law["zip"]["entry"], data)
        with zipfile.ZipFile(io.BytesIO(buffer.getvalue())) as archive:
            names, info = archive.namelist(), archive.getinfo(law["zip"]["entry"])
            if names != [law["zip"]["entry"]] or archive.read(law["zip"]["entry"]) != data or archive.testzip() is not None or info.CRC != zlib.crc32(data) or info.file_size != len(data):
                failures.append("zip method %d lost the entry" % method)
    if zipfile.is_zipfile(io.BytesIO(data)):
        failures.append("json bytes sniffed as a zip")


def check_text(law, failures):
    """📝️ UTF-8 lengths of every committed sample and the refusal of every committed malformed sequence."""
    for sample in law["text"]["samples"]:
        text = sample["text"]
        found = (len(text.encode("utf-8")), len(text), len(text.encode("utf-16-le")) // 2)
        if found != (sample["utf8Bytes"], sample["scalars"], sample["utf16Units"]) or text.encode("utf-8").decode("utf-8") != text:
            failures.append("text %r: bytes, scalars, utf-16 units %r, committed %r" % (text, found, (sample["utf8Bytes"], sample["scalars"], sample["utf16Units"])))
    for hexadecimal in law["text"]["malformed"]:
        try:
            bytes.fromhex(hexadecimal).decode("utf-8")
        except UnicodeDecodeError:
            continue
        failures.append("utf-8 accepted %s" % hexadecimal)


def check_codec_output(directory, kit, law, failures):
    """🦀️ Re-reads the bytes the Rust codecs produced: zip members and CRC-32, the json member and file equal to the kit, utf-8 text naming every row."""
    archive = directory / "archive.zip"
    if archive.exists():
        with zipfile.ZipFile(archive) as opened:
            names, spec = opened.namelist(), law["zip"]["archive"]
            if len(names) != 2 or not (names[0].startswith(spec["dslPrefix"]) and names[0].endswith(spec["dslSuffix"])) or names[1] != spec["jsonMember"]:
                failures.append("rust archive members %r" % names)
            if opened.testzip() is not None or any(info.compress_type not in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED) or info.CRC != zlib.crc32(opened.read(info)) for info in opened.infolist()):
                failures.append("rust archive failed its CRC-32 or method check")
            if json.loads(opened.read(spec["jsonMember"]).decode("utf-8")) != kit:
                failures.append("rust archive json member differs from the kit snapshot")
            dsl = opened.read(names[0]).decode("utf-8")
            if [item["id"] for item in kit["stockExtra"] if item["id"] not in dsl]:
                failures.append("rust archive dsl member lacks a stock id")
    document = directory / "snapshot.json"
    if document.exists() and json.loads(document.read_text(encoding="utf-8")) != kit:
        failures.append("rust json export differs from the kit snapshot")
    text = directory / "snapshot.txt"
    if text.exists():
        decoded = text.read_bytes().decode("utf-8")
        if decoded.startswith("\ufeff") or [item["objectId"] for item in kit["curated"] + kit["stockExtra"] if item.get("objectId", item.get("id")) not in decoded]:
            failures.append("rust text export has a byte order mark or lacks a row id")
    return [name for name in ("archive.zip", "snapshot.json", "snapshot.txt") if (directory / name).exists()]


def check_fidelity(law, failures):
    """⚖️ The fidelity every `ioEntries` row of the TypeScript mirror declares equals the committed matrix."""
    source = (SUBSET / "🚪️io" / "🟦️.ts").read_text(encoding="utf-8")
    names = dict(re.findall(r'const (\w+) = "([^"]+)"', source))
    declared = {}
    for left, right, fidelity in re.findall(r"from: (\w+), into: (\w+), fidelity: \"(\w+)\"", source):
        declared[names[left] + " -> " + names[right]] = fidelity
    if declared != law["fidelity"]:
        failures.append("declared fidelity %r, committed %r" % (declared, law["fidelity"]))


# endregion 🔖️Codecs


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares every codec with the committed expectations."""
    if len(arguments) != 2 or arguments[0] != "check":
        print("usage: python 🐍️.py check <path to 🧫️fixtures>")
        return 2
    root = Path(arguments[1])
    law, failures = json.loads((root / "🚪️io-curation-1" / "🔣️.json").read_text(encoding="utf-8")), []
    cases = check_sqlite(root, law, failures)
    data = check_json(root, law, failures)
    check_zip(data, law, failures)
    check_text(law, failures)
    check_fidelity(law, failures)
    kit = json.loads((root / law["sqlite"]["kit"]).read_text(encoding="utf-8"))
    output = os.environ.get("SEMIO_CURATION_IO_OUT")
    compared = check_codec_output(Path(output), kit, law, failures) if output else []
    print("rust codec output re-read: %s" % (", ".join(compared) if compared else "skipped (SEMIO_CURATION_IO_OUT unset or empty)"))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("sqlite %s: %d semantic-cells cases, %d json bytes" % (sqlite3.sqlite_version, cases, len(data)))
    print("check: %s" % ("%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
