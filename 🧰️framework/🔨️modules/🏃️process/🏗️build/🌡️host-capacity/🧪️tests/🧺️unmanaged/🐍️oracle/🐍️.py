"""🐍️ Independent model of the unmanaged memory and the census of a process table. argv[1] is unmanaged|census, argv[2] a JSON document."""
import json
import re
import sys

CLASSES = [
    ("agent", r"^(claude|codex|droid|kiro-cli|copilot|windsurf|cursor|antigravity)"),
    ("editor", r"^(code|code-oss|idea|zed|nvim|vim|emacs|sublime_text)"),
    ("watcher", r"^(semio-repo-watch|watchman|fswatch|inotifywait)"),
    ("tool", r"^(bun|node|deno|python[0-9.]*|cargo|rustc|tsc|esbuild)$"),
    ("other", r".*"),
]


def unmanaged(case):
    managed = sum(0 if "lentSinceMs" in g else (g["reservedMegabytes"] if g["observedMegabytes"] is None else g["observedMegabytes"]) for g in case["ledger"])
    return max(0, case["totalMegabytes"] - case["availableMegabytes"] - managed)


def census(case):
    rows = []
    for line in case["table"].split("\n"):
        m = re.match(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(.+?)\s*$", line)
        if m:
            rows.append((int(m[1]), int(m[2]), int(m[3]), m[4].split("/")[-1]))
    managed, frontier = set(), list(case["grantRoots"])
    while frontier:
        pid = frontier.pop()
        if pid in managed:
            continue
        managed.add(pid)
        frontier.extend(r[0] for r in rows if r[1] == pid)
    totals = {}
    for pid, _ppid, kb, name in rows:
        if pid in managed:
            continue
        klass = next(n for n, p in CLASSES if re.search(p, name))
        count, total = totals.get(klass, (0, 0))
        totals[klass] = (count + 1, total + kb)
    result = [{"name": n, "processes": c, "megabytes": -(-kb // 1024)} for n, (c, kb) in totals.items()]
    return sorted(result, key=lambda r: (-r["megabytes"], r["name"]))


MODES = {"unmanaged": lambda c: [unmanaged(x) for x in c], "census": census}
print(json.dumps(MODES[sys.argv[1]](json.loads(sys.argv[2]))))
