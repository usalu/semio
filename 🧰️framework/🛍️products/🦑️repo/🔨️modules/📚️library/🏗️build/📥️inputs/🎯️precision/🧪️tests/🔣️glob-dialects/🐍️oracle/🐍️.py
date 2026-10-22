"""🔣 Independent Python oracle of the three glob dialects of the build (standard library only): `ws` knows `*` and `**`, `parity` adds `?`, `input` adds `?` and brace groups; everything else is literal and `**` crosses directories wherever it stands. Requests arrive as JSON on stdin: {"cases": [{"engine", "pattern", "paths"}]}."""
import json, re, sys


def translate(engine, pattern):
    out, depth, index = [], 0, 0
    while index < len(pattern):
        ch = pattern[index]
        if ch == "*":
            if pattern[index + 1:index + 2] != "*":
                out.append("[^/]*")
            elif pattern[index + 2:index + 3] == "/":
                out.append("(?:.*/)?")
                index += 2
            else:
                out.append(".*")
                index += 1
        elif ch == "?" and engine in ("parity", "input"):
            out.append("[^/]")
        elif ch == "{" and engine == "input":
            depth += 1
            out.append("(?:")
        elif ch == "}" and engine == "input" and depth:
            depth -= 1
            out.append(")")
        elif ch == "," and engine == "input" and depth:
            out.append("|")
        else:
            out.append(re.escape(ch))
        index += 1
    return re.compile("^" + "".join(out) + r"\Z", re.DOTALL)


def run(case):
    try:
        compiled = translate(case["engine"], case["pattern"])
    except re.error:
        return {"error": True}
    return {"matches": [compiled.match(path) is not None for path in case["paths"]]}


print(json.dumps([run(case) for case in json.load(sys.stdin)["cases"]], ensure_ascii=False))
