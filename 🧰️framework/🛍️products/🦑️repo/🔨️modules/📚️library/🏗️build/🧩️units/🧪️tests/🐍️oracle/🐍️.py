#!/usr/bin/env python3
"""🔬️ Independent oracle for the polyglot unit model, written from the schema texts (policy `key.text`, `python.api`) with the standard library only (json, hashlib, ast, tokenize).
usage: 🐍️.py graph|keys|scenarios   < fixture.json     -> JSON answers per case
       🐍️.py python < fixture.json                      -> ast answers for the python analysis cases
       🐍️.py lexer FILE...                              -> tokenize answers (token texts) per file
       🐍️.py corpus OUTDIR FILE...                      -> writes mutated copies and ast answers (manifest.json) for the stdlib corpus test"""
import ast, hashlib, io, json, os, sys, tokenize


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def order_key(text):
    return [ord(c) for c in text]


def code_point_sorted(values):
    return sorted(values, key=order_key)


def unit_target_name(unit_id):
    import re, unicodedata
    slug = re.sub(r"[^A-Za-z0-9]+", "-", unicodedata.normalize("NFKD", unit_id)).strip("-").lower()[:48]
    return "unit-%s-%s" % (slug or "x", sha(unit_id)[:8])


# ---------------------------------------------------------------- graph laws, order

def graph_failures(graph, kinds):
    units = graph["units"]
    ids = [u["id"] for u in units]
    failures = []
    if len(set(ids)) != len(ids):
        failures.append("duplicate")
    by_id = {u["id"]: u for u in units}
    owner = {}
    for u in units:
        if not u["id"].startswith(u["language"] + ":"):
            failures.append("prefix")
        if u["interface"] and kinds is not None:
            kind = kinds.get(u["interface"]["kind"])
            if kind is None or kind["language"] != u["language"]:
                failures.append("kind")
        for path in u["sources"]:
            if path in owner and owner[path] != u["id"]:
                failures.append("shared-source")
            owner[path] = u["id"]
    for u in units:
        seen = set()
        for d in u["dependencies"]:
            t = by_id.get(d["unit"])
            if t is None:
                failures.append("unknown-dependency")
                continue
            if d["unit"] == u["id"]:
                failures.append("self")
            if d["unit"] in seen:
                failures.append("duplicate-dependency")
            seen.add(d["unit"])
            if d["use"] in ("interface", "both") and not t["interface"]:
                failures.append("no-interface")
    if not failures:
        state = {}
        def visit(i, trail):
            if state.get(i) == 2:
                return
            if i in trail:
                failures.append("cycle")
                return
            trail.add(i)
            for d in by_id[i]["dependencies"]:
                visit(d["unit"], trail)
            trail.discard(i)
            state[i] = 2
        for i in ids:
            visit(i, set())
    return failures


def layers_and_order(graph):
    by_id = {u["id"]: u for u in graph["units"]}
    layer = {}
    def height(i):
        if i not in layer:
            layer[i] = max([height(d["unit"]) + 1 for d in by_id[i]["dependencies"]], default=0)
        return layer[i]
    for i in by_id:
        height(i)
    order = sorted(code_point_sorted(by_id), key=lambda i: layer[i])
    return layer, order


def answer_graph(case, kinds):
    failures = graph_failures(case["graph"], kinds)
    if failures:
        return {"valid": False, "kinds": sorted(set(failures))}
    layer, order = layers_and_order(case["graph"])
    return {"valid": True, "order": order, "layer": layer}


# ---------------------------------------------------------------- keys and closures

def unit_key(u, digests, deps):
    dependencies = []
    for d in sorted(u["dependencies"], key=lambda d: order_key(d["unit"])):
        if d["use"] == "declared":
            digest = ""
        elif d["use"] == "interface":
            digest = deps[d["unit"]]["interfaceClosure"]
        elif d["use"] == "implementation":
            digest = deps[d["unit"]]["outputClosure"]
        else:
            digest = "%s|%s" % (deps[d["unit"]]["interfaceClosure"], deps[d["unit"]]["outputClosure"])
        dependencies.append([d["unit"], d["use"], digest])
    material = {
        "v": 1, "id": u["id"], "language": u["language"], "kind": u["kind"],
        "toolchain": {"id": u["toolchain"]["id"], "parts": u["toolchain"]["parts"]},
        "action": {"argv": u["action"]["argv"], "env": u["action"]["env"]},
        "interface": u["interface"]["kind"] if u["interface"] else None,
        "outputs": code_point_sorted(u["outputs"]),
        "sources": [[p, digests[p]] for p in code_point_sorted(u["sources"])],
        "inputs": [[p, digests[p]] for p in code_point_sorted(u["inputs"])],
        "externals": code_point_sorted(u["externals"]),
        "dependencies": dependencies,
    }
    return sha(canon(material))


