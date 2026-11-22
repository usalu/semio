/** 💾️ Contract: a durable publication orders create, write, file fsync, close, rename, directory fsync; a failure before the rename leaves no temporary and no target; Windows skips the directory sync; the published bytes equal Python's. See https://man7.org/linux/man-pages/man2/fsync.2.html */
import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {publishFileDurablySyncV1,syncDirectorySyncV1} from "../../🟦️.ts";

const bytes=Buffer.from('{"version":1,"files":["a.ts"]}'),scratch=()=>mkdtempSync(join(tmpdir(),"durable-"));
const recorder=(log:string[])=>{let next=0;return{open:(path:string,flags:string)=>{if(flags==="wx")writeFileSync(path,"");log.push("open:"+flags+":"+path.split(/[\\/]/u).at(-1)!.replace(/^~.*/u,"~tmp"));return ++next;},write:(_d:number,data:Uint8Array)=>{log.push("write:"+data.length);},fsync:(d:number)=>{log.push("fsync:"+d);},close:(d:number)=>{log.push("close:"+d);},rename:(from:string,to:string)=>{log.push("rename");writeFileSync(to,readFileSync(from));rmSync(from);}};};

test("file content is synced before the rename and the directory after it",()=>{
 const root=scratch(),log:string[]=[];
 try{
  publishFileDurablySyncV1(join(root,"~t"),join(root,"leaf.json"),bytes,{platform:"linux",operations:recorder(log)});
  expect(log).toEqual(["open:wx:~tmp","write:"+bytes.length,"fsync:1","close:1","rename","open:r:"+root.split(/[\\/]/u).at(-1),"fsync:2","close:2"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("Windows skips the directory sync",()=>{
 const log:string[]=[];syncDirectorySyncV1("C:\\x",{platform:"win32",operations:recorder(log)});expect(log).toEqual([]);
});

test("a directory that refuses sync keeps the rename",()=>{
 for(const code of["EINVAL","ENOTSUP","EPERM"])expect(()=>syncDirectorySyncV1("/x",{platform:"linux",operations:{open:()=>3,fsync:()=>{throw Object.assign(Error(code),{code});},close:()=>undefined}})).not.toThrow();
 expect(()=>syncDirectorySyncV1("/x",{platform:"linux",operations:{open:()=>3,fsync:()=>{throw Object.assign(Error("EIO"),{code:"EIO"});},close:()=>undefined}})).toThrow("EIO");
});

test("a failed fsync publishes nothing and leaves no temporary",()=>{
 const root=scratch();
 try{
  const log:string[]=[];expect(()=>publishFileDurablySyncV1(join(root,"~t"),join(root,"leaf.json"),bytes,{platform:"linux",operations:{...recorder(log),fsync:()=>{throw Object.assign(new Error("EIO"),{code:"EIO"});}}})).toThrow("EIO");
  expect(log).not.toContain("rename");expect(readdirSync(root)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the real filesystem publishes exactly the bytes and refuses an existing temporary",()=>{
 const root=scratch(),temporary=join(root,"~t"),path=join(root,"leaf.json");
 try{
  publishFileDurablySyncV1(temporary,path,bytes);
  expect(readFileSync(path)).toEqual(bytes);expect(existsSync(temporary)).toBe(false);
  writeFileSync(temporary,"x");expect(()=>publishFileDurablySyncV1(temporary,path,bytes)).toThrow();
  expect(readFileSync(temporary,"utf8")).toBe("x");
 }finally{rmSync(root,{recursive:true,force:true});}
});
