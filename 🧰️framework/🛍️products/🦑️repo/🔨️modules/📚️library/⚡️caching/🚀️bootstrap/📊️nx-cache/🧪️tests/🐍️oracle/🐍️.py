import json, re, sys

fixture = json.load(sys.stdin)
policy = fixture["policy"]
task = re.compile(policy["taskPattern"])
local = re.compile(policy["localPattern"])
remote = re.compile(policy["remotePattern"])
colour = re.compile("\u001b\\[[0-9;]*[A-Za-z]")


def outcome(line):
    text = colour.sub("", line).strip()
    if not task.match(text):
        return None
    if local.search(text):
        return "local"
    if remote.search(text):
        return "remote"
    return "executed"


json.dump([outcome(row["line"]) for row in fixture["cases"]], sys.stdout)
