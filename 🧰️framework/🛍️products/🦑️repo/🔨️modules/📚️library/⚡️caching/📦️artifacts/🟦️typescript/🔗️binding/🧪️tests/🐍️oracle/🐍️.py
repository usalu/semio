"""🔗️ Independent Python oracle of the TypeScript package binding plan (standard library only, POSIX paths): it derives the owner, the fixed links, the output and the receipt location from the workspace, the package, the mode and the products directory, with the artifact layout rules read from the authored protocol. Requests arrive as JSON on stdin."""
import hashlib, json, posixpath, sys


def canonical(path):
    return posixpath.isabs(path) and posixpath.normpath(path) == path


def utf16(text):
    return len(text.encode("utf-16-le")) // 2


def plan(row, protocol):
    workspace, package, products, mode, name = row["workspace"], row["package"], row["products"], row["mode"], row["packageName"]
    if not all(canonical(p) for p in (workspace, package, products)) or mode not in ("prototype", "release") or not name or utf16(name) > 256:
        return {"error": "Canonical package binding data required"}
    child = posixpath.relpath(package, workspace)
    if child in ("", ".") or child == ".." or child.startswith("../") or posixpath.isabs(child):
        return {"error": "Actual repository package owner required"}
    owner = child
    if utf16(owner) > 256:
        return {"error": "Closed TypeScript artifact layout required"}
    if "\\" in owner or ":" in owner or any(part in ("", ".", "..") for part in owner.split("/")) or any(ord(ch) < 32 or ord(ch) == 127 for ch in owner):
        return {"error": "Canonical portable artifact source owner required"}
    directory = posixpath.join(products, protocol["namespace"], hashlib.sha256(owner.encode()).hexdigest()[:32], mode)
    output = posixpath.join(directory, "out")
    if utf16(output) > protocol["maximumPathUnits"]:
        return {"error": "Finite artifact path capacity exceeded"}
    links = [{"path": posixpath.join(package, "dist"), "target": output}, {"path": posixpath.join(package, "📤\ufe0fdist", mode), "target": output}]
    if any(utf16(link["path"]) + 16 > 256 for link in links):
        return {"error": "Package binding path capacity exceeded"}
    return {"value": {"owner": owner, "output": output, "receipt": posixpath.join(posixpath.dirname(directory), "binding.json"), "links": links}}


def main():
    request = json.load(sys.stdin)
    print(json.dumps([plan(row, request["protocol"]) for row in request["rows"]], ensure_ascii=False))


main()
