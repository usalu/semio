#!/usr/bin/env python3
"""🐍️ Second implementation of the packed row form (see ../🧬️schema/🔣️.json): reads the language-agnostic fixture, packs every case from its lists and unpacks every packed form, and prints both as JSON so that the TypeScript test can compare them with its own."""
import json, sys

MINIMUM = 48
KEY = "$"


def units(text):
    raw = text.encode("utf-16-le", "surrogatepass")
    return [raw[i] | (raw[i + 1] << 8) for i in range(0, len(raw), 2)]


def text_of(codes):
    return b"".join(c.to_bytes(2, "little") for c in codes).decode("utf-16-le", "surrogatepass")


def shared_prefix(left, right):
    a, b = units(left), units(right)
    at = 0
    while at < min(len(a), len(b)) and a[at] == b[at]:
        at += 1
    if at > 0 and 0xD800 <= a[at - 1] <= 0xDBFF:
        at -= 1
    return at


def has_key(value):
    if isinstance(value, list):
        return any(has_key(v) for v in value)
    if isinstance(value, dict):
        return KEY in value or any(has_key(v) for v in value.values())
    return False


def count(value, seen):
    if isinstance(value, str):
        if len(units(value)) >= MINIMUM:
            seen[value] = seen.get(value, 0) + 1
    elif isinstance(value, list):
        for v in value:
            count(v, seen)
    elif isinstance(value, dict):
        for v in value.values():
            count(v, seen)


def pooled(value, index):
    if isinstance(value, str):
        return {KEY: index[value]} if value in index else value
    if isinstance(value, list):
        return [pooled(v, index) for v in value]
    if isinstance(value, dict):
        return {k: pooled(v, index) for k, v in value.items()}
    return value


def pack(lists, extras):
    rows, ids = [], {}
    numbers = []
    for rows_in in lists:
        out, previous = [], -1
        for row in rows_in:
            key = json.dumps([row["kind"], row["path"], "fact" in row, row.get("fact")], ensure_ascii=False, separators=(",", ":"))
            if key not in ids:
                ids[key] = len(rows)
                rows.append(row)
            n = ids[key]
            out.append(n - previous - 1)
            previous = n
        numbers.append(out)
    seen = {}
    clean = not any(has_key(v) for v in list(extras) + [r.get("fact") for r in rows])
    if clean:
        for r in rows:
            count(r.get("fact"), seen)
        for e in extras:
            count(e, seen)
    pool = [t for t, c in seen.items() if c > 1]
    index = {t: i for i, t in enumerate(pool)}
    use = (lambda v: v) if not pool else (lambda v: pooled(v, index))
    table, previous = [], ""
    for r in rows:
        prefix = shared_prefix(previous, r["path"])
        suffix = text_of(units(r["path"])[prefix:])
        previous = r["path"]
        entry = [r["kind"], prefix, suffix]
        if "fact" in r:
            entry.append(use(r["fact"]))
        table.append(entry)
    return {"table": table, "pool": pool, "lists": numbers, "extras": [use(e) for e in extras]}


def expand(value, pool):
    if isinstance(value, list):
        return [expand(v, pool) for v in value]
    if isinstance(value, dict):
        if list(value.keys()) == [KEY] and isinstance(value[KEY], int):
            return pool[value[KEY]]
        return {k: expand(v, pool) for k, v in value.items()}
    return value


def unpack(packed):
    pool = packed["pool"]
    use = (lambda v: v) if not pool else (lambda v: expand(v, pool))
    rows, previous = [], ""
    for entry in packed["table"]:
        path = text_of(units(previous)[: entry[1]]) + entry[2]
        previous = path
        row = {"kind": entry[0], "path": path}
        if len(entry) == 4:
            row["fact"] = use(entry[3])
        rows.append(row)
    lists = []
    for numbers in packed["lists"]:
        out, last = [], -1
        for d in numbers:
            last = last + d + 1
            out.append(rows[last])
        lists.append(out)
    return {"lists": lists, "extras": [use(e) for e in packed["extras"]]}


fixture = json.load(open(sys.argv[1], encoding="utf-8"))
result = []
for case in fixture["cases"]:
    result.append({"name": case["name"], "packed": pack(case["lists"], case["extras"]), "unpacked": unpack(case["packed"])})
json.dump(result, sys.stdout, ensure_ascii=False, separators=(",", ":"))
