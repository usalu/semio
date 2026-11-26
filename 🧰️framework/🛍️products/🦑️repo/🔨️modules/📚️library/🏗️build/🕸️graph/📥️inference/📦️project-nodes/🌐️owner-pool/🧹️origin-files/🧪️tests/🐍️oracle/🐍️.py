import json, re, sys

fixture = json.load(sys.stdin)
policy = fixture["policy"]
temporary = re.compile(policy["temporaryPattern"])
published = re.compile(policy["publishedPattern"])
apple = re.compile(policy["appleDoublePattern"])
out = []
for case in fixture["cases"]:
    alive = set(case["alive"])
    removed = []
    for file in case["files"]:
        name, age = file["name"], file["ageMilliseconds"]
        folder, _, leaf = name.rpartition("/")
        if folder in policy["appleDoubleDirectories"] and apple.match(leaf):
            removed.append(name)
            continue
        found = temporary.match(name)
        if found:
            pid = int(found.group(2))
            if pid != fixture["own"] and (pid not in alive or age > policy["temporaryMaximumAgeMilliseconds"]):
                removed.append(name)
        elif published.match(name) and age > policy["publishedRetentionMilliseconds"]:
            removed.append(name)
    out.append(sorted(removed))
json.dump(out, sys.stdout)
