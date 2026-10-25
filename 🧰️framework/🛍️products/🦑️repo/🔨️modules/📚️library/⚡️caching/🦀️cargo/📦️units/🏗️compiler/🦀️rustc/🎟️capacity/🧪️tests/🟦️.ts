/** 🎟 Contract: the capacity a Rust compilation asks of the host equals the authored profile rows in `./🔣️.json` and an independent Python derivation, every request is counted compile units (never the exclusive whole host): a default-backend, multi-unit, link-time-optimisation-free profile is bounded by its codegen units and the share of the cores, every other profile by the share of the cores, a profile outside the closed shape is refused, and every request is admitted by the capacity request schema of the build. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {rustCompilerCapacityRequestV1} from "../🟦️.ts";
import policy from "../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(profiles:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({cores:corpus.cores,profiles}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string|null=>{try{action();return null;}catch(error){return (error as Error).message;}};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,profiles:[{...corpus.profiles[0],request:{kind:"available",maximumParallelism:1,minimumParallelism:1}}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,profiles:[{...corpus.profiles[0],request:{kind:"whole-host"}}]})).toBe(false);
});

test("the resource classes are the ones the schema declares",()=>{
 expect(corpus.classes).toEqual({bounded:policy.definitions.Policy.const.boundedClass,exclusive:policy.definitions.Policy.const.exclusiveClass});
});

test("a profile asks for its codegen units within the share of the cores, and for the share of the cores otherwise",()=>{
 const answers=ask(corpus.profiles.map(row=>row.profile));
 for(const[index,row]of corpus.profiles.entries()){
  const actual=rustCompilerCapacityRequestV1(row.profile,corpus.cores);
  expect<unknown>(actual,JSON.stringify(row.profile)).toEqual({resourceClass:row.resourceClass,request:row.request});
  expect(answers[index],JSON.stringify(row.profile)).toEqual({resourceClass:row.resourceClass,request:row.request});
  expect(Object.isFrozen(actual)&&Object.isFrozen(actual.request)).toBe(true);
 }
 expect(corpus.profiles.filter(row=>row.resourceClass===corpus.classes.bounded).length).toBe(10);
});

test("a profile outside the closed shape is refused and the oracle refuses it too",()=>{
 const answers=ask(corpus.refused.map(row=>row.profile));
 for(const[index,row]of corpus.refused.entries()){
  expect(refusal(()=>rustCompilerCapacityRequestV1(row.profile as never)),row.name).toBe("Closed Rust compiler capacity profile required");
  expect(answers[index],row.name).toEqual({refused:true});
 }
});

test("the share of the cores equals the independent derivation for every host size",()=>{
 for(const row of corpus.shares){const actual=rustCompilerCapacityRequestV1({codegen_backend:null,codegen_units:null,lto:"off"},row.cores);expect(actual.request,String(row.cores)).toEqual({kind:"available",maximumParallelism:row.preferred,minimumParallelism:row.minimum});}
});
