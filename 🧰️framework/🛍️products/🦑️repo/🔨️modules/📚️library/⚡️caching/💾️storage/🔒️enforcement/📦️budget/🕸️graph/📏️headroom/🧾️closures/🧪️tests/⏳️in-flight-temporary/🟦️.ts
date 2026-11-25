/** ⏳️ Contract: an in-flight atomic-write temporary (`<publication>.<uuid>`, `.<pid>.tmp`, `.<hex>`, `~<hex>`) of every publication kind of the closure store is left to its writer and never fails the headroom pass with "Unknown closure reference publication"; it is an orphan once its writer process is dead or it is older than the schema-declared bound; a name that is no temporary of a known publication is refused and skipped, never thrown.
 * @see ../../🏷️names/🟦️.ts */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import {physicalSourceOriginV1} from "../../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs";

const origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
const source=(url:string)=>readFileSync(new URL(url,origin),"utf8");
const tree=ts.createSourceFile("closures.ts",source("../../🏷️names/🟦️.ts"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const declared=(name:string)=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!.getText(tree);
const schema=JSON.parse(source("../../🧬️schema/🔣️.json")),P=schema.definitions.Protocol.const;
const sessionTree=ts.createSourceFile("session.ts",source("../../📡️session/🟦️.ts"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS),change=Function(ts.transpileModule(sessionTree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="directoryChangeV1")!.getText(sessionTree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return directoryChangeV1")() as(before:string[],after:string[])=>{en:string;de:string};
const load=Function("P",ts.transpileModule(["role","spill","temporaryBase","orphanTemporary"].map(declared).join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return{role,spill,temporaryBase,orphanTemporary}")(P) as{role:(name:string)=>string;spill:(name:string)=>boolean;temporaryBase:(name:string)=>string|undefined;orphanTemporary:(name:string,modified:number,now:number,seconds:number,alive:(pid:number)=>boolean)=>boolean};
const hash="293fc434"+"0c6c0a1b".repeat(7),uuid="864d7e84-1733-47d8-a3fc-b7b35f232c42",scope="8d8f21d7".repeat(8),token="952099430c7a7f41bd3d7102";

test("the exact g1-m06 name is an in-flight temporary of its admission, not an unknown publication",()=>{
 const name="selected/"+hash+".admission.json."+uuid;
 expect(()=>load.role(name)).toThrow("Unknown closure reference publication: "+name);
 expect(load.temporaryBase(name)).toBe("selected/"+hash+".admission.json");
});

test("every publication kind has its temporaries recognised, whatever suffix its writer uses",()=>{
 const bases=["selected/"+scope+".json","selected/"+scope+".authority.json","selected/"+scope+".admission.json","canonical.json","authority.json","graph.json","format.json","directory-stamps.json","build-input-closures/"+scope+".json","build-lock-closures/"+scope+".json","snapshots/"+scope+".json","snapshots/"+scope+".gz","snapshots/retained.ledger","generations/current.json","generations/m/"+scope+".json","generations/o/"+scope+".json.br","generations/x/"+scope+".facts.br","generations/f/"+scope+".json.br","generations/members.jsonl","inference/selections/"+scope+"/np/"+token+".json","inference/selections/"+scope+"/ip/"+token+".json","inference/selections/"+scope+"/rr/"+token+".json","inference/selections/"+scope+"/ob/"+scope+".br","inference/selections/"+scope+"/members.jsonl","retired/"+scope+".json"];
 const suffixes=["."+uuid,".4242.tmp",".4242.1763470000000.tmp",".4242.k3j9x1.tmp",".0123456789abcdef.tmp",".0123456789ab",".0123456789abcdef","~0123abcd","~0123456789abcdef",".tmp","~7.next","~7.prior"];
 for(const base of bases)for(const suffix of suffixes)expect(load.temporaryBase(base+suffix),base+suffix).toBe(base);
 for(const directory of["build-input-closures","build-lock-closures"])expect(load.temporaryBase(directory+"/~"+uuid),directory).toBe(directory+"/");
});

test("a spill keeps its own handling and an unknown name is refused, not recognised",()=>{
 expect(load.temporaryBase("generations/s/"+token+".json.0123456789abcdef.tmp")).toBe("generations/s/"+token+".json");
 for(const name of["selected/"+scope+".other.json."+uuid,"selected/short.admission.json."+uuid,"unknown/"+scope+".json."+uuid,"selected/"+scope+".admission.json.not-a-uuid","selected/"+scope+".admission.json.","stray.bin","snapshots/"+scope+".json.bak","build-input-closures/~"+uuid+"x","other/~"+uuid])expect(load.temporaryBase(name),name).toBeUndefined();
});

test("a temporary is an orphan when its writer is dead or it is older than the schema-declared bound, never while a live recent writer holds it",()=>{
 expect(P.temporaryOrphanSeconds).toBeGreaterThan(0);
 const now=2_000_000_000_000,seconds=P.temporaryOrphanSeconds,name="selected/"+hash+".admission.json.",alive=()=>true,dead=()=>false;
 expect(load.orphanTemporary(name+uuid,now-1000,now,seconds,alive)).toBe(false);
 expect(load.orphanTemporary(name+uuid,now-1000,now,seconds,dead)).toBe(false);
 expect(load.orphanTemporary(name+uuid,now-seconds*1000-1,now,seconds,alive)).toBe(true);
 expect(load.orphanTemporary(name.slice(0,-1)+".4242.tmp",now-1000,now,seconds,alive)).toBe(false);
 expect(load.orphanTemporary(name.slice(0,-1)+".4242.tmp",now-1000,now,seconds,dead)).toBe(true);
 expect(load.orphanTemporary(name.slice(0,-1)+".4242.1763470000000.tmp",now-1000,now,seconds,pid=>pid!==4242)).toBe(true);
});

test("the classification equals an independent Python implementation on the same names",()=>{
 const names=["selected/"+hash+".admission.json."+uuid,"selected/"+hash+".json."+uuid,"selected/"+hash+".authority.json."+uuid,"snapshots/"+scope+".json.4242.tmp","canonical.json.4242.tmp","selected/"+scope+".bogus.json."+uuid,"build-input-closures/~"+uuid,"stray.bin"];
 const program="import json,re,sys\nU='[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}'\nK=[r'^selected/[a-f0-9]{64}(?:\\.authority|\\.admission)?\\.json$',r'^snapshots/[a-f0-9]{64}\\.json$',r'^canonical\\.json$']\nS=re.compile(r'^(.+?)(?:\\.'+U+r'|\\.[0-9]+(?:\\.[0-9a-z]+)?\\.tmp)$')\nout=[]\nfor n in json.load(sys.stdin):\n  s=re.match(r'^(build-input-closures|build-lock-closures)/~'+U+'$',n)\n  m=S.match(n)\n  out.append(s.group(1)+'/' if s else (m.group(1) if m and any(re.match(k,m.group(1)) for k in K) else None))\nprint(json.dumps(out))";
 const run=spawnSync("python3",["-I","-c",program],{input:JSON.stringify(names),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(names.map(name=>load.temporaryBase(name)??null)).toEqual(JSON.parse(run.stdout));
});

test("a directory refusal names what appeared and disappeared, capped at ten with a count, in English and German",()=>{
 const names=Array.from({length:12},(_,at)=>"f"+at);
 expect(change(["a","b"],["b","c.4242.tmp"])).toEqual({en:"appeared 1: c.4242.tmp; disappeared 1: a",de:"neu 1: c.4242.tmp; verschwunden 1: a"});
 expect(change([],names).en).toBe("appeared 12: f0, f1, f2, f3, f4, f5, f6, f7, f8, f9 +2; disappeared 0: ");
 expect(change(["a"],["a"])).toEqual({en:"no entry name changed",de:"kein Eintragsname geändert"});
});
