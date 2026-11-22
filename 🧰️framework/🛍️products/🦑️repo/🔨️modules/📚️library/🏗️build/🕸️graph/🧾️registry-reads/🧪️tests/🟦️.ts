import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync,utimesSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const reads=await import("../🟨️.mjs"),observation=await import("../../../👁️observation/🟨️.mjs"),admission=await import("../../🚪️admission/🟦️.ts");

type Registry=typeof fixture.registry;
const clone=<T>(value:T):T=>structuredClone(value);
const apply=(registry:Registry,edit:(typeof fixture.edits)[number]):Registry=>{
 const changed=clone(registry) as any,target=changed.owners.find((owner:any)=>owner.name===(edit as any).owner);
 if(edit.op==="root"){target.manifest=(edit.value as string)+"/"+target.manifest.split("/").at(-1);target.root=edit.value;}
 else if(edit.op==="packages")target.packages=edit.value;
 else if(edit.op==="reorder"){target.packages.reverse();target.nativeRoots?.reverse();}
 else if(edit.op==="add")changed.owners.push(edit.row);
 else if(edit.op==="remove")changed.owners=changed.owners.filter((owner:any)=>owner.name!==(edit as any).owner);
 else changed.catalogProviders[edit.index!].consumerRoots=edit.value;
 return changed;
};
const rowsOf=(registry:Registry)=>Object.fromEntries(registry.owners.map(owner=>[owner.name,reads.ownerRowDigestV1(owner)]));
const oracle=()=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("an edit of one owner row moves the digest of that row only, and an edit nobody asked about moves no answer, as an independent model says",()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);
 const expected=oracle(),base=rowsOf(fixture.registry),before=fixture.questions.map(question=>reads.answerOwnerRegistryQuestionV1(fixture.registry,question));
 expect(rowsOf(fixture.registry)).toEqual(expected.rows);
 expect(reads.ownerRegistryDigestV1(fixture.registry)).toBe(expected.digest);
 expect(before).toEqual(expected.answers);
 const actual=fixture.edits.map(edit=>{
  const changed=apply(fixture.registry,edit),rows=rowsOf(changed);
  return {rows:[...new Set([...Object.keys(rows),...Object.keys(base)])].filter(name=>rows[name]!==base[name]).sort(),digest:reads.ownerRegistryDigestV1(changed)!==expected.digest,answers:fixture.questions.map((question,at)=>JSON.stringify(reads.answerOwnerRegistryQuestionV1(changed,question))!==JSON.stringify(before[at])),scope:expected.scope};
 });
 expect(actual).toEqual(expected.edits);
 expect(expected.edits[0].rows).toEqual(["@semio-tech/epsilon"]);
 expect(expected.edits[2].rows).toEqual([]);expect(expected.edits[3].digest).toBe(false);
 expect(expected.edits[0].answers.filter(Boolean).length).toBeLessThan(fixture.questions.length);
});

test("the identity of a selected scope is the digest of the selection and never reads the registry",()=>{
 const expected=oracle(),selection=admission.readBuildGraphSelectionV1(fixture.selection);
 expect(admission.buildGraphScopeKeyV1(selection)).toBe(expected.scope);
 for(const edit of fixture.edits)expect((admission.buildGraphScopeKeyV1 as (...values:unknown[])=>string)(selection,apply(fixture.registry,edit))).toBe(expected.scope);
 expect(admission.buildGraphScopeKeyV1({kind:"all"})).not.toBe(expected.scope);
 expect(admission.buildGraphScopeKeyV1({...selection,projects:["@semio-tech/alpha"]})).not.toBe(expected.scope);
});

