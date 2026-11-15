"""Independent model of the host reload rule: replays the ticks of 🧫️cases.json and prints the trace as JSON."""
import json, sys

def run(spec, case):
    policy, running = spec["policy"], spec["implementation"]
    first = last = seen = None
    retry_at, finished, state, replaced, drained, trace = 0, False, "current", 0, False, []
    for tick in case["ticks"]:
        at, found = tick["at"], tick["identify"]
        if not finished and at >= retry_at:
            if "blocked" in found:
                state = "blocked"
            elif found["implementation"] == running:
                first = seen = None
                state = "current"
            else:
                if first is None:
                    first = at
                if found["implementation"] != seen:
                    seen, last = found["implementation"], at
                if at - last >= policy["quietMilliseconds"] or at - first >= policy["maximumDeferralMilliseconds"]:
                    replaced += 1
                    if case["replace"]["started"]:
                        finished, drained, state = True, True, "draining"
                    else:
                        retry_at, state = at + policy["retryMilliseconds"], "failed"
                else:
                    state = "pending"
        trace.append([state, replaced, drained])
    return trace

spec = json.load(open(sys.argv[1], encoding="utf8"))
print(json.dumps({case["name"]: run(spec, case) for case in spec["cases"]}))
