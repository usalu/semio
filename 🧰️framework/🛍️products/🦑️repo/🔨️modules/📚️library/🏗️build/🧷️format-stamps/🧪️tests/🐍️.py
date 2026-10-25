"""🐍️ Independent oracle of the format stamp table: resolves the declared stamps, decides admissions and renders the EN/DE reports without sharing a line with the implementation."""
import json
import os
import re
import sys


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def resolve(document, pointer):
    value = document
    for part in pointer.split("/")[1:]:
        value = value[part.replace("~1", "/").replace("~0", "~")]
    return value


def extract(entry, document):
    value = resolve(document, entry["pointer"])
    member = entry.get("member")
    if member is None:
        return value
    return {key: (row.get(member) if isinstance(row, dict) else None) for key, row in value.items()}


def text(value):
    return value if isinstance(value, str) else canonical(value)


def differences(identity, own, live):
    if isinstance(own, dict) and isinstance(live, dict):
        rows = []
        for key in sorted(set(own) | set(live)):
            before, after = own.get(key), live.get(key)
            if (before is None) != (after is None) or canonical(before) != canonical(after):
                rows.append({"id": identity + "." + key, "own": "-" if before is None else text(before), "live": "-" if after is None else text(after)})
        return rows
    return [] if canonical(own) == canonical(live) else [{"id": identity, "own": text(own), "live": text(live)}]


def bound(table, resource, declared):
    ids = []
    for row in table["resources"]:
        if resource.startswith(row["prefix"]):
            for identity in row["stamps"]:
                if identity not in ids:
                    ids.append(identity)
    return [identity for identity in ids if identity in declared]


def fill(template, values):
    return re.sub(r"\{([a-z]+)\}", lambda match: str(values[match.group(1)]), template)


def render(texts, language, rows, resource, maximum):
    shown = rows[:maximum]
    parts = [fill(texts[language]["moved"], row) for row in shown]
    if len(rows) > len(shown):
        parts.append(fill(texts[language]["more"], {"count": len(rows) - len(shown)}))
    parts.append(fill(texts[language]["yield"], {"resource": resource}))
    return " ".join(parts)


def cases(fixture_path, schema_path):
    fixture = load(fixture_path)
    protocol = load(schema_path)["$defs"]["Protocol"]["const"]
    table = fixture["table"]
    entries = {entry["id"]: entry for entry in table["stamps"]}
    out = []
    for case in fixture["cases"]:
        declared = {identity: extract(entries[identity], fixture["files"][entries[identity]["schema"]]) for identity in case["declared"]}
        files = {**fixture["files"], **case["live"]}
        rows = []
        if case["mode"] == "exclusive":
            for identity in bound(table, case["resource"], declared):
                rows += differences(identity, declared[identity], extract(entries[identity], files[entries[identity]["schema"]]))
        row = {"name": case["name"], "admission": "stale" if rows else "pass", "moved": rows}
        if rows:
            row["en"] = render(protocol["texts"], "en", rows, case["resource"], protocol["maximumReportedStamps"])
            row["de"] = render(protocol["texts"], "de", rows, case["resource"], protocol["maximumReportedStamps"])
        out.append(row)
    return out


def live(schema_path, library):
    protocol = load(schema_path)["$defs"]["Protocol"]["const"]
    return {entry["id"]: extract(entry, load(os.path.join(library, *entry["schema"].split("/")))) for entry in protocol["stamps"]}


if __name__ == "__main__":
    mode = sys.argv[1]
    print(json.dumps(cases(sys.argv[2], sys.argv[3]) if mode == "cases" else live(sys.argv[2], sys.argv[3]), ensure_ascii=False))
