#!/usr/bin/env python3
"""🧮️ Third-party ORACLE for the curation pool's stock queries: the typology filter, the sort and the availability counts.

The subject (Rust, `filtered_stock`, `pool_kinds`, `curation_decision_for_delta`/`curation_decision_for_set`) answers which kinds a
filter keeps, in which order, and how a count adjustment resolves against availability. This file reproduces every committed answer of
`🧫️fixtures/🧮️stock-curation-1/🔣️.json` twice: with a plain Python model of the stated semantics, and with SQLite, a query engine that
has never seen this repository (substring match, JSON-array prefix match, `ORDER BY` with the binary collation, `SUM` and `COUNT`).

Standalone use (no test host needed):

    python 🐍️.py check <path to 🧫️fixtures>

@see ../../🧬️schema/🧾️domain/🦀️.rs — `filtered_stock` and `curation_decision_for_set`
@see ../../✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs — `pool_kinds`
"""

# region 🔖️Imports
import json
import sqlite3
import sys
from pathlib import Path

# endregion 🔖️Imports


# region 🔖️Vocabulary
SORT_COLUMNS = {"name": "name", "module": "module_id", "availability": "availability"}
"""↕️ The sortable pool columns and the stock member each one orders by."""

# endregion 🔖️Vocabulary


# region 🔖️Model
def model_filter(stock, filters):
    """🧠️ The stated semantics: AND of query, module, typology prefix and minimum availability, then a stable sort."""
    query = filters.get("query", "").strip().lower()
    modules, prefix, minimum = filters.get("moduleIds", []), filters.get("typologyPath", []), filters.get("minAvailability", 0)
    kept = [kind for kind in stock if (not query or query in kind["name"].lower()) and (not modules or kind["moduleId"] in modules) and kind["typologyPath"][: len(prefix)] == prefix and kind["availability"] >= minimum]
    sort = filters.get("sort")
    if sort and sort["columnId"] in SORT_COLUMNS:
        member = {"name": "name", "module": "moduleId", "availability": "availability"}[sort["columnId"]]
        kept = sorted(kept, key=lambda kind: kind[member], reverse=sort["direction"] == "desc")
    return [kind["id"] for kind in kept]


def model_decision(curated, stock, op, object_id, value):
    """🧠️ The stated semantics of a count adjustment: clamp to `0..=availability`, then create, change, delete or leave."""
    kind = next((kind for kind in stock if kind["id"] == object_id), None)
    if kind is None:
        return {"kind": "noOp"}
    current = next((item["count"] for item in curated if item["objectId"] == object_id), None)
    target = max(0, min(kind["availability"], (current or 0) + value if op == "delta" else value))
    if current is not None:
        return {"kind": "delete"} if target == 0 else {"kind": "noOp"} if target == current else {"kind": "changeCount", "count": target}
    return {"kind": "create", "count": target} if target > 0 else {"kind": "noOp"}


# endregion 🔖️Model


# region 🔖️Engine
def database_of(stock):
    """🗄️ The stock as a SQLite table, one row per kind in document order."""
    database = sqlite3.connect(":memory:")
    database.execute("CREATE TABLE stock (position INTEGER PRIMARY KEY, id TEXT, name TEXT, module_id TEXT, typology TEXT, availability INTEGER)")
    database.executemany("INSERT INTO stock VALUES (?,?,?,?,?,?)", [(at, kind["id"], kind["name"], kind["moduleId"], json.dumps(kind["typologyPath"]), kind["availability"]) for at, kind in enumerate(stock)])
    return database


def engine_filter(database, filters):
    """🔍️ The same filter as one SQL statement; the sort tie-break is the document position, as a stable sort keeps it."""
    clauses, parameters = ["availability >= ?"], [filters.get("minAvailability", 0)]
    query = filters.get("query", "").strip().lower()
    if query:
        clauses.append("instr(lower(name), ?) > 0")
        parameters.append(query)
    modules = filters.get("moduleIds", [])
    if modules:
        clauses.append("module_id IN (%s)" % ",".join("?" * len(modules)))
        parameters += modules
    prefix = filters.get("typologyPath", [])
    if prefix:
        clauses.append("json_array_length(typology) >= ?")
        parameters.append(len(prefix))
        for at, segment in enumerate(prefix):
            clauses.append("json_extract(typology, '$[%d]') = ?" % at)
            parameters.append(segment)
    sort = filters.get("sort")
    order = "position"
    if sort and sort["columnId"] in SORT_COLUMNS:
        order = "%s COLLATE BINARY %s, position" % (SORT_COLUMNS[sort["columnId"]], "DESC" if sort["direction"] == "desc" else "ASC")
    return [row[0] for row in database.execute("SELECT id FROM stock WHERE %s ORDER BY %s" % (" AND ".join(clauses), order), parameters)]


# endregion 🔖️Engine


# region 🔖️Standalone
def main(arguments):
    """🏃️ `check` compares the model and the SQL engine with every committed answer."""
    if len(arguments) != 2 or arguments[0] != "check":
        print("usage: python 🐍️.py check <path to 🧫️fixtures>")
        return 2
    root = Path(arguments[1])
    law = json.loads((root / "🧮️stock-curation-1" / "🔣️.json").read_text(encoding="utf-8"))
    stock = json.loads((root / law["stock"]).read_text(encoding="utf-8"))
    database, failures = database_of(stock), []
    for case in law["filters"]:
        for engine, ids in (("model", model_filter(stock, case["filters"])), ("sqlite", engine_filter(database, case["filters"]))):
            if ids != case["ids"]:
                failures.append("filter %s (%s): %r, committed %r" % (case["id"], engine, ids, case["ids"]))
    counts = law["availability"]
    if database.execute("SELECT SUM(availability) FROM stock").fetchone()[0] != counts["total"] or sum(kind["availability"] for kind in stock) != counts["total"]:
        failures.append("availability total is not %d" % counts["total"])
    by_module = dict(database.execute("SELECT module_id, SUM(availability) FROM stock GROUP BY module_id").fetchall())
    if by_module != counts["byModule"]:
        failures.append("availability by module is %r, committed %r" % (by_module, counts["byModule"]))
    for row in counts["kindsAtLeast"]:
        found = database.execute("SELECT COUNT(*) FROM stock WHERE availability >= ?", [row["min"]]).fetchone()[0]
        if found != row["kinds"] or len(model_filter(stock, {"minAvailability": row["min"]})) != row["kinds"]:
            failures.append("kinds with availability >= %d: %d, committed %d" % (row["min"], found, row["kinds"]))
    for case in law["decisions"]:
        decision = model_decision(case["curated"], stock, case["op"], case["objectId"], case["value"])
        if decision != case["decision"]:
            failures.append("decision %s: %r, committed %r" % (case["id"], decision, case["decision"]))
    for failure in failures:
        print("[FAIL] %s" % failure)
    print("sqlite %s: %d filters, %d decisions, %d kinds" % (sqlite3.sqlite_version, len(law["filters"]), len(law["decisions"]), len(stock)))
    print("check: %s" % ("%d problem(s)" % len(failures) if failures else "oracle agrees"))
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

# endregion 🔖️Standalone
