import json, sys

rows = [line.split() for line in open(sys.argv[1]) if line.strip()]
starts, spans = {}, []
for kind, pid, variant, nanos in rows:
    key = (pid, variant)
    if kind == "start":
        starts[key] = int(nanos)
    elif kind == "end" and key in starts:
        spans.append((starts.pop(key), int(nanos), pid))
overlaps = sum(1 for i, a in enumerate(spans) for b in spans[i + 1:] if a[0] < b[1] and b[0] < a[1])
print(json.dumps({"runs": len(spans), "unfinished": len(starts), "overlaps": overlaps}))
sys.exit(0 if overlaps == 0 and not starts else 1)
