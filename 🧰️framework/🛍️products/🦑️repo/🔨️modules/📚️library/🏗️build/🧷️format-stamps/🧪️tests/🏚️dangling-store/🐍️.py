"""🐍️ Independent oracle for dangling dependency stores: names the bare package a module-resolution message asks for, `None` for every other message."""
import json,re,sys
def package(message):
    found=re.search(r"Cannot find (?:package|module) '([^']+)'",message)
    if not found or re.match(r"^(\.|/|[A-Za-z]:|node:|bun:)",found.group(1)):
        return None
    parts=found.group(1).split("/")
    return "/".join(parts[:2]) if found.group(1).startswith("@") else parts[0]
print(json.dumps([package(row["message"]) for row in json.load(open(sys.argv[1]))["messages"]]))
