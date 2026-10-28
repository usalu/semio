#!/usr/bin/env python3
"""🐍️ Independent oracle of the read oracle: recomputes the policy, the log parsing and the Landlock grants with plain Python, evaluates rendered Seatbelt profiles rule by rule (the last matching rule decides) and finds the smallest rule count of tiny trees by exhaustive search.
usage: 🐍️.py <corpus.json> <probe.json>
The probe holds what the TypeScript side rendered (Seatbelt profile text per policy case, Landlock grants); this script reads the RENDERED TEXT and evaluates it as the kernel would, and parses the log vectors with its own parser. Prints one JSON document."""
import itertools, json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
probe = json.load(open(sys.argv[2], encoding="utf-8"))
AMBIENT = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"]


def under(base, path):
    return path == base or path.startswith(base + "/")


def ignored(path, trees, names):
    return any(under(t, path) for t in trees) or path.rsplit("/", 1)[-1] in names


def direct_readable(case, files):
    """What the policy says in words: the declared files, everything below an ambient tree, the files named like a declared name."""
    trees = AMBIENT + case["ambient"]
    return sorted(path for path in files if path in case["allowed"] or ignored(path, trees, case["names"]))


def unquote(text):
    return text.replace('\\"', '"').replace("\\\\", "\\")


LINE = re.compile(r'\((allow|deny) file-read-data \((subpath|literal) "(.*)"\)\)|\(allow file-read-data \(regex #"(.*)"\)\)')


def seatbelt_readable(profile, root, files):
    """Evaluates the rendered Seatbelt text as the kernel does: the rules apply in order, the last matching rule decides, the directory rule concerns no file."""
    rules = []
    for line in profile.splitlines():
        m = LINE.fullmatch(line)
        if not m:
            continue
        if m.group(4) is not None:
            expression = re.compile(m.group(4))
            rules.append(("allow", lambda p, expression=expression: expression.fullmatch(p) is not None))
        elif m.group(2) == "subpath":
            base = unquote(m.group(3))
            rules.append((m.group(1), lambda p, base=base: under(base, p)))
        else:
            base = unquote(m.group(3))
            rules.append((m.group(1), lambda p, base=base: p == base))
    readable = []
    for path in files:
        decision = None
        for effect, matches in rules:
            if matches(root + "/" + path):
                decision = effect
        if decision == "allow":
            readable.append(path)
    return sorted(readable)


def rules_readable(case, rules, files):
    """Applies the plan rules in order (the root starts refused) plus the ambient trees and names that are readable anyway."""
    readable = []
    for path in files:
        decision = None
        for rule in rules:
            covers = path == rule["path"] if rule["kind"] == "literal" else (rule["path"] == "" or under(rule["path"], path))
            if covers:
                decision = rule["effect"]
        if decision == "allow" or ignored(path, AMBIENT + case["ambient"], case["names"]):
            readable.append(path)
    return sorted(readable)


def smallest_rule_count(case):
    """Exhaustive search over every way to label the nodes (root, directories, files) with none/allow/deny: the nearest labelled ancestor decides, the root starts refused; the fewest labels that read exactly the declared files (ignored files may go either way)."""
    files = case["files"]
    directories = {""}
    for f in files:
        parts = f.split("/")
        directories.update("/".join(parts[:i]) for i in range(1, len(parts)))
    nodes = sorted(directories) + sorted(files)
    choices = ("none", "allow", "deny") if case["refusals"] else ("none", "allow")
    trees = AMBIENT + case["ambient"]
    best = None
    for labels in itertools.product(choices, repeat=len(nodes)):
        used = sum(1 for label in labels if label != "none")
        if best is not None and used >= best:
            continue
        label_of = dict(zip(nodes, labels))
        ok = True
        for f in files:
            if ignored(f, trees, case["names"]):
                continue
            parts = f.split("/")
            chain = [f] + ["/".join(parts[:i]) for i in range(len(parts) - 1, 0, -1)] + [""]
            decision = "deny"
            for node in chain:
                if label_of[node] != "none":
                    decision = label_of[node]
                    break
            if (decision == "allow") != (f in case["allowed"]):
                ok = False
                break
        if ok:
            best = used
    return best


def grants_readable(grants, paths):
    return sorted(path for path in paths if any(path == grant or path.startswith(grant.rstrip("/") + "/") for grant in grants))


