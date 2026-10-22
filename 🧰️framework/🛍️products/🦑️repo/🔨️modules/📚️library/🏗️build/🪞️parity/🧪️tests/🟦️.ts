/** 🪞️ Build parity: the fixture is the contract; Ajv, an independent Python implementation, Bun.Glob, diff and shasum are the oracles. */
import {test,expect} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {compareParityClosureV1,compareParityGraphV1,compareParityInputsV1,compareParityOutputsV1,compareParityResultsV1,compareParitySampleV1,globMatchV1,parityCompareV1,parityNormalizedDigestsV1,readParityRulesV1,readParitySamplesV1,resolveParityOutputPatternsV1,resolveParitySampleV1,verifyParityV1,type ParityObservationV1,type ParityRunnerV1} from "../🟦️.ts";
import {groupParityBazelSpawnsV1,parseParityBazelEventsV1,parseParityBazelExecutionLogV1,parseParityFailedTasksV1,parseParityNxInputsV1,parseParityPlanV1} from "../📥️evidence/🟦️.ts";
import {captureParityOutputsV1,createParityRunnerV1,recoverParityOutputsV1,setParityOutputsAsideV1} from "../🏃️collect/🟦️.ts";
import {readParityArgumentsV1,runBuildParityV1,selectParitySamplesV1} from "../⌨️command/🟦️.ts";
import {parityLanguagesV1,renderParityHelpV1,renderParityReportV1} from "../📝️report/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import samplesData from "../🔣️samples.json" with {type:"json"};
import rulesData from "../🔣️rules.json" with {type:"json"};
import help from "../🔣️help.json" with {type:"json"};

const ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});
const fixtureRules=readParityRulesV1(fixture.outputCases[0]!.rules),cases=fixture as unknown as Record<string,any>;
const applied=()=>new Map<string,number>(),plain=(map:Map<string,number>)=>Object.fromEntries([...map].sort());

