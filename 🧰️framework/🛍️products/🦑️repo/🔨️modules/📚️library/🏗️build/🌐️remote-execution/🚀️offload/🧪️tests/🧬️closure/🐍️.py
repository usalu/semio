import json, os, re, sys

ARTIFACT = re.compile(r"^lib(.+)-([0-9a-f]{16})\.(rlib|rmeta|so|dylib|dll)$")

def closure(directories, direct, rlibs):
    by_suffix, by_name = {}, {}
    for directory in directories:
        if not os.path.isdir(directory):
            continue
        for name in sorted(os.listdir(directory)):
            match = ARTIFACT.match(name)
            if match:
                by_suffix.setdefault(match.group(2), {}).setdefault(directory, []).append(name)
                by_name.setdefault(match.group(1), set()).add(match.group(2))
    named = {os.path.basename(path) for path in direct}
    def record(suffix):
        for directory, files in by_suffix.get(suffix, {}).items():
            source = next((f for f in files if f.endswith(".rmeta")), None) or next((f for f in files if f.endswith(".rlib")), None)
            if source and os.path.exists(os.path.join(directory, source + ".semio-deps")):
                return json.load(open(os.path.join(directory, source + ".semio-deps")))
        return None
    chosen, queue, selected = set(), [], {}
    def take(suffix):
        if suffix in by_suffix and suffix not in chosen:
            chosen.add(suffix)
            queue.append(suffix)
    for name in named:
        match = ARTIFACT.match(name)
        if match:
            take(match.group(2))
    while queue:
        suffix = queue.pop(0)
        edges = record(suffix)
        for directory, files in by_suffix[suffix].items():
            metadata = any(f.endswith(".rmeta") for f in files)
            for f in files:
                if f.endswith(".rlib") and not rlibs and metadata and f not in named:
                    continue
                selected.setdefault(directory, []).append(f)
                if edges is not None or (metadata and not f.endswith(".rmeta")):
                    continue
                for token in re.findall(rb"[0-9a-f]{16}", open(os.path.join(directory, f), "rb").read()):
                    take(token.decode())
        if edges is not None:
            for spec, svh in edges["dependencies"]:
                name, _, extra = spec.rpartition("-")
                if not name:
                    continue
                if extra in by_suffix:
                    take(extra)
                    continue
                candidates = sorted(by_name.get(name, set()))
                exact = [c for c in candidates if (record(c) or {}).get("svh") == svh]
                for c in exact or [c for c in candidates if record(c) is None]:
                    take(c)
    return {directory: sorted(files) for directory, files in selected.items()}

request = json.load(sys.stdin)
print(json.dumps(closure(request["directories"], request["direct"], request["rlibs"]), sort_keys=True))