def decode(literal):
    out, i, raw = bytearray(), 0, literal
    named = {"n": 10, "t": 9, "r": 13, '"': 34, "\\": 92, "a": 7, "b": 8, "f": 12, "v": 11, "e": 27}
    while i < len(raw):
        c = raw[i]
        if c != "\\":
            out += c.encode("utf-8")
            i += 1
            continue
        i += 1
        if raw[i] == "x":
            out.append(int(raw[i + 1:i + 3], 16))
            i += 3
        elif raw[i] in "01234567":
            j = i
            while j < len(raw) and j < i + 3 and raw[j] in "01234567":
                j += 1
            out.append(int(raw[i:j], 8))
            i = j
        else:
            out.append(named.get(raw[i], ord(raw[i])))
            i += 1
    return out.decode("utf-8")


def seatbelt_denials(log, root):
    found = set()
    for line in log.split("\n"):
        m = re.search(r"deny\(1\) file-read-data (.*)$", line)
        if m and m.group(1).strip().startswith(root + "/"):
            found.add(m.group(1).strip()[len(root) + 1:])
    return sorted(found)


def strace_reads(lines, root, cwd):
    """Second implementation: first collect (pid, call, path, flags, directory) with their result in one pass over a pending table, then classify."""
    pending, calls = {}, []
    for line in lines:
        resumed = re.match(r"^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?<\.\.\.\s+(\w+)\s+resumed>.*\)\s+=\s+(-?\d+|\?)(?:\s+(E[A-Z]+))?", line)
        if resumed:
            key = (resumed.group(1) or resumed.group(2) or "", resumed.group(3))
            if key in pending:
                calls.append((pending.pop(key), resumed.group(5), resumed.group(4)))
            continue
        call = re.match(r"^(?:\[pid\s+(\d+)\]\s+|(\d+)\s+)?(openat2|openat|open|execveat|execve)\((.*)$", line)
        if not call:
            continue
        name, rest = call.group(3), call.group(4)
        literal = re.search(r'"((?:[^"\\]|\\.)*)"', rest)
        if not literal:
            continue
        path = decode(literal.group(1))
        base = cwd
        d = re.match(r"^(?:AT_FDCWD|\d+)(?:<([^>]*)>)?", rest)
        if (name.startswith("openat") or name == "execveat") and d and d.group(1):
            base = d.group(1)
        absolute = path if path.startswith("/") else base.rstrip("/") + "/" + path
        flags = rest[rest.index('",') + 2:] if '",' in rest else ""
        reads = name.startswith("exec") or (("O_RDONLY" in flags or "O_RDWR" in flags) and "O_PATH" not in flags and "O_DIRECTORY" not in flags)
        entry = (absolute, reads)
        if "<unfinished ...>" in rest:
            pending[(call.group(1) or call.group(2) or "", name)] = entry
            continue
        done = re.search(r"\)\s+=\s+(-?\d+|\?)(?:\s+(E[A-Z]+))?", rest)
        if done:
            calls.append((entry, done.group(2), done.group(1)))
    opened, denied = set(), set()
    for (absolute, reads), error, result in calls:
        if not reads or not absolute.startswith(root + "/"):
            continue
        relative = absolute[len(root) + 1:]
        if error in ("EACCES", "EPERM"):
            denied.add(relative)
        elif result not in ("-1", "?"):
            opened.add(relative)
    return {"opened": sorted(opened), "denied": sorted(denied)}


answer = {"policy": [], "seatbeltLog": [], "strace": [], "straceStrings": [decode(row["literal"]) for row in corpus["straceStrings"]], "linuxGrants": [], "random": [], "brute": []}
for case in corpus["policy"]:
    entry = probe["policy"][case["id"]]
    answer["policy"].append({"id": case["id"], "direct": direct_readable(case, case["tree"]), "seatbelt": seatbelt_readable(entry["profile"], probe["root"], case["tree"]), "rules": rules_readable(case, entry["rules"], case["tree"])})
for case in probe["random"]:
    answer["random"].append({"direct": direct_readable(case, case["files"]), "seatbelt": seatbelt_readable(case["profile"], case["root"], case["files"]), "runs": [seatbelt_readable(text, case["root"], case["files"]) for text in case["runProfiles"]]})
for case in probe["brute"]:
    answer["brute"].append(smallest_rule_count(case))
for row in corpus["seatbeltLog"]:
    answer["seatbeltLog"].append(seatbelt_denials(row["log"], row["root"]))
for row in corpus["strace"]:
    answer["strace"].append(strace_reads(row["log"], row["root"], row["cwd"]))
for case in corpus["linuxGrants"]:
    files = [f"{directory.rstrip('/')}/{name}" for directory, names in case["tree"].items() for name in names if f"{directory.rstrip('/')}/{name}" not in case["tree"]]
    answer["linuxGrants"].append({"id": case["id"], "readable": grants_readable(probe["linuxGrants"][case["id"]], files), "files": sorted(files)})
print(json.dumps(answer, ensure_ascii=False))
