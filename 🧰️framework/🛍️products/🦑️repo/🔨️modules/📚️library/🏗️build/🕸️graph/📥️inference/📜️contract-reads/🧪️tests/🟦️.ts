import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),nodes=await import("../../📦️project-nodes/🟨️.mjs"),reads=await import("../🟨️.mjs"),outputs=await import("../../🏭️generator-outputs/🟨️.mjs"),observation=await import("../../../../👁️observation/🟨️.mjs"),portable=await import("../../../../🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");

const record=(contracts:Record<string,unknown>)=>{const tracked=reads.trackGeneratorContractsV1(contracts);outputs.generatorOutputOwners(tracked.view,fixture.owner,"pa",{generate:{},other:{}});return tracked.tree();};

test("what an owner read of the generator contracts stays valid exactly while the facts its filters consult keep their values, as an independent model says",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 expect(inference).toBeDefined();
 const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as boolean[],tree=record(fixture.contracts);
 expect(reads.generatorContractReadsHoldV1(fixture.contracts,tree)).toBe(true);
 const actual=fixture.edits.map(edit=>{const changed=structuredClone(fixture.contracts) as Record<string,Record<string,unknown>>;changed[edit.id]![edit.field]=edit.value;return reads.generatorContractReadsHoldV1(changed,tree);});
 expect(actual).toEqual(expected);
 expect(expected.slice(0,3)).toEqual([true,true,true]);expect(expected.slice(3,5)).toEqual([false,false]);
 const added=structuredClone(fixture.contracts) as Record<string,unknown>;added["gen-z"]={ownership:"owned",ownerPath:"own/z",target:"pz:generate",outputRoots:[]};
 expect(reads.generatorContractReadsHoldV1(added,tree)).toBe(false);
 const removed=structuredClone(fixture.contracts) as Record<string,unknown>;delete removed["gen-c"];
 expect(reads.generatorContractReadsHoldV1(removed,tree)).toBe(false);
});

test("the key of an owner is the same for a whole read and a selected read and carries nothing of the generator contracts",()=>{
 const globals={registryDigest:"r1",commandRevision:"c1",late:true},plain=reads.ownerRecordHeadV1(globals,{native:false,pipeline:false,assemblyAuthority:undefined}),rust=reads.ownerRecordHeadV1(globals,{native:true,pipeline:false,assemblyAuthority:"asm"});
 expect(plain).toEqual(["r1",null,null,"closure-references/4"]);expect(rust).toEqual(["r1","c1","asm","closure-references/4"]);
 const json={name:"x",root:"own/a",targets:{}},whole=nodes.projectRecordKeyV1(plain,"own/a",json,undefined,[]),selected=nodes.projectRecordKeyV1(plain,"own/a",json,null,[]);
 expect(nodes.ownerRecordKeyTextV1(whole)).toBe(nodes.ownerRecordKeyTextV1(selected));
 expect(nodes.ownerRecordKeyTextV1(nodes.projectRecordKeyV1(reads.ownerRecordHeadV1({...globals,registryDigest:"r2"},{native:false,pipeline:false,assemblyAuthority:undefined}),"own/a",json,undefined,[]))).not.toBe(nodes.ownerRecordKeyTextV1(whole));
});

test("a global row holds while its resolver says so, never without one, and two checkouts at different paths write the same neutral row",()=>{
 const name="test-global",leaf=["gen-a","ownerPath"],row=(root:string)=>({kind:"global",path:JSON.stringify([name,root,leaf]),fact:JSON.stringify("own/a")});
 expect(observation.observationRowFreshV1(row("/Users/a/semio"))).toBe(false);
 observation.registerGlobalFactV1(name,(root:string,segments:readonly string[],fact:unknown)=>root.startsWith("/")&&reads.generatorContractReadHoldsV1(fixture.contracts,segments,fact));
 expect(observation.observationRowFreshV1(row("/Users/a/semio"))).toBe(true);
 const changed=structuredClone(fixture.contracts) as Record<string,Record<string,unknown>>;changed["gen-a"]!.ownerPath="own/q";
 observation.registerGlobalFactV1(name,(_root:string,segments:readonly string[],fact:unknown)=>reads.generatorContractReadHoldsV1(changed,segments,fact));
 expect(observation.observationRowFreshV1(row("/Users/a/semio"))).toBe(false);
 const hostOf=(root:string)=>({root,windows:false}),image=JSON.stringify({name:"x"}),recordOf=(root:string)=>({digest:"d",authority:[],observations:[row(root)],image});
 const one=portable.neutralRecordV1(recordOf("/Users/a/semio"),hostOf("/Users/a/semio")).record,two=portable.neutralRecordV1(recordOf("/home/b/work/semio"),hostOf("/home/b/work/semio")).record;
 expect(one).toEqual(two);expect(JSON.stringify(one)).not.toContain("/Users/a");
});

test("every read is told with its property path and holds against the same contracts and no longer against an edited one",()=>{
 const told:[readonly string[],unknown][]=[],tracked=reads.trackGeneratorContractsV1(fixture.contracts,(segments:readonly string[],fact:unknown)=>{told.push([segments,fact]);});
 outputs.generatorOutputOwners(tracked.view,fixture.owner,"pa",{generate:{},other:{}});
 expect(told.length).toBeGreaterThan(10);expect(told.some(([segments])=>segments.at(-1)==="\u0000keys")).toBe(true);
 expect(told.every(([segments,fact])=>reads.generatorContractReadHoldsV1(fixture.contracts,segments,fact))).toBe(true);
 const changed=structuredClone(fixture.contracts) as Record<string,Record<string,unknown>>;changed["gen-a"]!.target="pa:renamed";
 expect(told.every(([segments,fact])=>reads.generatorContractReadHoldsV1(changed,segments,fact))).toBe(false);
 expect(reads.generatorContractReadHoldsV1(fixture.contracts,[],1)).toBe(false);
});
