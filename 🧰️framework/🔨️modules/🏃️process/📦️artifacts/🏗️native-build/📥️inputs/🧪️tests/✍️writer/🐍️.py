"""✍️ Independent oracle for the compiler-writer order law: replays a case's acquisition sequence against the authored lease-order policy.
usage: python3 -I 🐍️.py <policy.json> <fixture.json>   prints {case id: granted|lease-order|refused}"""
import json, sys
policy, fixture = json.load(open(sys.argv[1], encoding="utf8")), json.load(open(sys.argv[2], encoding="utf8"))
def classify(resource):
    best = None
    for c in policy["classes"]:
        for m in c["match"]:
            hit = resource == m["value"] if m["kind"] == "exact" else resource.startswith(m["value"])
            if hit and (best is None or (m["kind"] == "exact", len(m["value"])) > (best[1] == "exact", len(best[2]))): best = (c, m["kind"], m["value"])
    return best[0] if best else {"id": None, "rank": 0}
allowed = {(e["from"], e["to"]) for e in policy["allowed"] if "binding" not in e}
def verdict(case):
    held = []
    resources = {"writer": [fixture["writer"] + "/image"], "recipe": [p + "/x" for p in fixture["acquisitions"]["recipe"]], "artifact": [p + "/x" for p in fixture["acquisitions"]["artifact"]]}
    entered = False
    for step in case["sequence"]:
        if step == "release-writer": held = [r for r in held if not r.startswith(fixture["writer"])]; continue
        if step == "writer" and case["port"] == "none": entered = True; continue
        if step == "writer": entered = True
        for resource in resources[step]:
            wanted = classify(resource)
            if wanted["rank"] > 0 and any(classify(h)["rank"] > wanted["rank"] and (classify(h)["id"], wanted["id"]) not in allowed for h in held): return "lease-order"
            if step in ("writer",): held.append(resource)
    if case["port"] == "offers" and not entered: return "refused"
    return "granted"
print(json.dumps({case["id"]: verdict(case) for case in fixture["cases"]}))
