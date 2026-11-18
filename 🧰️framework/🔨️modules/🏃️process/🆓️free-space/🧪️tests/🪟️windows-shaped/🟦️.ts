/** 🪟️ Contract: the free space of a volume is typed facts, bytes or unknown with a reason. A runtime that answers a row of zeros (the Windows `statfs` of a bun host) is unknown, never "0 MB free"; a filesystem without an inode table has no inode count, never 0; Windows asks the volume (`GetDiskFreeSpaceExW`) behind the same interface. The rows in `./🧫️fixtures/🔣️.json` are judged here and by the independent Python oracle in `./🐍️.py`. */
import{spawnSync}from"node:child_process";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{afterAll,expect,test}from"bun:test";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import{freeSpaceSyncV1,judgeFreeSpaceRowV1,readFreeSpaceV1}from"../../🟦️.ts";

const directories:string[]=[];afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const scratch=():string=>{const path=mkdtempSync(join(tmpdir(),"free-win-"));directories.push(path);return path;};
const python=(input:string,program:string[]):unknown=>{const run=spawnSync("python3",["-I",...program],{input,encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("every authored row is judged by TypeScript exactly as the independent Python oracle judges it",()=>{
 const oracle=python(JSON.stringify(fixture.rows.map(row=>row.row)),[fileURLToPath(new URL("./🐍️.py",import.meta.url))]) as ({bytes:number;total:number|null;inodes:number|null}|null)[];
 fixture.rows.forEach((row,index)=>{
  const facts=judgeFreeSpaceRowV1(row.row),shape=facts.known?{bytes:facts.availableBytes,total:facts.totalBytes??null,inodes:facts.freeInodes}:null;
  expect<unknown>(shape,row.name).toEqual(row.expected);expect<unknown>(oracle[index],row.name).toEqual(row.expected);
 });
});

test("a Windows host with a statfs that answers zeros is unknown with the reason, and bigint, NaN and negative rows are unknown too",()=>{
 const empty=judgeFreeSpaceRowV1({bsize:4096n,blocks:0n,bavail:0n,files:0n,ffree:0n});
 expect(empty).toEqual({known:false,reason:"empty-row",detail:"bsize 4096, blocks 0, bavail 0"});
 expect(judgeFreeSpaceRowV1({bsize:0,blocks:0,bavail:0})).toMatchObject({known:false,reason:"incoherent-row"});
 expect(judgeFreeSpaceRowV1({bsize:4096,blocks:10,bavail:Number.NaN})).toMatchObject({known:false,reason:"incoherent-row"});
 expect(judgeFreeSpaceRowV1({bsize:4096,blocks:10,bavail:-4n})).toMatchObject({known:false,reason:"incoherent-row"});
 expect(judgeFreeSpaceRowV1({bsize:4096n,blocks:1000n,bavail:250n,files:9n,ffree:3n})).toMatchObject({known:true,availableBytes:1024000,freeInodes:3});
 const directory=scratch(),zeros=(path:string)=>({bsize:4096,blocks:0,bavail:0,bfree:0,files:0,ffree:0,type:0,path});
 expect(readFreeSpaceV1(directory,{read:zeros,platform:"win32"})).toMatchObject({known:false,reason:"empty-row"});
 expect(freeSpaceSyncV1(directory,{read:zeros})).toBeUndefined();
 expect(readFreeSpaceV1(directory,{read:zeros,nearest:true})).toMatchObject({known:false,reason:"empty-row"});
});

test("Windows asks the volume for its free bytes behind the interface: bytes without inodes, the nearest existing ancestor for a path below the cache that is not there yet, a failing or nonsensical call unknown",()=>{
 const directory=scratch(),asked:string[]=[],missing=join(directory,"a","b",".cache");
 const known=readFreeSpaceV1(missing,{platform:"win32",nearest:true,windowsAvailableBytes:path=>{asked.push(path);return 120_000_000_000;}});
 expect(known).toEqual({known:true,availableBytes:120_000_000_000,totalBytes:undefined,freeInodes:null,source:"win32-host"});
 expect(asked).toEqual([directory]);
 expect(freeSpaceSyncV1(directory,{platform:"win32",windowsAvailableBytes:()=>0})).toEqual({availableBytes:0,totalBytes:undefined});
 expect(readFreeSpaceV1(directory,{platform:"win32",windowsAvailableBytes:()=>{throw Error("Host free space could not be read (Win32 error 3)");}})).toMatchObject({known:false,reason:"host-call",detail:expect.stringContaining("Win32 error 3")});
 for(const bad of[Number.NaN,-1,Number.POSITIVE_INFINITY])expect(readFreeSpaceV1(directory,{platform:"win32",windowsAvailableBytes:()=>bad})).toMatchObject({known:false,reason:"host-call"});
});

test("the Windows host call of a machine that is not Windows is unknown, never a number, and a missing path without nearest is unreadable",()=>{
 const directory=scratch();
 if(process.platform!=="win32")expect(readFreeSpaceV1(directory,{platform:"win32"})).toMatchObject({known:false,reason:"host-call"});
 expect(readFreeSpaceV1(join(directory,"nope","deeper"))).toMatchObject({known:false,reason:"unreadable"});
});

test("the facts of this machine's real volume equal the independent Python reading (shutil.disk_usage, os.statvfs)",()=>{
 if(process.platform==="win32")return;
 const directory=scratch(),facts=readFreeSpaceV1(directory);
 const reading=python("",["-c","import json,os,shutil,sys\nd=sys.argv[1];s=os.statvfs(d);u=shutil.disk_usage(d)\nprint(json.dumps([u.free,u.total,s.f_ffree if s.f_files>0 else None]))",directory]) as [number,number,number|null];
 expect(facts.known).toBe(true);if(!facts.known)return;
 expect(Math.abs(facts.availableBytes-reading[0])).toBeLessThan(2*1073741824);
 expect(Math.abs(facts.totalBytes!-reading[1])/reading[1]).toBeLessThan(0.01);
 expect(facts.freeInodes===null).toBe(reading[2]===null);
 if(reading[2]!==null)expect(Math.abs(facts.freeInodes!-reading[2])).toBeLessThan(Math.max(100000,reading[2]*0.01));
});
