/** 🪶️ Tests for storage extent. */
import{test,expect}from"bun:test";
import{closeSync,lstatSync,mkdtempSync,openSync,realpathSync,rmSync,truncateSync,writeFileSync,writeSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{nativeImageBackingAllocatedBytesV1}from"../🟦️.ts";
test("the pinned allocation of a backing is its declared extent for numbers and bigints",()=>{
 expect(nativeImageBackingAllocatedBytesV1({size:7})).toBe(7);expect(nativeImageBackingAllocatedBytesV1({size:7n})).toBe(7n);
});
test("a sparse file keeps its pinned allocation while its blocks change, a preallocated-looking file keeps it too",()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("extent"),"extent-"))),file=join(directory,"backing");
 try{
  writeFileSync(file,"");truncateSync(file,64*1048576);const hole=lstatSync(file);
  const fd=openSync(file,"r+");writeSync(fd,Buffer.alloc(4*1048576,1),0,4*1048576,0);closeSync(fd);const written=lstatSync(file);
  expect(nativeImageBackingAllocatedBytesV1(hole)).toBe(64*1048576);expect(nativeImageBackingAllocatedBytesV1(written)).toBe(nativeImageBackingAllocatedBytesV1(hole));
  expect(written.blocks*512).toBeGreaterThan(hole.blocks*512);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
