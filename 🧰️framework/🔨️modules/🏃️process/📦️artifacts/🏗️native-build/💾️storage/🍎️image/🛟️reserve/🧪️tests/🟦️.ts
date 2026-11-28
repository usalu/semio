/** 🛟️ Tests for the image reserve. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{ensureNativeImageReserveFileV1,ensureNativeImageReserveV1,nativeImageReservePathV1,releaseNativeImageReserveFileV1,releaseNativeImageReserveV1,withNativeImageReserveV1}from"../🟦️.ts";
import{nativeImageServiceEntryV1}from"../../../🌐️platform/📋️contract/🟦️.ts";
const policy=imageSchema.definitions.ImageProtocol.const.reclaim.availability.refresh.reserve;
const scratch=()=>mkdtempSync(join(testScratchRootV1("reserve"),"r-"));
const oracle=(row:typeof fixture.creation[number]):string=>{
 const python=testPythonV1();
 if(python===null)throw Error("Python required by the reserve oracle");
 const program="import sys\na,b,m,e=map(int,sys.argv[1:5])\nprint('present' if e>=b else ('created' if a>=b*m else 'skipped'))";
 const result=spawnSync(python,["-I","-c",program,String(row.availableBytes),String(row.bytes),String(row.multiplier),String(row.existing)],{encoding:"utf8"});
 if(result.status!==0)throw result.error??Error(result.stderr);
 return result.stdout.trim();
};
test("the reserve is declared by the schema and lives in the reserved service domain of the volume root",()=>{
 expect(policy.file).toBe(".semio-reserve");expect(nativeImageServiceEntryV1(policy.file)).toBe(true);
 expect(nativeImageReservePathV1("/m")).toBe("/m/.semio-reserve");
});
for(const row of fixture.creation)test("creation corpus: "+row.name,()=>{
 const directory=scratch();
 try{
  const path=join(directory,"reserve");
  if(row.existing>0)writeFileSync(path,Buffer.alloc(row.existing,1));
  const result=ensureNativeImageReserveFileV1(path,row.bytes,row.multiplier,()=>({availableBytes:row.availableBytes}));
  expect<unknown>(result).toBe(row.expected);expect(oracle(row)).toBe(row.expected);
  if(row.expected==="created")expect(statSync(path).size).toBe(row.bytes);
  expect(readdirSync(directory).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("the image reserve is created at its declared size, kept, released and created again",()=>{
 const root=scratch();
 try{
  const room=()=>({availableBytes:policy.bytes*policy.creationFreeMultiplier});
  expect(ensureNativeImageReserveV1(root,room)).toBe("created");expect(statSync(nativeImageReservePathV1(root)).size).toBe(policy.bytes);
  expect(ensureNativeImageReserveV1(root,room)).toBe("present");
  expect(releaseNativeImageReserveV1(root)).toBe(true);expect(existsSync(nativeImageReservePathV1(root))).toBe(false);
  expect(releaseNativeImageReserveV1(root)).toBe(false);
  expect(ensureNativeImageReserveV1(root,room)).toBe("created");
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a failed write never leaves a partial reserve and never throws",()=>{
 const root=scratch();
 try{
  const missing=join(root,"absent","reserve");
  expect(ensureNativeImageReserveFileV1(missing,1024,1,()=>({availableBytes:1<<30}))).toBe("skipped");
  expect(ensureNativeImageReserveFileV1(join(root,"reserve"),1024,1,()=>undefined)).toBe("skipped");
  expect(readdirSync(root)).toEqual([]);
  expect(releaseNativeImageReserveFileV1(missing)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a writer that meets ENOSPC gets the reserve back once and runs again; other errors and a missing reserve pass through",()=>{
 const root=scratch();
 try{
  mkdirSync(root,{recursive:true});writeFileSync(nativeImageReservePathV1(root),Buffer.alloc(4096,1));
  const full=Object.assign(Error("full"),{code:"ENOSPC"});let runs=0;
  expect(withNativeImageReserveV1(root,()=>{runs++;if(runs===1)throw full;return"done";})).toBe("done");
  expect(runs).toBe(2);expect(existsSync(nativeImageReservePathV1(root))).toBe(false);
  expect(()=>withNativeImageReserveV1(root,()=>{throw full;})).toThrow("full");
  writeFileSync(nativeImageReservePathV1(root),Buffer.alloc(4096,1));
  expect(()=>withNativeImageReserveV1(root,()=>{throw Error("other");})).toThrow("other");
  expect(existsSync(nativeImageReservePathV1(root))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
