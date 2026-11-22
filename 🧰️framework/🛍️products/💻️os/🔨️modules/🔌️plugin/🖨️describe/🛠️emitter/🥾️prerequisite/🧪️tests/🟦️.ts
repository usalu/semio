import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import {classifyLeaseResourceV1,leaseOrderPolicyV1} from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🧭️order/🟦️.ts";
import {staleDescriptorEmitterInputsV1} from "../../🌡️freshness/🟦️.ts";
import {descriptorEmitterPrerequisiteResourceV1,ensureFreshDescriptorEmitterWithV1,type DescriptorEmitterPrerequisitePortsV1,type DescriptorEmitterStateV1} from "../🟦️.ts";

const ABC="ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

/** 🧪️ A scripted emitter whose state each inspection pops, counting publications and recording the order of events. */
function scripted(states:readonly DescriptorEmitterStateV1[],publish:()=>Promise<void>=async()=>{}){
 const events:string[]=[],queue=[...states];
 const ports:DescriptorEmitterPrerequisitePortsV1={
  inspect:async()=>{events.push("inspect");return queue.length>1?queue.shift()!:queue[0]!;},
  publish:async()=>{events.push("publish");await publish();},
  serialize:async body=>{events.push("lock");try{return await body();}finally{events.push("unlock");}},
  report:line=>{events.push("report:"+line.split(";")[0]!);},
 };
 return {ports,events};
}

test("a fresh emitter is left alone without taking the publication lock",async()=>{
 const {ports,events}=scripted([{kind:"fresh"}]);
 expect(await ensureFreshDescriptorEmitterWithV1(ports)).toBe("fresh");
 expect(events).toEqual(["inspect"]);
});

test.each([
 [{kind:"stale",changed:2,first:"src/a.rs"} as const,/Descriptor emitter is stale \(2 changed source file\(s\), first: src\/a\.rs\).*Deskriptor-Emitter ist veraltet \(2 geänderte/su],
 [{kind:"missing"} as const,/not published.*nicht veröffentlicht/su],
 [{kind:"unreadable",reason:"bad receipt"} as const,/unusable \(bad receipt\).*unbrauchbar \(bad receipt\)/su],
])("a %o emitter is republished under the lock, after inspection released its reader",async(state,message)=>{
 const lines:string[]=[],{ports,events}=scripted([state]);
 expect(await ensureFreshDescriptorEmitterWithV1({...ports,report:line=>lines.push(line)})).toBe("built");
 expect(events).toEqual(["inspect","lock","inspect","publish","unlock"]);
 expect(lines.join("\n")).toMatch(message);
});

test("a concurrent caller that finds the emitter republished while it waited builds nothing",async()=>{
 const {ports,events}=scripted([{kind:"stale",changed:1,first:"x"},{kind:"fresh"}]);
 expect(await ensureFreshDescriptorEmitterWithV1(ports)).toBe("fresh");
 expect(events).toEqual(["inspect","lock","inspect","unlock"]);
});

test("several concurrent builds republish exactly once through one serializing lock",async()=>{
 let published=false,publications=0,chain=Promise.resolve();
 const make=():DescriptorEmitterPrerequisitePortsV1=>({
  inspect:async()=>published?{kind:"fresh"}:{kind:"stale",changed:1,first:"x"},
  publish:async()=>{publications++;await new Promise(done=>setTimeout(done,20));published=true;},
  serialize:body=>{const run=chain.then(body);chain=run.then(()=>undefined,()=>undefined);return run;},
  report:()=>{},
 });
 const outcomes=await Promise.all([1,2,3,4].map(()=>ensureFreshDescriptorEmitterWithV1(make())));
 expect(publications).toBe(1);
 expect(outcomes.filter(outcome=>outcome==="built")).toHaveLength(1);
});

test("a failed rebuild is a clear bilingual error naming the build target, keeping its cause",async()=>{
 const cause=new Error("linker exploded"),{ports}=scripted([{kind:"missing"}],async()=>{throw cause;});
 const error=await ensureFreshDescriptorEmitterWithV1(ports).then(()=>undefined,failure=>failure as Error);
 expect(error?.message).toMatch(/Descriptor emitter prerequisite build failed: linker exploded\. Run "repo run @semio-tech\/os-plugin-describe-rs:build".*Voraussetzungs-Build des Deskriptor-Emitters ist fehlgeschlagen: linker exploded\./su);
 expect(error?.cause).toBe(cause);
});

test("cancellation during the rebuild propagates unwrapped",async()=>{
 const controller=new AbortController(),{ports}=scripted([{kind:"missing"}],async()=>{controller.abort(new Error("cancelled by dev"));throw new Error("child killed");});
 await expect(ensureFreshDescriptorEmitterWithV1({...ports,signal:controller.signal})).rejects.toThrow("cancelled by dev");
});

test("the publication lock is a tools singleton ranked before the store and artifact custody it then takes",()=>{
 const policy=leaseOrderPolicyV1({}),rank=(resource:string)=>classifyLeaseResourceV1(policy,resource).rank;
 const resource=descriptorEmitterPrerequisiteResourceV1({mode:"prototype",platform:"darwin"});
 expect(resource).toBe("tools:descriptor-emitter:prototype:darwin");
 expect(rank(resource)).toBe(2);
 expect(rank(resource)).toBeLessThan(rank("artifact:descriptor-emitter"));
 expect(rank(resource)).toBeLessThan(rank("repo:compiler-writer:x"));
 expect(descriptorEmitterPrerequisiteResourceV1({mode:"release",platform:"linux"})).not.toBe(resource);
});

test("independent oracle: the stale decision flips exactly when a hashed source changes",async()=>{
 const root=mkdtempSync(join(tmpdir(),"emitter-prereq-")),db=new Database(":memory:");
 try{
  const file=join(root,"a.rs"),receipt=JSON.stringify({strings:[file],inputs:[[0,"file",ABC]]});
  writeFileSync(file,"abc");
  expect(createHash("sha256").update("abc").digest("hex")).toBe(ABC);
  const decide=async()=>{const {stale}=await staleDescriptorEmitterInputsV1(receipt,root);return stale.length?{kind:"stale",changed:stale.length,first:stale[0]!}:{kind:"fresh"};};
  const {ports,events}=scripted([await decide()]);
  expect(await ensureFreshDescriptorEmitterWithV1(ports)).toBe("fresh");
  writeFileSync(file,"abd");
  const verdict=db.query("select case when ?1=?2 then 'fresh' else 'stale' end as kind").get(ABC,createHash("sha256").update("abd").digest("hex")) as {kind:string};
  expect(verdict.kind).toBe("stale");
  const second=scripted([await decide()]);
  expect(await ensureFreshDescriptorEmitterWithV1(second.ports)).toBe("built");
  expect(events).toEqual(["inspect"]);
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
});
