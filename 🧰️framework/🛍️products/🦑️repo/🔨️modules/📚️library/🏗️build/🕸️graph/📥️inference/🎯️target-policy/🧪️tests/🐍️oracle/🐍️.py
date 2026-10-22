"""🎯 Independent Python oracle of the target cache policy (standard library only): name predicates, class lookup, the replay decision, the output admission and the execution contract. Requests arrive as JSON on stdin: {"cacheSchema", "executionSchema", "requests": [...]}."""
import json, re, sys


def undefined(value):
    return "undefined" if value is None else value


class Refused(Exception):
    pass


def conforms(contract, value):
    if "anyOf" in contract:
        return any(conforms(rule, value) for rule in contract["anyOf"])
    if contract.get("type") == "null":
        return value is None
    if "const" in contract and value != contract["const"]:
        return False
    if "enum" in contract:
        return value in contract["enum"]
    kind = contract.get("type")
    if kind == "object":
        if not isinstance(value, dict):
            return False
        props = contract.get("properties", {})
        if any(key not in value for key in contract.get("required", [])):
            return False
        if contract.get("additionalProperties") is False and any(key not in props for key in value):
            return False
        return all(conforms(rule, value[key]) for key, rule in props.items() if key in value)
    if kind == "array":
        if not isinstance(value, list):
            return False
        if "minItems" in contract and len(value) < contract["minItems"]:
            return False
        if "maxItems" in contract and len(value) > contract["maxItems"]:
            return False
        if not all(conforms(contract["items"], item) for item in value):
            return False
        return contract.get("uniqueItems") is not True or len({json.dumps(item, sort_keys=False, separators=(",", ":")) for item in value}) == len(value)
    if kind == "string":
        if not isinstance(value, str):
            return False
        length = len(value.encode("utf-16-le")) // 2
        if "minLength" in contract and length < contract["minLength"]:
            return False
        if "maxLength" in contract and length > contract["maxLength"]:
            return False
        return not contract.get("pattern") or re.search(contract["pattern"], value) is not None
    return "const" in contract


def run(request, schemas):
    kind = request["kind"]
    if kind == "predicates":
        return [{"mutating": bool(re.search(r"(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)", n)), "live": bool(re.search(r"(?:^|-)(?:e2e|live)(?:-|$)", n)), "uncached": uncached(n), "normalized": bare(n)} for n in request["names"]]
    if kind == "normalize":
        return [bare(n) for n in request["names"]]
    if kind == "matches":
        return [matches_uncached(n, request["policy"]) for n in request["names"]]
    if kind == "verify":
        return [verify(t) for t in request["targets"]]
    if kind == "classes":
        out = []
        for n in request["names"]:
            try:
                out.append({"decision": target_class(n, request["rows"], schemas)})
            except Refused as error:
                out.append({"error": str(error)})
        return out
    if kind == "decision":
        try:
            return {"decision": decision(request["class"], request["reason"], schemas)}
        except Refused as error:
            return {"error": str(error)}
    if kind == "policy":
        try:
            return {"result": policy(request["name"], request["target"], request["policy"], schemas)}
        except Refused as error:
            return {"error": str(error)}
    if kind == "outputs":
        return admitted(request["target"], schemas)
    if kind == "execution":
        return execution(request["target"], request.get("identity"), schemas)
    raise SystemExit("unknown request " + kind)


def bare(name):
    return re.sub(r"^[^A-Za-z0-9]+", "", name)


def has_head(n, h):
    return n == h or n.startswith(h + "-")


def has_tail(n, t):
    return n == t or n.endswith("-" + t)


def has_token(n, t):
    return has_head(n, t) or has_tail(n, t) or ("-" + t + "-") in n


def matches_command(n, commands):
    return any(n == c or n.startswith(c + "-") for c in commands)


def matches_uncached(n, p):
    return n in p["uncachedExact"] or matches_command(n, p["uncached"])


def verify(target):
    command = ((target or {}).get("options") or {}).get("command") or ""
    return re.search(r"(?:^|[\s\"'])verify(?:[\s\"']|$)", command) is not None


def uncached(n):
    if re.search(r"(?:^|-)(?:e2e|live)(?:-|$)", n) or n in ("os", "semio"):
        return ["live", "drives-running-process"]
    if re.search(r"^(?:audit|bench|cache-report|cache-verify|ci-baseline|disk-report|doctor|perf-baseline|test-doctor|test-inventory|parity)(?:-|$)", n):
        return ["diagnostic", "measures-environment"]
    if re.search(r"^(?:publish|deploy)(?:-|$)", n):
        return ["side-effecting", "publishes-remote"]
    return ["side-effecting", "mutates-workspace"]


