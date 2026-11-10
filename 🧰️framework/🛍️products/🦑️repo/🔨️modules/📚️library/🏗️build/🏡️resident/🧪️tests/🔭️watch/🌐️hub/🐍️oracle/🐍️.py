#!/usr/bin/env python3
"""🔎 Independent oracle for the watch hub: the `open` command line of supervisor arguments.
usage: 🐍️.py open FIXTURES.json"""
import json, sys

def option(args, name):
    return args[args.index(name) + 1] if name in args else ""

def values(args, name):
    return [args[i + 1] for i, item in enumerate(args) if item == name]

def line(tag, args):
    return "\t".join(["open", tag, option(args, "--root"), option(args, "--latency-ms") or "5", "1", "\x1f".join(values(args, "--exclude")), "\x1f".join(values(args, "--exclude-name")), option(args, "--since"), option(args, "--uuid")]) + "\n"

if __name__ == "__main__" and sys.argv[1] == "open":
    print(json.dumps([line(item["tag"], item["args"]) for item in json.load(open(sys.argv[2], encoding="utf-8"))["open"]], ensure_ascii=False))
