/** 🗓️ Contract: the scheduling policy is closed by its schema and refuses forged tables; every prefix is a real directory and every port a real file inside its own domain; a split keeps the entry and every output-producing file in the keyed part, moves exactly the files under the named prefixes into the overlay and leaves the keyed identity unchanged when only the overlay changes; the immutable loader merges the overlay beside a keyed snapshot, refuses every overlay it cannot prove (outside the declared domains, shadowing a keyed image, another root, hard-linked) and runs the edited scheduling code without the digest of the keyed snapshot ever moving. The cases live in `./🔣️.json`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {chmodSync,linkSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {buildSourceClosureIdentityV1,buildSourceClosureLoaderV1,type BuildSourceClosureV1} from "../../🟦️.ts";
import {buildSchedulingDomainOfV1,buildSchedulingExclusionPatternsV1,buildSchedulingExclusionsInputV1,buildSchedulingOverlayNameV1,buildSchedulingPortsV1,buildSchedulingPrefixesV1,buildSchedulingV1,splitBuildSourceClosureV1,type BuildSchedulingV1} from "../🟦️.ts";
import declared from "../🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const checkout=resolve(import.meta.dir,corpus.checkoutFromTests),sha=(value:string|Buffer)=>createHash("sha256").update(value).digest("hex");
const refusal=(table:unknown):unknown=>{try{buildSchedulingV1(table);}catch(error){return (error as {key?:string}).key;}return undefined;};
type Draft={version:1;domains:{id:string;contract:{en:string;de:string};prefixes:string[];ports:string[]}[]};
const copy=():Draft=>JSON.parse(JSON.stringify(declared));

test("the corpus and the policy are closed by their schemas",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(schema,declared),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,{...declared,extra:1})).toBe(false);
 expect(ajv.validate(schema,{...declared,domains:[]})).toBe(false);
 for(const domain of declared.domains){expect(domain.contract.en.length,domain.id).toBeGreaterThan(0);expect(domain.contract.de.length,domain.id).toBeGreaterThan(0);}
});

test("every forged policy is refused and the declared one is admitted once",()=>{
 expect(buildSchedulingV1()).toBe(buildSchedulingV1());
 const forged:Record<string,()=>unknown>={
  "duplicate-domain-id":()=>{const table=copy();table.domains=[table.domains[0]!,{...table.domains[1]!,id:table.domains[0]!.id}];return table;},
  "nested-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first+"inner/"],ports:[]}];return table;},
  "duplicate-prefix":()=>{const table=copy();const first=table.domains[0]!.prefixes[0]!;table.domains=[table.domains[0]!,{...table.domains[1]!,prefixes:[first],ports:[]}];return table;},
  "parent-segment":()=>{const table=copy();table.domains=[{...table.domains[0]!,prefixes:["a/../b/"],ports:[]}];return table;},
  "port-outside-domain":()=>{const table=copy();table.domains=[{...table.domains[0]!,ports:["elsewhere/file.ts"]}];return table;},
  "port-is-directory":()=>{const table=copy();table.domains=[{...table.domains[0]!,ports:[table.domains[0]!.prefixes[0]!]}];return table;},
  "extra-key":()=>({...copy(),extra:true}),
  "no-domains":()=>({...copy(),domains:[]}),
  "bad-id":()=>{const table=copy();table.domains=[{...table.domains[0]!,id:"Bad Id"}];return table;}
 };
 expect(Object.keys(forged).sort()).toEqual([...corpus.forgedPolicies].sort());
 for(const [name,make] of Object.entries(forged))expect(refusal(make()),name).toBe("build.scheduling.table-required");
});

test("every prefix is a real directory and every port a real file inside its own domain",()=>{
 for(const domain of buildSchedulingV1().domains){
  for(const prefix of domain.prefixes)expect(statSync(join(checkout,prefix)).isDirectory(),prefix).toBe(true);
  for(const port of domain.ports){expect(statSync(join(checkout,port)).isFile(),port).toBe(true);expect(buildSchedulingDomainOfV1(port),port).toBe(domain.id);}
 }
 expect(buildSchedulingPrefixesV1().length).toBe(new Set(buildSchedulingPrefixesV1()).size);
 expect(buildSchedulingPortsV1().size).toBeGreaterThan(0);
 expect(()=>buildSchedulingPrefixesV1(buildSchedulingV1(),["no-such-domain"])).toThrow();
 expect(buildSchedulingDomainOfV1("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts")).toBeUndefined();
});

const file=(path:string,text:string,imports:Record<string,string>={})=>({path,sha256:sha(text),base64:Buffer.from(text).toString("base64"),imports});

test("a split moves exactly the files under the named prefixes, keeps the entry keyed and leaves the identity to the keyed part",()=>{
 const [domain]=buildSchedulingV1().domains,prefix=domain!.prefixes[0]!;
 const snapshot:BuildSourceClosureV1={version:1,root:"/r",entry:"entry.ts",files:[file("entry.ts","a"),file("keyed.ts","b"),file(prefix+"x.ts","c"),file(prefix+"deep/y.json","{}")]};
 const {keyed,overlay}=splitBuildSourceClosureV1(snapshot,[domain!.id]);
 expect(keyed.files.map(row=>row.path)).toEqual(["entry.ts","keyed.ts"]);
 expect(overlay?.files.map(row=>row.path)).toEqual([prefix+"x.ts",prefix+"deep/y.json"]);
 expect(Object.keys(keyed)).toEqual(["version","root","entry","files"]);
 expect(Object.keys(overlay!)).toEqual(["version","root","entry","files"]);
 const edited={...snapshot,files:[...snapshot.files.slice(0,2),file(prefix+"x.ts","changed"),snapshot.files[3]!]};
 expect(buildSourceClosureIdentityV1(splitBuildSourceClosureV1(edited,[domain!.id]).keyed)).toBe(buildSourceClosureIdentityV1(keyed));
 const moved={...snapshot,files:[snapshot.files[0]!,file("keyed.ts","changed"),...snapshot.files.slice(2)]};
 expect(buildSourceClosureIdentityV1(splitBuildSourceClosureV1(moved,[domain!.id]).keyed)).not.toBe(buildSourceClosureIdentityV1(keyed));
 expect(splitBuildSourceClosureV1({...snapshot,files:snapshot.files.slice(0,2)},[domain!.id]).overlay).toBeUndefined();
 expect(splitBuildSourceClosureV1(snapshot,[]).overlay).toBeUndefined();
 expect(()=>splitBuildSourceClosureV1({...snapshot,entry:prefix+"x.ts"},[domain!.id])).toThrow();
 expect(buildSchedulingOverlayNameV1("implementation.executor-command.json")).toBe("implementation.executor-command.scheduling.json");
 expect(()=>buildSchedulingOverlayNameV1("implementation")).toThrow();
});

const run=(name:string)=>{
 const prefix=buildSchedulingPrefixesV1()[0]!,area=mkdtempSync(join(tmpdir(),"sch-")),root=join(tmpdir(),"sch-law-root");
 const keyed=[file("entry.ts",`import {k} from "./keyed.ts";import {s} from "./${prefix}probe.ts";export async function runCapturedBuildSourceV1(args:string[],proof:{sha256:string}){console.log(JSON.stringify([k,s,args,proof.sha256.length]));}`,{"./keyed.ts":"keyed.ts",[`./${prefix}probe.ts`]:prefix+"probe.ts"}),file("keyed.ts",'export const k="K1";')];
 const base={version:1,root,entry:"entry.ts",files:keyed},probe=(text:string)=>file(prefix+"probe.ts",text);
 const overlays:Record<string,object|undefined>={
  "merged":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";')]},
  "edited-overlay":{version:1,root,entry:"entry.ts",files:[probe('export const s="S2";')]},
  "missing-overlay":undefined,
  "outside-declared-domains":{version:1,root,entry:"entry.ts",files:[file("evil.ts",'export const s="E";')]},
  "shadows-keyed":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";'),file(prefix+"keyed.ts","x")]},
  "other-root":{version:1,root:root+"x",entry:"entry.ts",files:[probe('export const s="S1";')]},
  "writable-hardlink":{version:1,root,entry:"entry.ts",files:[probe('export const s="S1";')]}
 };
 if(name==="shadows-keyed")base.files=[...keyed,file(prefix+"keyed.ts","shadow")];
 const runner=join(area,"📜️script.ts");writeFileSync(runner,buildSourceClosureLoaderV1);
 const text=JSON.stringify(base),snapshot=join(area,"implementation.x.json");writeFileSync(snapshot,text);chmodSync(snapshot,0o400);
 const overlay=overlays[name];if(overlay){const target=join(area,buildSchedulingOverlayNameV1("implementation.x.json"));writeFileSync(target,JSON.stringify(overlay));chmodSync(target,0o400);if(name==="writable-hardlink")linkSync(target,join(area,"alias.json"));}
 const result=spawnSync("bun",[runner,snapshot,sha(text),"arg1"],{encoding:"utf8",cwd:area});rmSync(area,{recursive:true,force:true});
 return {status:result.status,stdout:result.stdout.trim(),stderr:result.stderr,digest:sha(text)};
};

for(const item of corpus.loaderCases)test(`the immutable loader ${item.outcome==="runs"?"merges":"refuses"} the scheduling overlay: ${item.case}`,()=>{
 const result=run(item.case);
 if(item.outcome==="runs"){expect(result.status,result.stderr).toBe(0);expect(result.stdout).toContain(item.case==="merged"?'"S1"':'"S2"');expect(result.stdout).toContain('"K1"');}
 else{expect(result.status).not.toBe(0);expect(result.stderr).toContain(item.message);}
},60000);

test("editing only the overlay never moves the digest of the keyed snapshot the action key is built from",()=>{
 const first=run("merged"),second=run("edited-overlay");
 expect(first.digest).toBe(second.digest);
 expect(first.stdout).not.toBe(second.stdout);
},60000);

test("the named input that removes scheduling code from declared inputs equals the policy in the build policy and in its Nx mirror",()=>{
 const expected=[...buildSchedulingExclusionPatternsV1()];
 for(const file of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json","nx.json"]){
  const named=JSON.parse(readFileSync(join(checkout,file),"utf8")).namedInputs as Record<string,string[]>;
  expect(named[buildSchedulingExclusionsInputV1],file).toEqual(expected);
 }
 expect(expected.length).toBe(buildSchedulingPrefixesV1().length);
 for(const pattern of expected){expect(pattern.startsWith("!{workspaceRoot}/")).toBe(true);expect(pattern.endsWith("/**/*")).toBe(true);}
});