const oracle=String.raw`
import json,re,sys,hashlib
def glob_re(g):
    s="";i=0
    while i<len(g):
        c=g[i]
        if c=="*":
            if g[i+1:i+2]=="*":
                if g[i+2:i+3]=="/": s+="(?:.*/)?";i+=3;continue
                s+=".*";i+=2;continue
            s+="[^/]*"
        elif c=="?": s+="[^/]"
        else: s+=re.escape(c)
        i+=1
    return re.compile("^"+s+"$",re.S)
def match(g,p): return glob_re(g).match(p) is not None
def key(plan,i):
    t=plan["tasks"].get(i)
    return i if t is None else t["target"]["project"]+":"+t["target"]["target"]+(":"+t["target"]["configuration"] if t["target"].get("configuration") else "")
def d(check,code,subject,nx=None,bazel=None):
    r={"check":check,"code":code,"subject":subject}
    if nx is not None: r["nx"]=nx
    if bazel is not None: r["bazel"]=bazel
    return r
def order(rows): return sorted(rows,key=lambda r:(r["code"],r["subject"]))
def graph(nx,bz):
    if nx is None or bz is None: return [d("graph","plan-unavailable","nx,bazel" if nx is None and bz is None else ("nx" if nx is None else "bazel"))]
    kn={key(nx,i):i for i in nx["tasks"]};kb={key(bz,i):i for i in bz["tasks"]};rows=[]
    for k in sorted(set(kn)-set(kb)): rows.append(d("graph","task-missing-bazel",k))
    for k in sorted(set(kb)-set(kn)): rows.append(d("graph","task-missing-nx",k))
    def edges(plan,names,field): return {k+" -> "+key(plan,x) for k,i in names.items() for x in (plan.get(field) or {}).get(i,[])}
    for field,only_nx,only_bz in (("dependencies","edge-missing-bazel","edge-missing-nx"),("continuousDependencies","continuous-edge-differs","continuous-edge-differs")):
        a=edges(nx,kn,field);b=edges(bz,kb,field)
        for e in sorted(a-b): rows.append(d("graph",only_nx,e,"present","absent"))
        for e in sorted(b-a): rows.append(d("graph",only_bz,e,"absent","present"))
    ra=sorted({key(nx,i) for i in nx["roots"]});rb=sorted({key(bz,i) for i in bz["roots"]})
    if ra!=rb: rows.append(d("graph","root-differs","roots",",".join(ra),",".join(rb)))
    return order(rows)
def closure(plan,c):
    if plan is None or c is None: return [d("closure","closure-unavailable","plan" if plan is None else "cquery")]
    planned={key(plan,i) for i in plan["tasks"]};q=set(c["queried"])
    return order([d("closure","closure-task-missing-in-query",k) for k in planned-q]+[d("closure","closure-task-extra-in-query",k) for k in q-planned])
def outputs(nx,bz,rules,mode="tools"):
    check="outputs" if mode=="tools" else "restore"
    names=dict(ml="output-missing-nx",mr="output-missing-bazel",kind="output-kind-differs",link="output-link-differs",mode="output-mode-differs",bytes="output-bytes-differ") if mode=="tools" else dict(ml="restore-missing",mr="restore-missing",kind="restore-kind-differs",link="restore-kind-differs",mode="restore-mode-differs",bytes="restore-bytes-differ")
    ign=[r for r in rules["rules"] if r["kind"]=="ignore-glob" and r["surface"]=="output"];mask=next((r for r in rules["rules"] if r["kind"]=="mode-mask"),None);rows=[];ap={}
    def bump(k): ap[k]=ap.get(k,0)+1
    for p in sorted(set(nx["files"])|set(bz["files"])):
        hit=next((r for r in ign if match(r["glob"],p)),None)
        if hit: bump(hit["id"]);continue
        a=nx["files"].get(p);b=bz["files"].get(p)
        if a is None: rows.append(d(check,names["ml"],p,"absent",b["kind"]));continue
        if b is None: rows.append(d(check,names["mr"],p,a["kind"],"absent"));continue
        if a["kind"]!=b["kind"]: rows.append(d(check,names["kind"],p,a["kind"],b["kind"]));continue
        if a["kind"]=="symlink":
            if a.get("link")!=b.get("link"): rows.append(d(check,names["link"],p,a.get("link"),b.get("link")))
            continue
        if a["kind"]!="file": continue
        bits=mask["mask"] if mask else 0o777
        if (a["mode"]&bits)!=(b["mode"]&bits): rows.append(d(check,names["mode"],p,format(a["mode"]&bits,"o"),format(b["mode"]&bits,"o")))
        elif mask and a["mode"]!=b["mode"]: bump(mask["id"])
        if a.get("sha256")==b.get("sha256"): continue
        na=a.get("normalized") or {};nb=b.get("normalized") or {}
        ex=next((k for k in na if k in nb and na[k]==nb[k]),None)
        if ex: bump(ex)
        else: rows.append(d(check,names["bytes"],p,"%s %s"%(a.get("size","?"),a["sha256"][:12]),"%s %s"%(b.get("size","?"),b["sha256"][:12])))
    if mode=="tools":
        for t in sorted(set(nx["roots"])|set(bz["roots"])):
            x=sorted(set(nx["roots"].get(t,[])));y=sorted(set(bz["roots"].get(t,[])))
            if x!=y: rows.append(d("outputs","output-root-differs",t,",".join(x),",".join(y)))
    return order(rows),ap
def results(nx,bz,expect):
    if nx is None or bz is None: return [d("results","run-unavailable","nx,bazel" if nx is None and bz is None else ("nx" if nx is None else "bazel"))]
    rows=[]
    if nx["exit"]!=bz["exit"]: rows.append(d("results","exit-differs","exit",str(nx["exit"]) if nx["exit"] is not None else "null",str(bz["exit"]) if bz["exit"] is not None else "null"))
    s=lambda v:"null" if v is None else str(v)
    if nx["exit"]!=expect: rows.append(d("results","exit-unexpected","nx expected %d"%expect,s(nx["exit"])))
    if bz["exit"]!=expect: rows.append(d("results","exit-unexpected","bazel expected %d"%expect,None,s(bz["exit"])))
    if nx["reason"]!=bz["reason"]: rows.append(d("results","reason-differs","reason",nx["reason"],bz["reason"]))
    if set(nx["failedTasks"])!=set(bz["failedTasks"]): rows.append(d("results","failed-tasks-differ","failedTasks",",".join(sorted(set(nx["failedTasks"]))),",".join(sorted(set(bz["failedTasks"])))))
    for t in sorted(set(nx["tests"])|set(bz["tests"])):
        a=nx["tests"].get(t,"no-status");b=bz["tests"].get(t,"no-status")
        if a!=b: rows.append(d("results","test-status-differs",t,a,b))
    return order(rows)
def inputs(nx,bz,rules):
    if nx is None or bz is None: return [d("inputs","inputs-unavailable","nx,bazel" if nx is None and bz is None else ("nx" if nx is None else "bazel"))],{}
    rs=rules["rules"];strip=[r for r in rs if r["kind"]=="strip-prefix"];inx=[r for r in rs if r["kind"]=="ignore-glob" and r["surface"]=="input-nx"];ibz=[r for r in rs if r["kind"]=="ignore-glob" and r["surface"]=="input-bazel"];amb=[r for r in rs if r["kind"]=="ambient-env"];cov=[r for r in rs if r["kind"]=="covered-by"];rows=[];ap={}
    def bump(k): ap[k]=ap.get(k,0)+1
    def ignored(lst,p):
        h=next((r for r in lst if match(r["glob"],p)),None)
        if h: bump(h["id"]);return True
        return False
    for t in sorted(set(nx)|set(bz)):
        a=nx.get(t);b=bz.get(t)
        if a is None or b is None: rows.append(d("inputs","inputs-unavailable",t,"present" if a else "absent","present" if b else "absent"));continue
        mapped=set()
        for p in b["inputs"]:
            if ignored(ibz,p): continue
            r=next((r for r in strip if p.startswith(r["prefix"])),None)
            if r: bump(r["id"])
            q=p[len(r["prefix"]):] if r else p
            if not ignored(ibz,q): mapped.add(q)
        files={p for p in a["files"] if not ignored(inx,p)}
        for p in sorted(files-mapped): rows.append(d("inputs","input-file-only-nx",t+" "+p))
        for p in sorted(mapped-files): rows.append(d("inputs","input-file-only-bazel",t+" "+p))
        for n in sorted(set(a["environment"])-set(b["env"])): rows.append(d("inputs","env-hashed-not-visible",t+" "+n))
        for n in sorted(set(b["env"])-set(a["environment"])):
            r=next((r for r in amb if any(match(x,n) for x in r["names"])),None)
            if r: bump(r["id"])
            else: rows.append(d("inputs","env-visible-not-hashed",t+" "+n))
        for cat,vals in (("external",a["external"]),("runtime",a["runtime"])):
            if not vals: continue
            r=next((r for r in cov if r["category"]==cat and any(match(x,p) for p in b["inputs"] for x in r["anyOf"])),None)
            if r: bump(r["id"])
            else: rows.append(d("inputs","external-uncovered" if cat=="external" else "runtime-uncovered",t,",".join(sorted(set(vals)))))
    return order(rows),ap
def digests(path,content,rules):
    out={}
    for r in rules["rules"]:
        if r["surface"]!="output" or "glob" not in r or not match(r["glob"],path): continue
        try: text=content.encode("utf-8").decode("utf-8")
        except Exception: continue
        if r["kind"]=="text-replace":
            fl=0
            if "i" in r["flags"]: fl|=re.I
            if "m" in r["flags"]: fl|=re.M
            if "s" in r["flags"]: fl|=re.S
            out[r["id"]]=hashlib.sha256(re.sub(r["pattern"],r["replacement"],text,count=0 if "g" in r["flags"] else 1,flags=fl).encode()).hexdigest()
        elif r["kind"]=="json-redact":
            try: doc=json.loads(text)
            except Exception: continue
            for ptr in r["pointers"]:
                parts=[x.replace("~1","/").replace("~0","~") for x in ptr[1:].split("/")];node=doc
                for x in parts[:-1]: node=node.get(x) if isinstance(node,dict) else None
                if isinstance(node,dict) and parts[-1] in node: node[parts[-1]]="<redacted>"
            out[r["id"]]=hashlib.sha256(json.dumps(doc,sort_keys=True,separators=(",",":"),ensure_ascii=False).encode()).hexdigest()
    return out
def patterns(ps,task,env):
    root="" if task["root"]=="." else task["root"].rstrip("/");res=[]
    for p in ps:
        def sub(m):
            node=task["execution"]
            for part in m.group(1).split("."): node=node.get(part) if isinstance(node,dict) else None
            if node is None and m.group(1).startswith("env."): node=env.get(m.group(1)[4:])
            if node is None: raise ValueError("option")
            return str(node)
        t=p.replace("{workspaceRoot}/","").replace("{workspaceRoot}","").replace("{projectRoot}",root).replace("{projectName}",task["project"])
        t=re.sub(r"\{options\.([^}]+)\}",sub,t)
        parts=[]
        for part in re.sub(r"^\./","",t).lstrip("/").split("/"):
            if part=="..":
                if not parts: raise ValueError("leaves")
                parts.pop()
            elif part not in ("","."): parts.append(part)
        if not parts: raise ValueError("workspace")
        res.append("/".join(parts))
    res=sorted(set(res));return [p for p in res if not any(q!=p and p.startswith(q+"/") for q in res)]
data=json.load(sys.stdin);k=data["kind"];cs=data["cases"]
if k=="glob": out=[match(c["glob"],c["path"]) for c in cs]
elif k=="graph": out=[graph(c["nx"],c["bazel"]) for c in cs]
elif k=="closure": out=[closure(c["plan"],c["closure"]) for c in cs]
elif k=="outputs": out=[outputs(c["nx"],c["bazel"],c["rules"]) for c in cs]
elif k=="restore": out=[outputs(c["fresh"],c["restored"],c["rules"],"restore")[0] for c in cs]
elif k=="results": out=[results(c["nx"],c["bazel"],c["expectExit"]) for c in cs]
elif k=="inputs": out=[inputs(c["nx"],c["bazel"],c["rules"]) for c in cs]
elif k=="normalize":
    out=[]
    for c in cs:
        a=digests(c["path"],c["left"],{"rules":c["rules"]["rules"]});b=digests(c["path"],c["right"],{"rules":c["rules"]["rules"]})
        out.append(None if not a and not b else all(a.get(x)==b.get(x) for x in a))
elif k=="patterns":
    out=[]
    for c in cs:
        try: out.append(patterns(c["patterns"],c["task"],c["environment"]))
        except ValueError: out.append(None)
print(json.dumps(out))
`;
const python=(kind:string,rows:unknown[])=>{const run=spawnSync(testPythonV1()??"python3",["-I","-c",oracle],{input:JSON.stringify({kind,cases:rows}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as any[];};

test("the schema, the authored data and the fixture agree with Ajv",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(validator("Samples")(samplesData)).toBe(true);
 expect(validator("Rules")(rulesData)).toBe(true);
 const samples=readParitySamplesV1(samplesData);
 expect(samples.samples.length).toBeGreaterThanOrEqual(20);
 expect(new Set(samples.samples.map(row=>row.area))).toEqual(new Set(schema.definitions.Area.enum));
 expect(new Set(samples.samples.map(row=>row.target)).size).toBe(samples.samples.length);
 expect(samples.samples.every(row=>row.id.length<=40)).toBe(true);
 const rules=readParityRulesV1(rulesData);
 for(const rule of rules.rules){expect(rule.justification.en.length,rule.id).toBeGreaterThan(20);expect(rule.justification.de.length,rule.id).toBeGreaterThan(20);expect(rule.justification.de,rule.id).not.toBe(rule.justification.en);expect(rule.evidence.length,rule.id).toBeGreaterThan(5);}
 const plan=validator("Plan"),outputs=validator("Outputs"),divergence=validator("Divergence"),run=validator("Run"),nxInputs=validator("NxInputs"),spawn=validator("Spawn"),glob=validator("GlobCase");
 for(const row of cases.graphCases){if(row.nx)expect(plan(row.nx),row.name).toBe(true);if(row.bazel)expect(plan(row.bazel),row.name).toBe(true);for(const entry of row.expected)expect(divergence(entry),row.name).toBe(true);}
 for(const row of cases.outputCases){expect(outputs(row.nx),row.name).toBe(true);expect(outputs(row.bazel),row.name).toBe(true);expect(validator("Rules")(row.rules),row.name).toBe(true);for(const entry of row.expected)expect(divergence(entry),row.name).toBe(true);}
 for(const row of cases.resultCases){if(row.nx)expect(run(row.nx),row.name).toBe(true);if(row.bazel)expect(run(row.bazel),row.name).toBe(true);}
 for(const row of cases.inputCases){for(const value of Object.values(row.nx??{}))expect(nxInputs(value),row.name).toBe(true);for(const value of Object.values(row.bazel??{}))expect(spawn(value),row.name).toBe(true);}
 for(const row of cases.globCases)expect(glob({glob:row.glob,path:row.path,match:row.match}),row.glob).toBe(true);
 for(const row of cases.parserCases.plans)if(row.expected)expect(plan(row.expected),row.name).toBe(true);
 for(const row of cases.parserCases.nxInputs)for(const value of Object.values(row.expected))expect(nxInputs(value),row.name).toBe(true);
});

test("every divergence code, check and status has text in every language",()=>{
 expect(parityLanguagesV1).toEqual(["en","de"]);
 expect(Object.keys(help.codes).sort()).toEqual([...schema.definitions.Code.enum].sort());
 expect(Object.keys(help.checks).sort()).toEqual([...schema.definitions.ReportedCheck.enum].sort());
 expect(Object.keys(help.status).sort()).toEqual(["diverged","equal","error","not-applicable","unavailable"]);
 for(const group of [help.codes,help.checks,help.status,help.labels] as Record<string,{en:string;de:string}>[])for(const[name,text]of Object.entries(group)){expect(text.en.length,name).toBeGreaterThan(0);expect(text.de.length,name).toBeGreaterThan(0);}
 for(const[name,text]of Object.entries(help.codes as Record<string,{en:string;de:string}>))expect(text.de,name).not.toBe(text.en);
});

test("samples and rules outside the contract are refused by the reader, and the schema rejects the structural ones",()=>{
 const semantic=new Set(["duplicate ids","duplicate rule ids","broken regular expression"]);
 for(const row of cases.refusedSamples){expect(()=>readParitySamplesV1(row.value),row.name).toThrow();expect(validator("Samples")(row.value),row.name).toBe(semantic.has(row.name));}
 for(const row of cases.refusedRules){expect(()=>readParityRulesV1(row.value),row.name).toThrow();expect(validator("Rules")(row.value),row.name).toBe(semantic.has(row.name));}
});

test("path patterns agree with Bun.Glob where it defines the same language, and with the Python implementation everywhere",()=>{
 const answers=python("glob",cases.globCases);
 for(const[index,row]of cases.globCases.entries()){
  expect(globMatchV1(row.glob,row.path),`${row.glob} ${row.path}`).toBe(row.match);
  expect(answers[index],`python ${row.glob} ${row.path}`).toBe(row.match);
  if(row.bun)expect(new Bun.Glob(row.glob).match(row.path),`repo ${row.glob} ${row.path}`).toBe(row.match);
 }
 expect(["b","a","é","𝒜","z"].sort(parityCompareV1)).toEqual(["a","b","z","é","𝒜"]);
});

test("declared output patterns resolve to workspace paths and refuse everything that leaves the workspace",()=>{
 const answers=python("patterns",[...cases.outputPatternCases,...cases.outputPatternRefused]);
 for(const[index,row]of cases.outputPatternCases.entries()){expect(resolveParityOutputPatternsV1(row.patterns,row.task,row.environment),index.toString()).toEqual(row.expected);expect(answers[index]).toEqual(row.expected);}
 for(const[index,row]of cases.outputPatternRefused.entries()){expect(()=>resolveParityOutputPatternsV1(row.patterns,row.task,row.environment),JSON.stringify(row.patterns)).toThrow();expect(answers[cases.outputPatternCases.length+index]).toBe(null);}
});

test("documented normalizations equate exactly what they document",()=>{
 const answers=python("normalize",cases.normalizeCases);
 for(const[index,row]of cases.normalizeCases.entries()){
  const rules=readParityRulesV1(row.rules),left=parityNormalizedDigestsV1(row.path,new TextEncoder().encode(row.left),rules),right=parityNormalizedDigestsV1(row.path,new TextEncoder().encode(row.right),rules);
  const verdict=!Object.keys(left).length&&!Object.keys(right).length?null:Object.keys(left).every(key=>left[key]===right[key]);
  expect(verdict,row.name).toBe(row.equal);
  expect(answers[index],"python "+row.name).toBe(row.equal);
  for(const digest of Object.values(left))expect(/^[a-f0-9]{64}$/u.test(digest)).toBe(true);
 }
});

test("every comparator answers exactly the authored divergences, and so does the independent Python implementation",()=>{
 const graph=python("graph",cases.graphCases),closure=python("closure",cases.closureCases),outputs=python("outputs",cases.outputCases),restore=python("restore",cases.restoreCases),results=python("results",cases.resultCases),inputs=python("inputs",cases.inputCases);
 for(const[index,row]of cases.graphCases.entries()){expect(compareParityGraphV1(row.nx,row.bazel),row.name).toEqual(row.expected);expect(graph[index],"python "+row.name).toEqual(row.expected);}
 for(const[index,row]of cases.closureCases.entries()){expect(compareParityClosureV1(row.plan,row.closure),row.name).toEqual(row.expected);expect(closure[index],"python "+row.name).toEqual(row.expected);}
 for(const[index,row]of cases.outputCases.entries()){const counts=applied(),rules=readParityRulesV1(row.rules);expect(compareParityOutputsV1(row.nx,row.bazel,rules,counts),row.name).toEqual(row.expected);expect(plain(counts),row.name).toEqual(row.applied);expect(outputs[index][0],"python "+row.name).toEqual(row.expected);expect(outputs[index][1],"python applied "+row.name).toEqual(row.applied);}
 for(const[index,row]of cases.restoreCases.entries()){expect(compareParityOutputsV1(row.fresh,row.restored,readParityRulesV1(row.rules),undefined,"restore"),row.name).toEqual(row.expected);expect(restore[index],"python "+row.name).toEqual(row.expected);}
 for(const[index,row]of cases.resultCases.entries()){expect(compareParityResultsV1(row.nx,row.bazel,row.expectExit),row.name).toEqual(row.expected);expect(results[index],"python "+row.name).toEqual(row.expected);}
 for(const[index,row]of cases.inputCases.entries()){const counts=applied(),rules=readParityRulesV1(row.rules);expect(compareParityInputsV1(row.nx,row.bazel,rules,counts),row.name).toEqual(row.expected);expect(plain(counts),row.name).toEqual(row.applied);expect(inputs[index][0],"python "+row.name).toEqual(row.expected);expect(inputs[index][1],"python applied "+row.name).toEqual(row.applied);}
});

test("tool output is read into the neutral records",()=>{
 for(const row of cases.parserCases.plans)expect(parseParityPlanV1(row.text),row.name).toEqual(row.expected);
 for(const row of cases.parserCases.nxInputs)expect(parseParityNxInputsV1(row.text,row.ids),row.name).toEqual(row.expected);
 for(const row of cases.parserCases.executionLog)expect(groupParityBazelSpawnsV1(parseParityBazelExecutionLogV1(row.text),row.labelToTask,row.declaredEnvironment),row.name).toEqual(row.expected);
 for(const row of cases.parserCases.events)expect(parseParityBazelEventsV1(row.text),row.name).toEqual(row.expected);
 for(const row of cases.parserCases.failedTasks)expect(parseParityFailedTasksV1(row.text,fixtureRules.extractors),row.name).toEqual(row.expected);
});

const plan=(...targets:string[])=>({roots:[targets[0]!],tasks:Object.fromEntries(targets.map(id=>[id,{id,target:{project:id.split(":")[0]!,target:id.split(":")[1]!}}])),dependencies:Object.fromEntries(targets.map((id,index)=>[id,targets.slice(index+1,index+2)]))});
const files={dist:{kind:"directory" as const,mode:0o755},"dist/a.js":{kind:"file" as const,mode:0o644,size:3,sha256:"a".repeat(64)}};
const observation=(sample:string,tool:string,overrides:Partial<ParityObservationV1>={}):ParityObservationV1=>({version:1,sample,tool,phase:"fresh",plan:plan("p:build","q:gen"),run:{exit:0,reason:"exit",failedTasks:[],tests:{}},outputs:{roots:{"p:build":["dist"]},files},inputs:tool==="nx"?{"p:build":{files:["a.ts"],environment:[],runtime:[],depOutputs:[],external:[]}}:null,spawns:tool==="bazel"?{"p:build":{inputs:["s/a.ts"],env:[],exit:0}}:null,closure:tool==="bazel"?{queried:["p:build","q:gen"]}:null,errors:[],...overrides});

test("a whole verification reports equal samples, diverging samples and failed observations, and counts the rules it applied",async()=>{
 const rules=readParityRulesV1({...rulesData,rules:rulesData.rules.filter(row=>row.id==="bazel-source-link")}),samples=readParitySamplesV1({...samplesData,samples:[{id:"one",area:"hub",target:"p:build",note:{en:"a",de:"b"},checks:["graph","closure","outputs","results","inputs"]},{id:"two",area:"hub",target:"p:build",note:{en:"a",de:"b"},checks:["graph","outputs","results"]},{id:"three",area:"hub",target:"p:build",note:{en:"a",de:"b"},checks:["results"]}]});
 const table:Record<string,Partial<Record<string,Partial<ParityObservationV1>>>>={two:{bazel:{outputs:{roots:{"p:build":["dist"]},files:{...files,"dist/a.js":{...files["dist/a.js"],sha256:"b".repeat(64)}}}}},three:{bazel:{errors:["no evidence"],run:null}}};
 const progress:string[]=[],runner:ParityRunnerV1={
  async plan(sample){return {plan:plan("p:build","q:gen"),errors:[]};},
  async roots(){return {roots:{"p:build":["dist"]},errors:[]};},
  async observe(sample,tool,phase){return {...observation(sample.id,tool),...table[sample.id]?.[tool],phase};}
 };
 const report=await verifyParityV1(runner,samples,rules,samples.samples,{progress:async label=>{progress.push(label);}});
 expect(validator("Report")(report)).toBe(true);
 expect(report.samples.map(row=>row.status)).toEqual(["equal","diverged","error"]);
 expect(report.totals).toEqual({samples:3,equal:1,diverged:1,error:1,divergences:report.samples.flatMap(row=>row.checks.flatMap(check=>check.divergences)).length});
 expect(report.samples[1]!.checks.find(row=>row.check==="outputs")!.divergences.map(row=>row.code)).toEqual(["output-bytes-differ"]);
 expect(report.rules).toEqual([{id:"bazel-source-link",applied:1}]);
 expect(progress).toContain("one:nx:plan");expect(progress).toContain("two:bazel:fresh");
 const only=await verifyParityV1(runner,samples,rules,samples.samples.slice(0,1),{plansOnly:true});
 expect(only.samples[0]!.checks.map(row=>row.check)).toEqual(["graph"]);
 expect(only.samples[0]!.status).toBe("equal");
 let cancelled=0;
 await expect(verifyParityV1(runner,samples,rules,samples.samples,{check:()=>{if(++cancelled>1)throw Error("cancelled");}})).rejects.toThrow("cancelled");
 const resolved=resolveParitySampleV1(samples,samples.samples[0]!);
 expect(compareParitySampleV1({...resolved,phases:["fresh","restored"],checks:["restore"]},{nx:{fresh:observation("one","nx"),restored:{...observation("one","nx"),phase:"restored"}},bazel:{fresh:observation("one","bazel"),restored:{...observation("one","bazel"),phase:"restored",outputs:{roots:{"p:build":["dist"]},files:{dist:files.dist}}}}},rules).checks.filter(row=>row.check==="restore").map(row=>row.status)).toEqual(["equal","diverged"]);
});

const system=(command:string,args:string[])=>spawnSync(command,args,{encoding:"utf8"});
const envelope=<T,>(body:(invocation:any)=>Promise<T>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-parity-test",maximumElapsedMilliseconds:120000},{},Date.now()),body as never) as Promise<T>;

test("captured outputs are hashed and copied byte for byte, checked against diff, shasum and Python",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"parity-io-")));
 try{
  const left=join(base,"left"),right=join(base,"right");
  for(const root of [left,right]){mkdirSync(join(root,"dist/sub"),{recursive:true});writeFileSync(join(root,"dist/a.js"),"alpha\n");writeFileSync(join(root,"dist/sub/b.txt"),"beta built at 10:00:00\n");writeFileSync(join(root,"dist/run.sh"),"#!/bin/sh\n");chmodSync(join(root,"dist/run.sh"),0o755);symlinkSync("a.js",join(root,"dist/link"));}
  const rules=readParityRulesV1(fixture.outputCases[0]!.rules),roots={"p:build":["dist","absent"]},copyLeft=join(base,"copy-left"),copyRight=join(base,"copy-right");
  const a=await captureParityOutputsV1(left,roots,copyLeft,rules,1<<20,()=>{}),b=await captureParityOutputsV1(right,roots,copyRight,rules,1<<20,()=>{});
  expect(Object.keys(a.files).sort()).toEqual(["dist","dist/a.js","dist/link","dist/run.sh","dist/sub","dist/sub/b.txt"]);
  expect(a.files["dist/link"]).toMatchObject({kind:"symlink",link:"a.js"});
  expect(a.files["dist/run.sh"]!.mode&0o111).toBe(0o111);
  expect(compareParityOutputsV1(a,b,rules)).toEqual([]);
  expect(system("diff",["-r",join(copyLeft,"dist/sub"),join(copyRight,"dist/sub")]).status).toBe(0);
  const sums=system("shasum",["-a","256",join(left,"dist/a.js"),join(left,"dist/sub/b.txt")]).stdout.split("\n").filter(Boolean).map(line=>line.split(/\s+/u)[0]);
  expect([a.files["dist/a.js"]!.sha256,a.files["dist/sub/b.txt"]!.sha256]).toEqual(sums);
  const hashed=spawnSync(testPythonV1()??"python3",["-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",join(left,"dist/a.js")],{encoding:"utf8"}).stdout.trim();
  expect(a.files["dist/a.js"]!.sha256).toBe(hashed);
  writeFileSync(join(right,"dist/sub/b.txt"),"beta changed\n");
  const c=await captureParityOutputsV1(right,roots,join(base,"copy-changed"),rules,1<<20,()=>{});
  expect(compareParityOutputsV1(a,c,rules).map(row=>row.code)).toEqual(["output-bytes-differ"]);
  expect(system("diff",["-r",join(copyLeft,"dist/sub"),join(base,"copy-changed/dist/sub")]).status).toBe(1);
  await expect(captureParityOutputsV1(left,roots,join(base,"copy-big"),rules,3,()=>{})).rejects.toThrow("exceed");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("output roots are set aside, restored and recovered after an interruption",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"parity-aside-")));
 try{
  mkdirSync(join(base,"w/dist"),{recursive:true});writeFileSync(join(base,"w/dist/old.txt"),"old");writeFileSync(join(base,"w/single.txt"),"single");
  const aside=join(base,"out/aside"),set=await setParityOutputsAsideV1(join(base,"w"),["dist","single.txt","absent"],aside);
  expect(set.journal.map(row=>row.how)).toEqual(["move","move","absent"]);
  expect(existsSync(join(base,"w/dist"))).toBe(false);
  mkdirSync(join(base,"w/dist"));writeFileSync(join(base,"w/dist/new.txt"),"new");writeFileSync(join(base,"w/absent"),"produced");
  await set.restore();
  expect(readdirSync(join(base,"w/dist"))).toEqual(["old.txt"]);expect(existsSync(join(base,"w/absent"))).toBe(false);expect(readFileSync(join(base,"w/single.txt"),"utf8")).toBe("single");
  const again=await setParityOutputsAsideV1(join(base,"w"),["dist"],aside);
  expect(again.journal[0]!.how).toBe("move");expect(existsSync(join(base,"w/dist"))).toBe(false);
  expect(await recoverParityOutputsV1(join(base,"out"))).toBe(1);
  expect(readFileSync(join(base,"w/dist/old.txt"),"utf8")).toBe("old");
 }finally{rmSync(base,{recursive:true,force:true});}
});

