import hashlib
import json
import os
import sqlite3
import sys


def expect(path):
    fixture = json.load(open(path, encoding="utf-8"))
    out = {"confirm": {}, "publish": {}, "writer": {}}
    for row in fixture["confirm"]:
        matched = row["replay"]["status"] == "match"
        out["confirm"][row["id"]] = {"via": "replay" if matched else "walk", "sha256": row["replay"]["sha256"] if matched else row["walk"], "walks": 0 if matched else 1}
    for row in fixture["publish"]:
        attempt = row["attempt"]
        if "throws" in attempt:
            out["publish"][row["id"]] = {"result": None, "reports": [], "rethrows": True} if row["aborted"] else {"result": "error", "reports": ["error:" + attempt["throws"]], "rethrows": False}
        else:
            out["publish"][row["id"]] = {"result": attempt["returns"], "reports": [attempt["returns"]], "rethrows": False}
    for row in fixture["writer"]:
        if row["lease"] == "free":
            result, released = ("published", True) if row["body"] == "published" else ("rethrow", True)
        elif row["lease"] == "timeout":
            result, released = ("rethrow" if row["aborted"] else "busy"), False
        else:
            result, released = "rethrow", False
        out["writer"][row["id"]] = {"expect": result, "released": released}
    return out


def pattern(size):
    return bytes((index * 31 + 7) & 255 for index in range(size))


def main():
    command = sys.argv[1]
    if command == "expect":
        print(json.dumps(expect(sys.argv[2])))
    elif command == "digest":
        print(hashlib.sha256(pattern(int(sys.argv[2]))).hexdigest())
    elif command == "hold":
        connection = sqlite3.connect(sys.argv[2], isolation_level=None, timeout=0)
        connection.execute("BEGIN EXCLUSIVE")
        print("held", flush=True)
        sys.stdin.readline()
        connection.execute("COMMIT")
        connection.close()
        try:
            print("released", flush=True)
        except BrokenPipeError:
            os._exit(0)
    else:
        raise SystemExit("usage: ingress.oracle.py expect FIXTURE | digest SIZE | hold DB")


main()
