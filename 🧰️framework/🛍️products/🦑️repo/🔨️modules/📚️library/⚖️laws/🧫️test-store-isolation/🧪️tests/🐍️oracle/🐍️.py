#!/usr/bin/env python3
"""Independent model of the test-store-isolation law (Python standard library only, cross-platform).

It shares only JSON data with the TypeScript law (the rules document, the budget policy and the fixtures), never code. Without a syntax tree it scans the text: identifiers initialised with a checkout path (the reference, a join/resolve/normalize/realpathSync/dirname call over one, or a ??/||/?: branch that is one) are tainted; a parenthesised callee counts by every store name inside it; a resolve/join of the file's own directory with exactly as many `..` as the file lies deep is the checkout; a store call whose balanced argument text holds a checkout reference is `checkout-store-call`; the innermost join/resolve call, template literal or `+` chain whose text holds both a live store literal and a checkout reference is `checkout-store-path`.
usage:
  python3 oracle.py fixtures RULES.json FIXTURES.json        (JSON list of {name, findings} per case)
  python3 oracle.py fence POLICY.json CHECKOUT               (JSON list of the fenced store directories)"""
import json, os, re, sys

def balanced(text, start):
    depth, i, quote = 0, start, None
    while i < len(text):
        c = text[i]
        if quote:
            if c == "\\": i += 2; continue
            if c == quote: quote = None
        elif c in "\"'`": quote = c
        elif c in "([{": depth += 1
        elif c in ")]}":
            depth -= 1
            if depth == 0: return i
        i += 1
    return len(text) - 1

def line_of(text, index): return text.count("\n", 0, index) + 1

FUNCTION = re.compile(r"\s*(?:async\s+)?(?:function\b|\([^()]*\)\s*(?::[^=]*)?=>|[A-Za-z_$][\w$]*\s*=>)")

def scan(source, rules, path="source.ts"):
    calls = [re.compile(r"(?<![\w$.])" + r"\s*\.\s*".join(map(re.escape, c.split("."))) + r"\s*\(") if "." in c else re.compile(r"(?<![\w$])" + re.escape(c) + r"\s*\(") for c in rules["checkoutCalls"]]
    env = rules["checkoutEnvironment"]
    tainted = set()
    levels = path.count("/")
    def upward(fragment):
        for m in re.finditer(r"(?<![\w$.])(?:path\.)?(?:resolve|join)\s*\(\s*(?:import\.meta\.(?:dir|dirname)|__dirname)\s*((?:,\s*\"[^\"]*\"\s*)+)\)", fragment):
            parts = [p for p in "/".join(re.findall(r"\"([^\"]*)\"", m.group(1))).split("/") if p]
            if parts and len(parts) == levels and all(p == ".." for p in parts): return True
        return False
    def checkout(fragment):
        if any(c.search(fragment) for c in calls) or upward(fragment): return True
        if any(re.search(r"env(?:\.%s\b|\[\s*[\"']%s[\"']\s*\])" % (re.escape(v), re.escape(v)), fragment) for v in env): return True
        return any(re.search(r"(?<![\w$.])%s(?![\w$])(?!\s*:)" % re.escape(n), fragment) for n in tainted)
    def is_path(initial):
        initial = re.sub(r"^(?:await\s+|\()+", "", initial)
        call = re.match(r"(?:[A-Za-z_$][\w$]*\.)*(join|resolve|normalize|realpathSync|dirname)\s*\(", initial)
        if call: return checkout(initial[:balanced(initial, call.end() - 1) + 1]) if upward(initial) else checkout(initial[call.end():balanced(initial, call.end() - 1)])
        head = re.split(r"\?\?|\|\||\?|:", initial)
        return any(checkout(part) and re.match(r"^\s*(?:[\w$.]+\s*\(\s*\)|[\w$.]+(?:\[[^\]]*\])?)\s*!?\s*(?:as\s+\w+)?\s*\)?\s*$", part) for part in head)
    grown = True
    while grown:
        grown = False
        for m in re.finditer(r"\b(?:const|let|var)\s+", source):
            i = m.end()
            while True:
                d = re.match(r"([A-Za-z_$][\w$]*)\s*=\s*", source[i:])
                if not d: break
                j, depth, quote = i + d.end(), 0, None
                while j < len(source):
                    c = source[j]
                    if quote:
                        if c == "\\": j += 2; continue
                        if c == quote: quote = None
                    elif c in "\"'`": quote = c
                    elif c in "([{": depth += 1
                    elif c in ")]}":
                        if depth == 0: break
                        depth -= 1
                    elif c in ",;\n" and depth == 0: break
                    j += 1
                name = d.group(1); initial = source[i + d.end():j]
                if name not in tainted and is_path(initial.strip()): tainted.add(name); grown = True
                if j < len(source) and source[j] == ",": i = j + 1; continue
                break
    found = set()
    store = [re.compile(p) for p in rules["storeCalls"]]
    literals = [re.compile(p) for p in rules["storeLiterals"]]
    for m in re.finditer(r"([A-Za-z_$][\w$]*)\s*\(", source):
        close = balanced(source, m.end() - 1); arguments = source[m.end():close]
        if any(p.search(m.group(1)) for p in store) and checkout(arguments): found.add((line_of(source, m.start()), "checkout-store-call"))
    for m in re.finditer(r"\(([^()]*)\)\s*\(", source):
        close = balanced(source, m.end() - 1)
        if any(p.search(name) for name in re.findall(r"[A-Za-z_$][\w$]*", m.group(1)) for p in store) and checkout(source[m.end():close]): found.add((line_of(source, m.start()), "checkout-store-call"))
    def has_literal(fragment): return any(p.search(x) for t in re.findall(r"\"([^\"]*)\"|'([^']*)'|`([^`]*)`", fragment) for x in t if x for p in literals)
    spans = []
    for m in re.finditer(r"(?:\bjoin|\bresolve)\s*\(", source):
        close = balanced(source, m.end() - 1); spans.append((m.start(), close + 1))
    for m in re.finditer(r"`[^`]*`", source): spans.append((m.start(), m.end()))
    hits = [(a, b) for a, b in spans if has_literal(source[a:b]) and checkout(source[a:b].replace("`", " ") if source[a] == "`" else source[a:b])]
    for a, b in hits:
        if not any((c, d) != (a, b) and a <= c and d <= b for c, d in hits): found.add((line_of(source, a), "checkout-store-path"))
    return [{"line": l, "rule": r} for l, r in sorted(found)]

def main():
    if sys.argv[1] == "fixtures":
        rules = json.load(open(sys.argv[2], encoding="utf-8")); fixtures = json.load(open(sys.argv[3], encoding="utf-8"))
        print(json.dumps([{"name": c["name"], "findings": scan(c["source"], rules, c.get("path", "source.ts"))} for c in fixtures["cases"]], ensure_ascii=False))
    elif sys.argv[1] == "fence":
        policy = json.load(open(sys.argv[2], encoding="utf-8")); checkout = sys.argv[3]
        print(json.dumps([os.path.join(checkout, *p["directory"].split("/")) for p in policy["partitions"].values()] + [os.path.join(checkout, ".cache", "b", "c")], ensure_ascii=False))
    else: sys.exit("usage: fixtures RULES FIXTURES | fence POLICY CHECKOUT")

main()
