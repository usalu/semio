import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,truncateSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readRemoteCachePolicyV1} from "../../../../🌐️remote-cache/🟦️.ts";
import type {RemoteExecutionContentV1} from "../../../🗄️content/🟦️.ts";
import {shipOffloadActionV1} from "../../📜️action/🟦️.ts";
const directories:string[]=[];
afterAll(()=>{for(const directory of directories)rmSync(directory,{recursive:true,force:true});});
test("an input larger than the content store's entry maximum is declined before any store call, naming file and size, in milliseconds",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-oversize-"));directories.push(directory);
 const server=readRemoteCachePolicyV1().server,limit=Math.min(server.maximumEntryBytes,Math.floor(server.maximumBytes/2)),big=join(directory,"libbig.rlib"),small=join(directory,"a.rs");
 writeFileSync(small,"fn main(){}\n");writeFileSync(big,"");truncateSync(big,limit+1);
 const touched:string[]=[],content=new Proxy({},{get:(_target,name)=>async()=>{touched.push(String(name));throw Error("content store touched");}}) as RemoteExecutionContentV1;
 const started=Date.now(),answer=await shipOffloadActionV1({spec:{files:new Map([["a.rs",small],["deps/libbig.rlib",big]]),arguments:["rustc"],environment:[],outputPaths:[],seconds:1},content,instance:"main",platform:[]});
 expect(answer).toMatchObject({declined:"inputs-too-large"});
 const detail=(answer as {detail:string}).detail;
 expect(detail).toContain("libbig.rlib");expect(detail).toContain(String(limit+1));
 expect(touched).toEqual([]);expect(Date.now()-started).toBeLessThan(2000);
});
test("an input exactly at the entry maximum is not declined for its size",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-oversize-"));directories.push(directory);
 const server=readRemoteCachePolicyV1().server,limit=Math.min(server.maximumEntryBytes,Math.floor(server.maximumBytes/2)),edge=join(directory,"edge.rlib");
 writeFileSync(edge,"");truncateSync(edge,limit);
 const content=new Proxy({},{get:()=>async()=>{throw Error("stop");}}) as RemoteExecutionContentV1;
 const answer=await shipOffloadActionV1({spec:{files:new Map([["deps/edge.rlib",edge]]),arguments:["rustc"],environment:[],outputPaths:[],seconds:1},content,instance:"main",platform:[],digest:()=>"0".repeat(64)}).catch(error=>({thrown:String(error)}));
 expect(answer).not.toMatchObject({declined:"inputs-too-large"});
});

test("the entry maximum admits the largest derive-crate rlib (1.09 GiB) and is the half of the store the server derives it from",()=>{
 const server=readRemoteCachePolicyV1().server;
 expect(server.maximumEntryBytes).toBeGreaterThanOrEqual(Math.ceil(1.09*2**30));expect(server.maximumEntryBytes).toBe(Math.floor(server.maximumBytes/2));
});
