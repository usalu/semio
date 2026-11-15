"""🐍 Independent oracle of the named key components: sha-256 of each component's JSON text, first 16 hex digits (stdlib only), with its own classifier of rustc arguments. usage: python -I this.py fixture.json"""
import hashlib, json, re, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
VALUE_FLAGS = {"--crate-name", "--crate-type", "--emit", "--out-dir", "--target", "--edition", "--extern", "-L", "-l", "-C", "--cfg", "--check-cfg", "--cap-lints", "--error-format", "--json", "--diagnostic-width", "--remap-path-prefix", "-A", "-W", "-D", "-F", "--force-warn", "-Z", "--codegen", "--sysroot", "--print", "--explain", "-o", "--color", "--allow", "--warn", "--deny", "--forbid", "--emit-dep-info"}
LINTS = {"-A", "-W", "-D", "-F", "--cap-lints", "--force-warn", "--allow", "--warn", "--deny", "--forbid"}

def part(value):
    return hashlib.sha256(json.dumps(value, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()[:16]

def classify(flag, value):
    if flag == "--edition": return "edition"
    if flag in ("--crate-name", "--crate-type", "--emit"): return "kind"
    if flag == "--target": return "target"
    if flag in ("-C", "--codegen"): return "metadata" if value.startswith(("metadata=", "extra-filename=")) else "codegen"
    if flag == "--cfg": return "features" if value.startswith("feature=") else "cfg"
    if flag == "--check-cfg": return "cfg"
    if flag in LINTS: return "lints"
    if flag in ("--remap-path-prefix", "--remap-path-scope"): return "remap"
    if flag in ("--error-format", "--json", "--color", "--diagnostic-width"): return "diagnostics"
    if flag in ("-L", "-l"): return "link"
    if flag == "-Z": return "unstable"
    return "other"

def classes(args):
    rows, index = {}, 0
    while index < len(args):
        argument = args[index]
        if argument == "-" or not argument.startswith("-"):
            rows.setdefault("source", []).append(argument); index += 1; continue
        equals = argument.find("=") if argument.startswith("--") else -1
        if equals > 0:
            flag, value, step = argument[:equals], argument[equals + 1:], 1
        elif argument in VALUE_FLAGS:
            flag, value, step = argument, args[index + 1] if index + 1 < len(args) else "", 2
        elif len(argument) > 2 and argument[1] in "LlCAWDFZo" and not argument.startswith("--"):
            flag, value, step = "-" + argument[1], argument[2:], 1
        else:
            flag, value, step = argument, "", 1
        index += step
        if flag == "--extern": continue
        rows.setdefault(classify(flag, value), []).append(flag + ("" if value == "" else "=" + value))
    return rows

def components(input, extra):
    rows = {"toolchain": part(input["toolchain"]), "source": input["sourceSha256"][:16]}
    if "platform" in extra: rows["platform"] = part(extra["platform"])
    if "roots" in extra: rows["roots"] = part(extra["roots"])
    for row in input["externs"]: rows["extern:" + row["name"]] = row["sha256"][:16] if re.fullmatch(r"[0-9a-f]{16,}", row["sha256"]) else part(row["sha256"])
    for name, value in input["environment"]: rows["env:" + name] = part(value)
    for name, items in classes(input["normalized"]).items(): rows["args." + name] = part(items)
    return rows

def variant(input, row):
    changed = {**input, **row.get("set", {})}
    if "from" in row:
        changed = {**changed, "normalized": [row["to"] if argument == row["from"] else argument for argument in changed["normalized"]]}
    return changed

print(json.dumps({"base": components(fixture["input"], fixture["extra"]), "variants": [components(variant(fixture["input"], row), row.get("extra", fixture["extra"])) for row in fixture["variants"]]}, ensure_ascii=False))
