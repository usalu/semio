/** 🧭️ Contract: a `resolve` row consults exactly the directory of its target and the target, and only a relative, absolute or `node:` request is ever vouched; an unsound set would let a stale resolution survive an edit. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {resolveConsultedPathsV1} from "../🟦️.ts";

test("the consulted paths equal the fixture and an independent CPython model",()=>{
  expect(new Ajv({strict:false}).validate({$ref:"#/definitions/Fixture",definitions:schema.definitions},fixture)).toBe(true);
  const actual=fixture.cases.map(row=>resolveConsultedPathsV1(JSON.stringify([row.from,row.request]),row.fact)??null);
  expect(actual).toEqual(fixture.cases.map(row=>row.consulted));
  const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  expect(JSON.parse(run.stdout)).toEqual(actual);
});

test("random requests agree with the model",()=>{
  const parts=["a","b","..",".","c.ts","d"],rows:{from:string;request:string;fact:string}[]=[];
  let seed=11;const random=(n:number)=>(seed=(seed*1103515245+12345)%2147483648)%n;
  for(let at=0;at<200;at++){
    const request=(random(3)===0?"/w/":random(3)===0?"":"./")+Array.from({length:1+random(3)},()=>parts[random(parts.length)]).join("/"),from="/w/"+Array.from({length:1+random(3)},()=>parts[random(2)]).join("/")+"/f.ts";
    rows.push({from,request,fact:random(4)===0?"!ENOENT":random(2)===0?from.replace("f.ts","c.ts"):"/w/"+parts[random(parts.length)]+".ts"});
  }
  const input={cases:rows.map(row=>({...row,consulted:null}))},path=join(process.env.TMPDIR??"/tmp","h60-resolve-consulted-"+process.pid+".json");
  require("node:fs").writeFileSync(path,JSON.stringify(input));
  try{
    const run=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),path],{encoding:"utf8"});
    expect(run.status,run.stderr).toBe(0);
    expect(JSON.parse(run.stdout)).toEqual(rows.map(row=>resolveConsultedPathsV1(JSON.stringify([row.from,row.request]),row.fact)??null));
  }finally{require("node:fs").rmSync(path,{force:true});}
});
