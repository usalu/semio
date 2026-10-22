"""🔏️ Independent Python oracle of the Rust unit cache key (standard library only): it re-reads rustc command lines (options, crate types, emit kinds, externs, native links), decides the bypass, normalizes the command line, derives the key with hashlib over compact JSON, selects the environment, parses dependency files and build script directives, expands response files and derives the broker facts, from the authored policy passed in the request. Requests arrive as JSON on stdin."""
import hashlib, json, posixpath, re, sys

VALUE_FLAGS = {"--crate-name", "--crate-type", "--emit", "--out-dir", "--target", "--edition", "--extern", "-L", "-l", "-C", "--cfg", "--check-cfg", "--cap-lints", "--error-format", "--json", "--diagnostic-width", "--remap-path-prefix", "-A", "-W", "-D", "-F", "--force-warn", "-Z", "--codegen", "--sysroot", "--print", "--explain", "-o", "--color", "--allow", "--warn", "--deny", "--forbid", "--emit-dep-info"}


def options(args):
    found, positional, index = [], [], 0
    while index < len(args):
        argument = args[index]
        if argument == "-" or not argument.startswith("-"):
            positional.append(argument)
            index += 1
            continue
        equals = argument.find("=") if argument.startswith("--") else -1
        if equals > 0:
            found.append((argument[:equals], argument[equals + 1:]))
        elif argument in VALUE_FLAGS:
            found.append((argument, args[index + 1] if index + 1 < len(args) else ""))
            index += 1
        else:
            short = re.fullmatch(r"-([LlCAWDFZo])(.+)", argument, re.S)
            found.append(("-" + short.group(1), short.group(2)) if short else (argument, ""))
        index += 1
    return found, positional


def parse(args):
    found, positional = options(args)

    def every(flag):
        return [value for name, value in found if name == flag]

    codegen = every("-C")
    incremental = next((v[len("incremental="):] for v in codegen if v.startswith("incremental=")), None)
    externs = []
    for value in every("--extern"):
        equals = value.find("=")
        spec = value if equals < 0 else value[:equals]
        path = "" if equals < 0 else value[equals + 1:]
        colon = spec.rfind(":")
        externs.append({"flags": "" if colon < 0 else spec[:colon + 1], "name": spec if colon < 0 else spec[colon + 1:], "path": path})
    json_value = ",".join(every("--json"))
    first = lambda flag: (every(flag) or [None])[0]
    return {"crateName": first("--crate-name") or "", "crateTypes": [p for v in every("--crate-type") for p in v.split(",")], "emit": [part.split("=")[0] for v in every("--emit") for part in v.split(",")],
            "outDir": first("--out-dir") or "", "source": positional[0] if positional else "", "target": first("--target"), "externs": externs, "links": every("-l"),
            "nativeSearch": any(re.match(r"(native|all)=", v) for v in every("-L")), "incremental": incremental, "artifacts": "artifacts" in json_value}


def bypass(unit, args, policy):
    if unit["source"] == "" or unit["source"] == "-" or not unit["source"].endswith(".rs"):
        return "no-source-file"
    if unit["crateName"] == "":
        return "no-crate-name"
    if unit["outDir"] == "":
        return "no-out-dir"
    if any(a in ("--print", "-vV", "--version", "-V", "--explain") or a.startswith("--print=") for a in args):
        return "query"
    if not unit["artifacts"]:
        return "no-artifact-messages"
    if "link" not in unit["emit"] and "metadata" not in unit["emit"]:
        return "no-emit"
    types = unit["crateTypes"] or ["bin"]
    if any(t not in policy["key"]["cacheableCrateTypes"] for t in types):
        return "crate-type:" + ",".join(types)
    if any(t in policy["key"]["linkedCrateTypes"] for t in types) and "link" in unit["emit"] and (unit["links"] or unit["nativeSearch"]):
        return "native-link"
    return None


def neutral(text, root, policy):
    if not root:
        return text
    forms = list(dict.fromkeys([root, root.replace("\\", "/"), root.replace("\\", "\\\\")]))
    for form in sorted(forms, key=lambda f: -len(f)):
        text = text.replace(form, policy["key"]["laneToken"])
    return text


def normalize(args, digests, root, policy):
    result, index = [], 0

    def take(flag, argument, index):
        if argument == flag:
            return True, args[index + 1] if index + 1 < len(args) else "", index + 1
        if argument.startswith(flag + "="):
            return True, argument[len(flag) + 1:], index
        return False, None, index

    while index < len(args):
        argument = args[index]
        hit, value, index = take("--out-dir", argument, index)
        if hit:
            index += 1
            continue
        following = args[index + 1] if index + 1 < len(args) else ""
        if argument == "-C" and following.startswith("incremental="):
            index += 2
            continue
        if argument.startswith("-Cincremental="):
            index += 1
            continue
        if argument == "-L" and following.startswith("dependency="):
            index += 2
            continue
        if argument.startswith("-Ldependency="):
            index += 1
            continue
        if argument == "--extern" or argument.startswith("--extern="):
            hit, value, index = take("--extern", argument, index)
            equals = value.find("=")
            spec = value if equals < 0 else value[:equals]
            path = "" if equals < 0 else value[equals + 1:]
            result += ["--extern", spec if path == "" else spec + "=#" + digests.get(path, "missing")]
            index += 1
            continue
        result.append(neutral(argument, root, policy))
        index += 1
    return result


