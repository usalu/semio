#!/usr/bin/env python3
"""🐍️ Independent model of the diagnostics channel (standard library only): `areas <schema> <corpus>` answers the enabled areas (or the refused names) of every environment of the corpus, `scan <schema> <framework-root>` lists every line that carries the forbidden debug marker in the declared roots."""
import json, os, sys

schema = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]
mode = sys.argv[1]

if mode == "areas":
    protocol, known = schema["Protocol"]["const"], sorted(schema["Areas"]["const"])
    rows = []
    for item in json.load(open(sys.argv[3], encoding="utf-8"))["environments"]:
        raw = (item["value"] or "").strip().lower()
        if raw in protocol["silent"]:
            rows.append({"areas": []}); continue
        names = [part.strip() for part in raw.split(",") if part.strip()]
        if protocol["everything"] in names:
            rows.append({"areas": known}); continue
        bad = [name for name in names if name not in known]
        rows.append({"refused": bad} if bad else {"areas": sorted(set(names))})
    print(json.dumps(rows))
else:
    law, base = schema["Law"]["const"], sys.argv[3]
    marker = "".join(law["forbiddenMarkerParts"]).encode("utf-8")
    found, seen = [], set()
    for root in law["roots"]:
        for directory, directories, files in os.walk(os.path.join(base, root)):
            directories[:] = sorted(name for name in directories if name not in law["skipDirectories"])
            for name in sorted(files):
                path = os.path.join(directory, name)
                real = os.path.realpath(path)
                if real in seen or os.path.islink(path) or os.path.getsize(path) > law["maximumFileBytes"]:
                    continue
                seen.add(real)
                with open(path, "rb") as handle:
                    for number, line in enumerate(handle.read().split(b"\n"), 1):
                        if marker in line:
                            found.append({"path": os.path.relpath(path, base).replace(os.sep, "/"), "line": number})
    print(json.dumps(found, ensure_ascii=False))
