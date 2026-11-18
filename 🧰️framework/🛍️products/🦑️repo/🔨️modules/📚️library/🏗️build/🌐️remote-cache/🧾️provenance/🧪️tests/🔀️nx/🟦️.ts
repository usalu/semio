import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {startRemoteCacheServerV1} from "../../../🖥️server/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("a read of an Nx task result or an install state counts as one consumption of its producer; a miss, a write and a blob read count nothing",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nx-consumption-"));scratch.push(directory);
 const server=await startRemoteCacheServerV1({directory,host:"127.0.0.1",port:0,maximumBytes:2**27,maximumEntryBytes:2**25,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w"}});
 const auth={authorization:"Bearer w"};
 expect((await fetch(`${server.url}/main/v1/cache/12345678901234567890`,{method:"PUT",headers:{...auth,"content-type":"application/octet-stream"},body:"task"})).status).toBe(200);
 expect((await fetch(`${server.url}/main/v1/cache/inst-abc`,{method:"PUT",headers:{...auth,"content-type":"application/octet-stream"},body:"install"})).status).toBe(200);
 expect((await fetch(`${server.url}/main/v1/cache/99999`,{headers:auth})).status).toBe(404);
 for(const key of["12345678901234567890","12345678901234567890","inst-abc"])expect((await (await fetch(`${server.url}/main/v1/cache/${key}`,{headers:auth})).text()).length).toBeGreaterThan(3);
 await server.stop();
 const record=JSON.parse(readFileSync(join(directory,"fleet","consumption.json"),"utf8")) as {total:number;pairs:Record<string,{hits:number;units:number}>};
 expect(record.total).toBe(3);expect(record.pairs["unknown>static"]).toEqual({hits:3,units:0});
});
