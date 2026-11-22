/** ♻️ Contract: a registry edit inside one process invalidates exactly the owners whose recorded registry rows it moves, in the pass memo of a read and in the key of the project farm: an independent model (Python, `../🐍️.py`) names which recorded questions an edit moves, and the memo of the owner loop keeps every other owner. */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "../🔣️.json";

const reads=await import("../../🟨️.mjs"),observation=await import("../../../../👁️observation/🟨️.mjs"),{createTakenMemoV1}=await import("../../../📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs");
const PROJECT_NODES=join(import.meta.dir,"..","..","..","📥️inference","📦️project-nodes"),directories:string[]=[];
afterAll(()=>{for(const directory of directories)rmSync(directory,{recursive:true,force:true});});

type Registry=typeof fixture.registry;
const apply=(registry:Registry,edit:(typeof fixture.edits)[number]):Registry=>{
 const changed=structuredClone(registry) as any,target=changed.owners.find((owner:any)=>owner.name===(edit as any).owner);
 if(edit.op==="root"){target.manifest=(edit.value as string)+"/"+target.manifest.split("/").at(-1);target.root=edit.value;}
 else if(edit.op==="packages")target.packages=edit.value;
 else if(edit.op==="reorder"){target.packages.reverse();target.nativeRoots?.reverse();}
 else if(edit.op==="add")changed.owners.push(edit.row);
 else if(edit.op==="remove")changed.owners=changed.owners.filter((owner:any)=>owner.name!==(edit as any).owner);
 else changed.catalogProviders[edit.index!].consumerRoots=edit.value;
 return changed;
};
const oracle=()=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"..","🐍️.py"),join(import.meta.dir,"..","🔣️.json")],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the owners of the pass memo that an edit of the registry file invalidates are exactly those whose recorded questions the independent model says it moves",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"registry-memo-"));directories.push(directory);
 const file=join(directory,"📋️build-owners.json"),expected=oracle();
 const write=(registry:Registry,second:number)=>{writeFileSync(file,JSON.stringify(registry));utimesSync(file,second,second);};
 const rowOf=(question:readonly unknown[])=>({kind:"global",path:JSON.stringify([reads.OWNER_REGISTRY_FACT_V1,directory,question]),fact:JSON.stringify(reads.answerOwnerRegistryQuestionV1(fixture.registry,question))});
 const holds=(rows:readonly unknown[])=>rows.every(row=>observation.observationRowFreshV1(row as never));
 let second=1_700_000_000;
 for(const[at,edit]of fixture.edits.entries()){
  write(fixture.registry,second++);
  const memo=createTakenMemoV1({holds,retain:()=>{}});
  fixture.questions.forEach((question,index)=>memo.remember("owner-"+index,{key:"k",project:{name:"owner-"+index},rows:[rowOf(question)]}));
  write(apply(fixture.registry,edit),second++);
  const reused=(await memo.replay(fixture.questions.map((_,index)=>({name:"owner-"+index,key:"k"})))).map(row=>row.name).sort();
  const kept=fixture.questions.map((_,index)=>"owner-"+index).filter((_,index)=>!expected.edits[at].answers[index]).sort();
  expect(reused,edit.name).toEqual(kept);
  expect(memo.stats().evictedChanged,edit.name).toBe(fixture.questions.length-kept.length);
 }
 expect(expected.edits[0].answers.filter(Boolean).length).toBeLessThan(fixture.questions.length);
 expect(expected.edits[2].answers.some(Boolean)).toBe(false);
});

test("the registry digest of a registry object is computed once, is null without a registry and follows the row law",()=>{
 const expected=oracle();
 expect(reads.ownerRegistryDigestOnceV1(fixture.registry)).toBe(expected.digest);
 expect(reads.ownerRegistryDigestOnceV1(fixture.registry)).toBe(expected.digest);
 expect(reads.ownerRegistryDigestOnceV1(null)).toBeNull();
 expect(reads.ownerRegistryDigestOnceV1(apply(fixture.registry,fixture.edits[0]!))).not.toBe(expected.digest);
 expect(reads.ownerRegistryDigestOnceV1(apply(fixture.registry,fixture.edits[2]!))).toBe(expected.digest);
});

test("neither the key of the pass memo nor the key of the project farm names the registry",()=>{
 const nodes=readFileSync(join(PROJECT_NODES,"🟨️.mjs"),"utf8"),farm=readFileSync(join(PROJECT_NODES,"🏊️project-farm","🟨️.mjs"),"utf8");
 const context=nodes.slice(nodes.indexOf("memoContext=takenMemo===undefined"),nodes.indexOf("memoKeyOf=job=>"));
 expect(context).toContain("serialize([registryDigest,commandRevision,late,[...projectsByRoot],componentRoots,contracts])");
 expect(context).not.toContain("context.registry");
 const key=farm.slice(farm.indexOf("function hubKeyV1"),farm.indexOf("/** 🏗️ Creates the shared state of one hub"));
 expect(key).not.toContain("registry");
 expect(farm).toContain("pool.farm.run({ ...job, registryDigest }, { signal })");
 expect(farm).toContain("pool.farm.run({ ...job, registryDigest }, { signal: jobSignal })");
});
