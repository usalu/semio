import json, math, sys
mode = sys.argv[1]
if mode == "memory":
    declared, share, minimum = int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    out = []
    for host in json.loads(sys.argv[5]):
        out.append(declared if host <= 0 else min(declared, max(minimum, math.floor(host * share / 10000))))
    print(json.dumps(out))
else:
    declared, default, percent = int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
    out = []
    for total in json.loads(sys.argv[5]):
        ceiling = math.inf if total is None else max(default, math.floor(total * percent / 100))
        out.append(min(declared, ceiling))
    print(json.dumps(out))
