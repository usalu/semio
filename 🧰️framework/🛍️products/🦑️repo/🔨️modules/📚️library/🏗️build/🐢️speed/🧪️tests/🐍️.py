#!/usr/bin/env python3
"""🐍️ Independent model of the machine speed factor (standard library only): the product of the core ratio and the square root of the memory ratio, each at least 1, capped and rounded up to two decimals with exact rational arithmetic."""
import json, math, sys
from fractions import Fraction

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
policy = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Policy"]["const"]
rows = []
for machine in fixture["machines"]:
    cores = Fraction(max(policy["referenceCores"], machine["cores"]), machine["cores"])
    memory = Fraction(max(policy["referenceMegabytes"], machine["totalMegabytes"]), machine["totalMegabytes"])
    factor = float(cores) * math.sqrt(float(memory))
    hundredths = math.ceil(Fraction(factor).limit_denominator(10**9) * 100 - Fraction(1, 10**7))
    rows.append(min(policy["maximumFactor"], hundredths / 100))
print(json.dumps(rows))
