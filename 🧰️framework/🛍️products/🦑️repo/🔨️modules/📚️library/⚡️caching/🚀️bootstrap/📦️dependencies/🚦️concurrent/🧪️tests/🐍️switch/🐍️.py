#!/usr/bin/env python3
"""Independent reader of the atomic pointer switch: polls a pointer file while another process replaces it and proves that every read is a complete, correctly sealed pointer whose epoch never moves backwards.

usage: switch.py <pointer-file> <stop-file> <deadline-seconds>
prints one JSON line {reads, invalid, missing, backwards, distinct, first, last}.
"""
import hashlib
import json
import os
import sys
import time


def canonical(value):
    if isinstance(value, dict):
        return "{" + ",".join(json.dumps(key, ensure_ascii=False) + ":" + canonical(value[key]) for key in sorted(value, key=lambda k: k.encode("utf-16-be"))) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical(item) for item in value) + "]"
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def main():
    path, stop, deadline = sys.argv[1], sys.argv[2], time.time() + float(sys.argv[3])
    reads = invalid = missing = backwards = 0
    epochs = set()
    last = 0
    first = None
    while time.time() < deadline and not os.path.exists(stop):
        try:
            with open(path, "rb") as handle:
                data = handle.read()
        except FileNotFoundError:
            missing += 1
            continue
        reads += 1
        try:
            value = json.loads(data.decode("utf-8"))
            body = {key: item for key, item in value.items() if key != "sha256"}
            if hashlib.sha256(canonical(body).encode("utf-8")).hexdigest() != value["sha256"]:
                raise ValueError("seal")
        except Exception:
            invalid += 1
            continue
        epoch = value["epoch"]
        if epoch < last:
            backwards += 1
        last = max(last, epoch)
        first = epoch if first is None else first
        epochs.add(epoch)
    print(json.dumps({"reads": reads, "invalid": invalid, "missing": missing, "backwards": backwards, "distinct": len(epochs), "first": first, "last": last}))


main()
