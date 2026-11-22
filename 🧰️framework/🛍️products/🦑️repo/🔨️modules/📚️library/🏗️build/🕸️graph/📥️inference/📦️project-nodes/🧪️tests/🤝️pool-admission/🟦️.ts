import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{ownerAdmissionOfV1,ownerAdmissionSelectionV1,ownerJobV1,projectRecordKeyV1}from"../../🟨️.mjs";
import{ownerPoolIdV1}from"../../🌐️owner-pool/🟦️.ts";

/** 🤝️ A machine that pulls an owner of a selected read files the record under the key the reading machine looks up: the admission travels with the unit and both sides build their job from the same function. */
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const workspaceRoot=mkdtempSync(join(tmpdir(),"pool-admission-"));scratch.push(workspaceRoot);
const environment={workspaceRoot,projectsByRoot:new Map(),componentRoots:new Map(),nativeCache:new Map(),nativeClosures:new Map()};
const globals={registryDigest:"r",commandRevision:"c",late:false};
const json={name:"p",root:"libs/p",targets:{build:{executor:"x"},test:{executor:"y"}}};
const admission={targets:{p:["build"]},inputs:{p:["b","a","a"]}};

test("a unit carries the admission of its owner and the job of the puller equals the selection of the reader",()=>{
 const unit=JSON.parse(JSON.stringify({name:"p",json,admission:ownerAdmissionOfV1(admission,{name:"p",json})}));
 expect(unit.admission).toEqual({targets:["build"],inputs:["a","b"]});
 const job=ownerJobV1(environment,globals,unit),reader=ownerAdmissionSelectionV1(admission,json);
 expect(job.selectedTargets).toEqual(reader.selectedTargets);
 expect(job.selectedInputs).toEqual(reader.selectedInputs);
 expect(job.admission).toBe(true);
 expect(projectRecordKeyV1(job.head,job.root,job.json,job.selectedTargets,job.selectedInputs)).toEqual([...job.head,"libs/p",json,["build"],["a","b"]]);
});

test("an owner the admission selects no target for is filed with no targets (null in the key), not with an empty list",()=>{
 const none={targets:{},inputs:{}},unit=JSON.parse(JSON.stringify({name:"p",json,admission:ownerAdmissionOfV1(none,{name:"p",json})}));
 expect("targets" in unit.admission).toBe(false);
 const job=ownerJobV1(environment,globals,unit);
 expect(job.selectedTargets).toBeUndefined();
 expect(projectRecordKeyV1(job.head,job.root,job.json,job.selectedTargets,job.selectedInputs).slice(-2)).toEqual([null,[]]);
});

test("a unit without an admission is the job it always was",()=>{
 const job=ownerJobV1(environment,globals,{name:"p",json});
 expect(job.selectedTargets).toBeUndefined();
 expect(job.selectedInputs).toEqual([]);
 expect("admission" in job).toBe(false);
 expect(projectRecordKeyV1(job.head,job.root,job.json,job.selectedTargets,job.selectedInputs).slice(-2)).toEqual([null,[]]);
});

test("the wave id tells two admissions of the same owners apart and stays the same without one",()=>{
 const head={version:1 as const,identity:"i",globals};
 const plain=[{name:"p",kind:"k",expectedMilliseconds:1,json}],one=[{...plain[0]!,admission:{targets:["build"],inputs:[]}}],two=[{...plain[0]!,admission:{targets:["test"],inputs:[]}}];
 expect(ownerPoolIdV1(head,one)).not.toBe(ownerPoolIdV1(head,two));
 expect(ownerPoolIdV1(head,one)).not.toBe(ownerPoolIdV1(head,plain));
 expect(ownerPoolIdV1(head,plain)).toBe(ownerPoolIdV1(head,[{...plain[0]!}]));
});
