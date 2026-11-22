"""Independent oracle of the canonical row order: rows sorted by kind, path and the compact JSON text of the fact, compared as UTF-16 code units (the order of JavaScript strings), joined with NUL. Reads the fixture file (argv[1]) and prints the sorted rows."""
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))


def text(row):
    return row["kind"] + "\u0000" + row["path"] + "\u0000" + json.dumps(row.get("fact"), separators=(",", ":"), ensure_ascii=False)


def units(value):
    return value.encode("utf-16-be")


print(json.dumps(sorted(fixture["rows"], key=lambda row: units(text(row))), ensure_ascii=False))