def interface_closure(u, own, deps, kinds, exposes=None):
    if own is None:
        return None
    direct = kinds is not None and u["interface"] and kinds[u["interface"]["kind"]]["closure"] == "direct"
    rows = [] if direct else sorted(([d["unit"], deps[d["unit"]]["interfaceClosure"]] for d in u["dependencies"] if d["use"] in ("interface", "both") and (exposes is None or d["unit"] in exposes)), key=lambda r: order_key(r[0]))
    return sha(canon({"i": own, "d": rows}))


def output_closure(u, own, deps):
    rows = sorted(([d["unit"], deps[d["unit"]]["outputClosure"]] for d in u["dependencies"] if d["use"] != "declared"), key=lambda r: order_key(r[0]))
    return sha(canon({"o": own, "d": rows}))


def answer_keys(case, kinds):
    u = case["unit"]
    return {"key": unit_key(u, case["digests"], case["dependencies"]),
            "interfaceClosure": interface_closure(u, case.get("interface"), case["dependencies"], kinds, case.get("exposes")),
            "outputClosure": output_closure(u, case["outputs"], case["dependencies"])}


# ---------------------------------------------------------------- early-cutoff scenarios (fake executor defined by the fixture text)

def run_scenarios(case, kinds):
    graph = case["graph"]
    by_id = {u["id"]: u for u in graph["units"]}
    layer, order = layers_and_order(graph)
    state = {i: {"body": 0, "iface": 0, "input": 0, "broken": False, "hard": False} for i in by_id}
    initial = json.loads(json.dumps(state))
    store, latest, answers = {}, {}, []
    for step in case["steps"]:
        for edit in step["edits"]:
            s, effect = state[edit["unit"]], edit["effect"]
            if effect == "body":
                s["body"] += 1
            elif effect == "interface":
                s["body"] += 1
                s["iface"] += 1
            elif effect == "input":
                s["input"] += 1
            elif effect == "fail":
                s["body"] += 1
                s["broken"] = True
            elif effect == "fail-hard":
                s["body"] += 1
                s["broken"] = True
                s["hard"] = True
            elif effect == "fix":
                s["body"] += 1
                s["broken"] = False
                s["hard"] = False
            elif effect == "revert":
                state[edit["unit"]] = json.loads(json.dumps(initial[edit["unit"]]))
        digests = {}
        for i, u in by_id.items():
            s = state[i]
            for p in u["sources"]:
                digests[p] = sha("%s#%d.%d" % (p, s["body"], s["iface"]))
            for p in u["inputs"]:
                digests[p] = sha("%s#%d" % (p, s["input"]))
        result = {"ran": [], "hit": [], "failed": [], "blocked": [], "cutoff": []}
        facts = {}
        for i in order:
            u = by_id[i]
            blocked = False
            for d in u["dependencies"]:
                if d["use"] == "declared":
                    continue
                f = facts.get(d["unit"])
                if f is None or (not f["ok"] and (d["use"] in ("implementation", "both") or f["interfaceClosure"] is None)):
                    blocked = True
            if blocked:
                result["blocked"].append(i)
                continue
            dep_facts = {d["unit"]: facts[d["unit"]] for d in u["dependencies"] if d["use"] != "declared"}
            key = unit_key(u, digests, dep_facts)
            receipt = store.get(key)
            status = "hit"
            cutoff = False
            if receipt is None:
                s = state[i]
                ok = not s["broken"]
                outputs = sha("o|%s|%d|%d|%s" % (i, s["body"], s["iface"], s["broken"]))
                own = None
                if u["interface"] and not s["hard"]:
                    own = sha("i|%s|%d" % (i, s["iface"]))
                receipt = {"ok": ok, "interface": own, "outputs": outputs, "exposes": case.get("exposes", {}).get(i)}
                store[key] = receipt
                status = "ran" if ok else "failed"
            elif not receipt["ok"]:
                status = "failed"
            current = {"ok": receipt["ok"], "interfaceClosure": interface_closure(u, receipt["interface"], dep_facts, kinds, receipt["exposes"]), "outputClosure": output_closure(u, receipt["outputs"], dep_facts)}
            if status == "ran":
                before = latest.get(i)
                cutoff = before is not None and current["interfaceClosure"] is not None and before["interfaceClosure"] == current["interfaceClosure"]
            latest[i] = {"interfaceClosure": current["interfaceClosure"], "outputClosure": current["outputClosure"]}
            facts[i] = current
            result[status].append(i)
            if cutoff:
                result["cutoff"].append(i)
        answers.append({k: sorted(v) for k, v in result.items()})
    return answers


