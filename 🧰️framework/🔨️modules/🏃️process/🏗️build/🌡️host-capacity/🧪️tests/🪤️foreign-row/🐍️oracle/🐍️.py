"""🪤️ Independent model of how a grant row of another format is isolated. argv[1] is a JSON document {current, fallbackMegabytes, rows: [{row, format}]}: format is what the sidecar of the row says; the stand-in each row gets and the megabytes the ledger holds for them."""
import json
import sys


def whole(value):
    return isinstance(value, int) and not isinstance(value, bool) and value >= 0


def stand_in(item, current, fallback):
    row, fmt = item["row"], item["format"]
    get = (lambda key: row.get(key)) if isinstance(row, dict) else (lambda key: None)
    known = fmt if whole(fmt) and fmt > 0 else None
    relation = "unversioned" if known is None else "older" if known < current else "newer" if known > current else "invalid"
    charge = get("reservedMegabytes")
    declared = whole(charge)
    return {"relation": relation, "megabytes": charge if declared else fallback, "declared": declared}


def main():
    case = json.loads(sys.argv[1])
    rows = [stand_in(item, case["current"], case["fallbackMegabytes"]) for item in case["rows"]]
    print(json.dumps({"rows": rows, "held": sum(row["megabytes"] for row in rows)}))


main()
