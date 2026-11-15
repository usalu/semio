/** 🍂️ Contract: a project record that is replaced while a lookup reads it (a farm worker, the fleet take or a peer renames a newer record over it) is read again and, if it keeps changing or is damaged, is a miss with one line saying why; any other failure of the read stays a failure. */
import{afterAll,expect,spyOn,test}from"bun:test";
import{copyFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import*as fs from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{inferenceObservationBlockPathV1,observeInference,observedFs,readInferenceHeaderV1,readInferenceRecordOrMissV1,settleObservationLeafV1}from"../../🟨️.mjs";

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

const changed=()=>Object.assign(Error("Observation leaf changed before delivery (the record was replaced while it was read)"),{code:"BUILD_INFERENCE_LEAF_CHANGED"});
const path="/ws/.cache/b/np/alpha.json";

test("a leaf replaced once or twice while it is read is read again and delivered",()=>{
 for(const failures of[1,2]){
  let calls=0;const lines:string[]=[],record={header:{},line:"l",value:{}};
  const got=readInferenceRecordOrMissV1(path,{read:()=>{if(++calls<=failures)throw changed();return record;},say:line=>lines.push(line)});
  expect({got,calls,lines}).toEqual({got:record,calls:failures+1,lines:[]});
 }
});

test("a leaf that keeps changing, and a damaged record, are a miss with one bilingual line; a record that is simply absent is a miss without a line",()=>{
 const lines:string[]=[];let calls=0;
 expect(readInferenceRecordOrMissV1(path,{read:()=>{calls++;throw changed();},say:line=>lines.push(line)})).toBeNull();
 expect(calls).toBe(3);expect(lines).toHaveLength(1);expect(lines[0]).toContain("np/alpha.json");expect(lines[0]).toContain("replaced while it was read");expect(lines[0]).toContain("abgeleitet");
 lines.length=0;
 expect(readInferenceRecordOrMissV1(path,{read:()=>{throw Object.assign(Error("Decompression failed"),{code:"Z_DATA_ERROR"});},say:line=>lines.push(line)})).toBeNull();
 expect(lines).toHaveLength(1);
 lines.length=0;
 expect(readInferenceRecordOrMissV1(path,{read:()=>null,say:line=>lines.push(line)})).toBeNull();expect(lines).toEqual([]);
});

test("every other failure of the read stays a failure and carries its message",()=>{
 expect(()=>readInferenceRecordOrMissV1(path,{read:()=>{throw Error("disk on fire");},say:()=>{}})).toThrow("disk on fire");
});

test("a leaf replaced while its block is published is read again and accepted, a leaf that keeps changing fails typed, every other failure at once",()=>{
 let calls=0;
 expect(settleObservationLeafV1(()=>{if(++calls<3)throw changed();return"held";})).toBe("held");expect(calls).toBe(3);
 calls=0;
 expect(()=>settleObservationLeafV1(()=>{calls++;throw changed();})).toThrow(expect.objectContaining({code:"BUILD_INFERENCE_LEAF_CHANGED"}));expect(calls).toBe(3);
 calls=0;
 expect(()=>settleObservationLeafV1(()=>{calls++;throw Error("disk on fire");})).toThrow("disk on fire");expect(calls).toBe(1);
});

const shared=async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"observation-leaf-race-"));roots.push(root);
 const sources=join(root,"sources"),first=join(root,"inference","np","aaaaaaaaaaaaaaaaaaaaaaaa.json"),second=join(root,"inference","np","bbbbbbbbbbbbbbbbbbbbbbbb.json");
 mkdirSync(sources,{recursive:true});mkdirSync(dirname(first),{recursive:true});
 for(let index=0;index<8;index++)writeFileSync(join(sources,`${index}.txt`),`source ${index}`);
 await Bun.sleep(2100);
 const compute=()=>Array.from({length:8},(_,index)=>observedFs.readFileSync(join(sources,`${index}.txt`),"utf8")).join("|");
 await observeInference({cachePath:first,key:"leaf-race"},compute);
 const block=inferenceObservationBlockPathV1(dirname(first),readInferenceHeaderV1(first).blocks[0]!);
 return{first,second,block,compute};
};
const replacing=(block:string,when:()=>boolean)=>{
 const stat=fs.lstatSync;let replaced=0;
 const spy=spyOn(fs,"lstatSync").mockImplementation(((target:any,...args:any[])=>{if(String(target)===block&&when()){copyFileSync(block,block+".peer");fs.renameSync(block+".peer",block);replaced++;}return(stat as any)(target,...args);}) as typeof fs.lstatSync);
 return{replaced:()=>replaced,restore:()=>spy.mockRestore()};
};

test("a shared block replaced by a peer with the same content while a record header is written is read again and the record is written",async()=>{
 const{first,second,block,compute}=await shared();let seen=0;const race=replacing(block,()=>++seen===2);
 try{await observeInference({cachePath:second,key:"leaf-race"},compute);}finally{race.restore();}
 expect(race.replaced()).toBe(1);expect(readInferenceHeaderV1(second).blocks).toEqual(readInferenceHeaderV1(first).blocks);
});

test("a shared block that keeps being replaced fails the record with BUILD_INFERENCE_LEAF_CHANGED",async()=>{
 const{second,block,compute}=await shared();const race=replacing(block,()=>true);let failure:any;
 try{await observeInference({cachePath:second,key:"leaf-race"},compute);}catch(error){failure=error;}finally{race.restore();}
 expect(failure?.code).toBe("BUILD_INFERENCE_LEAF_CHANGED");
});
