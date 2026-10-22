/** 🧊️ Contract: a block this process validated, or one a farm adoption verified by its hash, is not read and validated again while its leaf keeps its identity; a leaf that changed in any way is read again and refused when it does not hold what its name says. */
import {afterAll,expect,spyOn,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import * as fs from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
import {adoptVerifiedBlockV1,inferenceObservationBlockPathV1,observeInference,observedFs,readInferenceHeaderV1} from "../../🟨️.mjs";

const roots:string[]=[],scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"observation-verified-blocks-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const publish=async(root:string)=>{
 const sources=join(root,"sources"),cachePath=join(root,"inference","np","aaaaaaaaaaaaaaaaaaaaaaaa.json");
 mkdirSync(sources,{recursive:true});mkdirSync(dirname(cachePath),{recursive:true});
 for(let index=0;index<8;index++)writeFileSync(join(sources,`${index}.txt`),`source ${index}`);
 await Bun.sleep(2100);
 await observeInference({cachePath,key:"verified-blocks"},()=>Array.from({length:8},(_,index)=>observedFs.readFileSync(join(sources,`${index}.txt`),"utf8")).join("|"));
 const header=readInferenceHeaderV1(cachePath);
 expect(header.blocks.length).toBe(1);
 return {cachePath,digest:header.blocks[0]!,rows:header.observations};
};
const opens=(path:string,run:()=>void)=>{
 let count=0;const open=fs.openSync,spy=spyOn(fs,"openSync").mockImplementation(((target:any,...args:any[])=>{if(String(target)===path)count++;return (open as any)(target,...args);}) as typeof fs.openSync);
 try{run();}finally{spy.mockRestore();}
 return count;
};

test("a validated block is read once, again after its leaf changed identity, and refused when it no longer holds its name",async()=>{
 const root=scratch(),{cachePath,digest}=await publish(root),block=inferenceObservationBlockPathV1(dirname(cachePath),digest);
 expect(opens(block,()=>{readInferenceHeaderV1(cachePath);readInferenceHeaderV1(cachePath);})).toBe(0);
 const bytes=readFileSync(block);
 rmSync(block);writeFileSync(block,bytes);
 expect(opens(block,()=>{readInferenceHeaderV1(cachePath);})).toBe(1);
 expect(opens(block,()=>{readInferenceHeaderV1(cachePath);})).toBe(0);
 rmSync(block);writeFileSync(block,Buffer.concat([bytes.subarray(0,bytes.length-1),Buffer.from([bytes[bytes.length-1]!^1])]));
 expect(()=>readInferenceHeaderV1(cachePath)).toThrow();
});

test("a block adopted after its hash was verified is registered and its first reader does not validate the leaf",async()=>{
 const origin=scratch(),{cachePath,digest,rows}=await publish(origin),adopted=scratch(),target=join(adopted,"inference","np","bbbbbbbbbbbbbbbbbbbbbbbb.json"),block=inferenceObservationBlockPathV1(dirname(target),digest);
 mkdirSync(dirname(block),{recursive:true});
 copyFileSync(inferenceObservationBlockPathV1(dirname(cachePath),digest),block);copyFileSync(cachePath,target);
 adoptVerifiedBlockV1(block,brotliDecompressSync(readFileSync(block)).toString("utf8"));
 let header:any;
 expect(opens(block,()=>{header=readInferenceHeaderV1(target);})).toBe(0);
 expect(header.observations).toEqual(rows);
 const other=scratch(),foreign=inferenceObservationBlockPathV1(join(other,"inference","np"),digest);
 mkdirSync(dirname(foreign),{recursive:true});copyFileSync(block,foreign);copyFileSync(cachePath,join(other,"inference","np","cccccccccccccccccccccccc.json"));
 expect(opens(foreign,()=>{readInferenceHeaderV1(join(other,"inference","np","cccccccccccccccccccccccc.json"));})).toBe(1);
});
