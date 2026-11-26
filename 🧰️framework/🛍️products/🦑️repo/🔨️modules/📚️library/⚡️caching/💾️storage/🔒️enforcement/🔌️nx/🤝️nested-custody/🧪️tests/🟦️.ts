/** 🧪️ Nested Nx cache custody laws over the real task configuration of `@semio-tech/norm-din4108:build` (outputs `{projectRoot}/.🧾️store/build.json` and `dist`): the miss-run-put path, the remote-hit-restore path inside the shared batch resolution (the wall of g1-m10, 2026-11-18 04:04 UTC) and concurrent nested custodies, against an independent truth-table oracle and the closed schema. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {admitNestedNxCustodyV1,NxNestedCustodyRefusedErrorV1,nxNestedCustodyTaskV1,type NxNestedCustodyHeldV1,type NxNestedCustodyModeV1} from "../🟦️.ts";
import {classifyNxOutputV1} from "../../📦️outputs/🟦️.ts";
const workspace="/Users/ueli/Documents/semio",project=workspace+"/🧰️framework/🧮️norm/🏛️din4108",receipt=project+"/.🧾️store/build.json",dist=project+"/dist",digest="a".repeat(64),implementation="b".repeat(64),tooling="c".repeat(64);
const environment:Record<string,string|undefined>={SEMIO_BUILD_GRAPH_DIGEST:digest,SEMIO_NX_IMPLEMENTATION_IDENTITY:implementation,NX_TASK_TARGET_PROJECT:"@semio-tech/norm-din4108",NX_TASK_TARGET_TARGET:"build",NX_TASK_TARGET_CONFIGURATION:"prototype"};
const held=(mode:NxNestedCustodyModeV1,outputs:readonly string[]=[]):NxNestedCustodyHeldV1=>({workspace,toolingIdentity:tooling,environment,graphIdentity:digest,implementationIdentity:implementation,mode,outputs:new Map(outputs.map(path=>[path,"shared"])),cohorts:new Map()});
const request=(mode:NxNestedCustodyModeV1,paths:readonly string[]=[receipt,dist])=>({workspace,toolingIdentity:tooling,environment,mode,paths});
const validate=new Ajv({strict:false}).addSchema(schema).compile({$ref:schema.$id+"#/definitions/Refusal"});
const refusal=(action:()=>unknown):NxNestedCustodyRefusedErrorV1=>{try{action();}catch(error){expect(error).toBeInstanceOf(NxNestedCustodyRefusedErrorV1);return error as NxNestedCustodyRefusedErrorV1;}throw Error("a refusal was expected");};
test("the receipts output of the real task is a declared receipts partition",()=>{expect(classifyNxOutputV1({rust:workspace+"/.cache/b/t/m",products:workspace+"/.cache/b/p/m",receipts:workspace},receipt).partition).toBe("receipts");});
test("miss, run, put: a put inside no custody is not nested; the first custody is its own scope",()=>{const outer=held("exclusive");expect(admitNestedNxCustodyV1(outer,request("exclusive"))).toBe(outer);});
test("hit and restore: a remote hit restores through an exclusive custody inside the shared batch resolution that holds no output",()=>{
 const outer=held("shared"),inner=admitNestedNxCustodyV1(outer,request("exclusive"));
 expect(inner.mode).toBe("exclusive");expect(outer.mode).toBe("shared");expect(inner.workspace).toBe(outer.workspace);expect(inner.environment).toBe(outer.environment);expect(Object.isFrozen(inner)).toBe(true);
});
test("hit and restore: a restore inside the shared batch resolution and the apply of cached results stay shared",()=>{const outer=held("shared");expect(admitNestedNxCustodyV1(outer,request("shared"))).toBe(outer);});
test("an exclusive custody may nest a shared one",()=>{const outer=held("exclusive",[receipt]);expect(admitNestedNxCustodyV1(outer,request("shared"))).toBe(outer);});
test("a reader that holds outputs cannot become a writer: typed bilingual refusal naming task, paths and rule",()=>{
 const error=refusal(()=>admitNestedNxCustodyV1(held("shared",[receipt]),request("exclusive")));
 expect(error.rule).toBe("reader-holds-outputs");expect(error.task).toBe("@semio-tech/norm-din4108:build:prototype");expect(error.paths).toEqual([receipt,dist]);expect(validate(error.toJSON())).toBe(true);
 for(const text of [error.en,error.de]){expect(text).toContain("@semio-tech/norm-din4108:build:prototype");expect(text).toContain(receipt);expect(text).toContain(dist);}
 expect(error.en).toContain("Original nested cache custody refused");expect(error.en).toContain("cannot become a writer");expect(error.de).toContain("Verschachtelter Cache-Gewahrsam verweigert");expect(error.de).toContain("kein Schreiber");expect(error.message).toBe(error.en+" | "+error.de);
});
const mismatches:readonly [string,(value:NxNestedCustodyHeldV1)=>NxNestedCustodyHeldV1,string][]=[
 ["workspace",value=>({...value,workspace:"/elsewhere"}),"workspace"],
 ["tooling",value=>({...value,toolingIdentity:"d".repeat(64)}),"tooling"],
 ["environment",value=>({...value,environment:{...environment}}),"environment"],
 ["graph",value=>({...value,graphIdentity:"e".repeat(64)}),"graph"],
 ["implementation",value=>({...value,implementationIdentity:"f".repeat(64)}),"implementation"]
];
for(const [name,change,rule] of mismatches)test("a different "+name+" is refused with the rule "+rule,()=>{
 const error=refusal(()=>admitNestedNxCustodyV1(change(held("exclusive")),request("exclusive")));
 expect<unknown>(error.rule).toBe(rule);expect(validate(error.toJSON())).toBe(true);expect(error.en).toContain("on "+receipt);expect(error.de).toContain("auf "+receipt);
});
test("the refusal without a task name in the environment says so in both languages",()=>{
 const bare={...environment,NX_TASK_TARGET_PROJECT:undefined},error=refusal(()=>admitNestedNxCustodyV1({...held("shared",[receipt]),environment:bare},{...request("exclusive"),environment:bare}));
 expect(error.task).toBeNull();expect(error.en).toContain("unnamed task");expect(error.de).toContain("unbenannte Aufgabe");expect(validate(error.toJSON())).toBe(true);expect(nxNestedCustodyTaskV1({})).toBeNull();
});
test("every combination agrees with an independent truth-table oracle",()=>{
 const modes:NxNestedCustodyModeV1[]=["shared","exclusive"],bits=[false,true];let checked=0;
 for(const heldMode of modes)for(const requested of modes)for(const holds of bits)for(const same of bits){
  const outer={...held(heldMode,holds?[receipt]:[]),graphIdentity:same?digest:"0".repeat(64)},expected=!same?"graph":heldMode==="shared"&&requested==="exclusive"?holds?"reader-holds-outputs":"upgrade":"same";
  let actual:string;try{const inner=admitNestedNxCustodyV1(outer,request(requested));actual=inner===outer?"same":inner.mode===requested&&heldMode!==requested?"upgrade":"wrong";}catch(error){actual=(error as NxNestedCustodyRefusedErrorV1).rule;}
  expect(actual).toBe(expected);checked++;
 }
 expect(checked).toBe(16);
});
test("concurrent nested custodies inside one batch resolution are admitted independently",async()=>{
 const outer=held("shared"),admitted=await Promise.all(Array.from({length:64},async(_,at)=>{await Promise.resolve();return admitNestedNxCustodyV1(outer,request(at%2?"exclusive":"shared",[project+"/p"+at+"/.🧾️store/build.json"]));}));
 expect(admitted.filter(value=>value.mode==="exclusive").length).toBe(32);expect(admitted.filter(value=>value===outer).length).toBe(32);expect(outer.mode).toBe("shared");
});
