/** 📋️ Contract: every observation of a private-clone package file gets the disposition the unit's own corpus in `./🔣️.json` and the extra corpus `./📋️extra.json` state, an exhaustive sweep of the decision table agrees with an independent Python decision table, and ajv agrees with the repository validator on what a closed observation is. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {nxPristineFileDispositionV1,type NxPristineFileObservationV1} from "../🟦️.ts";
import unitSchema from "../🧬️schema/🔣️.json";
import own from "./🔣️.json";
import corpus from "./📋️extra.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(rows:unknown[]):{value?:string;error?:string}[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const outcome=(value:unknown):{value?:string;error?:string}=>{try{return{value:nxPristineFileDispositionV1(value as NxPristineFileObservationV1)};}catch(error){return{error:(error as Error).message};}};

test("the corpora are closed by their schemas and the observation schema agrees between ajv and the repository validator",()=>{
 const ajv=new Ajv({strict:false}),observation={...unitSchema,$ref:"#/definitions/Observation"};
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],expected:"keep"}]})).toBe(false);
 for(const row of [...own.map(item=>({name:item.name,observation:item.observation})),...corpus.cases])expect(validateJsonSchemaSubset(unitSchema.definitions.Observation,row.observation,unitSchema).length===0,row.name).toBe(ajv.validate(observation,row.observation));
});

test("the unit's own cases hold: refusals throw and the rest classify",()=>{
 for(const row of own){const result=outcome(row.observation);if(row.expected===null)expect(result.error,row.name).toBeDefined();else expect(result,row.name).toEqual({value:row.expected});}
});

test("every extra case matches the corpus, the Python table and the authored message",()=>{
 const answers=ask(corpus.cases.map(row=>row.observation).filter(value=>validateJsonSchemaSubset(unitSchema.definitions.Observation,value,unitSchema).length===0));
 let at=0;
 for(const row of corpus.cases){
  const result=outcome(row.observation),closed=validateJsonSchemaSubset(unitSchema.definitions.Observation,row.observation,unitSchema).length===0;
  if(row.expected===null)expect(result.error,row.name).toBe(row.error);else expect(result,row.name).toEqual({value:row.expected});
  if(closed)expect(answers[at++],row.name).toEqual(row.expected===null?{error:row.error}:{value:row.expected});
 }
});

test("the exhaustive decision table over every axis equals the Python table cell by cell",()=>{
 const axes=corpus.truthTable,rows:NxPristineFileObservationV1[]=[];
 for(const path of axes.paths)for(const present of axes.present)for(const links of axes.links)for(const archive of axes.flags)for(const shared of axes.flags)for(const matches of axes.flags)rows.push({path,present,links,archive,shared,matches});
 const answers=ask(rows);expect(rows.length).toBe(axes.paths.length*axes.present.length*axes.links.length*axes.flags.length**3);
 for(const[index,row]of rows.entries())expect(outcome(row),JSON.stringify(row)).toEqual(answers[index]);
 const kinds=new Set(rows.map(row=>outcome(row).value));
 expect([...kinds].sort()).toEqual(["nested","preserve","remove","replace",undefined].sort());
});
