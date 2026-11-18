import json, math, sys
runs = json.loads(sys.argv[1]); kept, floor, factor = int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
grown = [g for (s, o, g) in runs if s == "workspace" and o == "ok" and g is not None][-kept:]
print(max(floor, math.ceil(max(grown) * factor)) if grown else 4096)
