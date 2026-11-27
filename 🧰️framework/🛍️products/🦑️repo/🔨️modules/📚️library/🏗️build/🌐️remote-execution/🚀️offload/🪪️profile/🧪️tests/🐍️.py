import hashlib, json, sys

CODEGEN = {"opt-level", "lto", "codegen-units", "debuginfo", "target-cpu", "target-feature"}
OFF = {"no", "off", "false", "n"}

def identity(args):
    values, features, target = {}, set(), "-"
    items = []
    i = 0
    while i < len(args):
        a = args[i]
        if a in ("-C", "--codegen") and i + 1 < len(args):
            items.append(args[i + 1]); i += 2
        elif a.startswith("-C") and len(a) > 2:
            items.append(a[2:]); i += 1
        elif a.startswith("--codegen="):
            items.append(a[len("--codegen="):]); i += 1
        elif a == "--target" and i + 1 < len(args):
            target = args[i + 1]; i += 2
        elif a.startswith("--target="):
            target = a[len("--target="):]; i += 1
        elif a == "-O":
            values["opt-level"] = "2"; i += 1
        elif a == "-g":
            values["debuginfo"] = "2"; i += 1
        else:
            i += 1
    for item in items:
        name, _, value = item.partition("=")
        if name not in CODEGEN:
            continue
        if name == "target-feature":
            features.update(v for v in value.split(",") if v)
        elif name == "lto":
            values[name] = "off" if value in OFF else "fat" if value in ("", "yes", "on", "true", "y", "fat") else value
        else:
            values[name] = value
    return ";".join([
        "opt-level=" + values.get("opt-level", "0"),
        "lto=" + values.get("lto", "-"),
        "codegen-units=" + values.get("codegen-units", "-"),
        "debuginfo=" + values.get("debuginfo", "0"),
        "target=" + target,
        "target-cpu=" + values.get("target-cpu", "-"),
        "target-feature=" + (",".join(sorted(features)) or "-"),
    ])

def key(args):
    return hashlib.sha256(identity(args).encode()).hexdigest()[:10]

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
print(json.dumps({
    "identities": [identity(row["args"]) for row in fixture["cases"]],
    "same": [key(a) == key(b) for _, a, b in fixture["sameKey"]],
    "different": [key(a) != key(b) for _, a, b in fixture["differentKey"]],
    "settings": [key(row["sameAs"]) for row in fixture["settingsKeys"]],
    "peakKey": key(fixture["peakKey"]["args"]) + "_" + fixture["peakKey"]["crate"],
}))
