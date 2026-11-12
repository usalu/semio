"""🐍️ Independent model of the row log (stdlib only): the distinct rows of a set of owners and the rows of a log file, keyed by canonical JSON. It never imports the JavaScript implementation.

usage: 🐍️.py distinct FIXTURE   prints {"distinct": N, "owners": {name: [keys of its distinct rows in first-seen order]}}
       🐍️.py log LOGFILE       prints {"lines": N, "keys": [sorted keys of the rows the file holds]}"""
import json
import sys


def key(row):
    """🔑️ The canonical text of one row: sorted keys, no spaces, characters unescaped."""
    return json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def distinct(fixture):
    seen, owners = set(), {}
    for name, rows in fixture["owners"].items():
        mine, order = set(), []
        for row in rows:
            text = key(row)
            seen.add(text)
            if text not in mine:
                mine.add(text)
                order.append(text)
        owners[name] = order
    return {"distinct": len(seen), "owners": owners}


def log(path):
    with open(path, encoding="utf8") as handle:
        rows = [json.loads(line) for line in handle]
    return {"lines": len(rows), "keys": sorted(key(row) for row in rows)}


if __name__ == "__main__":
    if sys.argv[1] == "distinct":
        print(json.dumps(distinct(json.load(open(sys.argv[2], encoding="utf8"))), ensure_ascii=False))
    elif sys.argv[1] == "log":
        print(json.dumps(log(sys.argv[2]), ensure_ascii=False))
