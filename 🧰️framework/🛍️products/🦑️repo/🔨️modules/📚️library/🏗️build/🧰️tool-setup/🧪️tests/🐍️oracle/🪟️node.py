import json
import ntpath
import re
import sys

with open(sys.argv[1], encoding="utf-8") as source:
    fixture = json.load(source)

for case in fixture["cases"]:
    env = {key.lower(): value for key, value in case["environment"].items()}
    roots = [env.get(key, "") for key in ("programw6432", "programfiles")]
    appdata = env.get("localappdata", "")
    locations = [ntpath.join(appdata, "semio", "node", "v" + fixture["pin"])] if re.match(r"^[A-Za-z]:[/\\]", appdata) else []
    locations.append(ntpath.join(fixture["home"], ".local", "node", "bin"))
    locations += [ntpath.join(root, "nodejs") for root in roots if re.match(r"^[A-Za-z]:[/\\]", root)]
    locations.extend(filter(None, env.get("path", "").split(";")))
    files = case["executables"]
    location = next((file for directory in locations if (file := ntpath.join(directory, "node.exe")) in files), None)
    found = files[location].removeprefix("v") if location is not None else None
    status = "missing" if found is None else "ok" if found == fixture["pin"] else "differs"
    assert {"status": status, "location": location, "found": found} == case["expected"], case["name"]
    print(case["name"], status)
