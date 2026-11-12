import json, re, sys

project = json.load(sys.stdin)
names = sorted((n for n in project["namedInputs"] if re.fullmatch(r"commandSources\d+", n)), key=lambda n: int(n[14:]))
contents, rename, groups = {}, {}, []
for name in names:
    key = json.dumps(project["namedInputs"][name], ensure_ascii=False, separators=(",", ":"))
    if key not in contents:
        contents[key] = "commandSources%d" % len(contents)
        groups.append((contents[key], project["namedInputs"][name]))
    rename[name] = contents[key]
if all(rename[n] == n for n in names):
    print(json.dumps(project, ensure_ascii=False))
    sys.exit(0)
def rewrite(value):
    if isinstance(value, str):
        return rename.get(value, value)
    if isinstance(value, list):
        return [rewrite(item) for item in value]
    if isinstance(value, dict):
        return {k: rewrite(v) for k, v in value.items()}
    return value
kept = {k: v for k, v in project["namedInputs"].items() if k not in rename}
kept.update(dict(groups))
print(json.dumps({**project, "namedInputs": kept, "targets": rewrite(project["targets"])}, ensure_ascii=False))
