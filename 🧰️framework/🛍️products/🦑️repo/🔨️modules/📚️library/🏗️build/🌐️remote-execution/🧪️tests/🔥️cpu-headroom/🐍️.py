"""🔥️ Independent Python oracle of the CPU rule of remote-execution headroom (standard library only): saturated = load is not null and load * 1000 >= max(1, cores) * ceilingPermille; registered slots = max(1, concurrency - reserved) for an origin ending in the coordinator suffix, else concurrency. usage: 🐍️.py <schema.json> <vectors.json> prints {"saturation": [...], "slots": [...]}."""
import json, sys

policy = json.load(open(sys.argv[1]))["definitions"]["Policy"]["const"]
vectors = json.load(open(sys.argv[2]))
sat = [row["load"] is not None and row["load"] * 1000 >= max(1, row["cores"]) * policy["loadPerCoreCeilingPermille"] for row in vectors["saturation"]]
slots = [max(1, row["concurrency"] - policy["coordinatorReservedSlots"]) if row["origin"].endswith(policy["coordinatorOriginSuffix"]) else row["concurrency"] for row in vectors["slots"]]
print(json.dumps({"saturation": sat, "slots": slots}))
