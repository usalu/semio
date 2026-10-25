"""📌️ Independent oracle of the runtime pin guard: recomputes every expectation of the shared fixture from its inputs with the Python standard library only.

Usage: python3 oracle.py <fixture.json>   prints {"select": [...], "locate": [...], "cache": [...], "decide": [...], "split": [...], "procargs": [...], "cmdline": [...]}
"""
import json
import os
import subprocess
import sys
import tempfile


def select(registry, case):
    rows = registry["runtimes"][registry["selected"]]["distributions"]
    libc = "musl" if case["musl"] else "gnu"
    for row in rows:
        if row["platform"] == case["platform"] and row["arch"] == case["arch"] and row["libc"] in ("any", libc):
            return row["archive"]
    return None


def locate(registry, case):
    runtime = registry["runtimes"][registry["selected"]]
    row = next((r for r in runtime["distributions"] if r["platform"] == case["platform"] and r["arch"] == case["arch"] and r["libc"] in ("any", "musl" if case["musl"] else "gnu")), None)
    if row is None:
        return None
    name = runtime["executable"]["windows" if case["platform"] == "win32" else "unix"]
    with tempfile.TemporaryDirectory(prefix="pin-oracle-") as temporary:
        stores = {"tools": os.path.join(temporary, *registry["cache"]["toolsPartition"], registry["selected"], row["sha256"]),
                  "user": os.path.join(temporary, "cache", registry["selected"], row["sha256"])}
        for store in case["stores"]:
            directory = stores[store["store"]]
            os.makedirs(directory)
            if store["executable"] != "absent":
                path = os.path.join(directory, name)
                with open(path, "wb") as handle:
                    handle.write(b"12345")
                os.chmod(path, 0o755 if store["executable"] == "present" else 0o644)
            receipt = store["receipt"]
            if receipt != "absent":
                body = {"archive": row["sha256"] if receipt in ("ok", "size-mismatch") else "0" * 64}
                if receipt == "size-mismatch":
                    body["bytes"] = 99
                if receipt == "ok-sized":
                    body["archive"] = row["sha256"]
                    body["bytes"] = 5
                with open(os.path.join(directory, ".toolchain.json"), "w") as handle:
                    json.dump(body, handle)
        for origin in ("tools", "user"):
            directory = stores[origin]
            path = os.path.join(directory, name)
            receipt_path = os.path.join(directory, ".toolchain.json")
            if not (os.path.isfile(path) and not os.path.islink(path) and os.path.isfile(receipt_path)):
                continue
            if case["platform"] != "win32" and not os.stat(path).st_mode & 0o100:
                continue
            receipt = json.load(open(receipt_path))
            if receipt.get("archive") != row["sha256"]:
                continue
            if receipt.get("bytes") not in (None, 0, os.path.getsize(path)):
                continue
            return "tools-partition" if origin == "tools" else "user-cache"
    return None


def cache(case):
    environment, home, platform = case["environment"], case["home"], case["platform"]
    if platform == "win32":
        return environment.get("LocalAppData") or None
    if platform == "darwin":
        return os.path.join(home, "Library", "Caches") if home else None
    if environment.get("XDG_CACHE_HOME"):
        return environment["XDG_CACHE_HOME"]
    return os.path.join(home, ".cache") if home else None


def decide(case):
    if case["running"] == case["pin"]:
        return "proceed"
    if case["depth"] > 0:
        return "refuse-loop"
    if case["mode"] == "test" and not case["commandLine"]:
        return "refuse-test"
    if case["located"]:
        return "reexec"
    return "delegate" if case["launcher"] else "refuse-missing"


def split(case):
    return ["prog.exe"] + case["arguments"] if "arguments" in case else case["expected"]


def split_line(case):
    return subprocess.list2cmdline(["prog.exe"] + case["arguments"]) if "arguments" in case else case["line"]


def procargs(case):
    count = len(case["arguments"])
    body = b"".join(a.encode() + b"\0" for a in case["arguments"])
    return (count.to_bytes(4, "little") + case["executable"].encode() + b"\0" * (1 + case["padding"]) + body + b"PATH=/x\0").hex()


def cmdline(case):
    return "\0".join(case["arguments"]).encode().hex() + "00"


def main():
    fixture = json.load(open(sys.argv[1]))
    registry = fixture["registry"]
    print(json.dumps({
        "select": [select(registry, case) for case in fixture["select"]],
        "locate": [locate(registry, case) for case in fixture["locate"]],
        "cache": [cache(case) for case in fixture["cache"]],
        "decide": [decide(case) for case in fixture["decide"]],
        "split": [{"line": split_line(case), "arguments": split(case)} for case in fixture["split"]],
        "procargs": [procargs(case) for case in fixture["procargs"]],
        "cmdline": [cmdline(case) for case in fixture["cmdline"]],
    }, ensure_ascii=False))


main()