# ---------------------------------------------------------------- python analysis through ast

KEYWORD_DYNAMIC = {"globals", "vars", "setattr", "exec", "__dict__", "__getattr__", "locals"}


def is_public(name, exported):
    return name in exported or not name.startswith("_") or (name.startswith("__") and name.endswith("__") and len(name) > 4)


def target_names(node):
    if isinstance(node, ast.Name):
        return [node.id]
    if isinstance(node, (ast.Tuple, ast.List)):
        return [n for e in node.elts for n in target_names(e)]
    if isinstance(node, ast.Starred):
        return target_names(node.value)
    return []


def imports_of(tree):
    rows = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            rows.append(["", 0, [a.name for a in node.names]])
        elif isinstance(node, ast.ImportFrom):
            rows.append([node.module or "", node.level, [a.name for a in node.names]])
    return sorted(rows, key=lambda r: (r[0], r[1], r[2]))


def children(stmt):
    """🪜️ Statements nested directly under a compound statement, with the header entry."""
    if isinstance(stmt, ast.If):
        return [("if", [stmt.test], stmt.body), ("else", [], stmt.orelse)]
    if isinstance(stmt, (ast.For, ast.AsyncFor)):
        return [("for", [stmt.target, stmt.iter], stmt.body), ("else", [], stmt.orelse)]
    if isinstance(stmt, ast.While):
        return [("while", [stmt.test], stmt.body), ("else", [], stmt.orelse)]
    if isinstance(stmt, (ast.With, ast.AsyncWith)):
        return [("with", [i for i in stmt.items], stmt.body)]
    if isinstance(stmt, ast.Try):
        parts = [("try", [], stmt.body)]
        for h in stmt.handlers:
            parts.append(("except", [h.type, h.name], h.body))
        parts += [("else", [], stmt.orelse), ("finally", [], stmt.finalbody)]
        return parts
    return None


def dump(node):
    if node is None:
        return None
    if isinstance(node, str):
        return node
    if isinstance(node, list):
        return [dump(n) for n in node]
    return ast.dump(node, annotate_fields=True, include_attributes=False)


def function_bodies_with_global(tree):
    names = set()
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            for inner in ast.walk(node):
                if isinstance(inner, ast.Global):
                    names.update(inner.names)
    return names