def decision(cls, reason, schemas):
    definition = next((d for d in schemas["cache"]["definitions"].values() if d["properties"]["class"]["const"] == cls), None)
    value = {"version": 1, "class": cls, "reason": reason}
    if definition is None or reason is None or not conforms(definition, value):
        raise Refused("Repository target cache decision refused: %s/%s" % (undefined(cls), undefined(reason)))
    return value


def declared(value, schemas):
    if value is None:
        return None
    made = decision(value.get("class"), value.get("reason"), schemas)
    if len(value) != len(made) or value.get("version") != made["version"]:
        raise Refused("Repository target cache decision refused: %s/%s" % (undefined(value.get("class")), undefined(value.get("reason"))))
    return made


def with_decision(target, cls, reason, schemas):
    md = dict(target.get("metadata") or {})
    semio = dict(md.get("semio") or {})
    semio["cache"] = decision(cls, reason, schemas)
    md["semio"] = semio
    return {**target, "metadata": md}


def target_class(name, rows, schemas):
    n = bare(name)
    row = next((r for r in rows if n in r.get("exact", [])), None) or next((r for r in rows if any(has_head(n, h) for h in r.get("heads", [])) or any(has_tail(n, t) for t in r.get("tails", [])) or any(has_token(n, t) for t in r.get("tokens", []))), None)
    return decision(row["decision"]["class"], row["decision"]["reason"], schemas) if row else None


def policy(name, target, p, schemas):
    n = bare(name)
    if matches_command(n, p["continuous"]):
        return with_decision({**target, "cache": False, "continuous": True}, "continuous", "serves-until-stopped", schemas)
    semio = ((target.get("metadata") or {}).get("semio") or {})
    if target.get("cache") is False:
        declared(semio.get("cache"), schemas)
        return {"inputs": ["default"], "outputs": [], **target, "cache": False}
    if matches_uncached(n, p):
        return with_decision({**target, "cache": False}, *uncached(n), schemas)
    exact = n in p["cachedExact"]
    if not exact and (re.search(r"(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)", n) or re.search(r"(?:^|-)(?:e2e|live)(?:-|$)", n)):
        return with_decision({**target, "cache": False}, *uncached(n), schemas)
    made = declared(semio.get("cache"), schemas) or target_class(n, p["targetClasses"], schemas) or (decision("deterministic", "check-without-outputs", schemas) if exact or verify(target) else None)
    if not made:
        return with_decision({**target, "cache": False}, "unclassified", "no-cache-classification", schemas)
    if made["class"] != "deterministic":
        return with_decision({**target, "cache": False}, made["class"], made["reason"], schemas)
    return {"inputs": ["default", "^default"], "outputs": [], **with_decision(target, made["class"], made["reason"], schemas), "cache": True}


def admitted(target, schemas):
    made = ((target.get("metadata") or {}).get("semio") or {}).get("cache")
    if target.get("cache") is not True or (made or {}).get("class") != "deterministic" or made.get("reason") != "pure-function-of-inputs":
        return target
    semio = target["metadata"]["semio"]
    outputs = target.get("outputs")
    if (isinstance(outputs, list) and len(outputs) > 0) or any(key in semio for key in ("rustBuildIntent", "rustUnitAggregate", "hostPlatformOutputsV1")) or target.get("executor") == "repo:noop":
        return target
    return with_decision({**target, "cache": False}, "side-effecting", "mutates-workspace", schemas)


def execution(target, identity, schemas):
    value = ((target.get("metadata") or {}).get("semio") or {}).get("execution")
    if value is None:
        return {"kind": None}
    definition = next((d for d in schemas["execution"]["definitions"].values() if d["properties"]["kind"]["const"] == value.get("kind")), None)
    if definition is None or not conforms(definition, value):
        return {"error": "Repository target execution contract refused" + (": " + identity if identity else "")}
    return {"kind": value["kind"]}


request = json.load(sys.stdin)
schemas = {"cache": request["cacheSchema"], "execution": request["executionSchema"]}
print(json.dumps([run(r, schemas) for r in request["requests"]], ensure_ascii=False))
