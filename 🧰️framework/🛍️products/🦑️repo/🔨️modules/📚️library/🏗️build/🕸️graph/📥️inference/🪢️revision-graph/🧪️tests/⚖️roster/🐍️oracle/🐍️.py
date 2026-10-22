"""🪢 Independent Python oracle of the revision graph rules (standard library only): roster admission, registry keys, revision tokens, revision identity, revisioned import calls and wiring visibility. Requests arrive as JSON on stdin."""
import json, re, sys

REVISION = re.compile(r"^[a-f0-9]{64}\Z")
NAME = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*\Z")
PATH = re.compile(r"^\./(?:[^/\\?#.][^/\\?#]*/)*[^/\\?#.][^/\\?#]*\Z")
BADREV = "Revision graph requires a lowercase sha256 revision"


class Refused(Exception):
    pass


def roster(manifest):
    if not isinstance(manifest, dict) or manifest.get("version") != 1 or not isinstance(manifest.get("components"), list) or not manifest["components"]:
        raise Refused("Component roster version 1 with components required")
    order = {}
    for index, entry in enumerate(manifest["components"]):
        if not isinstance(entry, dict) or not isinstance(entry.get("name"), str) or not NAME.match(entry["name"]) or not isinstance(entry.get("path"), str) or not PATH.match(entry["path"]) or not isinstance(entry.get("needs"), list):
            raise Refused("Invalid component roster entry %d" % index)
        if entry["name"] in order:
            raise Refused("Duplicate component " + entry["name"])
        if len(set(map(json.dumps, entry["needs"]))) != len(entry["needs"]):
            raise Refused("Component %s repeats a need" % entry["name"])
        for need in entry["needs"]:
            if not isinstance(need, str) or need not in order:
                raise Refused("Component %s needs %s, which is not an earlier component" % (entry["name"], need))
        order[entry["name"]] = index
    return manifest["components"]


def key(name):
    return re.sub(r"-([a-z0-9])", lambda m: m.group(1).upper(), name)


def revision_ok(value):
    return isinstance(value, str) and REVISION.match(value) is not None


def identity(ops):
    bound, out = None, []
    for op in ops:
        if op["op"] == "read":
            out.append({"error": "Revision identity is not bound yet"} if bound is None else {"revision": bound})
        else:
            if not revision_ok(op["value"]):
                out.append({"error": BADREV})
            elif bound is not None and bound != op["value"]:
                out.append({"error": "Revision identity is already bound"})
            else:
                bound = op["value"]
                out.append({})
    return out


def imports(request):
    if not revision_ok(request["revision"]):
        return {"calls": [], "error": BADREV}
    calls = [path + "?revision=" + request["revision"] for path in request["paths"]]
    fail = request.get("failOn")
    return {"calls": calls, "error": "load refused: " + fail if fail in request["paths"] else None}


def wiring(request):
    components, wired, out = request["components"], False, []
    for attempt in request["attempts"]:
        modules = [c for c in components if not c.get("absent")]
        if len(modules) != len(components):
            out.append({"error": "Component modules differ from the roster", "order": [], "visible": None})
            continue
        problem = None
        for c in components:
            if c.get("component", c["name"]) != c["name"] or c.get("wire") is False:
                problem = "Component %s does not match its roster entry" % c["name"]
                break
            if wired:
                problem = "Component %s is already wired into another revision" % c["name"]
                break
        if problem:
            out.append({"error": problem, "order": [], "visible": None})
            continue
        order, visible, error = [], {}, None
        for c in components:
            order.append(c["name"])
            if attempt["fail"] == c["name"]:
                error = "wire refused: " + c["name"]
                break
            visible[c["name"]] = [key(need) for need in c["needs"]]
        if error:
            out.append({"error": error, "order": order, "visible": None})
        else:
            wired = True
            out.append({"error": None, "order": order, "visible": {n: [k for k in v] for n, v in visible.items()}})
    return out


def run(request):
    kind = request["kind"]
    if kind == "roster":
        try:
            roster(request["manifest"])
            return {"error": None}
        except Refused as error:
            return {"error": str(error)}
    if kind == "keys":
        return [key(n) for n in request["names"]]
    if kind == "revisions":
        return [revision_ok(v) for v in request["values"]]
    if kind == "identity":
        return identity(request["ops"])
    if kind == "imports":
        return imports(request)
    if kind == "wiring":
        return wiring(request)
    raise SystemExit("unknown request " + kind)


print(json.dumps([run(r) for r in json.load(sys.stdin)["requests"]], ensure_ascii=False))
