"""🧷 Independent text oracle for the compiler-writer order of the repository port and the native artifact consumer.
usage: python3 -I 🐍️.py <fixture.json> <fixture-dir>   prints the facts as JSON"""
import json, os, re, sys
fixture, here = json.load(open(sys.argv[1], encoding="utf8")), sys.argv[2]
def body(text, name):
    m = re.search(r"(?:async\s+)?function\s+" + re.escape(name) + r"\b[^{]*\{", text)
    depth, i = 0, m.end() - 1
    while True:
        c = text[i]
        depth += c == "{"; depth -= c == "}"
        if depth == 0: return text[m.start():i + 1]
        i += 1
def call_span(text, name):
    m = re.search(r"\b" + re.escape(name) + r"\(", text)
    if not m: return None
    depth, i = 0, m.end() - 1
    while True:
        depth += text[i] == "("; depth -= text[i] == ")"
        if depth == 0: return (m.start(), i + 1)
        i += 1
port = body(open(os.path.join(here, fixture["port"]["file"]), encoding="utf8").read(), fixture["port"]["function"])
consumer = open(os.path.join(here, fixture["consumer"]["file"]), encoding="utf8").read()
build, capture = body(consumer, fixture["consumer"]["build"]), body(consumer, fixture["consumer"]["capture"])
writer = re.search(r"\b" + fixture["port"]["property"] + r"\s*:\s*(?:<[^>]*>)?\(.*?\)\s*=>\s*" + fixture["port"]["writer"] + r"\(", port)
span = call_span(capture, fixture["consumer"]["enter"])
lease, captured = build.find(fixture["consumer"]["lease"] + "("), build.find(fixture["consumer"]["capture"] + "(")
print(json.dumps({
 "portOffersWriter": bool(writer) and port.count(fixture["port"]["writer"] + "(") == 1,
 "leaseBeforeCompile": 0 <= lease < captured and "actual." + fixture["consumer"]["enter"] in build[captured:],
 "insideWriter": bool(span) and all(re.search(r"\b" + n + r"\(", capture[span[0]:span[1]]) for n in fixture["consumer"]["insideWriter"]),
 "afterWriter": bool(span) and all(capture.find(n + "(", span[1]) > 0 and capture.find(n + "(", span[0], span[1]) < 0 for n in fixture["consumer"]["afterWriter"])
}, sort_keys=True))
