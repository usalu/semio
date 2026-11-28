"""🧭 Lists relative TS/JS module specifiers (static import, export-from, dynamic import(), require()) under the given roots that resolve to no existing file; strings, template literals, comments and regex literals are tokenised away, so fixtures and `toContain("import ...")` never count."""
import os, re, sys

SKIPPED = {"node_modules", "dist", "target", ".cache", "🤖️generated", "🗑️generated"}
EXTENSIONS = (".ts", ".tsx", ".mts", ".cts", ".mjs", ".cjs", ".js")
REGEX_PRECEDERS = set("(,=:[!&|?{};+-*%<>~^") | {"return", "typeof", "case", "do", "else", "in", "of", "void", "delete", "throw", "new", "yield", "await"}


ESCAPE = re.compile(r"\\u\{([0-9a-fA-F]+)\}|\\u([0-9a-fA-F]{4})|\\x([0-9a-fA-F]{2})|\\(.)", re.S)


def decode(value):
    """Decodes JS string escapes of a module specifier, joining UTF-16 surrogate pairs."""
    if "\\" not in value:
        return value
    def one(match):
        brace, unit, byte, char = match.groups()
        code = brace or unit or byte
        return chr(int(code, 16)) if code else {"n": "\n", "t": "\t"}.get(char, char)
    return ESCAPE.sub(one, value).encode("utf-16", "surrogatepass").decode("utf-16", "replace")


def scan(text):
    """Yields (kind, value) code tokens: words, punctuation and string literals; skips comments, template text and regexes."""
    i, n, out = 0, len(text), []
    braces = []

    def template(resume=False):
        nonlocal i
        if not resume:
            i += 1
        while i < n:
            c = text[i]
            if c == "\\":
                i += 2
            elif c == "`":
                i += 1
                return
            elif c == "$" and text[i + 1:i + 2] == "{":
                i += 2
                braces.append("t")
                return "expr"
            else:
                i += 1
        return

    def previous_allows_regex():
        if not out:
            return True
        kind, value = out[-1]
        if kind == "str" or kind == "num":
            return False
        if kind == "word":
            return value in REGEX_PRECEDERS
        return value not in (")", "]", "}")

    while i < n:
        c = text[i]
        if c in " \t\r\n":
            i += 1
        elif text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            i = n if j < 0 else j + 2
        elif c in "\"'":
            j = i + 1
            while j < n and text[j] != c and text[j] != "\n":
                j += 2 if text[j] == "\\" else 1
            out.append(("str", decode(text[i + 1:j])))
            i = j + 1
        elif c == "`":
            if template() == "expr":
                out.append(("punct", "${"))
        elif c == "{":
            braces.append("b")
            out.append(("punct", c))
            i += 1
        elif c == "}":
            if braces and braces.pop() == "t":
                out.append(("punct", "}"))
                i += 1
                if template(True) == "expr":
                    out.append(("punct", "${"))
            else:
                out.append(("punct", c))
                i += 1
        elif c == "/" and previous_allows_regex():
            j, cls = i + 1, False
            while j < n and text[j] != "\n":
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == "[":
                    cls = True
                elif text[j] == "]":
                    cls = False
                elif text[j] == "/" and not cls:
                    break
                j += 1
            i = j + 1
            while i < n and (text[i].isalpha()):
                i += 1
            out.append(("num", "regex"))
        elif c.isalpha() or c in "_$" or ord(c) > 127:
            j = i
            while j < n and (text[j].isalnum() or text[j] in "_$" or ord(text[j]) > 127):
                j += 1
            out.append(("word", text[i:j]))
            i = j
        elif c.isdigit():
            j = i
            while j < n and (text[j].isalnum() or text[j] in "._"):
                j += 1
            out.append(("num", text[i:j]))
            i = j
        else:
            out.append(("punct", c))
            i += 1
    return out


def specifiers(text):
    tokens = scan(text)
    for index, (kind, value) in enumerate(tokens):
        if kind != "str" or not value.startswith(("./", "../")):
            continue
        before = [v for _, v in tokens[max(0, index - 3):index]]
        if before[-1:] == ["from"] or before[-1:] == ["import"] or before[-2:] in (["import", "("], ["require", "("]):
            if before[-2:-1] == ["."]:
                continue
            yield value


def resolves(directory, spec):
    base = os.path.normpath(os.path.join(directory, spec.split("?", 1)[0].split("#", 1)[0]))
    if os.path.exists(base):
        return True
    stem, ext = os.path.splitext(base)
    swaps = {".js": (".ts", ".tsx", ".mts"), ".mjs": (".mts", ".ts"), ".cjs": (".cts",), ".jsx": (".tsx",)}.get(ext, ())
    if any(os.path.exists(stem + s) for s in swaps):
        return True
    suffixes = EXTENSIONS + (".json", ".d.ts", ".d.mts")
    return any(os.path.exists(base + s) or os.path.exists(os.path.join(base, "index" + s)) for s in suffixes)


missing = 0
for root in sys.argv[1:]:
    for directory, directories, files in os.walk(root):
        directories[:] = [d for d in directories if d not in SKIPPED]
        if "🗑️generated" in directory:
            continue
        for name in files:
            if not name.endswith(EXTENSIONS) or name.endswith(".d.ts"):
                continue
            path = os.path.join(directory, name)
            text = open(path, encoding="utf8", errors="ignore").read()
            for spec in specifiers(text):
                if not resolves(directory, spec):
                    missing += 1
                    print(f"{path} -> {spec}")
sys.exit(1 if missing else 0)
