"""⏱️ Independent Python oracle of the recency rules (standard library only): it summarizes file metadata with arbitrary-precision integers, decides the diagnostic byte funding and builds the retirement mismatch report. Requests arrive as JSON on stdin."""
import json, re, sys

DIGITS = re.compile(r"^[0-9]{1,32}$")
FIELDS = ("atimeNs", "mtimeNs")


def summarize(rows):
    if len(rows) > 4096:
        return {"error": "Bounded recency metadata required"}
    ordered = sorted(rows, key=lambda row: row["path"])
    seen, total, maximum = set(), 0, None
    for row in ordered:
        values = [row["device"], row["inode"], row["bytes"], row["atimeNs"], row["mtimeNs"], row["ctimeNs"]]
        if not row["path"] or len(row["path"].encode("utf8")) > 256 or row["path"] in seen or not all(DIGITS.fullmatch(v) for v in values):
            return {"error": "Exact bounded regular file facts required"}
        seen.add(row["path"])
        total += int(row["bytes"])
        for field in FIELDS:
            value = int(row[field])
            maximum = value if maximum is None or value > maximum else maximum
    sources = []
    for row in ordered:
        for field in FIELDS:
            if int(row[field]) == maximum and len(sources) < 8:
                sources.append({**row, "field": field})
    return {"value": {"files": len(rows), "bytes": str(total), "maximumNs": None if maximum is None else str(maximum), "sources": sources}}


def whole(value):
    return isinstance(value, (int, float)) and float(value).is_integer() and abs(value) <= 2 ** 53 - 1


def fits(used, source, policy):
    return whole(used) and used >= 0 and used <= policy["maximumScanSourceBytes"] and whole(source) and source > 0 and source <= policy["maximumSourceBytes"] and source <= policy["maximumScanSourceBytes"] - used


def describe(root, expected, current, limit):
    def part(unit, capture):
        source = unit.get("recencySource") if capture else None
        return {"bytes": unit["bytes"], "recencyMs": unit["recencyMs"], "lockHeld": unit["lockHeld"], "recencySource": source if capture and source else None, "sourceStatus": "captured" if capture and source else "not-retained"}

    def build(capture):
        return {"root": root[:256], "unit": expected["path"][:256], "expected": part(expected, capture), "current": part(current, capture) if current else None,
                "changed": {"missing": not current, "bytes": bool(current and current["bytes"] != expected["bytes"]), "recency": bool(current and current["recencyMs"] != expected["recencyMs"]), "lockHeld": bool(current and current["lockHeld"])}}
    value = build(True)
    return value if len(json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf8")) <= limit else build(False)


def generated(count):
    return [{"path": "p%04d" % i, "device": "1", "inode": "1", "bytes": "1", "atimeNs": str(i), "mtimeNs": "0", "ctimeNs": "0"} for i in range(count)]


def main():
    request = json.load(sys.stdin)
    op = request["op"]
    if op == "summarize":
        print(json.dumps([summarize(row["rows"] + generated(row.get("generate", 0))) for row in request["rows"]]))
    elif op == "fits":
        print(json.dumps([fits(r["used"], r["bytes"], request["policy"]) for r in request["rows"]]))
    elif op == "describe":
        print(json.dumps([describe(r["root"], r["expected"], r["current"], request["limit"]) for r in request["rows"]]))
    else:
        raise SystemExit("unknown op " + op)


main()
