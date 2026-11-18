/** 🧾️ Contract: what the dependency installation consumes is its CONTENT (the bytes of bun.lock, the package.json of every workspace member, patches, configuration; the kind of every path), never inode, mode or times. A rewrite of identical bytes by a replica or an editor (new inode, new attributes, new times, a replaced directory) is not a change; a different byte, a vanished, an appeared or a retyped input is, and is classified as the concurrent edit that the funding re-runs from its published progress. */
import{test,expect}from"bun:test";
import{chmodSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,unlinkSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createHash}from"node:crypto";
import{concurrentEditOfV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import{recheckJavascriptDependencyInputsV1,type JavascriptDependencySourceV1}from"../../🟦️.ts";
const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish(){},yieldContinuation:async()=>{}}) as never;
const capture=(path:string,kind:"file"|"directory"|"missing"):JavascriptDependencySourceV1=>{const facts=kind==="missing"?undefined:lstatSync(path,{bigint:true});return Object.freeze({path,kind,bytes:kind==="file"?readFileSync(path):Buffer.alloc(0),dev:facts?String(facts.dev):"",ino:facts?String(facts.ino):"",mode:facts?Number(facts.mode):0});};
const reason=(sources:readonly JavascriptDependencySourceV1[])=>{try{recheckJavascriptDependencyInputsV1(sources,control());return null;}catch(error){return concurrentEditOfV1(error)?.reason??"hard";}};
const digest=(sources:readonly JavascriptDependencySourceV1[])=>createHash("sha256").update(JSON.stringify(sources.map(row=>[row.path,row.kind,row.bytes.toString("base64")]))).digest("hex");
test("a rewrite of identical bytes is not a change: new inode, new mode, new times and a replaced directory pass, and the independent content digest agrees",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r85-input-"))),dir=join(root,"pkg"),file=join(dir,"package.json"),lock=join(root,"bun.lock");
 try{
  mkdirSync(dir);writeFileSync(file,'{"name":"a","version":"1.0.0"}');writeFileSync(lock,"lock v1\n");
  const sources=[capture(dir,"directory"),capture(file,"file"),capture(lock,"file")],before=digest(sources),inode=lstatSync(file).ino;
  expect(reason(sources)).toBeNull();
  writeFileSync(`${file}.tmp`,'{"name":"a","version":"1.0.0"}');renameSync(`${file}.tmp`,file);
  expect(lstatSync(file).ino,"the replica really gave the file a new inode").not.toBe(inode);expect(reason(sources)).toBeNull();
  if(process.platform!=="win32"){chmodSync(file,0o600);expect(lstatSync(file).mode&0o777).toBe(0o600);}
  utimesSync(file,new Date(1e12),new Date(1e12));utimesSync(lock,new Date(2e12),new Date(2e12));expect(reason(sources)).toBeNull();
  renameSync(dir,`${dir}.old`);mkdirSync(dir);writeFileSync(file,'{"name":"a","version":"1.0.0"}');expect(reason(sources)).toBeNull();
  expect(digest([capture(dir,"directory"),capture(file,"file"),capture(lock,"file")])).toBe(before);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a change of content is a change: a different byte of the same length, a vanished, an appeared and a retyped input",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r85-input-"))),file=join(root,"package.json"),absent=join(root,"bunfig.toml");
 try{
  writeFileSync(file,'{"v":"1.0.0"}');
  const sources=[capture(file,"file"),capture(absent,"missing")];expect(reason(sources)).toBeNull();
  writeFileSync(file,'{"v":"1.0.1"}');expect(reason(sources)).toBe("bytes");
  writeFileSync(file,'{"v":"1.0.0"}');expect(reason(sources)).toBeNull();
  writeFileSync(absent,"x");expect(reason(sources)).toBe("appeared");unlinkSync(absent);
  unlinkSync(file);expect(reason(sources)).toBe("vanished");
  mkdirSync(file);expect(reason(sources)).toBe("identity");
 }finally{rmSync(root,{recursive:true,force:true});}
});
