"""🧭 Independent Python oracle of declared input patterns (standard library only): a `{projectRoot}` or `{workspaceRoot}` input becomes a normalised workspace-relative path, or nothing when it names a place outside the workspace. Requests arrive as JSON on stdin."""
import json, re, sys


def pattern(value, root):
    if not isinstance(value, str) or not re.match(r"^\{(projectRoot|workspaceRoot)\}/", value):
        return None
    joined = (("" if root == "." else root + "/") + value[len("{projectRoot}/"):]) if value.startswith("{projectRoot}/") else value[len("{workspaceRoot}/"):]
    parts = []
    for part in joined.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            if not parts:
                return None
            parts.pop()
        else:
            parts.append(part)
    return "/".join(parts)


print(json.dumps([pattern(r["input"], r["projectRoot"]) for r in json.load(sys.stdin)["requests"]], ensure_ascii=False))
