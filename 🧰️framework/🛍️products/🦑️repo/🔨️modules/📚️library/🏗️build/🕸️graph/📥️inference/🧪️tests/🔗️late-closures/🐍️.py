import json, os, re, sys

root, entry, question = sys.argv[1], sys.argv[2], json.loads(sys.argv[3])
seen, pending = [], [entry]
while pending:
    path = pending.pop()
    if path in seen:
        continue
    seen.append(path)
    text = open(os.path.join(root, path), encoding="utf8").read()
    for spec in re.findall(r'import\s+"(\./[^"]+)"', text):
        pending.append(os.path.normpath(os.path.join(os.path.dirname(path), spec)))
closure = sorted(seen)
print(json.dumps({"closure": closure, "answer": any(part in "/" + file for file in closure for part in question)}))
