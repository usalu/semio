/** 🧮️ Contract: the build preflight lists every unmet requirement of a host exactly as an independent Python decision table does, over the exhaustive sweep of the axes in `./🔣️.json` for the authored platform rows and two synthetic ones; invalid requirements and facts are refused by the same verdict ajv gives, and only the bare command is accepted. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildPreflightProblemsV1,readBuildPreflightArgumentsV1,readBuildPreflightPolicyV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import policyFile from "../../🔣️policy.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(rows:unknown[]):string[][]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8",maxBuffer:128*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const subsets=<T>(items:readonly T[]):T[][]=>items.reduce<T[][]>((all,item)=>all.flatMap(set=>[set,[...set,item]]),[[]]);
const policy=readBuildPreflightPolicyV1();
const requirements=corpus.requirements.map(row=>({name:row.name,value:("policy" in row?(policy.platforms as Record<string,unknown>)[row.policy as string]:row.value) as any}));

test("the corpus is closed by its schema and the authored policy is closed by the unit's schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,axes:{...corpus.axes,exchange:["maybe"]}})).toBe(false);
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Policy"},policyFile),JSON.stringify(ajv.errors)).toBe(true);
 expect<unknown>(policy).toEqual(policyFile);
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Policy"},{...policyFile,platforms:{...policyFile.platforms,freebsd:policyFile.platforms.linux}})).toBe(false);
});

test("the exhaustive sweep of requirement sets and host facts equals the Python decision table cell by cell",()=>{
 const rows:{name:string;required:unknown;facts:unknown}[]=[];
 for(const requirement of requirements)for(const architecture of corpus.axes.architecture)for(const offset of corpus.axes.byteOffsets)for(const tools of subsets(corpus.axes.toolPool))for(const exchange of corpus.axes.exchange)for(const caseSensitive of corpus.axes.caseSensitive){
  const minimum=requirement.value.minimumAvailableBytes;
  rows.push({name:requirement.name+" "+architecture+" "+offset+" "+tools.join("+")+" "+exchange+" "+caseSensitive,required:requirement.value,facts:{...corpus.base,architecture,availableBytes:minimum+offset,tools,exchange,caseSensitive}});
 }
 const answers=ask(rows);let clean=0,problemSets=new Set<string>();
 for(const[index,row]of rows.entries()){
  const actual=buildPreflightProblemsV1(row.required,row.facts);expect(actual,row.name).toEqual(answers[index]);
  if(actual.length===0)clean++;problemSets.add(actual.length+":"+actual.map(text=>text.split(" ")[0]).join(","));
 }
 expect(rows.length).toBe(requirements.length*corpus.axes.architecture.length*corpus.axes.byteOffsets.length*2**corpus.axes.toolPool.length*corpus.axes.exchange.length*corpus.axes.caseSensitive.length);
 expect(clean).toBeGreaterThan(0);expect(problemSets.size).toBeGreaterThan(8);
});

test("invalid requirements and facts are refused with the same verdict as ajv and the authored messages",()=>{
 const ajv=new Ajv({strict:false}),platform=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Platform"}),facts=ajv.compile({definitions:schema.definitions,$ref:"#/definitions/Facts"}),good=policy.platforms.linux;
 for(const row of corpus.invalidRequirements){
  expect(platform(row.requirement),row.name).toBe(false);
  expect(refusal(()=>buildPreflightProblemsV1(row.requirement,corpus.base)),row.name).toBe("Closed build preflight requirements required");
 }
 for(const row of corpus.invalidFacts){
  const value:Record<string,unknown>={...corpus.base,...((row as any).patch??{})};if((row as any).drop)delete value[(row as any).drop];
  expect(facts(value),row.name).toBe(false);expect(refusal(()=>buildPreflightProblemsV1(good,value)),row.name).toBe("Closed build preflight facts required");
 }
 expect(facts(corpus.base)).toBe(true);expect(platform(good)).toBe(true);
});

test("only the bare preflight command is accepted",()=>{
 for(const args of corpus.arguments.accepted)expect(refusal(()=>readBuildPreflightArgumentsV1(args)),args.join(" ")).toBe("");
 for(const args of corpus.arguments.rejected)expect(refusal(()=>readBuildPreflightArgumentsV1(args)),JSON.stringify(args)).toBe("Expected repo preflight without arguments");
});
