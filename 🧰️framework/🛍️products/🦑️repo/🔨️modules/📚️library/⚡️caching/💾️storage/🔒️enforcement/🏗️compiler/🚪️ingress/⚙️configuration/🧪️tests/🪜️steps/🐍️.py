"""🪜 Independent oracle for Cargo step admission: a step is admissible exactly when its rendered `--config` selectors before the program delimiter equal those of the compiler view.
usage: python3 -I 🐍️.py <observations.json>   prints {step id: admitted|refused}"""
import json, sys
rows = json.load(open(sys.argv[1], encoding="utf8"))
def selectors(args):
    head = args[:args.index("--")] if "--" in args else args
    out, i = [], 0
    while i < len(head):
        if head[i] == "--config": out += head[i:i + 2]; i += 2; continue
        if head[i].startswith("--config="): out.append(head[i])
        i += 1
    return out
print(json.dumps({row["id"]: ("admitted" if row["rendered"] is not None and selectors(row["rendered"]) == selectors(row["view"]) else "refused") for row in rows}))
