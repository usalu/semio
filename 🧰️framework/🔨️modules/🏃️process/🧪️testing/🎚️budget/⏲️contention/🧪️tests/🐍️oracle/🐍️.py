"""⏲️ Independent Python oracle of the load-aware test budget clock. It shares no code with the TypeScript module and integrates exactly where the module steps: reads {"policy":…,"clocks":[…],"probes":[…]} on stdin and answers, per clock, the wall time at which a budget of work is used up on a host whose load is a piecewise constant trace (`expiry`), and the wall time at which a replay that needs `work` milliseconds on an idle host finishes there (`finish`); per probe sequence, the smoothed contention a probing platform reports."""
import json, sys

def stretch(policy, ratio):
    low = policy["stretch"]["minimumBasisPoints"] / 10000
    high = policy["stretch"]["maximumBasisPoints"] / 10000
    return min(high, max(low, ratio))

def segments(trace, cores):
    rows = sorted(trace, key=lambda row: row[0])
    for index, (start, load) in enumerate(rows):
        end = rows[index + 1][0] if index + 1 < len(rows) else None
        yield start, end, load / cores

def used_up(trace, cores, amount, speed):
    needed = float(amount)
    for start, end, ratio in segments(trace, cores):
        factor = speed(ratio)
        if end is None:
            return start + needed * factor
        capacity = (end - start) / factor
        if needed <= capacity:
            return start + needed * factor
        needed -= capacity
    raise ValueError("a trace needs a segment")

def clocks(policy, rows):
    out = []
    for row in rows:
        trace, cores = row["trace"], row["cores"]
        expiry = used_up(trace, cores, row["budget"], lambda ratio: stretch(policy, ratio))
        finish = used_up(trace, cores, row["work"], lambda ratio: max(1.0, ratio))
        out.append({"expiry": expiry, "finish": finish, "survives": finish <= expiry + 1e-6})
    return out

def probes(policy, rows):
    weight = policy["probe"]["smoothingBasisPoints"] / 10000
    every = policy["probe"]["everyMilliseconds"]
    out = []
    for row in rows:
        smoothed, probed_at, answers = 1.0, None, []
        for at, measured in row["steps"]:
            if probed_at is None or at - probed_at >= every:
                smoothed = measured if probed_at is None else smoothed + weight * (measured - smoothed)
                probed_at = at
            answers.append(smoothed)
        out.append(answers)
    return out

def main():
    request = json.load(sys.stdin)
    policy = request["policy"]
    json.dump({"clocks": clocks(policy, request.get("clocks", [])), "probes": probes(policy, request.get("probes", []))}, sys.stdout)

main()
