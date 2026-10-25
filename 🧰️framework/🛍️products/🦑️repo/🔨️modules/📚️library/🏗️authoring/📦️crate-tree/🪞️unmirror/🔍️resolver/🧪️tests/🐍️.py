#!/usr/bin/env python3
"""Independent Python model of module-level name resolution (explicit imports, globs, re-exports, cuts, visibility, macros, path mounts) over the corpus.

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def ident(name):
    return name.replace("-", "_")

TOK = re.compile(r'#\[[^\]]*\]|"(?:[^"\\]|\\.)*"|[A-Za-z_]\w*|::|[{}()\[\];,*=!<>&\-\.\']|\S')
DECL = ("fn", "struct", "enum", "const", "static", "trait", "type", "union")
PRELUDE = {"Some", "None", "Ok", "Err", "Self", "Vec", "String", "Option", "Result", "Box", "Default", "Clone", "Copy"}


class Mod:
    n = 0

    def __init__(self, crate, name, parent, vis, file, region):
        Mod.n += 1
        self.id, self.crate, self.name, self.parent, self.vis, self.file, self.region = Mod.n, crate, name, parent, vis, file, region
        self.children, self.items, self.weak, self.uses, self.opaque = {}, {}, set(), [], False


def group_end(toks, i):
    depth = 0
    while i < len(toks):
        if toks[i][0] in "({[":
            depth += 1
        elif toks[i][0] in ")}]":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return len(toks) - 1


def parse_tree(toks, i, prefix):
    leaves = []
    segs = list(prefix)
    while True:
        if toks[i][0] == "{":
            i += 1
            while toks[i][0] != "}":
                sub, i = parse_tree(toks, i, segs)
                leaves += sub
                if toks[i][0] == ",":
                    i += 1
            return leaves, i + 1
        if toks[i][0] == "*":
            return leaves + [(segs, None, True)], i + 1
        segs.append(toks[i][0])
        i += 1
        if i < len(toks) and toks[i][0] == "::":
            i += 1
            continue
        alias = None
        if i < len(toks) and toks[i][0] == "as":
            alias = toks[i + 1][0]
            i += 2
        return leaves + [(segs, alias, False)], i


class Parser:
    def __init__(self, files, crate_idents):
        self.files, self.nodes_by_file, self.macros, self.idents = files, {}, {}, crate_idents
        self.mounted = {}

    def load(self, crate, path, name, parent, vis, file_node_text=None):
        text = self.files.get(path)
        if text is None:
            return Mod(crate, name, parent, vis, path, (0, 0))
        node = Mod(crate, name, parent, vis, path, (0, len(text)))
        self.nodes_by_file.setdefault(path, []).append(node)
        toks = [(m.group(0), m.start()) for m in TOK.finditer(text)]
        self.items(node, toks, 0, text, path, posix_dir(path), True, posix_dir(path))
        return node

    def items(self, node, toks, i, text, path, filedir, in_file, base):
        while i < len(toks) and toks[i][0] != "}":
            attrs = []
            while toks[i][0].startswith("#["):
                attrs.append(toks[i][0])
                i += 1
            vis = ""
            if toks[i][0] == "pub":
                vis = "pub"
                i += 1
                if toks[i][0] == "(":
                    end = group_end(toks, i)
                    vis = "pub(" + "".join(t[0] for t in toks[i + 1:end]) + ")"
                    i = end + 1
            word = toks[i][0]
            if word == "mod":
                name = toks[i + 1][0]
                pathattr = None
                for a in attrs:
                    m = re.search(r'path\s*=\s*"([^"]*)"', a)
                    if m:
                        pathattr = m.group(1)
                rvis = "pub" if vis == "pub" else vis
                if toks[i + 2][0] == "{":
                    end = group_end(toks, i + 2)
                    child = Mod(node.crate, name, node, rvis, path, (toks[i + 2][1] + 1, toks[end][1]))
                    self.nodes_by_file[path].append(child)
                    node.children.setdefault(name, []).append(child)
                    self.items(child, toks, i + 3, text, path, filedir, False, norm(base + "/" + (pathattr or name)))
                    i = end + 1
                else:
                    node.children.setdefault(name, [])
                    if pathattr is None:
                        node.children[name].append(Mod(node.crate, name, node, rvis, path, (0, 0)))
                    else:
                        rel = norm((filedir if in_file else base) + "/" + pathattr)
                        node.children[name].append(self.load(node.crate, rel, name, node, rvis))
                    i += 3
                continue
            if word == "use":
                end = i
                while toks[end][0] != ";":
                    end += 1
                leaves, _ = parse_tree(toks[:end], i + 1, [])
                for segs, alias, glob in leaves:
                    node.uses.append({"vis": vis, "path": segs, "binding": alias or segs[-1], "glob": glob, "id": (path, toks[i][1], len(node.uses), node.id)})
                i = end + 1
                continue
            if word == "macro_rules" and toks[i + 1][0] == "!":
                name = toks[i + 2][0]
                end = group_end(toks, i + 3)
                body = text[toks[i + 3][1]:toks[end][1] + 1]
                self.macros[(path, name)] = re.findall(r"\b(?:static|const|fn|struct|enum|type|trait|mod)\s+([A-Za-z_]\w*)", body)
                node.items[name] = ""
                i = end + 1
                continue
            if word in DECL:
                node.items[toks[i + 1][0]] = vis
                i = skip(toks, i + 2)
                continue
            if re.match(r"[A-Za-z_]\w*$", word) and toks[i + 1][0] == "!" and toks[i + 2][0] in "({[":
                end = group_end(toks, i + 2)
                body = text[toks[i][1]:toks[end][1] + 1]
                node.opaque = True
                for t in re.findall(r"[A-Za-z_]\w*", text[toks[i + 2][1]:toks[end][1] + 1]):
                    if re.match(r"[A-Z][A-Za-z0-9_]*$", t) and t not in PRELUDE:
                        node.weak.add(t)
                for d in self.macros.get((path, word), []):
                    node.weak.add(d)
                i = end + 1
                if i < len(toks) and toks[i][0] == ";":
                    i += 1
                continue
            i += 1
        return i


def skip(toks, i):
    while i < len(toks):
        if toks[i][0] == ";":
            return i + 1
        if toks[i][0] in "({[":
            end = group_end(toks, i)
            if toks[i][0] == "{":
                return end + 1
            i = end + 1
            continue
        i += 1
    return i


def posix_dir(p):
    return p.rsplit("/", 1)[0] if "/" in p else "."


def norm(p):
    out = []
    for part in p.split("/"):
        if part in ("", "."):
            continue
        if part == "..":
            out.pop()
        else:
            out.append(part)
    return "/".join(out)


def modpath(n):
    names = []
    c = n
    while c.parent is not None:
        names.insert(0, c.name)
        c = c.parent
    return "::".join([n.crate] + names)


def rep_text(r):
    return "::".join([r[0]] + list(r[1])) + (" cut" if r[2] else "")


def show(h):
    kind, node, name, rep = h
    base = "module " + modpath(node) if kind == "module" else "item " + modpath(node) + "::" + name
    return base + (" => " + rep_text(rep) if rep else "")


def merge_rep(inner, outer, seg):
    if inner:
        return (inner[0], inner[1], True) if outer and outer[2] and not inner[2] else inner
    return (outer[0], tuple(outer[1]) + (seg,), outer[2]) if outer else None


class Resolver:
    def __init__(self, roots, crates, cut, api):
        self.roots, self.crates, self.cut, self.api, self.ign = roots, crates, set(cut), set(api), False

    def visible(self, vis, owner, frm):
        return vis == "pub" or self.ign or frm.crate == owner.crate

    def member(self, node, name, frm, stack=frozenset()):
        key = (node.id, name, frm.crate == node.crate, self.ign)
        if key in stack:
            return []
        stack = stack | {key}
        out = []
        for c in node.children.get(name, []):
            if self.visible(c.vis, node, frm):
                out.append(("module", c, None, None))
        if name in node.items and self.visible(node.items[name], node, frm):
            out.append(("item", node, name, None))
        for u in node.uses:
            if u["glob"] or u["binding"] != name or not self.visible(u["vis"], node, frm):
                continue
            hits = self.leaf(node, u, stack)
            if not hits and self.external(node, u):
                out.append(("item", node, name, None))
                continue
            cut = "::".join(u["path"]) in self.cut
            api = "::".join(u["path"]) in self.api
            for h in hits:
                if api:
                    out.append((h[0], h[1], h[2], None))
                elif cut and h[3]:
                    out.append((h[0], h[1], h[2], (h[3][0], h[3][1], True)))
                else:
                    out.append(h)
        shadowed = {h[0] for h in out}
        for u in node.uses:
            if not u["glob"] or not self.visible(u["vis"], node, frm):
                continue
            cut = "::".join(u["path"]) in self.cut
            for raw in self.leaf(node, u, stack):
                if raw[0] != "module":
                    continue
                target = (raw[0], raw[1], raw[2], (raw[3][0], raw[3][1], True) if cut and raw[3] else raw[3])
                for sub in self.member(target[1], name, frm, stack):
                    if sub[0] not in shadowed:
                        out.append((sub[0], sub[1], sub[2], merge_rep(sub[3], target[3], name)))
        if not out and name in node.weak:
            out.append(("item", node, name, None))
        return out

    def external(self, node, u):
        head = u["path"][0]
        return head not in ("crate", "self", "super") and head not in self.crates and head not in node.children and head not in node.items and not any(o["binding"] == head for o in node.uses)

    def leaf(self, node, u, stack):
        key = ("leaf", u["id"])
        if key in stack:
            return []
        res = self.path(node, u["path"], stack | {key})
        if not res:
            return []
        hits, consumed = res
        if consumed == len(u["path"]):
            return hits
        rest = u["path"][consumed:]
        return [(h[0], h[1], h[2], (h[3][0], tuple(h[3][1]) + tuple(rest), h[3][2]) if h[3] else None) for h in hits if h[0] == "item"]

    def path(self, node, path, stack=frozenset()):
        head = path[0]
        consumed = 1
        if head == "crate":
            hits = [("module", self.roots[node.crate], None, None)]
        elif head == "self":
            hits = [("module", node, None, None)]
        elif head == "super":
            cur, k = node, 0
            while k < len(path) and path[k] == "super" and cur is not None:
                cur, k = cur.parent, k + 1
            consumed = k
            hits = [("module", cur, None, None)] if cur is not None else []
        else:
            hits = self.member(node, head, node, stack)
            if not hits and head in self.crates:
                hits = [("module", self.roots[head], None, None if head == node.crate else (head, (), False))]
        if not hits:
            return None
        while consumed < len(path):
            nxt = []
            for h in hits:
                if h[0] != "module":
                    continue
                for sub in self.member(h[1], path[consumed], node, stack):
                    nxt.append((sub[0], sub[1], sub[2], merge_rep(sub[3], h[3], path[consumed])))
            if not nxt:
                break
            hits, consumed = nxt, consumed + 1
        return hits, consumed

    def opaque(self, node, seen=frozenset()):
        if node.id in seen:
            return False
        if node.opaque:
            return True
        for u in node.uses:
            if u["glob"] and any(h[0] == "module" and self.opaque(h[1], seen | {node.id}) for h in self.leaf(node, u, frozenset())):
                return True
        return False


def addr(roots, ref):
    parts = ref.split("::")
    n = roots[parts[0]]
    for p in parts[1:]:
        n = n.children[p][0]
    return n


def innermost(nodes, offset):
    best = None
    for n in nodes:
        if n.region[0] <= offset <= n.region[1] and (best is None or n.region[1] - n.region[0] <= best.region[1] - best.region[0]):
            best = n
    return best


for case in corpus["cases"]:
    idents = {ident(c["name"]) for c in case["crates"]}
    parser = Parser(case["files"], idents)
    roots = {}
    for c in case["crates"]:
        roots[ident(c["name"])] = parser.load(ident(c["name"]), c["root"], "", None, "pub")
    res = Resolver(roots, idents, case["cut"], case["api"])
    for q in case["queries"]:
        label = case["name"] + " / " + q["op"] + " " + json.dumps(q["path"] or q["name"] or q["needle"])
        if q["op"] == "path":
            r = res.path(addr(roots, q["at"]), q["path"])
            got = None if r is None else {"consumed": r[1], "hits": sorted(show(h) for h in r[0])}
            want = q["expect"] and {"consumed": q["expect"]["consumed"], "hits": sorted(q["expect"]["hits"])}
            ok = got == want
        elif q["op"] == "member":
            res.ign = q["ignoreVisibility"]
            node = addr(roots, q["at"])
            got = sorted(show(h) for h in res.member(node, q["name"], addr(roots, q["from"] or q["at"])))
            res.ign = False
            ok = got == sorted(q["expectHits"])
        elif q["op"] == "opaque":
            got = res.opaque(addr(roots, q["at"]))
            ok = got == q["expectFlag"]
        else:
            text = case["files"][q["file"]]
            node = innermost(parser.nodes_by_file[q["file"]], text.index(q["needle"]))
            got = modpath(node)
            ok = got == q["expectNode"]
        if not ok:
            failures.append({"case": label, "got": got})

for row in corpus["merge"]:
    t = lambda r: None if r is None else (r["ident"], tuple(r["segs"]), r["cut"])
    got = merge_rep(t(row["inner"]), t(row["outer"]), row["segment"])
    if got != t(row["expected"]):
        failures.append({"merge": row["name"], "got": got})
for row in corpus["texts"]:
    got = "::".join([row["rep"]["ident"]] + row["rep"]["segs"])
    if got != row["expected"].replace(" cut", ""):
        failures.append({"text": row["expected"], "got": got})
for row in corpus["keywords"]:
    if (row["head"] in ("crate", "self", "super")) != row["expected"]:
        failures.append({"keyword": row["head"]})

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)
