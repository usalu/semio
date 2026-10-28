"""🧹️ Independent decision-table model of the ticket generated-output purge.

Every implementation (Go, Rust, TypeScript) must reproduce `expect` of each case in
`../🧫️fixtures/🧹️purge-generated/🧹️cases.json` from real files on disk. This model derives the same
`expect` from the declared facts alone, and `check` cross-validates the declared `tracked` flags against
`git ls-files` of a real throwaway repository outside the checkout.

Usage: python3 -I 🐍️.py check | write
@see ../../🧬️schema/🔣️.json (PurgePolicy, PurgePlan, PurgeCase)
"""
import json
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
CASES = os.path.join(HERE, "..", "🧫️fixtures", "🧹️purge-generated", "🧹️cases.json")
SCHEMA = os.path.join(HERE, "..", "..", "🧬️schema", "🔣️.json")


def decide(policy, case):
    """Returns {removals, skips} for one case from the declared facts only."""
    generated = policy["generatedDirectory"]
    request = case["request"]
    everything = request["selection"] == "generated"
    removals, skips = [], []
    for ticket in sorted(case["tickets"], key=lambda t: t["id"]):
        if request["scope"] == "ticket" and ticket["id"] != request["ticket"]:
            continue
        if request["scope"] == "closed" and ticket["id"].split("/")[-1].startswith("."):
            continue
        if ticket["manifest"] != "closed":
            skips.append({"ticket": ticket["id"], "reason": "not-closed"})
            continue
        root = next((e for e in ticket["entries"] if e["path"] == generated), None)
        under = [e for e in ticket["entries"] if e["path"].startswith(generated + "/")]
        if root is None and not under:
            continue
        if root is not None and root["kind"] == "symlink":
            skips.append({"ticket": ticket["id"], "reason": "symlink-root"})
            continue
        if root is not None and root["kind"] == "file":
            skips.append({"ticket": ticket["id"], "reason": "not-directory"})
            continue
        if not case["git"]:
            skips.append({"ticket": ticket["id"], "reason": "tracked-unknown"})
            continue
        files = [e for e in under if e["kind"] == "file"]
        if any(f["ageSeconds"] < policy["liveWriterSeconds"] for f in files):
            skips.append({"ticket": ticket["id"], "reason": "live-writer"})
            continue
        directories = {generated}
        for e in under:
            parts = e["path"].split("/")
            for i in range(1, len(parts) if e["kind"] == "file" or e["kind"] == "symlink" else len(parts) + 1):
                directories.add("/".join(parts[:i]))
        def below(prefix):
            return [f for f in files if f["path"].startswith(prefix + "/")]
        removed = []
        def covered(path):
            return any(path == r or path.startswith(r + "/") for r in removed)
        for directory in sorted(directories, key=lambda d: (d.count("/"), d)):
            if directory == generated and not everything:
                continue
            if covered(directory):
                continue
            contained = below(directory)
            size = sum(f["bytes"] for f in contained)
            if (everything or size > policy["oversizedFolderBytes"]) and not any(f["tracked"] for f in contained):
                removed.append(directory)
                removals.append({"path": ticket["id"] + "/" + directory, "kind": "directory", "bytes": size})
        for f in sorted(files, key=lambda f: f["path"]):
            if covered(f["path"]) or f["tracked"]:
                continue
            if everything or f["bytes"] > policy["oversizedFileBytes"]:
                removals.append({"path": ticket["id"] + "/" + f["path"], "kind": "file", "bytes": f["bytes"]})
    removals.sort(key=lambda r: r["path"])
    skips.sort(key=lambda s: s["ticket"])
    return {"removals": removals, "skips": skips}


def load():
    with open(CASES, encoding="utf-8") as handle:
        return json.load(handle)


def write(data):
    with open(CASES, "w", encoding="utf-8") as handle:
        json.dump(data, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def tracked_by_git(case):
    """Third-party oracle: real `git ls-files` of a throwaway repository outside the checkout."""
    if not case["git"]:
        return None
    with tempfile.TemporaryDirectory(prefix="purge-oracle-") as root:
        env = dict(os.environ, GIT_DIR=os.path.join(root, ".git"), GIT_WORK_TREE=root, GIT_CONFIG_GLOBAL=os.devnull, GIT_CONFIG_SYSTEM=os.devnull)
        subprocess.run(["git", "init", "-q", root], check=True, env=env)
        declared = []
        for ticket in case["tickets"]:
            for e in ticket["entries"]:
                if e["kind"] == "file":
                    path = os.path.join(root, "t", ticket["id"], e["path"])
                    os.makedirs(os.path.dirname(path), exist_ok=True)
                    open(path, "wb").close()
                    if e["tracked"]:
                        declared.append("t/" + ticket["id"] + "/" + e["path"])
        if declared:
            subprocess.run(["git", "add", "-f", "--pathspec-from-file=-", "--pathspec-file-nul"], input="\0".join(declared).encode(), check=True, env=env, cwd=root)
        listed = subprocess.run(["git", "ls-files", "-z"], check=True, env=env, cwd=root, capture_output=True).stdout.decode().split("\0")
        return sorted(p for p in listed if p), sorted(declared)


def main(argv):
    data = load()
    policy = data["policy"]
    if argv[1:] == ["write"]:
        for case in data["cases"]:
            case["expect"] = decide(policy, case)
        write(data)
        print("wrote", len(data["cases"]), "expectations")
        return 0
    failures = []
    for case in data["cases"]:
        if decide(policy, case) != case.get("expect"):
            failures.append(case["id"] + ": expect differs from the model")
        oracle = tracked_by_git(case)
        if oracle is not None and oracle[0] != oracle[1]:
            failures.append(case["id"] + ": git ls-files differs from declared tracked flags")
    with open(SCHEMA, encoding="utf-8") as handle:
        constants = json.load(handle)["$defs"]["PurgePolicy"]["const"]
    if constants != policy:
        failures.append("policy differs from schema PurgePolicy")
    for failure in failures:
        print("FAIL", failure)
    print(len(data["cases"]), "cases,", len(failures), "failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
