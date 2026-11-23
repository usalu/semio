import json, math, sys
runs = json.loads(sys.argv[1]); kept, floor, factor = int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
total_mb = float(sys.argv[5]) if len(sys.argv) > 5 and sys.argv[5] != "" else None
percent = float(sys.argv[6]) if len(sys.argv) > 6 else 100
measured_cap = float(sys.argv[7]) if len(sys.argv) > 7 else math.inf
ceiling = max(floor, math.floor(total_mb * percent / 100)) if total_mb else math.inf
grown = [g for (s, o, g) in runs if s == "workspace" and o == "ok" and g is not None][-kept:]
need = min(ceiling, measured_cap, max(floor, math.ceil(max(grown) * factor))) if grown else min(4096, ceiling)
print(need)
