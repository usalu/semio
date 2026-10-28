"""🐍️ Independent oracle of the frontier fact rule: a recorded inode or member mode of `0` is unknown (the native scan on Windows has no inode or POSIX mode without an open handle) and agrees with everything; any other recorded value agrees only with itself. usage: python3 -I oracle.py < [{"present": n, "recorded": n}, ...] -> [bool, ...]"""
import json, sys
json.dump([row["recorded"] == 0 or row["present"] == 0 or row["present"] == row["recorded"] for row in json.load(sys.stdin)], sys.stdout)