const fakeCli=String.raw`
const args=process.argv.slice(2),tool=(args.find(a=>a.startsWith("--tool="))||"--tool=nx").slice(7),env=process.env,fs=require("node:fs"),path=require("node:path");
const plan={tasks:{roots:["q:gen"],tasks:{"p:build":{id:"p:build",target:{project:"p",target:"build"}},"q:gen":{id:"q:gen",target:{project:"q",target:"gen"}},"p:test":{id:"p:test",target:{project:"p",target:"test"}}},dependencies:{"p:build":["q:gen"],"q:gen":[],"p:test":["p:build"]}},graph:{}};
if(args[0]==="show"){console.log(JSON.stringify({name:args[2],root:args[2],targets:{build:{execution:{kind:"command"},outputs:["{projectRoot}/dist"]},gen:{execution:{kind:"command"},outputs:["{workspaceRoot}/gen.json"]},test:{execution:{kind:"command"},outputs:[]}}}));process.exit(0);}
if(args.includes("inputs")){console.log("[nx] inputs");console.log(JSON.stringify({project:"p",target:"build",files:["p/a.ts","bun.lock"],runtime:[],environment:["PARITY_FLAG"],depOutputs:[],external:[]},null,2));process.exit(0);}
if(args.includes("--dry-run")){console.log("[log] planning");console.log(JSON.stringify(plan));process.exit(0);}
const diverge=env.FAKE_DIVERGE==="1"&&tool==="bazel",fail=env.FAKE_FAIL==="1";
fs.mkdirSync("p/dist",{recursive:true});fs.writeFileSync("p/dist/a.js",diverge?"different":"same");fs.writeFileSync("gen.json",JSON.stringify({tool:"x",v:1}));
if(env.FAKE_STALE_CHECK==="1"&&fs.existsSync("p/dist/old.txt")){console.error("stale output visible");process.exit(9);}
if(tool==="nx"){if(fail){console.log("   Failed tasks:\n\n   - p:test\n");process.exit(1);}console.log("Successfully ran target build");process.exit(0);}
const dir=env.SEMIO_BUILD_EVIDENCE_DIR;
fs.writeFileSync(path.join(dir,"bazel.closure.json"),JSON.stringify({labelToTask:{"//:t_1":"p:build","//:t_2":"q:gen","//:t_3":"p:test"},queried:["//:t_1","//:t_2","//:t_3"],declaredEnvironment:{"//:t_1":["PARITY_FLAG"]}}));
fs.writeFileSync(path.join(dir,"bazel.bep.jsonl"),[{id:{targetCompleted:{label:"//:t_1"}},completed:{success:true}},{id:{testSummary:{label:"//:t_3"}},testSummary:{overallStatus:fail?"FAILED":"PASSED"}},{id:{buildFinished:{}},finished:{exitCode:fail?{name:"BUILD_FAILURE",code:1}:{name:"SUCCESS"}}}].map(row=>JSON.stringify(row)).join("\n")+"\n");
fs.writeFileSync(path.join(dir,"bazel.execlog.jsonl"),JSON.stringify({targetLabel:"//:t_1",mnemonic:"RepoTask",exitCode:0,environmentVariables:[{name:"PATH",value:"/bin"}],inputs:[{path:"s/p/a.ts"},{path:"s/bun.lock"},{path:"BUILD.bazel"}]})+"\n");
process.exit(fail?1:0);
`;
const fakeWorkspace=():string=>{const base=realpathSync(mkdtempSync(join(tmpdir(),"parity-live-")));writeFileSync(join(base,"package.json"),JSON.stringify({name:"fake",scripts:{repo:"bun fake.ts"}}));writeFileSync(join(base,"fake.ts"),fakeCli);return base;};
const liveRules=readParityRulesV1({...rulesData,rules:[...rulesData.rules.filter(row=>["bazel-source-link","bazel-build-file","lock-covers-external"].includes(row.id)),{id:"ambient",kind:"ambient-env",surface:"env-bazel",names:["PATH"],justification:{en:"The strict Bazel action environment always names PATH.",de:"Die strikte Bazel-Aktionsumgebung nennt immer PATH."},evidence:"fake"}]});
const liveSamples=readParitySamplesV1({...samplesData,samples:[{id:"live",area:"hub",target:"p:build",note:{en:"live",de:"live"},phases:["fresh","restored"]}]});

