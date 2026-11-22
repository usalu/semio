import json, os, re, sys

config = json.load(open(sys.argv[1], encoding="utf-8"))
root = sys.argv[2]
threshold = config["thresholdMilliseconds"]
forms = [re.compile(form["pattern"], re.M) for form in config["forms"]]
test_file = re.compile(config["testFile"])
skipped = set(config["skippedDirectories"])
found = {}
for directory, names, files in os.walk(root, followlinks=False):
    names[:] = [name for name in names if name not in skipped and not os.path.islink(os.path.join(directory, name))]
    for name in files:
        path = os.path.join(directory, name)
        relative = os.path.relpath(path, root).replace(os.sep, "/")
        if os.path.islink(path) or not re.search(r"\.(ts|mjs|js)$", name) or not test_file.search(relative):
            continue
        with open(path, encoding="utf-8", errors="replace") as handle:
            text = handle.read()
        count = sum(1 for form in forms for match in form.finditer(text) if int(match.group(1).replace("_", "")) >= threshold)
        if count:
            found[relative] = count
json.dump(dict(sorted(found.items(), key=lambda row: row[0].encode("utf-8"))), sys.stdout, ensure_ascii=False)
