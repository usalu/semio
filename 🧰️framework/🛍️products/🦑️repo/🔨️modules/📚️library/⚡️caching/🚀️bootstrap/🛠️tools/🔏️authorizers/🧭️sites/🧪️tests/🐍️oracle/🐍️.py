#!/usr/bin/env python3
"""🐍️ Independent token-based oracle for literal module sites: reads the fixture JSON given as argv[1] and prints {id: {specifiers, computed}}."""
import json, re, sys

IDENT = re.compile(r"[A-Za-z_$][\w$]*")
REGEX_PRIOR = set("(,=:[!&|?{};+-*%<>~^") | {"return", "typeof", "case", "do", "else", "in", "of", None}

def tokenize(text):
    tokens, i, n = [], 0, len(text)
    def prior():
        return tokens[-1][1] if tokens else None
    while i < n:
        c = text[i]
        if c.isspace():
            i += 1
        elif text.startswith("//", i):
            j = text.find("\n", i)
            i = n if j < 0 else j
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            i = n if j < 0 else j + 2
        elif c in "'\"":
            j = i + 1
            while j < n and text[j] != c:
                j += 2 if text[j] == "\\" else 1
            tokens.append(("str", text[i + 1:j]))
            i = j + 1
        elif c == "`":
            j, plain = i + 1, True
            while j < n and text[j] != "`":
                if text[j] == "\\":
                    j += 2
                    continue
                if text.startswith("${", j):
                    plain = False
                j += 1
            tokens.append(("str", text[i + 1:j]) if plain else ("tpl", ""))
            i = j + 1
        elif c == "/" and (prior() in REGEX_PRIOR):
            j, cls = i + 1, False
            while j < n:
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
            j += 1
            while j < n and text[j].isalpha():
                j += 1
            tokens.append(("regex", ""))
            i = j
        else:
            m = IDENT.match(text, i)
            if m:
                tokens.append(("id", m.group(0)))
                i = m.end()
            else:
                tokens.append(("p", c))
                i += 1
    return [(k, v) for k, v in tokens]

def sites(text):
    toks = tokenize(re.sub(r"^#![^\n]*", "", text))
    kinds = [k for k, _ in toks]
    vals = [v for _, v in toks]
    n = len(toks)
    def at(i):
        return (kinds[i], vals[i]) if 0 <= i < n else (None, None)
    def balanced(i):
        depth = 0
        for j in range(i, n):
            if kinds[j] == "p" and vals[j] == "(":
                depth += 1
            elif kinds[j] == "p" and vals[j] == ")":
                depth -= 1
                if depth == 0:
                    return j
        return n - 1
    def is_create(i):
        return at(i) == ("id", "createRequire") and at(i + 1) == ("p", "(")
    bound = {"require"}
    for i in range(n):
        if at(i) == ("p", "=") and kinds[i - 1] == "id" if i else False:
            j = i + 1
            if at(j)[0] == "id" and at(j + 1) == ("p", ".") and at(j + 2) == ("id", "createRequire"):
                j += 2
            if is_create(j):
                bound.add(vals[i - 1])
    literals, computed = set(), 0
    def take(i, resolve_after=None):
        nonlocal computed
        if at(i) == ("p", "("):
            kind, value = at(i + 1)
            if kind == "str":
                literals.add(value)
            else:
                computed += 1
    for i in range(n):
        k, v = at(i)
        prev = at(i - 1)
        if k == "id" and v == "import":
            if at(i + 1) == ("p", "("):
                take(i + 1)
            elif at(i + 1) == ("p", "."):
                if at(i + 2) == ("id", "meta") and at(i + 3) == ("p", ".") and at(i + 4) == ("id", "resolve"):
                    take(i + 5)
            elif at(i + 1) == ("id", "type") and (at(i + 2)[0] == "id" or at(i + 2) in (("p", "{"), ("p", "*"))) and at(i + 2) != ("id", "from"):
                continue
            else:
                depth, j = 0, i + 1
                while j < n and not (vals[j] == ";" and kinds[j] == "p"):
                    if kinds[j] == "p" and vals[j] == "{":
                        depth += 1
                    elif kinds[j] == "p" and vals[j] == "}":
                        depth -= 1
                    elif kinds[j] == "str" and depth == 0:
                        literals.add(vals[j])
                        break
                    j += 1
        elif k == "id" and v == "from" and at(i + 1)[0] == "str" and prev != ("p", "."):
            j = i - 1
            while j >= 0 and not (kinds[j] == "p" and vals[j] == ";"):
                j -= 1
            if at(j + 1) == ("id", "export") and at(j + 2) != ("id", "type"):
                literals.add(vals[i + 1])
        elif k == "id" and v in bound and prev != ("p", "."):
            if at(i + 1) == ("p", "("):
                take(i + 1)
            elif at(i + 1) == ("p", ".") and at(i + 2) == ("id", "resolve"):
                take(i + 3)
        elif is_create(i):
            end = balanced(i + 1)
            if at(end + 1) == ("p", "("):
                take(end + 1)
            elif at(end + 1) == ("p", ".") and at(end + 2) == ("id", "resolve"):
                take(end + 3)
        elif k == "id" and v == "require" and at(i + 1) == ("p", ".") and at(i + 2) == ("id", "resolve"):
            pass
    return sorted(literals), computed

if __name__ == "__main__":
    fixture = json.load(open(sys.argv[1], encoding="utf8"))
    print(json.dumps({c["id"]: dict(zip(("specifiers", "computed"), sites(c["source"]))) for c in fixture["cases"]}, ensure_ascii=False))
