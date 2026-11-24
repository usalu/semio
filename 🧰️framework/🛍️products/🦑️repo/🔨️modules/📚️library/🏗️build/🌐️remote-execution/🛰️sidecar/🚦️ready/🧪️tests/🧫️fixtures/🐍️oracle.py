import hashlib, json, sys

def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()

def digests(schema, platform, files, rustup_bridge):
    stages = schema["definitions"]["Stages"]["const"]
    boot = schema["definitions"]["Policy"]["const"]["bootstrap"]["windows" if platform == "windows" else "unix"]
    rows = []
    for stage in stages:
        paths = list(stage["inputs"])
        if stage["bootstrapInput"]:
            paths.append("/".join(boot))
            if platform == "windows":
                paths.append(rustup_bridge)
        text = "%s\n%s\n%s\n" % (stage["name"], platform, "0" * 64)
        for path in paths:
            text += "%s\0%s\n" % (path, files.get(path, "absent"))
        rows.append({"name": stage["name"], "digest": sha(text)})
    return {"digest": sha("\n".join(r["digest"] for r in rows)), "stages": rows}

def backoff(policy, attempts, unit):
    base = min(policy["backoffMaximumMilliseconds"], policy["backoffInitialMilliseconds"] * 2 ** max(0, attempts - 1))
    return min(policy["backoffMaximumMilliseconds"], int(base * (10000 + int(unit * policy["backoffJitterBasisPoints"])) // 10000))

def jobs(policy, cores, available):
    by_cores = max(1, int(cores) - 1)
    if available is None:
        return by_cores
    return max(1, min(by_cores, int(available // policy["megabytesPerJob"])))

def tree(rows, root):
    parent, size = {}, {}
    for line in rows.split("\n"):
        parts = line.split()
        if len(parts) == 3:
            parent[int(parts[0])] = int(parts[1])
            size[int(parts[0])] = int(parts[2])
    total = 0
    for pid, rss in size.items():
        at, depth = pid, 0
        while at is not None and depth < 64:
            if at == root:
                total += rss
                break
            at, depth = parent.get(at), depth + 1
    return int(total / 1024 + 0.5)

def decide(policy, i):
    def act(action, index, code, wait):
        return {"action": action, "stageIndex": index, "code": code, "waitMilliseconds": wait}
    if i["server"]:
        return act("server", -1, "server", policy["idlePollMilliseconds"])
    stale = next((k for k, s in enumerate(i["stages"]) if i["done"].get(s["name"]) != s["digest"]), -1)
    if stale < 0 and i["repo"]:
        return act("idle", -1, None, policy["idlePollMilliseconds"]) if i["marker"] else act("mark", -1, None, 0)
    index = 0 if stale < 0 or not i["repo"] else stale
    if i["human"]:
        return act("observe", index, "human-running", policy["observePollMilliseconds"])
    if i["lease"] == "other":
        return act("observe", index, "lease-held", policy["observePollMilliseconds"])
    if i["inflight"] is not None:
        return act("run", index, None, 0)
    f = i["failure"]
    if f is not None and f["digest"] == i["digest"] and f["nextAttemptMs"] > i["now"]:
        return act("backoff", index, None, f["nextAttemptMs"] - i["now"])
    if not i["memoryOk"]:
        return act("wait", index, "memory-low", policy["memoryPollMilliseconds"])
    return act("run", index, None, 0)

def human(policy, i):
    if i["touchedAgeMilliseconds"] >= policy["humanFreshMilliseconds"]:
        return False
    limit = policy["humanMaximumMilliseconds"]
    if i["record"] is None:
        return i["legacyProcess"] and i["birthAgeMilliseconds"] < limit
    return i["pidAlive"] and i["now"] - i["record"]["startedMs"] < limit

def workspace_invalidation(fixture, schema, policy, rustup_bridge):
    results = []
    for platform in fixture["platforms"]:
        old = digests(schema, platform, {}, rustup_bridge)
        for case in fixture["cases"]:
            files = {} if case["path"] is None else {case["path"]: "a" * 64}
            current = digests(schema, platform, files, rustup_bridge)
            changed = [b["name"] for a, b in zip(old["stages"], current["stages"]) if a["digest"] != b["digest"]]
            decision = decide(policy, dict(current, server=False, done={stage["name"]: stage["digest"] for stage in old["stages"]}, marker=True, repo=True, human=False, lease="free", inflight=None, memoryOk=True, failure=None, now=1))
            results.append({"changed": changed, "decision": decision})
    return results

fixture = json.load(open(sys.argv[1], encoding="utf8"))
schema = json.load(open(sys.argv[2], encoding="utf8"))
policy = schema["definitions"]["Policy"]["const"]
native = json.load(open(sys.argv[3], encoding="utf8"))
rustup_bridge = native["definitions"]["RustupInstallation"]["const"]["windowsProcess"]
out = {
    "digests": [digests(schema, c["platform"], c["files"], rustup_bridge) for c in fixture["digests"]],
    "backoff": [backoff(policy, c["attempts"], c["unit"]) for c in fixture["backoff"]],
    "jobs": [jobs(policy, c["cores"], c["availableMegabytes"]) for c in fixture["jobs"]],
    "tree": [tree(c["rows"], c["root"]) for c in fixture["tree"]],
    "decide": [decide(policy, c["input"]) for c in fixture["decide"]],
    "human": [human(policy, c["input"]) for c in fixture["human"]],
}
if sys.argv[4:] == ["workspace"]:
    out = {"workspaceInvalidation": workspace_invalidation(fixture["workspaceInvalidation"], schema, policy, rustup_bridge)}
print(json.dumps(out, ensure_ascii=False))
