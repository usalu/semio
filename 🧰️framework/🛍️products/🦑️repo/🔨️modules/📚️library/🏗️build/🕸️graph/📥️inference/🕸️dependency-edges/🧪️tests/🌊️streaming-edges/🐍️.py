#!/usr/bin/env python3
"""🐍️ Independent oracle of the edges of the owners of a workspace: relative imports go to the deepest owner root, bare imports to the workspace package of their first one or two segments, the dependencies of a package.json to the workspace package of that name; the file of an edge is the file that holds the import or the package.json. The workspace project and unknown projects have none. No code of the build library is shared.

usage: 🐍️.py < {"worlds":[{"projects":{name:{"root","files":{path:text}}},"byPackage":{package:name}}]}   prints {"answers":[{"edges":{name:[[target,file],...]}}]}"""
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

    edges = {}
    for name, project in world["projects"].items():
        found = set()
        if name != "workspace":
            for path, text in project["files"].items():
                if path.endswith("package.json"):
                    manifest = json.loads(text)
                    for key in ("dependencies", "peerDependencies", "optionalDependencies"):
                        for dependency in manifest.get(key, {}):
                            target = world["byPackage"].get(dependency)
                            if target and target != name:
                                found.add((target, path))
                    continue
                for match in IMPORT.finditer(text):
                    specifier = match.group(1) or match.group(2)
                    if specifier.startswith("."):
                        target = owning(posixpath.normpath(posixpath.join(posixpath.dirname(path), specifier)))
                    else:
                        segments = specifier.split("/")
                        target = world["byPackage"].get("/".join(segments[:2]) if specifier.startswith("@") else segments[0])
                    if target and target != name:
                        found.add((target, path))
        edges[name] = sorted(map(list, found))
    return {"edges": edges}


print(json.dumps({"answers": [answer(world) for world in json.load(sys.stdin)["worlds"]]}))