def analyze_ast(text, init, unannotated="ignore"):
    tree = ast.parse(text)
    exported = set()
    for node in tree.body:
        value = None
        if isinstance(node, (ast.Assign, ast.AugAssign, ast.AnnAssign)):
            tg = node.targets if isinstance(node, ast.Assign) else [node.target]
            if any(isinstance(t, ast.Name) and t.id == "__all__" for t in tg):
                value = node.value
        elif isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and isinstance(node.value.func, ast.Attribute) and isinstance(node.value.func.value, ast.Name) and node.value.func.value.id == "__all__":
            value = node.value
        if value is not None:
            for sub in ast.walk(value):
                if isinstance(sub, ast.Constant) and isinstance(sub.value, str):
                    exported.add(sub.value)
    bound = set()
    dynamic = False
    entries = []

    def import_entry(node, depth):
        rows = []
        for a in node.names:
            if isinstance(node, ast.Import):
                name = a.asname or a.name.split(".")[0]
                redundant = a.asname is not None and a.asname == a.name
            else:
                name = a.asname or a.name
                redundant = a.name == "*" or (a.asname is not None and a.asname == a.name)
            rows.append((name, redundant))
        return rows

    def walk(body, depth, in_class, module_level):
        nonlocal dynamic
        for stmt in body:
            if isinstance(stmt, (ast.FunctionDef, ast.AsyncFunctionDef)):
                if module_level:
                    bound.add(stmt.name)
                if is_public(stmt.name, exported):
                    entry = ["def", isinstance(stmt, ast.AsyncFunctionDef), stmt.name, dump(stmt.args), dump(stmt.returns), dump(stmt.decorator_list), depth]
                    if unannotated == "body" and stmt.returns is None:
                        entry.append(dump(stmt.body))
                    entries.append(entry)
            elif isinstance(stmt, ast.ClassDef):
                if module_level:
                    bound.add(stmt.name)
                if is_public(stmt.name, exported):
                    entries.append(["class", stmt.name, dump(stmt.bases), dump(stmt.keywords), dump(stmt.decorator_list), depth])
                    walk(stmt.body, depth + 1, True, False)
            elif children(stmt) is not None:
                if module_level and isinstance(stmt, (ast.For, ast.AsyncFor)):
                    bound.update(target_names(stmt.target))
                if module_level and isinstance(stmt, (ast.With, ast.AsyncWith)):
                    for item in stmt.items:
                        if item.optional_vars is not None:
                            bound.update(target_names(item.optional_vars))
                for header, parts, inner in children(stmt):
                    if header in ("else", "finally") and not inner:
                        continue
                    entries.append(["compound", header, dump(parts), depth])
                    walk(inner, depth + 1, in_class, module_level)
            elif isinstance(stmt, (ast.Import, ast.ImportFrom)):
                rows = import_entry(stmt, depth)
                if module_level:
                    for name, _ in rows:
                        if name == "*":
                            dynamic = True
                        else:
                            bound.add(name)
                if any(r or n in exported or n == "*" for n, r in rows) or init:
                    entries.append(["import", dump(stmt), depth])
            elif isinstance(stmt, ast.Assign):
                names = [n for t in stmt.targets for n in target_names(t)]
                if module_level:
                    bound.update(names)
                if any(is_public(n, exported) for n in names):
                    entries.append(["assign", dump(stmt.targets), dump(stmt.value), depth])
            elif isinstance(stmt, ast.AnnAssign):
                if isinstance(stmt.target, ast.Name):
                    name = stmt.target.id
                    if module_level:
                        bound.add(name)
                    if is_public(name, exported):
                        final = "Final" in ast.dump(stmt.annotation)
                        entries.append(["ann", name, dump(stmt.annotation), dump(stmt.value) if (in_class or final) else None, depth])
            elif isinstance(stmt, ast.AugAssign):
                if isinstance(stmt.target, ast.Name) and stmt.target.id == "__all__":
                    entries.append(["aug", dump(stmt), depth])
            elif isinstance(stmt, ast.Expr):
                v = stmt.value
                if isinstance(v, ast.Call) and isinstance(v.func, ast.Attribute) and isinstance(v.func.value, ast.Name) and v.func.value.id == "__all__":
                    entries.append(["all-call", dump(stmt), depth])
                if module_level:
                    for sub in ast.walk(v):
                        if isinstance(sub, ast.NamedExpr):
                            bound.add(sub.target.id)
    walk(tree.body, 0, False, True)
    bound.update(function_bodies_with_global(tree))
    for tok in tokenize.generate_tokens(io.StringIO(text).readline):
        if tok.type == tokenize.NAME and tok.string in KEYWORD_DYNAMIC:
            dynamic = True
    return {"imports": imports_of(tree), "names": sorted(bound), "dynamic": dynamic, "fingerprint": sha(canon([entries, [] if dynamic else sorted(bound), dynamic]))}


# ---------------------------------------------------------------- corpus mutations

