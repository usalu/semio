#!/usr/bin/env python3
"""🐍️ Independent oracle of the import edges of a workspace: reads the authored worlds (projects with their files and import lines, the package names) and answers, per world, the edges (source project, target project, source file) a relative import to the deepest owner root, a bare import to the workspace package of its first one or two segments and nothing else make; no code of the build library is shared.

usage: 🐍️.py < {"worlds":[{"projects":{name:{"root","files":{path:text}}},"byPackage":{package:name},"retain":[names]}]}
prints {"answers":[{"edges":[[source,target,file],...],"published":{name:[file,...]}}]}"""
import json, posixpath, re, sys

IMPORT = re.compile(r'^\s*(?:import|export)\b[^"\'\n]*?\bfrom\s*["\']([^"\']+)["\']|^\s*import\s*["\']([^"\']+)["\']', re.M)


def answer(world):
    owners = {}
    for name, project in world["projects"].items():
        if project["root"] != ".":
            owners.setdefault(project["root"], []).append(name)
    owner_of = {root: min(names) for root, names in owners.items()}

    def owning(path):
        parts = path.split("/")
        for end in range(len(parts), 0, -1):
            root = "/".join(parts[:end])
            if root in owner_of:
                return owner_of[root]
        return None

    edges, published = set(), {}
    for name, project in world["projects"].items():
        for path, text in project["files"].items():
            if name in world["retain"]:
                published.setdefault(name, []).append(path)
            if name == "workspace":
                continue
            for match in IMPORT.finditer(text):
                specifier = match.group(1) or match.group(2)
                if specifier.startswith("."):
                    target = owning(posixpath.normpath(posixpath.join(posixpath.dirname(path), specifier)))
                else:
                    segments = specifier.split("/")
                    target = world["byPackage"].get("/".join(segments[:2]) if specifier.startswith("@") else segments[0])
                if target and target != name:
                    edges.add((name, target, path))
    return {"edges": sorted(map(list, edges)), "published": {name: sorted(files) for name, files in published.items()}}


print(json.dumps({"answers": [answer(world) for world in json.load(sys.stdin)["worlds"]]}))
