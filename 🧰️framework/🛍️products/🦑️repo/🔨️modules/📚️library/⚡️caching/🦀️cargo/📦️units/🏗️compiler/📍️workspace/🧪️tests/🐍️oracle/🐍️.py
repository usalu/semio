"""📍️ Independent Python oracle of the funded compiler workspace rule (standard library only): it reads the same request through posixpath and ntpath and decides whether the directory is one unit below the units directory. Requests arrive as JSON on stdin."""
import json, ntpath, posixpath, sys


def decide(row, policy):
    paths = ntpath if row["platform"] == "win32" else posixpath
    units = paths.join(paths.join(row["root"], policy["buildNamespace"], policy["modes"][row["mode"]]), "units")
    try:
        own = paths.relpath(row["directory"], units)
    except ValueError:
        return False
    return own not in ("", ".") and own != ".." and paths.sep not in own and (paths is not ntpath or "/" not in own) and row["temporary"] == row["directory"] + ".tmp"


def main():
    request = json.load(sys.stdin)
    print(json.dumps([decide(row, request["policy"]) for row in request["rows"]]))


main()
