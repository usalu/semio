"""🐍️ Independent model of the stack-sample summary: reads a macOS `sample` report from the file argv[1] and the policy JSON from stdin, prints the summary of the main thread (samples, own time by frame, end of the heaviest path) as JSON. See https://ss64.com/mac/sample.html"""
import json, re, sys

def frame(text, policy):
    bare = re.sub(r"\s+\[0x[0-9a-f]+\]$", "", text)
    if re.match(r"^\?\?\?\s+\(in <unknown binary>\)", bare): return "JIT"
    unnamed = re.match(r"^\?\?\?\s+\(in (.+?)\)\s+load address 0x[0-9a-f]+ \+ (0x[0-9a-f]+)$", bare)
    name = unnamed.group(1) + "+" + unnamed.group(2) if unnamed else re.sub(r"\s+\+\s+\d+$", "", bare)
    return re.sub(r"\s+", " ", name)[: policy["frameCharacters"]]

def summary(text, policy):
    lines = text.split("\n")
    starts = [i for i, line in enumerate(lines) if line.strip() == "Call graph:"]
    if not starts: return None
    threads, stack = [], []
    for line in lines[starts[0] + 1:]:
        if line.strip() == "": break
        found = re.match(r"^([ +!:|]*)(\d+) +(.+)$", line)
        if not found: continue
        depth, rest = len(found.group(1)), found.group(3).strip()
        node = {"depth": depth, "count": int(found.group(2)), "frame": rest, "children": [], "thread": None}
        while stack and stack[-1]["depth"] >= depth: stack.pop()
        if not stack:
            if not rest.startswith("Thread_"): continue
            node["thread"] = rest; threads.append(node)
        else:
            node["frame"] = frame(rest, policy); stack[-1]["children"].append(node)
        stack.append(node)
    mains = [t for t in threads if "com.apple.main-thread" in t["thread"]] or threads
    if not mains: return None
    main, own = mains[0], {}
    def visit(node):
        own_count = node["count"] - sum(child["count"] for child in node["children"])
        if own_count > 0: own[node["frame"]] = own.get(node["frame"], 0) + own_count
        for child in node["children"]: visit(child)
    for child in main["children"]: visit(child)
    path, node = [], main
    while node["children"]:
        best = node["children"][0]
        for child in node["children"][1:]:
            if child["count"] > best["count"]: best = child
        node = best; path.append({"frame": node["frame"], "count": node["count"]})
    rows = sorted(({"frame": f, "count": c} for f, c in own.items()), key=lambda row: (-row["count"], row["frame"].encode("utf-16-be")))
    return {"total": main["count"], "self": rows[: policy["frames"]], "path": path[-policy["pathFrames"]:]}

policy = json.load(sys.stdin)
with open(sys.argv[1], encoding="utf8") as handle: print(json.dumps(summary(handle.read(), policy)))
