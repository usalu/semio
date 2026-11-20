/** 🔖️ The stamp of a path observed by the current dependency bindings: a directory is identified by what it is, never by what is inside it, so a peer or the replica adding a file to a pinned package directory cannot fail a read or a plan of the dependency store. */
import{test,expect}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,appendFileSync,rmSync,symlinkSync,unlinkSync,lstatSync,realpathSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{javascriptDependencyBindingStampV1 as stamp,readJavascriptDependencyCurrentBindingsV1}from"../../🟦️.ts";

const stat=(path:string)=>lstatSync(path,{bigint:true,throwIfNoEntry:false});
const fresh=()=>realpathSync(mkdtempSync(join(tmpdir(),"binding-stamp-")));

test("a directory keeps its stamp while entries are added, written to and removed, although size, link count and times move",()=>{
 const root=fresh(),directory=join(root,"package");
 try{
  mkdirSync(directory);const before=stat(directory)!,first=stamp(before);
  mkdirSync(join(directory,"sub"));writeFileSync(join(directory,"a.ts"),"a");appendFileSync(join(directory,"a.ts"),"bb");unlinkSync(join(directory,"a.ts"));
  const after=stat(directory)!;
  expect(after.nlink!==before.nlink||after.mtimeNs!==before.mtimeNs||after.ctimeNs!==before.ctimeNs,"the directory facts really moved").toBe(true);
  expect(stamp(after)).toBe(first);
  expect(first!.split(":")).toHaveLength(3);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a directory is told from another directory, from a link and from a file, and a missing path has no stamp",()=>{
 const root=fresh();
 try{
  mkdirSync(join(root,"one"));mkdirSync(join(root,"two"));writeFileSync(join(root,"file"),"x");symlinkSync(join(root,"one"),join(root,"link"));
  const stamps=["one","two","file","link"].map(name=>stamp(stat(join(root,name))));
  expect(new Set(stamps).size).toBe(4);
  expect(stamp(stat(join(root,"absent")))).toBeNull();
  expect(stamp(undefined)).toBeNull();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a link and a file keep the whole stamp: a retargeted link and a rewritten file are different observations",()=>{
 const root=fresh(),link=join(root,"node_modules");
 try{
  mkdirSync(join(root,"g1"));mkdirSync(join(root,"generation-two"));symlinkSync(join(root,"g1"),link);const first=stamp(stat(link));
  expect(stamp(stat(link))).toBe(first);
  unlinkSync(link);symlinkSync(join(root,"generation-two"),link);expect(stamp(stat(link))).not.toBe(first);
  writeFileSync(join(root,"receipt"),"a");const file=stamp(stat(join(root,"receipt")));appendFileSync(join(root,"receipt"),"b");expect(stamp(stat(join(root,"receipt")))).not.toBe(file);
  expect(file!.split(":")).toHaveLength(7);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the device, inode and mode of a directory agree with an independent reading by Python",()=>{
 const root=fresh();
 try{
  const run=spawnSync("python3",["-I","-c","import os,sys;s=os.lstat(sys.argv[1]);print('%d:%d:%d'%(s.st_dev,s.st_ino,s.st_mode))",root],{encoding:"utf8"});
  if(run.status!==0)return;
  expect(stamp(stat(root))).toBe(run.stdout.trim());
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the current bindings read observes every path through that stamp, so the plan and the read agree while directories change",()=>{
 const source=readJavascriptDependencyCurrentBindingsV1.toString();
 expect(source).toMatch(/stamp\s*=\s*javascriptDependencyBindingStampV1/u);
 expect(source).not.toContain('"mtimeNs","ctimeNs"');
});
