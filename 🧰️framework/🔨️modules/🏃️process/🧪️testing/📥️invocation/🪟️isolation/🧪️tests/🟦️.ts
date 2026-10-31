import {beforeAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import primaryCorpus from "./📥️primary.json";
import reasonCorpus from "./✂️reasons.json";
import {isolatedBuildTestBudgetV1,isolatedBuildTestPrimaryMatchesV1,isolatedBuildTestFailureReasonV1} from "../🟦️.ts";
import {registerIsolatedBuildTestV1,registerBuildTestFixtureV1,withBuildTestInvocationV1} from "../../🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";

const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));
void withBuildTestInvocationV1(async()=>undefined);
const held=registerBuildTestFixtureV1<{preparedMilliseconds:number}>(async(original,retain)=>{
 const control=createIssuedResourceControlV1(original,"isolation:preparation",0),born=performance.now();checkResourceLeaseControl(control);
 await sleep(2500);checkResourceLeaseControl(control);await retain({preparedMilliseconds:performance.now()-born});checkResourceLeaseControl(control);
});
beforeAll(()=>held.run(async()=>undefined),20000);

test("isolated phase budgets agree with schema and independent SQL",()=>{
 const db=new Database(":memory:"),validate=new Ajv({strict:true}).compile(schema);
 try{for(const row of corpus){
  const policy={bodyMilliseconds:row.body,scope:row.scope==="body"?{kind:"body"}:{kind:"file",maximumElapsedMilliseconds:row.file}},oracle=db.query("select case when ?1>=2000 and (?2='body' or ?3>=?1) and (?5 is null or ?5>0) and (?2='body' or ?3>?4) then min(case when ?2='body' then ?1 else ?3-?4 end,coalesce(?5,2147483647)) end budget").get(row.body,row.scope,row.file,row.age,row.original) as {budget:number|null};
  expect(oracle.budget).toBe(row.budget);if(row.budget===null)expect(()=>isolatedBuildTestBudgetV1(policy,row.age,row.original)).toThrow();else{expect(validate(policy)).toBe(true);expect(isolatedBuildTestBudgetV1(policy,row.age,row.original)).toBe(row.budget);}
 }}finally{db.close();}
});

test("isolated primary observation distinguishes expected failure from independent cleanup",()=>{
 const db=new Database(":memory:");try{for(const row of primaryCorpus){
  const oracle=db.query("select ?1='owned' and ?2=0 and not exists(select 1 from json_each(?3) where json_extract(value,'$.reason')<>'wanted' or (cast(key as integer)>0 and json_extract(value,'$.phase')='cleanup')) accepted").get(row.test,Number(row.overflow),JSON.stringify(row.failures)) as {accepted:number};
  expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(isolatedBuildTestPrimaryMatchesV1({version:1,test:row.test,overflow:row.overflow,failures:row.failures},"owned",/^wanted$/u)).toBe(row.accepted);
 }}finally{db.close();}
});

registerIsolatedBuildTestV1(import.meta.path,"prepared isolated observation keeps its genuine preparation outside the body",{bodyMilliseconds:2000,scope:{kind:"file",maximumElapsedMilliseconds:20000}},()=>withBuildTestInvocationV1(()=>held.run(async value=>{expect(value.preparedMilliseconds).toBeGreaterThanOrEqual(2400);}))); 
registerIsolatedBuildTestV1(import.meta.path,"prepared isolated observation never widens its authored body timeout",{bodyMilliseconds:2000,scope:{kind:"file",maximumElapsedMilliseconds:20000}},checkpoint=>withBuildTestInvocationV1(original=>held.run(async()=>{checkpoint();await sleep(2500);original.control.signal.throwIfAborted();})),/Build test receiver closed with active callback/u);

test("isolated failure truncation is explicit against independent SQLite lengths",()=>{const db=new Database(":memory:");try{for(const row of reasonCorpus){const reason=row.character.repeat(row.count),oracle=db.query("select substr(?1,1,?2) reason,length(?1)>?2 overflow").get(reason,schema.definitions.Protocol.const.maximumReasonCharacters) as {reason:string;overflow:number};expect(oracle.reason.length).toBe(row.retained);expect(Boolean(oracle.overflow)).toBe(row.overflow);expect(isolatedBuildTestFailureReasonV1(Error(reason))).toEqual({reason:oracle.reason,overflow:Boolean(oracle.overflow)});}}finally{db.close();}});
