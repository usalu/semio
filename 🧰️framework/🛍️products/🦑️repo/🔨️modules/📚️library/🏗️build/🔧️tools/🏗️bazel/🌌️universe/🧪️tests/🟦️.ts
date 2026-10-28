/** 🌌️ Contract and oracles of the source universe: the fixture is the contract, an independent Python derivation and picomatch (third party) are the oracles, and the per-pattern roots equal the planner's own traversal roots. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import picomatch from "picomatch";
import {spawnSync} from "node:child_process";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {bazelMinimalRootsV1,bazelPatternRootV1,bazelPrunedNamesV1,bazelSourceUniverseV1,bazelUniversePrunesV1} from "../🟦️.ts";
import {bazelInputTraversalRootsV1} from "../../📜️script.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import inventory from "../../🧪️tests/📥️inventory/🔣️.json" with {type:"json"};

const ajv=new Ajv({strict:true,allowUnionTypes:true}),compile=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`}),groupsValid=compile("Groups"),universeValid=compile("Universe"),policy=schema.definitions.Policy.const;

test("the fixture agrees with the schema and the implementation gives every expected universe",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 for(const row of fixture.cases){expect(groupsValid(row.groups),row.name).toBe(true);expect(universeValid(row.expected),row.name).toBe(true);expect(bazelSourceUniverseV1(row.groups),row.name).toEqual(row.expected as never);}
});

test("per-pattern roots made minimal equal the planner's own traversal roots",()=>{
 for(const patterns of[...inventory.cases.map(row=>row.patterns),...fixture.cases.map(row=>row.groups.flat().filter(value=>!value.startsWith("!")))])expect(bazelMinimalRootsV1(patterns.map(bazelPatternRootV1)),JSON.stringify(patterns)).toEqual(bazelInputTraversalRootsV1(patterns));
});

test("picomatch (third party) excludes every probe below a pruned name and refuses the non-pruning negations",()=>{
 for(const row of fixture.cases)for(const group of row.groups){
  const negations=group.filter(value=>value.startsWith("!")).map(value=>value.slice(1)),pruned=bazelPrunedNamesV1(negations),excluded=(path:string)=>negations.some(pattern=>picomatch(pattern,{dot:true})(path));
  for(const name of[...pruned.names,...(pruned.dot?[policy.dotProbeName,".git-like"]:[])])for(const path of[`${name}/a.json`,`x/${name}/y/z.json`,`x/y/${name}/🧬️schema/s.json`])expect(excluded(path),row.name+": "+path).toBe(true);
  for(const name of pruned.names)expect(bazelUniversePrunesV1(pruned,name)).toBe(true);
 }
 for(const negation of fixture.probeRefusals){expect(bazelPrunedNamesV1([negation]),negation).toEqual({names:[],dot:false});const match=picomatch(negation,{dot:true});expect(["q/keep/p/r.ts","keep/r.ts"].some(path=>!match(path))||negation==="target/**",negation).toBe(true);}
});

const python=String.raw`
import json, re, sys
def root(p):
    m = re.search(r"[?*{\[]", p)
    pre = p if m is None else p[:m.start()]
    cut = pre if m is None or pre.endswith("/") else pre[:pre.rfind("/") + 1]
    r = cut.replace("\\", "/")
    r = r[2:] if r.startswith("./") else r
    r = r.rstrip("/")
    return "" if r == "." else r
def minimal(rs):
    s = sorted(set(rs))
    if s and s[0] == "": return [""]
    return [r for r in s if not any(r.startswith(o + "/") for o in s if o != r)]
def pruned(negs):
    names, dot = set(), False
    for n in negs:
        if not (n.startswith("**/") and n.endswith("/**")): continue
        body = n[3:-3]
        alts = body[1:-1].split(",") if body.startswith("{") and body.endswith("}") else [body]
        for a in alts:
            if a == ".*": dot = True
            elif a and not any(c in a for c in "{}*?[]/!,"): names.add(a)
    return names, dot
def universe(groups):
    roots, whole, unbounded, names, dot = [], False, False, None, True
    for g in groups:
        own = None
        for p in g:
            if p.startswith("!"): continue
            r = root(p)
            if r != "": roots.append(r); continue
            whole = True
            if own is None: own = pruned([v[1:] for v in g if v.startswith("!")])
            names = set(own[0]) if names is None else names & own[0]
            dot = dot and own[1]
        if own is not None and not own[0] and not own[1]: unbounded = True
    m = minimal(roots)
    if not whole: return {"roots": m, "whole": None}
    if unbounded or (not names and not dot): return {"roots": m, "whole": "unbounded"}
    return {"roots": m, "whole": {"names": sorted(names, key=lambda s: s.encode("utf-16-be")), "dot": dot}}
print(json.dumps([universe(c["groups"]) for c in json.load(sys.stdin)], ensure_ascii=False))
`;

test("an independent Python derivation agrees on every case",()=>{
 const run=spawnSync(testPythonV1()??"python3",["-I","-c",python],{input:JSON.stringify(fixture.cases),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout) as unknown[];
 for(const[index,row]of fixture.cases.entries()){expect(answers[index],row.name).toEqual(row.expected as never);expect(answers[index],row.name).toEqual(bazelSourceUniverseV1(row.groups) as never);}
});
