#!/usr/bin/env python3
"""Independent model of the daemon yield protocol: an eviction gets a namespace exactly when no peer eviction holds the request, no loader holds it and no daemon with an open request holds it. usage: 🐍️.py FIXTURE"""
import json, sys

fixture = json.load(open(sys.argv[1]))


def outcome(row):
    return not row["peerEviction"] and not row["loader"] and row["daemon"] != "active"


import re


def wait(row):
    value = row["environment"].get("SEMIO_NX_GRAPH_CAPACITY_WAIT_MS")
    return int(value) if value is not None and re.fullmatch(r"\d{1,9}", value) else 900000


print(json.dumps({"outcomes": [outcome(row) for row in fixture["scenarios"]], "capacityWait": [wait(row) for row in fixture["capacityWait"]]}))
