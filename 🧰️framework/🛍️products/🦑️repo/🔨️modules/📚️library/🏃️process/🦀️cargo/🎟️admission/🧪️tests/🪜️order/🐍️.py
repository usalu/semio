"""🪢 Independent text oracle for the order of the repository Cargo test admission port and the neutral test and lint runners.
usage: python3 -I 🐍️.py <fixture.json> <fixture-dir>   prints the facts as JSON"""
import json, os, re, sys
fixture, here = json.load(open(sys.argv[1], encoding="utf8")), sys.argv[2]
def span(text, start, open_, close):
    depth, i = 0, start
    while True:
        depth += text[i] == open_; depth -= text[i] == close
        if depth == 0: return i + 1
        i += 1
def body(text, name):
    m = re.search(r"(?:async\s+)?function\s+" + re.escape(name) + r"\b[^(]*\(", text)
    start = text.index("{", span(text, m.end() - 1, "(", ")"))
    return text[m.start():span(text, start, "{", "}")]
def calls(text, name):
    return [(m.start(), span(text, m.end() - 1, "(", ")")) for m in re.finditer(r"\b" + re.escape(name) + r"\(", text)]
port = body(open(os.path.join(here, fixture["port"]["file"]), encoding="utf8").read(), fixture["port"]["function"])
writer = re.search(r"\b" + fixture["port"]["writerProperty"] + r"\s*:\s*\([^)]*\)\s*=>\s*" + fixture["port"]["writer"] + r"\(", port)
admit = re.search(r"\b" + fixture["port"]["admissionProperty"] + r"\s*:\s*\w+\s*=>\s*" + fixture["port"]["admission"] + r"\(", port)
runner = open(os.path.join(here, fixture["runner"]["file"]), encoding="utf8").read()
def ordered(name):
    text, r = body(runner, name), fixture["runner"]
    scopes, leases, enters = calls(text, r["scope"]), calls(text, r["lease"]), calls(text, r["enter"])
    if len(scopes) != 1 or len(leases) != 1 or len(enters) != 1: return False
    (s0, s1), (l0, l1), (e0, e1) = scopes[0], leases[0], enters[0]
    return s0 < l0 < l1 <= e0 < e1 <= s1
print(json.dumps({
 "portOffersWriter": bool(writer) and port.count(fixture["port"]["writer"] + "(") == 1,
 "portAdmitsSteps": bool(admit),
 "runnersOrdered": all(ordered(name) for name in fixture["runner"]["functions"])
}, sort_keys=True))
