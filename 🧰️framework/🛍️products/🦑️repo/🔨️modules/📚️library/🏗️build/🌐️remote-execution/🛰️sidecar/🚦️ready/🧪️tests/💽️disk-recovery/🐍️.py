import json, math, sys
runs = json.loads(sys.argv[1]); kept, floor, factor = int(sys.argv[2]), int(sys.argv[3]), float(sys.argv[4])
total_mb = float(sys.argv[5]) if len(sys.argv) > 5 and sys.argv[5] != "" else None
percent = float(sys.argv[6]) if len(sys.argv) > 6 else 100
measured_cap = float(sys.argv[7]) if len(sys.argv) > 7 else math.inf
ceiling = max(floor, math.floor(total_mb * percent / 100)) if total_mb else math.inf
own = [(o, g) for (s, o, g) in runs if s == "workspace"]
grown = sorted(g for (o, g) in [(o, g) for (o, g) in own if o in ("ok", "stage-failed") and g is not None][-kept:])
need = min(ceiling, measured_cap, max(floor, math.ceil(grown[len(grown) // 2] * factor))) if grown and any(o == "ok" and g is not None for (o, g) in own) else min(4096, ceiling)
print(need)
