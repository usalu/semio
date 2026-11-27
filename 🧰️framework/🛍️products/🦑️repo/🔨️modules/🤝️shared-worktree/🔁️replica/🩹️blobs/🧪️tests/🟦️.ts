import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {readOthersWantsV1,readWantsV1,validWantsV1,wantsLimitV1,writeWantsV1,type BlobWantsV1} from "../🟦️.ts";

const scratch:string[]=[],python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py"),fixtures=join(import.meta.dirname,"🧫️fixtures","🔣️.json");
const temp=():string=>{const path=mkdtempSync(join(tmpdir(),"blobs-"));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const ajv=new Ajv({strict:true}),ajvValid=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Wants"});
const vectors=fixture as unknown as {valid:{id:string;value:unknown}[];invalid:{id:string;value:unknown}[]};

test("the wants record is accepted and refused alike by our validator, by ajv against the schema and by the Python oracle",()=>{
 const lines=python===null?null:Bun.spawnSync([python,oracle,fixtures]).stdout.toString().trim().split("\n");
 if(lines!==null)expect(lines.length).toBe(vectors.valid.length+vectors.invalid.length);
 for(const kind of ["valid","invalid"] as const)for(const vector of vectors[kind]){
  const expected=kind==="valid";
  expect([kind,vector.id,validWantsV1(vector.value),ajvValid(vector.value)]).toEqual([kind,vector.id,expected,expected]);
  if(lines!==null)expect(lines).toContain(`${kind}:${vector.id}:${expected}`);
 }
 expect(wantsLimitV1).toBe(64);
});

test("wants are written atomically into the own origin directory, read back closed, and the others' non-empty wants are listed without the own",async()=>{
 const store=temp(),record=(originId:string,count:number):BlobWantsV1=>({version:1,originId,wallMs:1,wants:Array.from({length:count},(_,index)=>({sha256:index.toString(16).padStart(64,"0"),path:`f${index}`}))});
 for(const origin of ["a","b","c"])mkdirSync(join(store,"v1","origins",origin),{recursive:true});
 await writeWantsV1(store,record("a",2));await writeWantsV1(store,record("b",0));await writeWantsV1(store,record("c",1));
 expect(await readWantsV1(store,"a")).toEqual(record("a",2));
 expect((await readOthersWantsV1(store,"a")).map(item=>item.originId)).toEqual(["c"]);
 expect((await readOthersWantsV1(store,"b")).map(item=>item.originId)).toEqual(["a","c"]);
 await expect(writeWantsV1(store,record("a",wantsLimitV1+1))).rejects.toThrow();
 expect(await readWantsV1(store,"missing")).toBeUndefined();
});
