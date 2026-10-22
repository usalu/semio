"""🐍️ Independent model of the inverted fact index (stdlib only): owners sorted by code point, rows numbered by first appearance, and classes = rows that have exactly the same consumers, in the order of their first row, followed by one class per owner without rows.

usage: oracle.py index CORPUS   CORPUS = {"owners": {name: [row, ...]}}; prints {"owners", "rows", "classes"} as JSON"""
import json, sys


def row_key(row):
    return json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def index(corpus):
    owners = sorted(corpus["owners"])
    ids, rows, consumers = {}, [], []
    for number, name in enumerate(owners):
        for row in corpus["owners"][name]:
            key = row_key(row)
            if key not in ids:
                ids[key] = len(rows)
                rows.append(row)
                consumers.append([])
            if not consumers[ids[key]] or consumers[ids[key]][-1] != number:
                consumers[ids[key]].append(number)
    groups, order = {}, []
    for identifier, who in enumerate(consumers):
        key = tuple(who)
        if key not in groups:
            groups[key] = {"r": [], "o": list(who)}
            order.append(key)
        groups[key]["r"].append(identifier)
    classes = [groups[key] for key in order] + [{"r": [], "o": [number]} for number, name in enumerate(owners) if not corpus["owners"][name]]
    return {"owners": owners, "rows": rows, "classes": classes}


if __name__ == "__main__":
    if sys.argv[1] != "index":
        raise SystemExit("unknown command")
    print(json.dumps(index(json.load(open(sys.argv[2], encoding="utf8"))), ensure_ascii=False))
