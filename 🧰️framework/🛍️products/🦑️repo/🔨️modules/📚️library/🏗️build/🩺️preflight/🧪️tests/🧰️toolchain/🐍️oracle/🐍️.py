"""🐍️ Independent reader of the package lists that provision a Linux machine: the apt packages a Dockerfile installs in its `apt-get install` command and the ones the bootstrap script names for `apt-get`, parsed with shlex instead of the TypeScript reader. usage: python3 -I 🐍️.py <Dockerfile> <bootstrap.sh>"""
import json, re, shlex, sys

dockerfile = open(sys.argv[1], encoding="utf8").read().replace("\\\n", " ")
installed = set()
for command in re.findall(r"apt-get install\b([^&;]*)", dockerfile):
    installed.update(word for word in shlex.split(command) if not word.startswith("-"))
bootstrap = open(sys.argv[2], encoding="utf8").read()
line = next(l for l in bootstrap.splitlines() if l.strip().startswith("apt-get)"))
named = set(shlex.split(line.split("printf", 1)[1].split(";;")[0])[1:])
print(json.dumps({"dockerfile": sorted(installed), "bootstrap": sorted(named)}))
