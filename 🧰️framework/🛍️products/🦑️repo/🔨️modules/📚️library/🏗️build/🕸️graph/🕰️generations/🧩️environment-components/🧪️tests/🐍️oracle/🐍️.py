#!/usr/bin/env python3
"""🐍️ H102 oracle: independent, naive implementation of the owner edge record validity rule, plus a ground-truth toy edge function. Writes worlds.json for check.mjs.
Usage: python3 oracle.py <out.json> [seed] [worlds]"""
import hashlib, json, random, sys

def H(v): return hashlib.sha256(json.dumps(v, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()
def within(root, path): return root == "." or path == root or path.startswith(root + "/")
def related(a, b): return within(a, b) or within(b, a)
def dir_of(p): return p[:p.rfind("/")] if "/" in p else "."

class Env:
    def __init__(s, parts):
        s.p = parts
        s.rows = {r[0]: r for r in parts["owners"]}
        s.entries = []
        for name, root, native in parts["owners"]:
            s.entries.append((root, name))
            if native is not None and native != root: s.entries.append((native, name))
        s.ws = [(dir_of(path), d) for path, d in parts["workspaces"]]
    def row_digest(s, n): return H(s.rows[n]) if n in s.rows else "-"
    def ent(s, pred):
        sel = sorted((l, n) for l, n in s.entries if pred(l))
        return H([[l, n, s.row_digest(n)] for l, n in sel]) if sel else "-"
    def digest(s, key):
        if key == "T": return str(s.p["taxonomy"])
        if key == "L": return "-" if s.p.get("lock") is None else str(s.p["lock"])
        k, a = key[0], key[2:]
        if k == "o": return s.row_digest(a)
        if k == "t": return s.ent(lambda l: l != a and within(a, l))
        if k == "c": return s.ent(lambda l: related(l, a))
        if k in "pm":
            vs = sorted(v for kk, v in s.p["packages" if k == "p" else "modules"] if kk == a)
            return H(vs) if vs else "-"
        if k == "w":
            vs = sorted(d for dd, d in s.ws if dd == a); return H(vs) if vs else "-"
        if k == "W":
            vs = sorted([dd, d] for dd, d in s.ws if related(dd, a)); return H(vs) if vs else "-"
        raise ValueError(key)
    def table(s): return H([s.p["taxonomy"], s.p.get("lock"), s.p["owners"], s.p["packages"], s.p["modules"], s.p["workspaces"]])

def toy_edges(spec, env):
    """Ground truth: what the owner's edges are under env, and what it read while computing them."""
    name = spec["name"]; row = env.rows[name]; roots = [r for r in (row[1], row[2]) if r]
    reads = {"owners": set(), "paths": set(), "packages": set(), "modules": set(), "workspaces": set(), "lock": False}
    def terr(root): return sorted((l, n) for l, n in env.entries if l != root and within(root, l))
    out = {"self": list(row), "taxonomy": env.p["taxonomy"], "terr": [terr(r) for r in roots]}
    res = []
    for t in spec["owners"]:
        reads["owners"].add(t); r = env.rows.get(t)
        res.append([t, list(r) if r else None, [terr(x) for x in (r[1], r[2]) if x] if r else None])
    out["owners"] = res
    miss = []
    for p in spec["miss"]:
        reads["paths"].add(p)
        cands = [(l, n) for l, n in env.entries if within(l, p)]
        miss.append([p, max(cands, key=lambda x: (len(x[0]) if x[0] != "." else 0, x[0]))[1] if cands else None])
    out["miss"] = miss
    out["pk"] = []
    for k in spec["packages"]:
        reads["packages"].add(k); out["pk"].append([k, sorted(v for kk, v in env.p["packages"] if kk == k)])
    out["md"] = []
    for k in spec["modules"]:
        reads["modules"].add(k); out["md"].append([k, sorted(v for kk, v in env.p["modules"] if kk == k)])
    if spec["cargo"]:
        reads["workspaces"].add(roots[0])
        cands = [(d, g) for d, g in env.ws if within(d, roots[0])]
        out["ws"] = max(cands, key=lambda x: (len(x[0]) if x[0] != "." else 0, x[0])) if cands else None
    if spec["lock"]:
        reads["lock"] = True; out["lock"] = env.p.get("lock")
    return out, {k: (sorted(v) if isinstance(v, set) else v) for k, v in reads.items()}

def keys_of(row, reads, env):
    keys = {"T", "o|" + row[0]}; roots = [r for r in (row[1], row[2]) if r]
    for r in roots: keys.add("t|" + r)
    for t in reads["owners"]:
        keys.add("o|" + t)
        if t in env.rows:
            for r in env.rows[t][1:]:
                if r: keys.add("t|" + r)
    for p in reads["paths"]: keys.add("c|" + p)
    for k in reads["packages"]: keys.add("p|" + k)
    for k in reads["modules"]: keys.add("m|" + k)
    for d in reads["workspaces"]: keys.add("w|" + d)
    if reads["workspaces"]:
        for r in roots: keys.add("W|" + r)
    if reads["lock"]: keys.add("L")
    return sorted(keys, key=lambda s: [ord(c) for c in s])

def reads_digest(keys, env): return H([[k, env.digest(k)] for k in keys])

def make_world(rng, n_owners=40, n_mut=6):
    dirs = ["."] + ["d%d" % i for i in range(6)]
    dirs += ["%s/s%d" % (d, i) for d in dirs[1:] for i in range(3)]
    owners = []
    roots = rng.sample(dirs[1:], min(n_owners, len(dirs) - 1)) if n_owners <= len(dirs) - 1 else dirs[1:]
    for i in range(n_owners):
        root = roots[i] if i < len(roots) else "%s/o%d" % (rng.choice(dirs), i)
        owners.append(["o%d" % i, root, rng.choice([None, None, "n/" + root.replace("/", "_")])])
    pk = [["pk%d" % i, owners[rng.randrange(n_owners)][0]] for i in range(10)]
    md = [["md%d" % i, owners[rng.randrange(n_owners)][0]] for i in range(6)]
    ws = [[dirs[rng.randrange(len(dirs))].replace(".", "") + "/Cargo.toml" if rng.random() < .9 else "Cargo.toml", H(["ws", i])] for i in range(4)]
    ws = [[p.lstrip("/"), d] for p, d in ws]
    parts = {"owners": owners, "packages": pk, "modules": md, "workspaces": ws, "lock": H("lock0"), "taxonomy": H("tax0")}
    specs = []
    for o in owners:
        specs.append({"name": o[0], "owners": [x[0] for x in rng.sample(owners, rng.randint(0, 4))], "miss": rng.sample(dirs, rng.randint(0, 2)),
                      "packages": ["pk%d" % rng.randrange(10) for _ in range(rng.randint(0, 2))], "modules": ["md%d" % rng.randrange(6) for _ in range(rng.randint(0, 1))],
                      "cargo": rng.random() < .25, "lock": rng.random() < .15})
    muts = []
    new = json.loads(json.dumps(parts))
    for _ in range(n_mut):
        r = rng.random(); a = new["owners"]
        if r < .2 and len(a) > 5:
            gone = a.pop(rng.randrange(len(a))); muts.append(["remove", gone[0]])
        elif r < .35:
            a.append(["x%d" % rng.randrange(10**6), rng.choice(dirs) + "/new%d" % rng.randrange(99), None]); muts.append(["add", a[-1][0]])
        elif r < .5:
            i = rng.randrange(len(a)); a[i] = [a[i][0], rng.choice(dirs), a[i][2]]; muts.append(["move", a[i][0]])
        elif r < .65:
            i = rng.randrange(len(new["packages"])); new["packages"][i][1] = rng.choice(a)[0]; muts.append(["package", new["packages"][i][0]])
        elif r < .72:
            i = rng.randrange(len(new["modules"])); new["modules"][i][1] = rng.choice(a)[0]; muts.append(["module", new["modules"][i][0]])
        elif r < .82:
            new["workspaces"].append([rng.choice(dirs).replace(".", "") .lstrip("/") + "/Cargo.toml", H(["w", rng.random()])]); muts.append(["workspace-add"])
        elif r < .9:
            new["workspaces"][rng.randrange(len(new["workspaces"]))][1] = H(["w", rng.random()]); muts.append(["workspace-edit"])
        elif r < .97:
            new["lock"] = H(["lock", rng.random()]); muts.append(["lock"])
        else:
            new["taxonomy"] = H(["tax", rng.random()]); muts.append(["taxonomy"])
    return parts, new, specs, muts

def run(seed, worlds):
    rng = random.Random(seed); out = []
    for w in range(worlds):
        e1p, e2p, specs, muts = make_world(rng, 40, rng.choice([1, 1, 2, 3, 6]))
        e1, e2 = Env(e1p), Env(e2p); recs = []; truth = []; flagged = []
        for s in specs:
            if s["name"] not in e1.rows: continue
            edges1, reads = toy_edges(s, e1)
            keys = keys_of(e1.rows[s["name"]], reads, e1)
            rd = reads_digest(keys, e1)
            recs.append({"name": s["name"], "keys": keys, "readsDigest": rd, "reads": reads})
            if s["name"] not in e2.rows: truth.append(s["name"]); flagged.append(s["name"]); continue
            edges2, _ = toy_edges(s, e2)
            if edges1 != edges2: truth.append(s["name"])
            if reads_digest(keys, e2) != rd: flagged.append(s["name"])
        out.append({"env1": e1p, "env2": e2p, "records": recs, "mutations": muts, "truth": sorted(truth), "flagged": sorted(flagged),
                    "table1": e1.table(), "table2": e2.table()})
    return out

if __name__ == "__main__":
    path = sys.argv[1]; seed = int(sys.argv[2]) if len(sys.argv) > 2 else 102; worlds = int(sys.argv[3]) if len(sys.argv) > 3 else 300
    data = run(seed, worlds)
    miss = sum(1 for w in data for n in w["truth"] if n not in w["flagged"])
    print(json.dumps({"worlds": len(data), "records": sum(len(w["records"]) for w in data), "truth": sum(len(w["truth"]) for w in data), "flagged": sum(len(w["flagged"]) for w in data), "unsound": miss}))
    json.dump(data, open(path, "w"))