test("a recorded question holds while the registry on disk answers it the same, per question and not per file",()=>{
 const directory=mkdtempSync(join(tmpdir(),"registry-reads-")),file=join(directory,"📋️build-owners.json");
 try{
  const write=(registry:Registry,second:number)=>{writeFileSync(file,JSON.stringify(registry));utimesSync(file,second,second);};
  write(fixture.registry,1_700_000_000);
  const rowOf=(question:readonly unknown[])=>({kind:"global",path:JSON.stringify([reads.OWNER_REGISTRY_FACT_V1,directory,question]),fact:JSON.stringify(reads.answerOwnerRegistryQuestionV1(fixture.registry,question))});
  const rows=fixture.questions.map(rowOf);
  expect(rows.every(row=>observation.observationRowFreshV1(row))).toBe(true);
  const expected=oracle();
  fixture.edits.forEach((edit,at)=>{
   write(apply(fixture.registry,edit),1_700_000_100+at);
   expect(rows.map(row=>!observation.observationRowFreshV1(row))).toEqual(expected.edits[at].answers);
  });
  write(fixture.registry,1_700_001_000);
  expect(rows.every(row=>observation.observationRowFreshV1(row))).toBe(true);
  expect(observation.observationRowFreshV1({kind:"global",path:JSON.stringify([reads.OWNER_REGISTRY_FACT_V1,directory,["unknown"]]),fact:"1"})).toBe(false);
  rmSync(file);
  expect(observation.observationRowFreshV1(rows[0])).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the lookups of the owner discovery answer as the registry does and every answer is recorded as a question that holds against the same registry and fails against an edited one",async()=>{
 const registry=fixture.registry,view=reads.observedOwnerRegistryViewV1("/w",registry);
 const captured=await observation.captureObservedSourceInputsV1(async()=>[
  view.byName.has("@semio-tech/alpha"),view.byName.has("nobody"),view.byPackage.get("alpha-core"),view.byPackage.get("nope"),view.byRoot.get("🌎️beta/🦀️native-two"),view.byRoot.get("nothing"),
  view.relativeProject("🧰️alpha/🧩️sub/src/a.ts"),view.relativeProject("🧰️alpha/x.ts"),view.relativeProject("top.ts"),view.relativeProject("unknown/a/b.ts"),view.byRoot.size>5,
 ]);
 expect(captured.value).toEqual([true,false,"@semio-tech/alpha",undefined,"@semio-tech/beta-rs",undefined,"@semio-tech/alpha-sub","@semio-tech/alpha",undefined,undefined,true]);
 const rows=(captured.observations as {kind:string;path:string;fact:string}[]).filter(row=>row.kind==="global");
 expect(rows.map(row=>JSON.parse(row.path)[2]).sort()).toEqual([["package","alpha-core"],["package","nope"],["root","🌎️beta/🦀️native-two"],["root","nothing"],["row","@semio-tech/alpha"],["row","nobody"],["within","."],["within","unknown/a"],["within","🧰️alpha"],["within","🧰️alpha/🧩️sub/src"]].sort());
 for(const row of rows)expect(reads.answerOwnerRegistryQuestionV1(registry,JSON.parse(row.path)[2])).toEqual(JSON.parse(row.fact));
 const moves=(edit:(typeof fixture.edits)[number])=>{const edited=apply(registry,edit);return rows.filter(row=>JSON.stringify(reads.answerOwnerRegistryQuestionV1(edited,JSON.parse(row.path)[2]))!==row.fact).map(row=>JSON.parse(row.path)[2]).sort();};
 expect(moves(fixture.edits[5]!)).toEqual([]);expect(moves(fixture.edits[0]!)).toEqual([]);
 expect(moves({name:"an owner on a directory the sub owner read",op:"add",row:{name:"x",root:"🧰️alpha/🧩️sub/src",manifest:"🧰️alpha/🧩️sub/src/📋️project.json",packages:[]}} as never)).toEqual([["within","🧰️alpha/🧩️sub/src"]]);
 expect(moves({name:"a new owner named nobody",op:"add",row:{name:"nobody",root:"🪐️nobody",manifest:"🪐️nobody/📋️project.json",packages:["nope"]}} as never)).toEqual([["package","nope"],["row","nobody"]]);
});
