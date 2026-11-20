import json
import sys

fixture = json.load(sys.stdin)
birth = None
previous = {}
output = []
for sample in fixture["samples"]:
    rows = {r[0]: r for r in sample["rows"] or [] if r[0] in fixture["members"]}
    root = rows.get(fixture["root"])
    if sample.get("cancelled") or not root or birth and root[3] != birth:
        output.append(None)
        continue
    birth = root[3]
    cpu = {pid: (r[3], r[4], r[5]) for pid, r in rows.items() if r[4] is not None and r[5] is not None}
    if len(cpu) > sample.get("maximum", fixture["bound"]):
        cpu = {}
    elapsed_ticks = 0
    for pid in cpu.keys() & previous.keys():
        identity, kernel, user = cpu[pid]
        before, old_kernel, old_user = previous[pid]
        if identity == before and kernel >= old_kernel and user >= old_user:
            elapsed_ticks += kernel - old_kernel + user - old_user
    working = [r[6] for r in rows.values()]
    memory = (sum(working) + (1 << 19)) // (1 << 20) if all(working) else None
    output.append(dict(progress=elapsed_ticks >= fixture["cpuMilliseconds"] * 10000, megabytes=memory, cpu=len(cpu)))
    previous = cpu
print(json.dumps(output))
