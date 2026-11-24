import json, math, statistics, sys

def margin(megabytes, t):
    return min(t["maximumMegabytes"], max(t["minimumMegabytes64"], math.ceil(megabytes * (100 + t["marginPercent"]) / 100)))

def family(samples, t):
    ordered = sorted(value for value in samples if value > 0)
    if len(ordered) < t["minimumCrates"]:
        return None
    return math.ceil(statistics.quantiles(ordered, n=100, method="inclusive")[t["quantilePercent"] - 1])

def expense(state, t):
    linked = state["linked"]
    if state["learned"] is not None:
        peak = margin(state["learned"], t)
    elif linked and family(state["familyPeaks"], t) is not None:
        peak = margin(family(state["familyPeaks"], t), t)
    else:
        peak = max(t["unknownMegabytes"], t["coldMegabytes"] if linked else 0)
    if state["cost"] is not None:
        source, milliseconds = "record", state["cost"]
    elif not linked:
        source, milliseconds = "none", None
    elif family(state["familyCosts"], t) is not None:
        source, milliseconds = "family", family(state["familyCosts"], t)
    else:
        source, milliseconds = "seed", t["seedMilliseconds"]
    megabytes = peak if state["learned"] is not None or linked else None
    expensive = (milliseconds is not None and milliseconds >= t["minimumMilliseconds"]) or (megabytes is not None and megabytes >= t["minimumMegabytes"])
    return {"expensive": expensive, "source": source, "peak": peak}

request = json.load(sys.stdin)
print(json.dumps([expense(state, request["thresholds"]) for state in request["states"]]))
