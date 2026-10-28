"""🐍️ Independent oracle of the filesystem race classifier: decides, observes and parses with the Python standard library (posixpath, ntpath) and the table of the schema, and reports one answer per vector id; it shares no code with the TypeScript implementation. A second part reproduces real races on a real directory with `os` and reports the errno names the operating system gives."""
import errno
import json
import ntpath
import os
import posixpath
import sys
import tempfile


def flavor(platform):
    return ntpath if platform == "win32" else posixpath


def decide(table, facts):
    path_module = flavor(facts["platform"])
    kind = table["codes"]["all"].get(facts["code"])
    if kind is None and facts["platform"] == "win32":
        kind = table["codes"]["win32"].get(facts["code"])
    if kind is None:
        return {"disposition": "none", "reason": None, "cause": "code"}
    if facts["syscall"] is not None and facts["syscall"] not in table["readSyscalls"]:
        return {"disposition": "none", "reason": None, "cause": "write"}
    if not path_module.isabs(facts["path"]):
        return {"disposition": "none", "reason": None, "cause": "relative"}
    if facts["root"] is None:
        return {"disposition": "none", "reason": None, "cause": "root"}
    try:
        relation = path_module.relpath(facts["path"], facts["root"])
    except ValueError:
        return {"disposition": "none", "reason": None, "cause": "outside"}
    parts = [] if relation == "." else relation.replace("\\", "/").split("/") if path_module is ntpath else relation.split("/")
    if parts and parts[0] == "..":
        return {"disposition": "none", "reason": None, "cause": "outside"}
    if any(part in table["ownedSegments"] for part in parts):
        return {"disposition": "none", "reason": None, "cause": "owned"}
    if kind == "busy":
        return {"disposition": "transient", "reason": table["reasons"]["busy"], "cause": "busy"}
    change = facts["changeMs"]
    if change is None or change < facts["startedAtMs"] - table["witness"]["slackMilliseconds"]:
        return {"disposition": "none", "reason": None, "cause": "unchanged"}
    if facts["probe"] == "unknown":
        return {"disposition": "none", "reason": None, "cause": "probe"}
    if facts["probe"] == "absent":
        return {"disposition": "persistent", "reason": table["persistentReason"], "cause": "absent"}
    if kind == "shape":
        return {"disposition": "persistent", "reason": table["reasons"]["shape"], "cause": "shape"}
    return {"disposition": "transient", "reason": table["reasons"][kind], "cause": "present"}


def observe(row):
    path_module = flavor(row["platform"])
    table = row["lstat"]
    own = table[row["path"]]
    if own == "unknown":
        return {"probe": "unknown", "changeMs": None}
    newest = lambda item: max(item["mtimeMs"], item["ctimeMs"])
    current = row["path"]
    while True:
        parent = path_module.dirname(current)
        if parent == current:
            return {"probe": "absent" if own == "absent" else "present", "changeMs": None if own == "absent" else newest(own)}
        entry = table[parent]
        if entry == "unknown":
            return {"probe": "unknown", "changeMs": None}
        if entry == "absent":
            current = parent
            continue
        best = newest(entry) if own == "absent" else max(newest(entry), newest(own))
        return {"probe": "absent" if own == "absent" else "present", "changeMs": best}


def quoted(text):
    out, at = [], 0
    while True:
        start = text.find("'", at)
        if start < 0:
            return out
        end = text.find("'", start + 1)
        if end < 0:
            return out
        if "\n" not in text[start + 1:end] and end > start + 1:
            out.append(text[start + 1:end])
        at = end + 1


def property_value(text, key):
    needle = key + ': "'
    start = text.find(needle)
    if start < 0:
        return None
    start += len(needle)
    end = text.find('"', start)
    return None if end < 0 else text[start:end]


def parse(row):
    text, path_module = row["text"][-4096:], flavor(row["platform"])
    code = property_value(text, "code")
    if code is None:
        for token in text.replace("\n", " ").split(" "):
            if token.endswith(":") and 4 <= len(token) - 1 <= 11 and token[0] == "E" and all(ch.isupper() or ch.isdigit() for ch in token[:-1]):
                code = token[:-1]
                break
    if code is None:
        return None
    syscall = property_value(text, "syscall")
    if syscall is None:
        for part in text.split(", ")[1:]:
            word = part.split(" ")[0]
            if word.isalpha() and 3 <= len(word) <= 12 and part[len(word):len(word) + 2] == " '":
                syscall = word
                break
    candidates = quoted(text)
    prop = property_value(text, "path")
    if prop is not None:
        candidates.append(prop)
    for candidate in candidates:
        if path_module.isabs(candidate):
            return {"code": code, "syscall": syscall, "path": candidate}
    return None


def real_errnos():
    out = {}
    with tempfile.TemporaryDirectory() as root:
        def attempt(name, action):
            try:
                action()
                out[name] = None
            except OSError as error:
                out[name] = errno.errorcode[error.errno]
        file_path, directory = os.path.join(root, "f"), os.path.join(root, "d")
        open(file_path, "w").close()
        os.mkdir(directory)
        attempt("lstat-missing", lambda: os.lstat(os.path.join(root, "missing")))
        attempt("lstat-under-file", lambda: os.lstat(os.path.join(file_path, "x")))
        attempt("open-missing-parent", lambda: open(os.path.join(root, "gone", "x"), "rb"))
        attempt("open-directory", lambda: open(directory, "rb"))
        attempt("scandir-file", lambda: os.scandir(file_path))
        attempt("scandir-missing", lambda: os.scandir(os.path.join(root, "missing")))
        attempt("readlink-missing", lambda: os.readlink(os.path.join(root, "missing")))
        os.symlink(os.path.join(root, "loop"), os.path.join(root, "loop"))
        attempt("open-symlink-loop", lambda: open(os.path.join(root, "loop"), "rb"))
        moved = os.path.join(root, "moved")
        os.rename(directory, moved)
        attempt("lstat-after-directory-move", lambda: os.lstat(os.path.join(directory, "x")))
        os.remove(file_path)
        os.mkdir(file_path)
        attempt("open-file-replaced-by-directory", lambda: open(file_path, "rb"))
        attempt("lstat-below-moved-directory", lambda: os.lstat(os.path.join(directory, "nope", "deeper")))
    return out


def main():
    table = json.load(open(sys.argv[1], encoding="utf-8"))["definitions"]["Table"]["const"]
    vectors = json.load(open(sys.argv[2], encoding="utf-8"))
    json.dump({
        "decide": {row["id"]: decide(table, row["facts"]) for row in vectors["decide"]},
        "observe": {row["id"]: observe(row) for row in vectors["observe"]},
        "text": {row["id"]: parse(row) for row in vectors["text"]},
        "errnos": real_errnos(),
    }, sys.stdout, ensure_ascii=False)


main()
