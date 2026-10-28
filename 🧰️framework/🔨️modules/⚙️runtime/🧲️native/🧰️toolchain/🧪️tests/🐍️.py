"""🐍️ Independent implementation of the native toolchain probes: from the same table and the same machine descriptions it decides what the probes must find, with Python's own path rules (ntpath on Windows, posixpath elsewhere). usage: python3 -I 🐍️.py <schema.json> <scenarios.json> [real] — `real` instead probes this machine with shutil.which and subprocess."""
import json, ntpath, posixpath, shutil, subprocess, sys, os

table = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Protocol"]["const"]


def decide(scenario):
    win = scenario["platform"] == "win32"
    path = ntpath if win else posixpath
    env = scenario["env"]
    files, dirs = set(scenario["files"]), set(scenario["directories"])
    commands = {(c["program"], tuple(c["arguments"])): c for c in scenario["commands"]}

    def var(name):
        for key, value in env.items():
            if (key.lower() == name.lower()) if win else (key == name):
                return value or None
        return None

    def path_dirs():
        return [d for d in (var("PATH") or "").split(";" if win else ":") if d]

    def file_name(program):
        return program + ".exe" if win and "." not in program else program

    def first(directories, programs):
        for d in directories:
            for p in programs:
                candidate = path.join(d, file_name(p))
                if candidate in files:
                    return candidate
        return None

    def run(program, args):
        c = commands.get((program, tuple(args)))
        return None if c is None else (c["status"], c["stdout"])

    def probe(tool_id):
        row = table["tools"][tool_id]
        kind = row["kind"]
        if kind == "path":
            hit = first(path_dirs(), row["programs"])
            return (hit is not None, hit, "path" if hit else "none")
        if kind == "rust":
            named = var(row["environment"]["compiler"])
            if named:
                return (True, named, "environment") if named in files else (False, None, "none")
            hit = first(path_dirs(), row["programs"])
            if hit:
                return (True, hit, "path")
            dirs_ = []
            for entry in row["directories"]:
                base = var(entry["environment"]) or path.join(scenario["home"], *entry["homeSegments"])
                dirs_.append(path.join(base, *entry["segments"]))
            hit = first(dirs_, row["programs"])
            return (True, hit, "cargo-home") if hit else (False, None, "none")
        if kind == "developer-directory":
            named = var(row["environment"])
            if named and named in dirs:
                return (True, named, "environment")
            result = run(row["program"], row["arguments"])
            if result and result[0] == 0:
                lines = result[1].splitlines()
                printed = lines[0].strip() if lines else ""
                if printed and printed in dirs:
                    return (True, printed, "developer-directory")
            return (False, None, "none")
        if kind == "visual-studio":
            hit = first(path_dirs(), [row["linker"]])
            if hit:
                return (True, hit, "path")
            named = var(row["installation"]["environment"])
            tools_in = lambda inst: path.join(inst, *row["toolsSegments"]) in dirs
            if named and tools_in(named):
                return (True, named, "environment")
            component = row["components"].get(scenario["arch"])
            if component is None:
                return (False, None, "none")
            locator = row["locator"]
            bases = [var(n) for n in locator["environments"]]
            lead = locator["segments"][:-1]
            directories = [path.join(b, *lead) for b in bases if b]
            where = first(directories, [locator["segments"][-1]]) or first(path_dirs(), [locator["fallback"]])
            if where is None:
                return (False, None, "none")
            result = run(where, [a.replace("{component}", component) for a in row["arguments"]])
            if result and result[0] == 0:
                installation = next((l.strip() for l in result[1].splitlines() if l.strip()), None)
                if installation and tools_in(installation):
                    return (True, installation, "installer")
            return (False, None, "none")
        raise SystemExit("unknown kind " + kind)

    return [{"id": i, "found": f, "location": l, "how": h} for i in table["requirements"][scenario["platform"]] for (f, l, h) in [probe(i)]]


def real():
    platform = {"darwin": "darwin", "linux": "linux", "win32": "win32"}[sys.platform if sys.platform in ("darwin", "win32") else "linux"]
    out = {}
    for tool in table["requirements"][platform]:
        if tool == "cc":
            out[tool] = shutil.which("cc") is not None
        elif tool == "rust":
            cargo = os.environ.get("CARGO_HOME") or os.path.join(os.path.expanduser("~"), ".cargo")
            names = ("rustc", "rustup")
            named = os.environ.get("SEMIO_RUSTC")
            if named:
                out[tool] = os.path.isfile(named)
            else:
                out[tool] = any(shutil.which(n) for n in names) or any(shutil.which(n, path=os.path.join(cargo, "bin")) for n in names)
        elif tool == "developer-tools":
            if os.environ.get("DEVELOPER_DIR") and os.path.isdir(os.environ["DEVELOPER_DIR"]):
                out[tool] = True
            else:
                run = subprocess.run(["/usr/bin/xcode-select", "-p"], capture_output=True, text=True)
                out[tool] = run.returncode == 0 and os.path.isdir(run.stdout.strip())
        elif tool == "msvc-linker":
            out[tool] = shutil.which("link") is not None or bool(os.environ.get("VSINSTALLDIR"))
    return out


if len(sys.argv) > 3 and sys.argv[3] == "real":
    print(json.dumps(real()))
else:
    scenarios = json.load(open(sys.argv[2], encoding="utf8"))["scenarios"]
    print(json.dumps({s["id"]: decide(s) for s in scenarios}, ensure_ascii=False))
