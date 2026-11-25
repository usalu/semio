"""🗜️ Independent Python oracle of the recorded unit peak (standard library only), written from the prose: a compiler reports its peak resident bytes when it ends and a charged-bytes sample every second while it runs; the unit is recorded with the largest number among the resident peak and every charged sample. usage: 🐍️.py <corpus.json> prints the expected peak of every case as JSON."""
import json, sys

with open(sys.argv[1], encoding="utf-8") as handle:
    corpus = json.load(handle)
print(json.dumps([sorted([case["peakRssBytes"], *case["chargedSamples"]])[-1] for case in corpus["cases"]]))