test("the live runner drives both tools through the repo command, captures into separate roots and leaves the checkout as it was",async()=>{
 const base=fakeWorkspace();
 try{
  mkdirSync(join(base,"p/dist"),{recursive:true});writeFileSync(join(base,"p/dist/old.txt"),"old");
  const out=join(base,"out");
  const report=await envelope(async invocation=>{
   const runner=createParityRunnerV1({root:base,invocation,out,environment:{...process.env,FAKE_STALE_CHECK:"1"},rules:liveRules,progress:async()=>{}});
   return verifyParityV1(runner,liveSamples,liveRules,liveSamples.samples);
  });
  expect(validator("Report")(report)).toBe(true);
  expect(report.samples[0]!.status,JSON.stringify(report.samples[0]!.checks.filter(row=>row.divergences.length))).toBe("equal");
  expect(report.samples[0]!.checks.map(row=>`${row.check}:${row.phase}:${row.status}`)).toEqual(["graph:fresh:equal","closure:fresh:equal","outputs:fresh:equal","results:fresh:equal","inputs:fresh:equal","outputs:restored:equal","results:restored:equal","restore:restored:equal","restore:restored:equal"]);
  expect(readdirSync(join(base,"p/dist"))).toEqual(["old.txt"]);expect(existsSync(join(base,"gen.json"))).toBe(false);
  expect(readFileSync(join(out,"live/nx-fresh/outputs/p/dist/a.js"),"utf8")).toBe("same");
  expect(existsSync(join(out,"live/bazel-fresh/evidence/bazel.execlog.jsonl"))).toBe(true);
  const diverged=await envelope(async invocation=>verifyParityV1(createParityRunnerV1({root:base,invocation,out:join(base,"out2"),environment:{...process.env,FAKE_DIVERGE:"1"},rules:liveRules,progress:async()=>{}}),liveSamples,liveRules,liveSamples.samples));
  expect(diverged.samples[0]!.status).toBe("diverged");
  expect(diverged.samples[0]!.checks.flatMap(row=>row.divergences).map(row=>`${row.check}:${row.code}:${row.subject}`)).toContain("outputs:output-bytes-differ:p/dist/a.js");
  const failing=await envelope(async invocation=>verifyParityV1(createParityRunnerV1({root:base,invocation,out:join(base,"out3"),environment:{...process.env,FAKE_FAIL:"1"},rules:liveRules,progress:async()=>{}}),readParitySamplesV1({...liveSamples,samples:[{...liveSamples.samples[0]!,id:"failing",target:"p:test",expectExit:1,phases:["fresh"],checks:["results"]}]}),liveRules,[{...liveSamples.samples[0]!,id:"failing",target:"p:test",expectExit:1,phases:["fresh"],checks:["results"]}]));
  expect(failing.samples[0]!.status,JSON.stringify(failing.samples[0]!.checks)).toBe("equal");
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("the command reads a closed grammar, selects samples and speaks both languages",async()=>{
 expect(readParityArgumentsV1(["--only=area:hub,x","--plans-only","--json","--out","d","--lang=de"])).toMatchObject({only:["area:hub","x"],plansOnly:true,json:true,out:"d"});
 expect(readParityArgumentsV1(["--only="])).toMatchObject({only:[],plansOnly:false});
 expect(()=>readParityArgumentsV1(["--nope"])).toThrow("Unknown argument");
 expect(()=>readParityArgumentsV1(["--out"])).toThrow("requires a value");
 const samples=readParitySamplesV1(samplesData).samples;
 expect(selectParitySamplesV1(samples,["area:teaching"]).every(row=>row.area==="teaching")).toBe(true);
 expect(selectParitySamplesV1(samples,[samples[0]!.id])).toHaveLength(1);
 expect(()=>selectParitySamplesV1(samples,["nothing"])).toThrow("No parity sample");
 expect(renderParityHelpV1("en")).toContain("Runs every sample target with Nx and with Bazel");
 expect(renderParityHelpV1("de")).toContain("Führt jede Stichprobe mit Nx und mit Bazel");
 const rules=readParityRulesV1(rulesData),report={version:1 as const,samples:[{sample:"s",area:"hub" as const,target:"p:t",status:"diverged" as const,checks:[{check:"outputs" as const,phase:"fresh" as const,status:"diverged" as const,divergences:[{check:"outputs" as const,code:"output-bytes-differ",subject:"a|b",nx:"1",bazel:"2"}]}]}],totals:{samples:1,equal:0,diverged:1,error:0,divergences:1},rules:rules.rules.map(row=>({id:row.id,applied:0}))};
 const en=renderParityReportV1(report,rules,"en"),de=renderParityReportV1(report,rules,"de");
 expect(en).toContain("The output files differ byte for byte.");expect(de).toContain("Die Ausgabedateien unterscheiden sich Byte für Byte.");expect(en).toContain("a\\|b");
 await envelope(async invocation=>{
  const lines:string[]=[],original=process.stdout.write;process.stdout.write=((chunk:unknown)=>{lines.push(String(chunk));return true;}) as typeof process.stdout.write;
  try{await runBuildParityV1(process.cwd(),["--help","--lang=de"],invocation,{});}finally{process.stdout.write=original;}
  expect(lines.join("\n")).toContain("Führt jede Stichprobe");
 });
});
