#!/usr/bin/env python3
"""Independent Python model of cfg predicates (test-support gate features and every required feature) over the corpus.

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


def tokenize(s):
    return re.findall(r'"(?:[^"\\]|\\.)*"|[A-Za-z_][A-Za-z0-9_]*|[(),=]|\S', s)


def parse_pred(tok, i):
    if i >= len(tok) or not re.match(r"[A-Za-z_]", tok[i]):
        return None, i
    name = tok[i]
    i += 1
    if i < len(tok) and tok[i] == "=":
        if i + 1 >= len(tok) or not tok[i + 1].startswith('"'):
            return None, i
        return ("value", name, tok[i + 1][1:-1]), i + 2
    if i < len(tok) and tok[i] == "(":
        i += 1
        args = []
        while True:
            if i >= len(tok):
                return None, i
            if tok[i] == ")":
                return ("call", name, args), i + 1
            node, i = parse_pred(tok, i)
            if node is None:
                return None, i
            args.append(node)
            if i < len(tok) and tok[i] == ",":
                i += 1
    return ("atom", name), i


def predicate(attr):
    s = attr.strip()
    if not re.match(r"cfg\s*\(", s):
        return None
    node, _ = parse_pred(tokenize(s), 0)
    return node


def walk(node, negated, on_value, on_call):
    if node[0] == "value":
        on_value(node, negated)
    elif node[0] == "call":
        on_call(node, negated)
        for a in node[2]:
            walk(a, negated or node[1] == "not", on_value, on_call)


def uniq(xs):
    out = []
    for x in xs:
        if x not in out:
            out.append(x)
    return out


def gate_features(attr):
    node = predicate(attr)
    out = []
    if node is None:
        return out

    def on_value(n, neg):
        if not neg and n[1] == "feature" and n[2] == "testkit":
            out.append(n[2])

    def on_call(n, neg):
        if n[1] == "any" and not neg and any(a == ("atom", "test") for a in n[2]):
            out.extend(a[2] for a in n[2] if a[0] == "value" and a[1] == "feature")

    walk(node, False, on_value, on_call)
    return uniq(out)


def cfg_features(attr):
    node = predicate(attr)
    out = []
    if node is None:
        return out
    walk(node, False, lambda n, neg: (not neg and n[1] == "feature") and out.append(n[2]), lambda n, neg: None)
    return uniq(out)


for row in corpus["cfg"]:
    got = {"gate": gate_features(row["attribute"]), "cfg": cfg_features(row["attribute"])}
    if got["gate"] != row["gate"] or got["cfg"] != row["cfg"]:
        failures.append({"attribute": row["attribute"], "got": got})
    test_only = bool(re.match(r"^cfg\(\s*test\s*\)$", row["attribute"].strip())) or bool(got["gate"])
    if test_only != row["testOnly"]:
        failures.append({"attribute": row["attribute"], "testOnly": test_only})
for row in corpus["keys"]:
    m = re.match(r"^\s*([A-Za-z0-9_-]+)(?:\.[A-Za-z0-9_-]+)?\s*=", row["row"])
    if (m.group(1) if m else "") != row["key"] or row["key"].replace("-", "_") != row["ident"]:
        failures.append({"row": row["row"]})
for row in corpus["only"]:
    found = None
    for a in row["attributes"]:
        if re.match(r"^cfg\(\s*test\s*\)$", a.strip()) or gate_features(a):
            found = a
            break
    if found != row["expected"]:
        failures.append({"only": row["attributes"], "got": found})
for row in corpus["required"]:
    got = sorted(f for f, names in row["gated"].items() if set(names) & set(row["targetTokens"]))
    if got != row["expected"]:
        failures.append({"required": row["name"], "got": got})
for row in corpus["featureRows"]:
    if row["defect"]:
        continue
    f = row["feature"]
    if f["name"] + " = [" + ", ".join(json.dumps(e) for e in f["entries"]) + "]" != row["expected"]:
        failures.append({"featureRow": f["name"]})

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
