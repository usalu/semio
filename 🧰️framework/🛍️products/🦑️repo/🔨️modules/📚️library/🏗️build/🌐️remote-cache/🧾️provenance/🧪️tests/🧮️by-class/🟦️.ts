import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {encodeProtobufV1,REAPI_V1} from "../../../../🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
import {fleetSignedFetchV1,syncFleetIdentityV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {requireRemoteCacheShapeV1} from "../../../🟦️.ts";
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
  expect((await put(b,key,named(paths.filter(path=>path!=="unit.json"),"second"))).status,label).toBe(label.startsWith("record")||label==="closure"?200:409);
 }
 const status=await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;since:number;byClass:Record<string,number>;superseded:Record<string,number>}};
 expect(status.divergences.count).toBe(2);expect((status.divergences as unknown as {current:{count:number;byClass:Record<string,number>};lastHour:{count:number}}).current).toMatchObject({count:2,byClass:{other:1,unit:1}});expect((status.divergences as unknown as {lastHour:{count:number}}).lastHour.count).toBe(2);expect(status.divergences.byClass).toEqual({other:1,unit:1});expect(status.divergences.superseded).toEqual({closure:1,"project-inference":2});
 expect(status.divergences.since).toBeGreaterThan(0);
});

test("a note two machines write differently is replaced by the newer one and counted as advice; a unit divergence names the output that differs and survives a server restart",async()=>{
 const store=directory("group2"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const cache=directory("cache2"),options={directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure" as const,fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}};
 const running=await startRemoteCacheServerV1(options);servers.push(running);
 const base=`http://127.0.0.1:${running.port}/semio`,put=(root:string,key:string,bytes:Uint8Array)=>fleetSignedFetchV1({root,audience:"cache"})(`${base}/ac/${key}`,{method:"PUT",body:bytes}),get=async(key:string)=>new Uint8Array(await (await fleetSignedFetchV1({root:a,audience:"cache"})(`${base}/ac/${key}`)).arrayBuffer());
 const note=sha("note"),first=named(["failure.br"],"first"),second=named(["failure.br"],"second");
 expect((await put(a,note,first)).status).toBe(200);expect((await put(b,note,second)).status).toBe(200);
 expect(Buffer.compare(Buffer.from(await get(note)),Buffer.from(second))).toBe(0);
 const unit=sha("unit"),build=(seed:string)=>encodeProtobufV1(REAPI_V1.actionResult,{output_files:[{path:"unit.json",digest:{hash:sha("m"+seed),size_bytes:3}},{path:"out/libx.rmeta",digest:{hash:sha("meta"),size_bytes:3}},{path:"out/libx.rlib",digest:{hash:sha("rlib"+seed),size_bytes:3}}],exit_code:0});
 expect((await put(a,unit,build("one"))).status).toBe(200);expect((await put(b,unit,build("two"))).status).toBe(409);
 const ask=async(port:number)=>(await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${port}/status`)).json()) as {divergences:{evidence:unknown;count:number;byClass:Record<string,number>;advisory:Record<string,number>;recent:{sort:string;differing:string[]}[]}};
 let status=await ask(running.port);for(let wait=0;wait<50&&(status.divergences.evidence as {count:number}).count===0;wait++){await new Promise(done=>setTimeout(done,40));status=await ask(running.port);}
 expect(status.divergences.count).toBe(1);expect(status.divergences.byClass).toEqual({unit:1});expect(status.divergences.advisory).toEqual({"failure-note":1});expect(status.divergences.recent[0]).toMatchObject({sort:"unit",differing:["out/libx.rlib"]});
 expect(()=>requireRemoteCacheShapeV1("ServerStatus",status,"closed")).not.toThrow();
 expect(status.divergences.evidence).toMatchObject({count:1,byClass:{unit:1}});
 const listed=await (await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/evidence`)).json() as {evidence:{id:string;differing:string[]}[]};
 expect(listed.evidence.length).toBe(1);expect(listed.evidence[0]!.differing).toEqual(["out/libx.rlib"]);
 const refused=await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/evidence/${listed.evidence[0]!.id}/refused.ac`);
 expect(Buffer.compare(Buffer.from(await refused.arrayBuffer()),Buffer.from(build("two")))).toBe(0);
 expect((await fleetSignedFetchV1({root:a,audience:"cache"})(`http://127.0.0.1:${running.port}/evidence/${listed.evidence[0]!.id}/..%2Fmeta.json`)).status).toBe(404);
 await running.stop();servers.pop();
 const again=await startRemoteCacheServerV1(options);servers.push(again);
 const restarted=await ask(again.port);expect(restarted.divergences.count).toBe(1);expect((restarted.divergences as unknown as {current:{count:number}}).current.count).toBe(0);expect(restarted.divergences.byClass).toEqual({unit:1});
});

test("a graph record replaces the older record of its key and the replaced one is kept as evidence and counted as superseded, a closure bundle is replaced the same way, while a unit and an owner pool wave stay first-writer-wins",async()=>{
 const store=directory("group3"),a=machine(store,"g1-m01"),b=machine(store,"g1-m02"),server=machine(store,"g1-server");
 for(const root of[a,b,server])expect((await syncFleetIdentityV1({root})).error).toBeUndefined();
 const running=await startRemoteCacheServerV1({directory:directory("cache3"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"semio",tokens:{},validation:"structure",fleet:{storeRoot:store,group:"g1",networks:["10.77.0.0/16"]}});servers.push(running);
 const base=`http://127.0.0.1:${running.port}/semio`,call=(root:string)=>fleetSignedFetchV1({root,audience:"cache"}),put=(root:string,key:string,bytes:Uint8Array)=>call(root)(`${base}/ac/${key}`,{method:"PUT",body:bytes});
 const record=sha("record-key"),older=named(["record.br","leaf/a.json"],"older"),newer=named(["record.br","leaf/a.json"],"newer");
 expect((await put(a,record,older)).status).toBe(200);expect((await put(b,record,newer)).status).toBe(200);
 expect(Buffer.compare(Buffer.from(await (await call(a)(`${base}/ac/${record}`)).arrayBuffer()),Buffer.from(newer))).toBe(0);
 const wave=sha("wave-key");expect((await put(a,wave,named(["head.br"],"w1"))).status).toBe(200);expect((await put(b,wave,named(["head.br"],"w2"))).status).toBe(409);
 let status=await (await call(a)(`http://127.0.0.1:${running.port}/status`)).json() as {divergences:{count:number;byClass:Record<string,number>;superseded:Record<string,number>;evidence:{count:number}}};
 for(let wait=0;wait<50&&status.divergences.evidence.count<2;wait++){await new Promise(done=>setTimeout(done,40));status=await (await call(a)(`http://127.0.0.1:${running.port}/status`)).json() as typeof status;}
 expect(status.divergences.count).toBe(1);expect(status.divergences.byClass).toEqual({"owner-pool":1});expect(status.divergences.superseded).toEqual({"project-inference":1});
 const listed=await (await call(a)(`http://127.0.0.1:${running.port}/evidence`)).json() as {evidence:{id:string;outcome:string;sort:string;keptOrigin:string;refusedOrigin:string}[]};
 const replaced=listed.evidence.find(row=>row.outcome==="superseded")!;
 expect(replaced).toMatchObject({sort:"project-inference",keptOrigin:"g1-m01",refusedOrigin:"g1-m02"});expect(listed.evidence.find(row=>row.outcome==="refused")!.sort).toBe("owner-pool");
 expect(Buffer.compare(Buffer.from(await (await call(a)(`${base.replace("/semio","")}/evidence/${replaced.id}/kept.ac`)).arrayBuffer()),Buffer.from(older))).toBe(0);
});
