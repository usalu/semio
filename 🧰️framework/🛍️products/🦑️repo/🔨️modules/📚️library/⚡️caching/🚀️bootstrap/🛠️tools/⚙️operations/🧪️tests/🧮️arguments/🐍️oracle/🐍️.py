"""🧬️ Independent Python oracle of the Nx operation translation (standard library only): the portable changed-file rule of `affected` (tracked changes against HEAD plus untracked files unless a base, head, file list or one change class is chosen) and the cache and inputs operations. Requests arrive as JSON on stdin."""
import json, re, sys

NAMES = ["all", "base", "head", "files", "uncommitted", "untracked"]
VALUED = ["base", "head", "files"]


def affected(args):
    boundary = args.index("--") if "--" in args else None
    head = args[:boundary] if boundary is not None else args
    tail = args[boundary:] if boundary is not None else []
    if any(re.match(r"^(--projects|-p)(=|$)", argument) for argument in head):
        return {"error": "Nx affected cannot be narrowed with --projects; select the projects with run-many or drop --projects"}
    state, kept, index = {}, [], 0
    while index < len(head):
        argument = head[index]
        separator = argument.find("=")
        name = (argument if separator < 0 else argument[:separator])
        name = name[2:] if name.startswith("--") else name
        if not argument.startswith("--") or name not in NAMES:
            kept.append(argument)
            index += 1
            continue
        if name in VALUED:
            if separator < 0:
                index += 1
                value = head[index] if index < len(head) else None
            else:
                value = argument[separator + 1:]
            if value is not None:
                state[name] = value
        else:
            state[name] = separator < 0 or argument[separator + 1:] != "false"
        index += 1
    if state.get("all") is True:
        return {"value": ["run-many"] + kept[1:] + tail}
    uncommitted, untracked = state.get("uncommitted") is True, state.get("untracked") is True
    if "files" in state:
        selection = ["--files=" + state["files"]]
    elif uncommitted and untracked:
        selection = ["--base=HEAD"]
    elif uncommitted:
        selection = ["--uncommitted"]
    elif untracked:
        selection = ["--untracked"]
    else:
        selection = ["--base=" + state.get("base", "HEAD")] + (["--head=" + state["head"]] if "head" in state else [])
    return {"value": kept + selection + tail}


def operation(args):
    if args and args[0] == "cache":
        return {"value": ["reset", "--onlyCache"] if len(args) > 1 and args[1] == "clear" else None}
    if args and args[0] == "inputs":
        if "--help" in args or "-h" in args:
            return {"value": None}
        if len(args) != 2 or not re.fullmatch(r"[^\s:]+:[^\s:]+(:[^\s:]+)?", args[1]):
            return {"error": "Expected repo inputs <project:target[:configuration]>"}
        return {"value": ["show", "target", "inputs", args[1], "--json"]}
    return {"error": "not an operation of this oracle"}


request = json.load(sys.stdin)
print(json.dumps([(affected if request["op"] == "affected" else operation)(row) for row in request["rows"]]))
