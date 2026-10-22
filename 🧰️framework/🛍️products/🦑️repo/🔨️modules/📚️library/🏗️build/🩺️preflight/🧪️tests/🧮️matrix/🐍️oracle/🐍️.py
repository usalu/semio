"""🧮️ Independent Python oracle of the build preflight verdict (standard library only): it lists every unmet authored requirement, in the authored order and wording, for one requirement set and one set of host facts. Requests arrive as JSON on stdin."""
import json, sys


def problems(required, facts):
    out = []
    if facts["architecture"] not in required["architectures"]:
        out.append("architecture %s is not one of %s" % (facts["architecture"], ", ".join(required["architectures"])))
    if facts["availableBytes"] < required["minimumAvailableBytes"]:
        out.append("available bytes %d are below required %d" % (facts["availableBytes"], required["minimumAvailableBytes"]))
    if required["exchange"] and facts["exchange"] != "supported":
        out.append("atomic or journaled directory swap is unsupported by the host or the workspace filesystem")
    if required["exchange"] and facts["caseSensitive"] != "yes":
        out.append("a case-sensitive workspace filesystem is required for directory-backed partitions")
    for tool in required["tools"]:
        if tool not in facts["tools"]:
            out.append("missing executable: " + tool)
    return out


def main():
    request = json.load(sys.stdin)
    print(json.dumps([problems(row["required"], row["facts"]) for row in request["rows"]]))


main()
