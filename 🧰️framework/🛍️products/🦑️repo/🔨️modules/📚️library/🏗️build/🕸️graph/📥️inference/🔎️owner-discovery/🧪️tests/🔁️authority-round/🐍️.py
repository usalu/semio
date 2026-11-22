"""Independent oracle of the assembly authority of a native owner: the sorted workspace-relative roots of the path-dependency closure (dev-dependencies only of the start crate), each paired with its project (or null), hashed as compact JSON. Reads {root, crates, projects} on stdin (the fixture manifests are one `name={path=\"...\"}` line per dependency, so a line parser stands for a TOML parser), prints {crate: digest}."""
import hashlib
import json
import sys
import re
from pathlib import Path

request = json.load(sys.stdin)
root = Path(request["root"])
projects = request["projects"]


def deps(directory, with_dev):
    section, found = None, []
    for line in (directory / "Cargo.toml").read_text(encoding="utf8").splitlines():
        head = re.match(r"^\[(.+)\]$", line)
        if head:
            section = head.group(1)
            continue
        spec = re.match(r'^[\w-]+=\{path="([^"]+)"\}$', line)
        if spec and (section in ("dependencies", "build-dependencies") or with_dev and section == "dev-dependencies"):
            found.append((directory / spec.group(1)).resolve())
    return found


def closure(start):
    seen = {start}
    stack = [(child, False) for child in deps(start, True)]
    while stack:
        directory, _ = stack.pop()
        if directory in seen:
            continue
        seen.add(directory)
        stack.extend((child, False) for child in deps(directory, False))
    return sorted(str(path.relative_to(root.resolve())) for path in seen)


answer = {}
for name in request["crates"]:
    owners = [[owner, projects.get(owner)] for owner in closure((root / "crates" / name).resolve())]
    answer[name] = hashlib.sha256(json.dumps([owners, None], separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()
print(json.dumps(answer, ensure_ascii=False))
