import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {classifyFailureV1,createTrustTrackerV1,decideTrustV1,requireTrustShapeV1,sourcePathV1,trustPolicyV1,type TrustPartV1,type TrustV1} from "../🟦️.ts";

setDefaultTimeout(60_000);
type Step=Readonly<{at:number;do:string;key?:string;kind?:string;part?:string;startedMs?:number;trust?:unknown;tier?:string}>;
const isPart=(part:string|undefined):part is TrustPartV1=>part==="file"||part==="directory"||part==="walk";
const ajv=new Ajv({strict:true,allowUnionTypes:true}),python=testPythonV1();

test("the fixture satisfies the closed schema and the Python oracle agrees with every vector",()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(validate(fixture),JSON.stringify(validate.errors?.[0])).toBe(true);
 requireTrustShapeV1("Fixture",fixture,"fixture");
 if(python===null)return;
 const result=Bun.spawnSync([python,"-I",join(import.meta.dirname,"🐍️.py")],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stdout.toString()+result.stderr.toString()).toBe(0);
});

test("the policy constant of the code equals the schema constant",()=>{
 expect((schema.definitions.Policy as {const:unknown}).const).toEqual(trustPolicyV1);
 requireTrustShapeV1("Policy",trustPolicyV1,"policy");
});

test("the scope of every path of the fixture is decided as declared",()=>{
 for(const item of fixture.scopes)expect(sourcePathV1(item.path),item.path).toBe(item.inScope);
});

test("every failure key of the fixture is classified as declared",()=>{
 expect(fixture.failures.length).toBeGreaterThan(30);
 for(const item of fixture.failures)expect<unknown>(classifyFailureV1(item.key),item.name).toBe(item.class);
});

test("every health file of the fixture gets its tier",()=>{
 for(const item of fixture.tiers)expect<unknown>(decideTrustV1(item.health as never,item.now,()=>item.alive),item.name).toBe(item.expected);
});

test("every tracker sequence of the fixture yields the declared statement and tier",()=>{
 for(const item of fixture.trackers){
  let now=0;
  const tracker=createTrustTrackerV1(()=>now);
  const steps:readonly Step[]=item.steps;
  for(const [index,step] of steps.entries()){
   now=step.at;
   const label=`${item.name} #${index}`;
   if(step.do==="failure")tracker.failure(step.key!);
   else if(step.do==="lost")tracker.lost(step.kind!);
   else if(step.do==="verified"){if(!isPart(step.part))throw new Error(label);tracker.verified(step.part,step.startedMs!);}
   else{
    expect(tracker.record(),label).toEqual(step.trust as TrustV1);
    expect<unknown>(decideTrustV1({wallMs:now,pid:1,store:"ok",trust:tracker.record()},now,()=>true),label).toBe(step.tier!);
   }
  }
 }
});

test("the lifetime counters that kept a healthy daemon at verify (126 360 blocked pack events, 9 refused ticket paths, 4 attest events) do not matter any more",()=>{
 const tracker=createTrustTrackerV1(()=>5000);
 tracker.verified("file",1000);tracker.verified("directory",1000);
 for(let index=0;index<1000;index++){tracker.failure("blocked:objects:pack-incomplete");tracker.failure(`refused:.🧬semio/🦑️repo/🎫️tickets/a/${index}.md:ticket-boundary`);tracker.failure("attest:x");}
 const health={wallMs:5000,pid:1,store:"ok",trust:tracker.record(),failures:[{kind:"blocked",count:126360,lastMs:4999}]};
 expect(decideTrustV1(health,5000,()=>true)).toBe("trusted");
 tracker.failure("blocked:src/a.rs:EACCES");
 expect(decideTrustV1({...health,trust:tracker.record()},5000,()=>true)).toBe("verify");
});
