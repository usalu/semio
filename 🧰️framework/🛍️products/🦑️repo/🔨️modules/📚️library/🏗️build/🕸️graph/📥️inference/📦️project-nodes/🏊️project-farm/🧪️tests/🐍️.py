#!/usr/bin/env python3
"""🐍️ Independent model of the project farm laws (standard library only): the plan of the farm, the languages and rendered texts, the heap environment of a worker and the tree of leaves a graph holds after the leaves of every project of a corpus were adopted, whatever the worker count, the assignment and the order. Usage: 🐍️.py <fixture.json> <schema.json> <plan|languages|messages|runtime|tree>"""
import hashlib, json, re, sys

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
schema = json.load(open(sys.argv[2], encoding="utf-8"))
policy = schema["$defs"]["Policy"]["const"]
messages = schema["$defs"]["Messages"]["const"]
sha = lambda data: hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()


def language_oracle():
    """🗣️ The independent oracle of the one language protocol (`⚙️runtime/🗣️language`) and the protocol it reads."""
    import importlib.util
    import os
    root = os.path.dirname(os.path.abspath(__file__))
    while not os.path.isdir(os.path.join(root, "🧰️framework")):
        root = os.path.dirname(root)
    folder = os.path.join(root, "🧰️framework", "🔨️modules", "⚙️runtime", "🗣️language")
    spec = importlib.util.spec_from_file_location("language_oracle", os.path.join(folder, "🧪️tests", "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    with open(os.path.join(folder, "🔣️.json"), encoding="utf-8") as handle:
        return module, json.load(handle)


def plan(projects, environment, cores):
    text = environment.get(policy["workersEnvironment"])
    text = None if text is None else text.strip().lower()
    if text in ("0", "off", "false"):
        return {"workers": 0, "reason": "disabled"}
    if text not in (None, "", "auto"):
        if not re.fullmatch(r"[0-9]+", text) or int(text) > policy["maximumForcedWorkers"]:
            return {"error": True}
        forced = int(text)
        return {"workers": forced, "reason": "forced" if forced else "disabled"}
    if projects < policy["minimumProjects"]:
        return {"workers": 0, "reason": "small"}
    bounds = [("work", projects // policy["projectsPerWorker"]), ("cpu", cores - policy["cpuReserve"])]
    best = bounds[0]
    for row in bounds[1:]:
        if row[1] < best[1]:
            best = row
    reason, workers = best
    return {"workers": 0, "reason": reason} if workers < 2 else {"workers": workers, "reason": reason}


def languages(environment):
    oracle, protocol = language_oracle()
    return oracle.notices(protocol, policy["languages"], environment)


def render(key, fields, environment):
    return [re.sub(r"\{([a-z]+)\}", lambda match: str(fields[match.group(1)]), messages[language][key]) for language in languages(environment)]


def runtime(name, megabytes):
    row = policy["runtime"].get(name)
    if row is None:
        return {}
    environment = dict(row["environment"])
    if name == "bun":
        environment["BUN_JSC_forceRAMSize"] = str(int(megabytes * row["ramFactor"]) * 1048576)
    elif name == "node":
        environment["NODE_OPTIONS"] = "--max-old-space-size=" + str(int(megabytes * row["ramFactor"]))
    return environment


def own_grant(environment):
    """🎫️ Carried plans without a live reservation cannot fund an owned pool."""
    return None


def granted_pool(granted):
    if granted is None or granted["jobs"] < policy["minimumGrantedWorkers"]:
        return None
    return {"source": "host", "workers": granted["jobs"], "megabytes": granted["megabytes"]}


def choose(pools):
    ready = [(index, pool) for index, pool in enumerate(pools) if pool["ready"] and pool["usable"]]
    for index, pool in ready:
        if pool["idle"]:
            return index
    best, ratio = -1, None
    for index, pool in ready:
        load = pool["pending"] / pool["width"]
        if ratio is None or load < ratio:
            best, ratio = index, load
    return best


def recycle(row):
    if row["rssMegabytes"] > row["megabytes"] * policy["recycleMemoryRatio"]:
        return "memory"
    if row["jobs"] >= policy["recycleAfterProjects"]:
        return "projects"
    return None


def tree():
    corpus = fixture["corpus"]
    lines = {}
    for index in range(len(corpus["closures"])):
        row = corpus["closures"][index]
        lines[row["namespace"] + "/" + sha(row["text"]) + ".json"] = sha(row["text"])
    for text in corpus["blocks"]:
        lines["inference/np/ob/" + sha(text) + ".br"] = sha(text)
    for project in corpus["projects"]:
        record = "inference/np/" + sha(project["root"])[:24] + ".json"
        lines[record] = sha(project["record"])
        for receipt in project["receipts"]:
            lines["inference/" + receipt["name"]] = sha(receipt["text"])
        for pack in project["packs"]:
            lines["inference/" + pack["name"]] = sha(pack["text"])
    used_closures = {index for project in corpus["projects"] for index in project["closures"]}
    used_blocks = {index for project in corpus["projects"] for index in project["blocks"]}
    keep = {}
    for index in used_closures:
        row = corpus["closures"][index]
        keep[row["namespace"] + "/" + sha(row["text"]) + ".json"] = sha(row["text"])
    for index in used_blocks:
        text = corpus["blocks"][index]
        keep["inference/np/ob/" + sha(text) + ".br"] = sha(text)
    for name, digest in lines.items():
        if not name.startswith(("build-", "inference/np/ob/")):
            keep[name] = digest
    body = "".join(name + "\0" + keep[name] + "\n" for name in sorted(keep))
    return {"digest": sha(body), "names": sorted(keep)}


command = sys.argv[3]
if command == "plan":
    out = [plan(row["projects"], row["environment"], row["cores"]) for row in fixture["plan"]]
elif command == "languages":
    out = [languages(row["environment"]) for row in fixture["languages"]]
elif command == "messages":
    out = [[render(row["key"], row["fields"], environment) for environment in ({}, {"LANG": "de_DE.UTF-8"}, {"LANG": "en_GB"})] for row in fixture["messages"]]
elif command == "runtime":
    out = [runtime(row["runtime"], row["megabytes"]) for row in fixture["runtime"]]
elif command == "pools":
    out = {"own": [own_grant(row["environment"]) for row in fixture["pools"]["own"]], "granted": [granted_pool(row["granted"]) for row in fixture["pools"]["granted"]], "choose": [choose(row["pools"]) for row in fixture["pools"]["choose"]]}
elif command == "recycle":
    out = [recycle(row) for row in fixture["recycle"]]
elif command == "tree":
    out = tree()
else:
    raise SystemExit("unknown command")
print(json.dumps(out, ensure_ascii=False))
