import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,unlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readRemoteCacheEntryV1} from "../🟦️.ts";

const folder=mkdtempSync(join(tmpdir(),"rcache-entry-"));
afterAll(()=>rmSync(folder,{recursive:true,force:true}));

test("a value removed after it was read is whole, one removed before is a miss, an empty value is empty and a large one is left to the caller",async()=>{
 const path=join(folder,"value"),bytes=new Uint8Array(1<<20).map((_,index)=>index%251);writeFileSync(path,bytes);
 const read=await readRemoteCacheEntryV1(path,bytes.length);unlinkSync(path);
 expect(read).toBeInstanceOf(Uint8Array);expect(Buffer.from(read as Uint8Array).equals(Buffer.from(bytes))).toBe(true);
 expect(await readRemoteCacheEntryV1(path,bytes.length)).toBeUndefined();
 expect((await readRemoteCacheEntryV1(path,0) as Uint8Array).length).toBe(0);
 expect(await readRemoteCacheEntryV1(path,bytes.length,1024),"larger than the limit").toBe("lazy");
});
