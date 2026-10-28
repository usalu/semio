import {expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {FILL_POLICY_V1 as policy,fillRatesV1,fillReportV1,readFillSamplesV1,recordFillSampleV1,type FillSampleV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true});ajv.addSchema(componentSchema,"fill.json");
const python=(rows:readonly unknown[]):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"../🧬️schema/🔣️.json")],{stdin:new TextEncoder().encode(JSON.stringify(rows))});expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);return JSON.parse(new TextDecoder().decode(run.stdout));};

test("the fixture is closed and the rates equal the independent Python oracle on every vector and on 300 seeded random sample series",()=>{
 const validate=ajv.compile({$ref:"fill.json#/definitions/Vector"});for(const vector of fixture.vectors)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.vectors)expect(fillRatesV1(vector.samples as never,vector.nowMs) as unknown,vector.name).toEqual(vector.rates);
 expect(python(fixture.vectors.map(vector=>({nowMs:vector.nowMs,samples:vector.samples})))).toEqual(fixture.vectors.map(vector=>vector.rates));
 let seed=20261024;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},now=1000*3600000,states:{nowMs:number;samples:FillSampleV1[]}[]=[];
 for(let index=0;index<300;index++){const samples:FillSampleV1[]=[];for(let row=0;row<next(9);row++){const sample:{atMs:number;freeBytes:number;volumeBytes:number;profiles?:Record<string,number>}={atMs:now-next(60*60)*60000+next(3)*7000,freeBytes:next(48)*1073741824+next(1000),volumeBytes:51539607552};if(next(2))sample.profiles={prototype:next(20)*1073741824,wasm:next(10)*1073741824,...next(2)?{debug:next(3)*1073741824}:{}};samples.push(sample);}states.push({nowMs:now,samples});}
 const expected=python(states);states.forEach((state,at)=>expect(fillRatesV1(state.samples,state.nowMs) as unknown,JSON.stringify(state).slice(0,160)).toEqual(expected[at]));
});

test("samples are appended to the telemetry file, a file above its maximum keeps its newest rows, broken rows are skipped, and the report speaks both languages",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"fill-"));
 try{
  expect(readFillSamplesV1(root)).toEqual([]);
  for(let index=0;index<3;index++)recordFillSampleV1(root,{atMs:index*1000,freeBytes:10-index,volumeBytes:100});
  writeFileSync(join(root,policy.file),readFileSync(join(root,policy.file),"utf8")+"not json\n{\"atMs\":\"x\"}\n");
  expect(readFillSamplesV1(root).map(row=>row.atMs)).toEqual([0,1000,2000]);
  writeFileSync(join(root,policy.file),Array.from({length:policy.keepRows+500},(_,index)=>JSON.stringify({atMs:index,freeBytes:1,volumeBytes:2,pad:"x".repeat(300)})).join("\n")+"\n");
  recordFillSampleV1(root,{atMs:10**9,freeBytes:1,volumeBytes:2});expect(readFillSamplesV1(root).length).toBeLessThanOrEqual(policy.keepRows+1);expect(readFillSamplesV1(root).at(-1)!.atMs).toBe(10**9);
  const rates=fillRatesV1(fixture.vectors[6]!.samples as never,fixture.vectors[6]!.nowMs),english=fillReportV1(rates,8*1073741824,"en"),german=fillReportV1(rates,8*1073741824,"de");
  expect(english).toContain("GiB/h");expect(german).toContain("GiB/h");expect(english).toContain("prototype");expect(fillReportV1(fillRatesV1([],0),0,"en")).toContain("not enough");expect(fillReportV1(fillRatesV1([],0),0,"de")).toContain("zu wenige");
 }finally{rmSync(root,{recursive:true,force:true});}
});
