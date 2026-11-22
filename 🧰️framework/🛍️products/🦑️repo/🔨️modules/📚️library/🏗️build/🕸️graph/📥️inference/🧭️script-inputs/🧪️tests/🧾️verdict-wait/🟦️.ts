import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import {loadProductFunctionsSyncV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import { closeSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import cases from "./🔣️.json" with {type:"json"};

const names=["observeVerdictV1","sharedVerdictV1"];

for(const row of cases)test("shared layer verdict " + row.mode,async()=>{
 const root=mkdtempSync(join(tmpdir(),"sv-")),file=join(root,"verdict"),controller=new AbortController(),primary=Error("original verdict cancellation"),db=new Database(":memory:");
 let computed=0,delays=0;
 const events:string[]=[],verdictSeenV1=new Map<string,boolean>(),verdictCountsV1={read:0,waited:0,written:0};
 const refused=()=>Object.assign(Error("permission denied"),{code:"EACCES"});
 const delay=async(_ms:number,_value?:unknown,options?:{signal?:AbortSignal})=>{
  delays++;events.push("delay");
  if(delays===1&&row.mode==="peer-finishes")writeFileSync(file,"0");
  if(delays===1&&row.mode==="peer-disappears")rmSync(file);
  if(row.mode==="cancel-wait")controller.abort(primary);
  options?.signal?.throwIfAborted();
 };
 const ports={
  dirname,verdictSeenV1,verdictCountsV1,vReadV1:readFileSync,vLstatV1:lstatSync,vFstatV1:fstatSync,vCloseV1:closeSync,vRmV1:rmSync,
  verdictDirMadeV1:(path:string)=>mkdirSync(path,{recursive:true}),
  vOpenV1:(path:string,flags:string)=>{if(row.mode==="unavailable")throw refused();return openSync(path,flags);},
  vWriteV1:(path:string|number,value:string,options?:{flag:string})=>{if(row.mode==="unavailable"&&options?.flag==="wx")throw refused();writeFileSync(path,value,options);},
  readVerdictV1:(path:string)=>{try{const text=readFileSync(path,"utf8");return text==="0"?false:text==="1"?true:undefined;}catch{return undefined;}},
  writeVerdictV1:(path:string,value:boolean)=>{writeFileSync(path,value?"1":"0");},
  claimDelayV1:delay,setTimeout:(action:()=>void,ms:number)=>{void delay(ms).then(action,action);},
 };
 const run=loadProductFunctionsSyncV1(new URL("../../🟨️.mjs",import.meta.url),names,ports).sharedVerdictV1;
 try{
  if(["peer-finishes","peer-disappears","cancel-wait","bounded-timeout"].includes(row.mode))writeFileSync(file,"");
  if(row.mode==="malformed")writeFileSync(file,"x");
  const result=await run(file,async()=>{
   computed++;events.push("compute");
   if(row.mode==="peer-completes-own-path")writeFileSync(file,"0");
   if(row.mode==="owner-replaced"){renameSync(file,file+".original");writeFileSync(file,"",{flag:"wx"});}
   if(row.mode==="cancel-owner"||row.mode==="owner-replaced")controller.abort(primary);
   return true;
  },controller.signal).then((value:boolean)=>({value,reason:undefined}),(reason:unknown)=>({value:null,reason}));
  expect(result.value).toBe(row.value);
  expect(result.reason).toBe(row.value===null?primary:undefined);
  expect({computed,delays,file:lstatSync(file,{throwIfNoEntry:false})?readFileSync(file,"utf8"):null}).toEqual({computed:row.computed,delays:row.delays,file:row.file});
  db.exec("create table events(kind text)");
  for(const event of events)db.query("insert into events values(?)").run(event);
  expect(db.query("select count(*) n from events where kind='compute' and rowid>(select coalesce(max(rowid),0) from events where kind='delay')").get()).toEqual({n:row.computed});
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
});
