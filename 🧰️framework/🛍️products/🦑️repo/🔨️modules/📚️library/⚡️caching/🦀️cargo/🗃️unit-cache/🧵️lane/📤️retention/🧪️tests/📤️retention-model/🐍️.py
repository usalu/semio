"""📤 Independent Python model of the Cargo lane retention rule (standard library only, no code shared with the TypeScript reference).
usage: 🐍️.py verdicts <corpus.json>   reads the corpus on a file or stdin ("-") and prints {"plan":[...],"origin":[...]} as JSON

Plan. Lane files whose name matches the receipt pattern, in name order, at most `maximumFiles` of them: over `maximumFileBytes` is refused; a name the shared directory lacks is published; the same name with the same digest is dropped (same); the same name with another digest is a conflict.
Origin. A compiler origin is owned when it is the shared build and target directory pair, or the build directory `<root>/<lanes>/<id>/<build>` of a lane with a valid id whose target is `<root>/<lanes>/<id>/<target>` or lies strictly inside that build directory. Posix paths, normalised by hand (no `os.path`).
"""
import json
import re
import sys


def normalise(path):
    parts = []
    for part in path.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            if parts:
                parts.pop()
            continue
        parts.append(part)
    return "/" + "/".join(parts)


def plan(case, policy):
    pattern = re.compile(policy["ledgerPattern"])
    shared = {row["name"]: row for row in case["shared"]}
    out = []
    for row in sorted((r for r in case["lane"] if pattern.fullmatch(r["name"])), key=lambda r: r["name"])[: policy["maximumFiles"]]:
        old = shared.get(row["name"])
        if row["bytes"] > policy["maximumFileBytes"]:
            action = "refuse"
        elif old is None:
            action = "publish"
        elif old["sha256"] == row["sha256"]:
            action = "same"
        else:
            action = "conflict"
        out.append({"action": action, "name": row["name"], "bytes": row["bytes"]})
    return out


def owned(case, policy):
    root = normalise(case["cacheRoot"])
    build, target = normalise(case["origin"]["buildDirectory"]), normalise(case["origin"]["targetDirectory"])
    if build == normalise(case["shared"]["build"]) and target == normalise(case["shared"]["target"]):
        return True
    lanes = root + "/" + policy["layout"]["lanes"] + "/"
    if not build.startswith(lanes):
        return False
    parts = build[len(lanes):].split("/")
    if len(parts) != 2 or parts[1] != policy["layout"]["build"] or not re.fullmatch(r"[a-z0-9]{1,12}", parts[0]):
        return False
    lane = lanes + parts[0]
    return target == lane + "/" + policy["layout"]["target"] or (target.startswith(build + "/") and len(target) > len(build) + 1)


def lane_owned(case, policy):
    root = normalise(case["cacheRoot"])
    build = normalise(case["origin"]["buildDirectory"])
    lanes = root + "/" + policy["layout"]["lanes"] + "/"
    if not build.startswith(lanes):
        return False
    parts = build[len(lanes):].split("/")
    return len(parts) == 2 and parts[1] == policy["layout"]["build"] and re.fullmatch(r"[a-z0-9]{1,12}", parts[0]) is not None


def main():
    source = sys.argv[2]
    text = sys.stdin.read() if source in ("-", "/dev/stdin") else open(source, encoding="utf-8").read()
    corpus = json.loads(text)
    policy = corpus["policy"]
    json.dump({"plan": [plan(c, policy) for c in corpus["plan"]], "origin": [owned(c, policy) for c in corpus["origin"]], "lane": [lane_owned(c, policy) for c in corpus["origin"]]}, sys.stdout)


if __name__ == "__main__":
    main()