def mutate(text):
    """🧬️ A: a statement inserted into the first function body (API unchanged). B: trailing comments and blank lines (API unchanged). C: first parameter of the first public top-level function renamed (API changed)."""
    tree = ast.parse(text)
    lines = text.split("\n")
    a = None
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and len(node.body) >= 1:
            first = node.body[0]
            target = node.body[1] if (isinstance(first, ast.Expr) and isinstance(getattr(first, "value", None), ast.Constant) and isinstance(first.value.value, str) and len(node.body) > 1) else first
            if target.lineno > node.lineno and target.col_offset > 0 and not lines[target.lineno - 1][:target.col_offset].strip():
                out = list(lines)
                out.insert(target.lineno - 1, " " * target.col_offset + "_polyglot_probe = 0  # probe")
                candidate = "\n".join(out)
                try:
                    compile(candidate, "<probe>", "exec")
                    a = candidate
                    break
                except SyntaxError:
                    continue
    b = text + "\n\n# trailing comment\n\n\n"
    c = None
    for node in tree.body:
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and not node.name.startswith("_") and node.args.args:
            arg = node.args.args[0]
            line = lines[arg.lineno - 1]
            if line[arg.col_offset:arg.col_offset + len(arg.arg)] == arg.arg:
                out = list(lines)
                out[arg.lineno - 1] = line[:arg.col_offset] + arg.arg + "_x" + line[arg.col_offset + len(arg.arg):]
                c = "\n".join(out)
                try:
                    compile(c, "<rename>", "exec")
                except SyntaxError:
                    c = None
                break
    return a, b, c


def main():
    command = sys.argv[1]
    if command == "lexer":
        out = {}
        for path in sys.argv[2:]:
            with open(path, "rb") as handle:
                data = handle.read()
            tokens = []
            for tok in tokenize.tokenize(io.BytesIO(data).readline):
                if tok.type in (tokenize.NAME, tokenize.NUMBER, tokenize.STRING, tokenize.OP):
                    tokens.append(tok.string)
                elif tok.type == tokenize.ERRORTOKEN and tok.string.strip():
                    tokens.append(tok.string)
            out[path] = tokens
        json.dump(out, sys.stdout, ensure_ascii=False)
        return
    if command == "corpus":
        outdir = sys.argv[2]
        os.makedirs(outdir, exist_ok=True)
        manifest = []
        for index, path in enumerate(sys.argv[3:]):
            try:
                text = open(path, encoding="utf-8").read()
                init = os.path.basename(path) == "__init__.py"
                base = analyze_ast(text, init)
                a, b, c = mutate(text)
            except (SyntaxError, UnicodeDecodeError, RecursionError, ValueError):
                continue
            row = {"index": index, "path": path, "init": init, "imports": base["imports"], "names": base["names"], "dynamic": base["dynamic"], "mutations": {}}
            for name, mutated in (("A", a), ("B", b), ("C", c)):
                if mutated is None:
                    continue
                target = os.path.join(outdir, "%d.%s.py" % (index, name))
                with open(target, "w", encoding="utf-8") as handle:
                    handle.write(mutated)
                row["mutations"][name] = {"file": target, "same": analyze_ast(mutated, init)["fingerprint"] == base["fingerprint"]}
            with open(os.path.join(outdir, "%d.orig.py" % index), "w", encoding="utf-8") as handle:
                handle.write(text)
            manifest.append(row)
        json.dump(manifest, open(os.path.join(outdir, "manifest.json"), "w"), ensure_ascii=False)
        return
    fixture = json.load(sys.stdin)
    kinds = fixture.get("policy", {}).get("interfaceKinds")
    if command == "graph":
        out = {c["name"]: answer_graph(c, kinds) for c in fixture["graphs"]}
    elif command == "keys":
        out = {c["name"]: answer_keys(c, kinds) for c in fixture["keys"]}
    elif command == "scenarios":
        out = {c["name"]: run_scenarios(c, kinds) for c in fixture["scenarios"]}
    elif command == "names":
        out = {c["id"]: unit_target_name(c["id"]) for c in fixture["names"]}
    elif command == "python":
        out = {}
        for c in fixture["python"]["analyze"]:
            r = analyze_ast(c["source"], c.get("init", False), c.get("unannotatedReturns", "ignore"))
            out[c["name"]] = {"imports": r["imports"], "names": r["names"], "dynamic": r["dynamic"]}
        for c in fixture["python"]["pairs"]:
            a = analyze_ast(c["before"], c.get("init", False), c.get("unannotatedReturns", "ignore"))["fingerprint"]
            b = analyze_ast(c["after"], c.get("init", False), c.get("unannotatedReturns", "ignore"))["fingerprint"]
            out[c["name"]] = {"same": a == b}
    else:
        raise SystemExit("unknown command " + command)
    json.dump(out, sys.stdout, ensure_ascii=False)


if __name__ == "__main__":
    main()
