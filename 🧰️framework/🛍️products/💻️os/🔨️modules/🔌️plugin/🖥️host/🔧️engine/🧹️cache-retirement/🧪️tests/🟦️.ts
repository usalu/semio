/** 🧪️ Contract: the compiled Wasmtime cache has a schema-declared size cap and age limit, retired least recently used first; an independent TypeScript oracle judges the same fixture as the Rust selector, and the engine start of the runtime goes through the retirement. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import policy from "../⚙️configuration/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

type EntryV1={name:string;bytes:number;idleSeconds:number;scratch?:boolean};
type LimitsV1={maximumBytes:number;maximumAgeSeconds:number;minimumIdleSeconds:number;scratchGraceSeconds:number};

const oracle=(entries:readonly EntryV1[],limits:LimitsV1):string[]=>{
 const retired=new Set<string>();
 let kept=0;
 const candidates:EntryV1[]=[];
 for(const entry of entries){
  if(entry.scratch===true){if(entry.idleSeconds>limits.scratchGraceSeconds)retired.add(entry.name);continue;}
  if(entry.idleSeconds>=limits.minimumIdleSeconds&&entry.idleSeconds>limits.maximumAgeSeconds){retired.add(entry.name);continue;}
  kept+=entry.bytes;
  if(entry.idleSeconds>=limits.minimumIdleSeconds)candidates.push(entry);
 }
 candidates.sort((left,right)=>right.idleSeconds-left.idleSeconds||(left.name<right.name?-1:1));
 for(const entry of candidates){if(kept<=limits.maximumBytes)break;kept-=entry.bytes;retired.add(entry.name);}
 return [...retired].sort();
};

test("the shipped limits are declared in the schema-checked JSON: 8 GiB cap and 14 days",()=>{
 expect(validateJsonSchemaSubset(schema as never,policy)).toEqual([]);
 expect(policy.maximumBytes).toBe(8*1024**3);
 expect(policy.maximumAgeSeconds).toBe(14*86400);
 expect(policy.minimumIdleSeconds).toBeGreaterThan(0);
 expect(policy.minimumIntervalSeconds).toBeGreaterThanOrEqual(3600);
});

test("a policy with an unknown or missing key is refused by the schema",()=>{
 expect(validateJsonSchemaSubset(schema as never,{...policy,extra:1}).length).toBeGreaterThan(0);
 const {maximumBytes:_,...partial}=policy;
 expect(validateJsonSchemaSubset(schema as never,partial).length).toBeGreaterThan(0);
});

test("the TypeScript oracle retires exactly what the shared fixture declares",()=>{
 for(const row of fixture.cases)expect(oracle(row.entries as EntryV1[],fixture.policy),row.name).toEqual([...row.retired].sort());
});

test("the engine start of the plugin runtime prepares its cache root through the retirement",()=>{
 const host=join(import.meta.dir,"..","..","..");
 const runtime=readFileSync(join(host,"🎭️guest-runtime","🐎️wasmtime-runtime","🦀️.rs"),"utf8");
 expect(runtime).toContain("cache_root: prepared_compiled_cache_root().await");
 expect(runtime).not.toContain("default_compiled_cache_root");
 const engine=readFileSync(join(host,"🔧️engine","🦀️.rs"),"utf8");
 expect(engine).toContain("retire_compiled_cache_if_due(&root,");
 expect(engine).toContain("touch_compiled_cache_entry(path,");
});
