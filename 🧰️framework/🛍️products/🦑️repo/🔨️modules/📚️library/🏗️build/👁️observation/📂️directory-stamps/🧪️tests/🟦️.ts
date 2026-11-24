/** 📂️ Contract: the fact of a directory listing is remembered by the stamps of the directories it was computed from. With a warm memo the verification of N unchanged directories makes N stats and no listing; a change inside one directory drops exactly the facts that read it (also the fact of a parent whose hollow walk read it); a directory younger than the racy window is never remembered; every fact equals the independent Python reader.
 * @see ../../🟨️.mjs
 * @see ../🟨️.mjs */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {directoryFactOfV1,observationDirectoryFactCountsV1,observationProbeCountsV1,observationsHold} from "../../🟨️.mjs";
import {createDirectoryFactMemoV1,directoryStampV1} from "../🟨️.mjs";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),policy=join(here,"../../../../⚡️caching/🔣️policy.json"),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const calm=()=>new Promise(done=>setTimeout(done,2200));
const put=(base:string,path:string,text="x\n")=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);};
function tree(count:number):{base:string;directories:string[]}{
 const base=mkdtempSync(join(tmpdir(),"directory-stamps-"));scratch.push(base);
 const directories:string[]=[];
 for(let at=0;at<count;at++){const directory=join(base,"d"+at);put(base,"d"+at+"/a.ts");put(base,"d"+at+"/b.json");mkdirSync(join(directory,"sub"),{recursive:true});put(base,"d"+at+"/sub/c.ts");directories.push(directory);}
 put(base,"outer/ghost/inner/.DS_Store");
 return {base,directories};
}
const rowsOf=(directories:readonly string[])=>directories.map(path=>({kind:"readdir",path,fact:directoryFactOfV1("readdir",path)}));

test("a memo keeps a fact with the stamps of what it read and drops it when one stamp moved, is refused for a young stamp or a failed listing, and forgets the oldest first",()=>{
 const old=directoryStampV1({mtimeMs:1,ctimeMs:2,ino:3,size:4}),young=directoryStampV1({mtimeMs:1e12,ctimeMs:1e12,ino:3,size:4});
 const memo=createDirectoryFactMemoV1({entries:2,calmMilliseconds:2000}),stampOf=(path:string)=>path==="/a"||path==="/a/x"?old.text:undefined;
 expect(memo.record("/a",[["/a",old],["/a/x",old]],"fact-a",1e12)).toBe(true);
 expect(memo.lookup("/a",stampOf)).toBe("fact-a");
 expect(memo.lookup("/a",()=>"1:2:3:5")).toBeUndefined();
 expect(memo.lookup("/a",stampOf)).toBeUndefined();
 expect(memo.record("/b",[["/b",young]],"fact-b",1e12)).toBe(false);
 expect(memo.record("/b",[["/b",old],null],"fact-b",1e12)).toBe(false);
 for(const name of["/c","/d","/e"])memo.record(name,[[name,old]],name,1e12);
 expect([...memo.entries().keys()]).toEqual(["/d","/e"]);
});

test("with a warm memo the verification of N unchanged directories makes N stats and no listing, and every fact equals the independent Python reader",async()=>{
 const {base,directories}=tree(40);
 await calm();
 const rows=rowsOf(directories),listed=observationProbeCountsV1().readdir;
 expect(await observationsHold(rows)).toBe(true);
 expect(observationProbeCountsV1().readdir).toBe(listed);
 const before=observationProbeCountsV1().readdir,hits=observationDirectoryFactCountsV1().hits;
 expect(await observationsHold(rows)).toBe(true);
 expect(observationProbeCountsV1().readdir).toBe(before);
 expect(observationDirectoryFactCountsV1().hits-hits).toBe(directories.length);
 if(!python)return;
 const expected=JSON.parse(Bun.spawnSync([python,"-I",join(here,"🐍️.py"),policy,...directories,base]).stdout.toString());
 for(const row of rows)expect(row.fact).toBe(expected[row.path]);
 expect(directoryFactOfV1("readdir",base)).toBe(expected[base]);
});

test("a name added in one directory re-lists only the facts that read it: its own, and none of the other directories",async()=>{
 const {directories}=tree(12);
 await calm();
 const rows=rowsOf(directories);
 expect(await observationsHold(rows)).toBe(true);
 put(directories[3],"new.ts");
 const before=observationProbeCountsV1().readdir,stale:string[]=[];
 expect(await observationsHold(rows,64,new Set(),new Set(),undefined,row=>stale.push(row.path),undefined,true)).toBe(false);
 expect(stale).toEqual([directories[3]]);
 expect(observationProbeCountsV1().readdir-before).toBe(1);
});

test("a file added deep below a hollow directory moves the fact of its ancestors that listed it, so the memo never serves the old name set",async()=>{
 const {base}=tree(1);
 await calm();
 const fact=directoryFactOfV1("readdir",base);
 expect(directoryFactOfV1("readdir",base)).toBe(fact);
 put(base,"outer/ghost/inner/real.ts");
 const moved=directoryFactOfV1("readdir",base);
 expect(moved).not.toBe(fact);
 if(!python)return;
 expect(JSON.parse(Bun.spawnSync([python,"-I",join(here,"🐍️.py"),policy,base]).stdout.toString())[base]).toBe(moved);
 rmSync(join(base,"outer/ghost/inner/real.ts"));
 expect(directoryFactOfV1("readdir",base)).toBe(fact);
});

test("a directory changed within the racy window is listed again every time, and a directory that disappears reads as missing",async()=>{
 const {directories}=tree(2);
 put(directories[0],"fresh.ts");
 const first=observationProbeCountsV1().readdir;
 directoryFactOfV1("readdir",directories[0]);directoryFactOfV1("readdir",directories[0]);
 expect(observationProbeCountsV1().readdir-first).toBe(2);
 rmSync(directories[1],{recursive:true});
 expect(directoryFactOfV1("readdir",directories[1])).toBe("!ENOENT");
});
