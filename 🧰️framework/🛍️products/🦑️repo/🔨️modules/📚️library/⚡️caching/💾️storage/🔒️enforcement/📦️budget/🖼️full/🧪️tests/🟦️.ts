/** 🖼️ A full cache image is named, with its fill, instead of the host partition. */
import{expect,test}from"bun:test";
import{mkdtempSync,mkdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import policy from"../../🔣️policy.json";
import{fullImageOfV1,imageFilledByV1}from"../🟦️.ts";

const GiB=2**30,root="/w",mountOf=(id:string)=>join(root,(policy.partitions as Record<string,{directory:string}>)[id]!.directory,"m");
const spaces=(table:Record<string,[number,number]>)=>(mount:string)=>{const id=Object.keys(table).find(key=>mountOf(key)===mount);return id?{sizeBytes:table[id]![0],usedBytes:table[id]![0]-table[id]![1],freeBytes:table[id]![1]}:undefined;};

test("the image whose mount holds the error path is named; else the fullest one under its floor; an ample or unmounted image is never named",()=>{
 const base={mounted:(_d:string,m:string)=>Object.keys(policy.partitions).some(id=>mountOf(id)===m),measure:()=>"inference/selections (10.7 GiB)"},table={graph:[12*GiB,40*2**20],products:[24*GiB,10*GiB],rust:[40*GiB,300*2**20]} as Record<string,[number,number]>;
 const graph=fullImageOfV1(root,{...base,space:spaces(table)})!;
 expect(graph.id).toBe("graph");expect(graph.sizeBytes).toBe(12*GiB);expect(graph.freeBytes).toBe(40*2**20);expect(graph.filledBy).toBe("inference/selections (10.7 GiB)");
 expect(fullImageOfV1(root,{...base,space:spaces(table),paths:[join(mountOf("rust"),"x/y")]})!.id).toBe("rust");
 expect(fullImageOfV1(root,{...base,space:spaces({graph:[12*GiB,8*GiB],products:[24*GiB,10*GiB]})})).toBeUndefined();
 expect(fullImageOfV1(root,{...base,mounted:()=>false,space:spaces(table)})).toBeUndefined();
 expect(fullImageOfV1(root,{...base,space:spaces({graph:[12*GiB,500*2**20]})})).toBeUndefined();
});

test("the largest subtrees are listed biggest first with their size",()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"filled-by-"));
 try{
  mkdirSync(join(dir,"inference/selections"),{recursive:true});mkdirSync(join(dir,"derived/n"),{recursive:true});
  writeFileSync(join(dir,"inference/selections/x"),Buffer.alloc(3*1048576));writeFileSync(join(dir,"derived/n/y"),Buffer.alloc(1048576));
  const line=imageFilledByV1(dir,2);
  expect(line.startsWith("inference/selections (0.0 GiB)")).toBe(true);expect(line).toContain("derived/n (0.0 GiB)");
 }finally{rmSync(dir,{recursive:true,force:true});}
});
