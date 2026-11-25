/** 🧬️ Tests for the instances (namespaces) a cache server stores: a write into another one is refused with a text in both languages, reads and the served instance are untouched. */
import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../🟦️.ts";

const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
afterAll(async()=>{for(const server of servers)await server.stop();for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("a server that stores one instance refuses writes and slots into every other, and answers reads of them as misses",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"instances-"));scratch.push(directory);
 const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",instances:["main"],tokens:{},validation:"off"});servers.push(server);
 const bytes=new TextEncoder().encode("one blob"),digest=createHash("sha256").update(bytes).digest("hex");
 const put=(path:string)=>fetch(server.url+path,{method:"PUT",body:bytes});
 expect((await put("/main/cas/"+digest)).status,"the served instance").toBeLessThan(300);
 expect((await put("/cas/"+digest)).status,"no prefix is the default instance").toBeLessThan(300);
 const refused=await put("/semio/cas/"+digest);
 expect(refused.status).toBe(421);const text=await refused.text();expect(text).toContain("not semio");expect(text).toContain("nicht semio");
 expect((await fetch(server.url+"/semio/v1/slots",{method:"POST",body:JSON.stringify({entries:[]})})).status).toBe(421);
 expect((await fetch(server.url+"/semio/cas/"+digest)).status,"a read is a plain miss").toBe(404);
 expect((await fetch(server.url+"/main/cas/"+digest)).status).toBe(200);
});

test("a server without the option stores every instance, as before",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"instances-open-"));scratch.push(directory);
 const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"off"});servers.push(server);
 const bytes=new TextEncoder().encode("another blob"),digest=createHash("sha256").update(bytes).digest("hex");
 expect((await fetch(server.url+"/semio/cas/"+digest,{method:"PUT",body:bytes})).status).toBeLessThan(300);
});
