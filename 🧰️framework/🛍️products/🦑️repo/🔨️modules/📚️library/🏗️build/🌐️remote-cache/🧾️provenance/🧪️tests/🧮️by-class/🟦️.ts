import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {encodeProtobufV1,REAPI_V1} from "../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {fleetSignedFetchV1,syncFleetIdentityV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../🖥️server/🟦️.ts";

/** 🧮️ The cache counts the results two origins stored differently by what the result is: a graph owner record, a closure bundle, a compiled unit or anything else, so a regression of one kind shows in `/status` at once. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-cls-${label}-`));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(value:string)=>createHash("sha256").update(value).digest("hex");
function machine(store:string,origin:string):string{
 const root=directory(origin);mkdirSync(join(root,".cache","f"),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"k"}));
 mkdirSync(join(store,"v1","origins",origin),{recursive:true});return root;
}
const named=(paths:readonly string[],seed:string)=>encodeProtobufV1(REAPI_V1.actionResult,{output_files:paths.map(path=>({path,digest:{hash:sha(seed+path),size_bytes:3}})),exit_code:0});

test("a divergence is counted under the kind of result that diverged and /status tells the counts since the server started",async()=>{
 const store=directory("group"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}});
 servers.push(running);
 const base=`http://127.0.0.1:${running.port}/semio`,put=(root:string,key:string,bytes:Uint8Array)=>fleetSignedFetchV1({root,audience:"cache"})(`${base}/ac/${key}`,{method:"PUT",body:bytes});
 const cases:[string,string[]][]=[["record",["record.br","closure-1.br"]],["record2",["record.br"]],["closure",["closure-9.br"]],["unit",["unit.json","out/o0"]],["other",["o0"]]];
 for(const [label,paths] of cases){
  const key=sha(label);
  expect((await put(a,key,named(paths,"first"))).status).toBe(200);
  expect((await put(b,key,named(paths.filter(path=>path!=="unit.json"),"second"))).status,label).toBe(409);
 }
 const status=await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;since:number;byClass:Record<string,number>}};
 expect(status.divergences.count).toBe(5);expect(status.divergences.byClass).toEqual({closure:1,other:2,"project-inference":2});
 expect(status.divergences.since).toBeGreaterThan(0);
});
