#!/usr/bin/env python3
"""🧠 Independent model of the graph memo decision (shares no code with 🟦️.ts): `🐍️.py CASES.json` prints, per case, what each fact row reads, which directories a change must not touch, and which changes are relevant.

The policy table (which kind reads what) is DATA and is read from the schema file; everything else is re-derived here: a row reads the entries it names (`path`), the entries inside its key (`text`), a request and its result (`resolve`), nothing (`none`) or follows symbolic links (`link`, always checked again); a read outside the root or inside an excluded name is checked again; a bare module request is checked again; the directories are every entry, its directory and all directories above up to the root; a change is relevant when it names one of them. Directories are told apart by what read them: a STAMPED directory (a frontier directory with a usable stamp that only entries and name/type listings of the policy's `stamped` kinds read) reacts to a row directly inside it that adds, removes or renames a name, and a row that only says 'modified' there is PENDING (the stamp must confirm it); a STRICT directory (named by a resolution or a derived fact, or its parent; read by a kind of the policy's `following`; listed but without a stamp) makes every row directly inside it relevant, except the modification of a directory; any other directory reacts only to rows that name an entry or an ancestor."""
import json
import os
import posixpath
import re
import sys
import unicodedata

SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🧬️schema", "🔣️.json")
POLICY = json.load(open(SCHEMA, encoding="utf8"))["definitions"]["Policy"]["const"]
TABLE = POLICY["relevance"]
STAMPED = set(POLICY["stamped"])
FOLLOWING = set(POLICY["following"])


def fold(path):
    return unicodedata.normalize("NFC", path).lower()


def relative(root, path):
    text = path.replace("\\", "/")
    base = root.replace("\\", "/").rstrip("/")
    if not (text.startswith("/") or re.match(r"^[A-Za-z]:/", text)):
        clean = posixpath.normpath(text)
        return "" if clean == "." else clean
    if text == base:
        return ""
    if text.startswith(base + "/"):
        return posixpath.normpath(text[len(base) + 1:])
    return None


def reach(row, root, excluded_names):
    how = TABLE.get(row["kind"])
    if how is None:
        return None
    if how == "none":
        return {"mentions": [], "recheck": False}
    mentions, state = [], {"recheck": how == "link"}

    def take(path):
        if not isinstance(path, str):
            state["recheck"] = True
            return
        rel = relative(root, path)
        if rel is None or any(part in excluded_names for part in rel.split("/")):
            state["recheck"] = True
        else:
            mentions.append(rel)

    if how in ("path", "analysis", "link"):
        take(row["path"])
    elif how == "resolve":
        try:
            parsed = json.loads(row["path"])
            origin, request = parsed[0], parsed[1]
            assert isinstance(origin, str) and isinstance(request, str)
        except Exception:
            return {"mentions": [], "recheck": True}
        take(origin)
        if request.startswith("."):
            take(posixpath.join(posixpath.dirname(origin.replace("\\", "/")), request))
        elif request.startswith("/") or re.match(r"^[A-Za-z]:[\\/]", request):
            take(request)
        else:
            state["recheck"] = True
        if isinstance(row["fact"], str) and not row["fact"].startswith("!"):
            take(row["fact"])
    else:
        base = root.replace("\\", "/").rstrip("/")
        for found in re.finditer(re.escape(base) + r'(?:/[^"\\\x00-\x1f]*)?', row["path"].replace("\\\\", "/")):
            take(found.group(0))
    return {"mentions": mentions, "recheck": state["recheck"]}


def directories(mentions):
    seen = set()
    for mention in mentions:
        path = mention
        while path not in seen:
            seen.add(path)
            if path == "":
                break
            path = path.rsplit("/", 1)[0] if "/" in path else ""
    return seen


def classes(kind, mentions):
    how = TABLE[kind]
    around = []
    for mention in mentions:
        around += [mention, mention.rsplit("/", 1)[0] if "/" in mention else ""]
    if how == "path" and kind in STAMPED:
        return mentions, [], mentions
    if how == "path" and kind in FOLLOWING:
        return mentions, mentions, []
    if how in ("resolve", "text"):
        return mentions, around, []
    return mentions, [], []


def decide(names, strict, stamped, change, insensitive):
    key = fold if insensitive else (lambda value: value)
    held, hard, stamps = ({key(item) for item in group} for group in (names, strict, stamped))
    kind, directory, path = change
    parent = path.rsplit("/", 1)[0] if "/" in path else ""
    if key(path) in held:
        return [True, None]
    if key(parent) in hard and not (kind == "m" and directory == 1):
        return [True, None]
    if key(parent) in stamps:
        return [True, None] if kind != "m" else [False, parent]
    return [False, None]


def leaves(row):
    yield row
    if row["kind"] == "analysis" and isinstance(row["fact"], dict):
        for inner in row["fact"].get("nestedObservations", []) or []:
            yield from leaves(inner)


def solve(case):
    root, names = case["root"], set(case["excludedNames"])
    rows_out, vouched, hard, listing = [], [], [], []
    for row in case["rows"]:
        for leaf in leaves(row):
            result = reach(leaf, root, names)
            rows_out.append(None if result is None else {"kind": leaf["kind"], "mentions": result["mentions"], "recheck": result["recheck"]})
            if result is not None and not result["recheck"]:
                entries, strict, listed = classes(leaf["kind"], result["mentions"])
                vouched.extend(entries)
                hard.extend(strict)
                listing.extend(listed)
    dirs = directories(vouched)
    stampable = set(case["stampable"])
    final = set(hard) | {item for item in listing if item not in stampable}
    stamped = sorted(item for item in stampable if item not in final)
    return {"rows": rows_out, "directories": sorted(dirs), "strict": sorted(final), "stamped": stamped, "relevant": [decide(dirs, final, stamped, change, case["insensitive"]) for change in case["changes"]]}


if __name__ == "__main__":
    json.dump([solve(case) for case in json.load(open(sys.argv[1], encoding="utf8"))], sys.stdout, ensure_ascii=False)