def select_env(environment, root, names, prefixes, policy):
    return [[name, neutral(environment[name], root, policy)] for name in sorted(environment) if name in names or any(name.startswith(p) for p in prefixes)]


def depinfo(text):
    dependencies, environment, done = [], [], False
    for raw in re.split(r"\r?\n", text):
        if raw.startswith("# env-dep:"):
            body = raw[len("# env-dep:"):]
            equals = body.find("=")
            environment.append({"name": body, "value": None} if equals < 0 else {"name": body[:equals], "value": body[equals + 1:].replace("\\n", "\n").replace("\\r", "\r").replace("\\\\", "\\")})
            continue
        if raw.startswith("#") or raw.strip() == "" or done:
            continue
        match = re.search(r"(?<!\\): |(?<!\\):$", raw)
        if match is None:
            continue
        for token in re.split(r"(?<!\\) ", raw[match.start() + 1:].strip()):
            if token != "":
                dependencies.append(token.replace("\\ ", " "))
        done = True
    return {"dependencies": dependencies, "environment": environment}


def directives(stdout):
    files, variables = [], []
    for line in re.split(r"\r?\n", stdout):
        match = re.fullmatch(r"cargo::?(rerun-if-changed|rerun-if-env-changed)=(.*)", line)
        if match:
            (files if match.group(1) == "rerun-if-changed" else variables).append(match.group(2))
    return {"files": files, "variables": variables}


def dumps(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def admit(args, environment, cwd):
    unit = parse(args)
    if unit["crateName"] == "" or any(a in ("-vV", "-V", "--print", "-") or a.startswith("--print=") for a in args):
        return None
    extra = ""
    for index, argument in enumerate(args):
        value = args[index + 1] if argument == "-C" and index + 1 < len(args) else (argument[2:] if argument.startswith("-C") else None)
        if value is not None and value.startswith("extra-filename="):
            extra = value[len("extra-filename="):]
    absolute = lambda path: path if path == "" or path.startswith("/") else posixpath.normpath(posixpath.join(cwd, path))
    return {"crate": unit["crateName"], "package": environment.get("CARGO_PKG_NAME", ""), "version": environment.get("CARGO_PKG_VERSION", ""), "target": unit["target"] or "host",
            "crateTypes": unit["crateTypes"] or ["lib"], "outDir": absolute(unit["outDir"]), "extraFilename": extra, "source": absolute(unit["source"]), "manifestDir": environment.get("CARGO_MANIFEST_DIR", ""),
            "primary": "CARGO_PRIMARY_PACKAGE" in environment, "buildScriptOutDir": environment.get("OUT_DIR"), "externs": [absolute(e["path"]) for e in unit["externs"] if e["path"] != ""]}


request = json.load(sys.stdin)
policy, op = request["policy"], request["op"]
if op == "commands":
    out = []
    for args in request["vectors"]:
        unit = parse(args)
        out.append({"unit": unit, "bypass": bypass(unit, args, policy), "normalized": normalize(args, request["externDigests"], request["laneRoot"], policy), "admit": admit(args, {"CARGO_PKG_NAME": "p", "CARGO_PKG_VERSION": "1", "CARGO_PRIMARY_PACKAGE": "1"}, request["cwd"])})
    print(json.dumps(out))
elif op == "lanes":
    print(json.dumps([{"neutral": neutral(row["text"], row["root"], policy), "restored": (neutral(row["text"], row["root"], policy).replace(policy["key"]["laneToken"], row["root"]) if row["root"] else row["text"])} for row in request["rows"]]))
elif op == "environments":
    root = request["laneRoot"]
    print(json.dumps([{"key": select_env(env, root, set(policy["key"]["environmentNames"]), policy["key"]["environmentPrefixes"], policy), "build": select_env(env, root, set(policy["buildScript"]["environmentNames"]), policy["buildScript"]["environmentPrefixes"], policy)} for env in request["rows"]]))
elif op == "depinfo":
    print(json.dumps([depinfo(t) for t in request["rows"]]))
elif op == "directives":
    print(json.dumps([directives(t) for t in request["rows"]]))
elif op == "keys":
    print(json.dumps([hashlib.sha256(dumps([policy["namespace"], k["toolchain"], k["normalized"], k["sourceSha256"], [[e["name"], e["sha256"]] for e in k["externs"]], k["environment"]]).encode("utf8")).hexdigest() for k in request["rows"]]))
elif op == "hashes":
    print(json.dumps([hashlib.sha256(t.encode("utf8")).hexdigest() for t in request["rows"]]))
elif op == "responses":
    def expand(row):
        out = []
        for argument in row["args"]:
            if not argument.startswith("@") or len(argument) < 2 or argument[1:] not in row["files"]:
                out.append(argument)
            else:
                out += [line for line in re.split(r"\r?\n", row["files"][argument[1:]]) if line]
        return out
    print(json.dumps([expand(row) for row in request["rows"]]))
elif op == "admit":
    print(json.dumps([admit(r["args"], r["environment"], request["cwd"]) for r in request["rows"]]))
