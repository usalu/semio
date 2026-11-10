"""🔭️ Independent oracle of the planning observation: reads the markers and counters from the scenario document and the recorded lines of the fixture, sharing no code with the TypeScript.
usage: python3 -I oracle.py SCENARIOS FIXTURE -> {case: {phaseMs, counters}}"""
import json, re, sys

PHASES = ["facts", "verification", "inference", "planning"]


def observe(doc, lines, wall):
    first = {}
    counters = {}
    for at, text in lines:
        for marker in doc["markers"]:
            if marker["phase"] not in first and re.search(marker["pattern"], text):
                first[marker["phase"]] = at
        for counter in doc["counters"]:
            hit = re.search(counter["pattern"], text)
            if hit:
                counters[counter["name"]] = counters.get(counter["name"], 0) + int(hit.group(counter["group"]))
    start = {"facts": 0}
    for phase in PHASES[1:]:
        start[phase] = first.get(phase)
    nxt = wall
    for phase in reversed(PHASES[1:]):
        start[phase] = min(start[phase], nxt) if start[phase] is not None else nxt
        nxt = start[phase]
    marks = [start[p] for p in PHASES] + [wall]
    for i in range(1, len(marks)):
        marks[i] = max(marks[i], marks[i - 1])
    return {"phaseMs": {p: marks[i + 1] - marks[i] for i, p in enumerate(PHASES)}, "counters": counters}


def main(argv):
    doc = json.load(open(argv[1], encoding="utf-8"))
    fixture = json.load(open(argv[2], encoding="utf-8"))
    print(json.dumps({c["id"]: observe(doc, c["lines"], c["wallMs"]) for c in fixture["cases"]}))


if __name__ == "__main__":
    main(sys.argv)
