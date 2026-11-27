import json,sys
"""Independent oracle: the newest cancel record whose pid is the stage pid, by atMs."""
out=[]
for case in json.loads(sys.argv[1]):
    hits=[c["atMs"] for c in case["cancels"] if c["pid"]==case["pid"]]
    out.append(max(hits) if hits else None)
print(json.dumps(out))
