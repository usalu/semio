/** 🧒️ Contract: with four children verifying the records, a reader of 50 owners spends under 50 ms of its own CPU per owner, settles, verifies and probes nothing, and the verdicts come from several children; an edit of an input while the read runs is seen by the child that is asked next. @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const observation=await import("../../🟨️.mjs") as any;
const calls={settle:0,verify:0};
observation.registerClosureProviderV1({answer:()=>"true",settle:async()=>{calls.settle++;},verify:async()=>{calls.verify++;return true;}});
const childPath=join(import.meta.dir,"👷️child","🟦️.ts");
type Child={ask:(request:unknown)=>Promise<any>;stop:()=>void};
const spawnChild=():Child=>{
 const process_=Bun.spawn([process.execPath,childPath],{stdin:"pipe",stdout:"pipe",stderr:"inherit"}),waiting:((line:string)=>void)[]=[],reader=process_.stdout.getReader(),decoder=new TextDecoder();
 let buffer="";
 (async()=>{for(;;){const{value,done}=await reader.read();if(done)return;buffer+=decoder.decode(value);for(let cut=buffer.indexOf("\n");cut>=0;cut=buffer.indexOf("\n")){const line=buffer.slice(0,cut);buffer=buffer.slice(cut+1);waiting.shift()?.(line);}}})();
 let tail:Promise<unknown>=Promise.resolve();
 const ask=(request:unknown)=>{const run=tail.then(()=>new Promise<any>(resolve=>{waiting.push(line=>resolve(JSON.parse(line)));process_.stdin.write(JSON.stringify(request)+"\n");process_.stdin.flush();}));tail=run.catch(()=>undefined);return run;};
 return{ask,stop:()=>{try{process_.stdin.end();}catch{}process_.kill();}};
};

test("four children verify 50 owners and the reader spends under 50 ms of CPU per owner on them",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"adopted-children-")),input=join(root,"input.txt"),children=Array.from({length:4},spawnChild);
 try{
  writeFileSync(input,"one");
  await new Promise(done=>setTimeout(done,2200));
  const owners=await Promise.all(Array.from({length:50},async(_all,at)=>{
   const cachePath=join(root,"np","owner-"+at+".json"),paths=Array.from({length:200},(_row,row)=>JSON.stringify([root,"owner-"+at+"-"+row,["digest"]]));
   await observation.observeInference({cachePath,key:["owner-"+at]},async()=>{for(const path of paths)observation.recordClosureFactV1(path,"true");return{text:observation.observedFs.readFileSync(input,"utf8")};});
   return{cachePath,key:["owner-"+at]};
  }));
  calls.settle=0;calls.verify=0;
  const verifiers=new Set<number>();let next=0,asked=0;
  const before=process.cpuUsage();
  await Promise.all(children.map(async(child,lane)=>{
   while(next<owners.length){
    const owner=owners[next++]!;
    const value=await observation.observeInference({cachePath:owner.cachePath,key:owner.key,verdict:async()=>{asked++;const said=await child.ask(owner);verifiers.add(said.verifier);return said;}},async()=>{throw new Error("computed");});
    expect(value.text).toBe("one");
    expect(lane).toBeGreaterThanOrEqual(0);
   }
  }));
  const used=process.cpuUsage(before),perOwner=(used.user+used.system)/1000/owners.length;
  expect(asked).toBe(50);
  expect(verifiers.size).toBeGreaterThan(1);
  expect(calls).toEqual({settle:0,verify:0});
  expect(perOwner).toBeLessThan(50);
 }finally{for(const child of children)child.stop();rmSync(root,{recursive:true,force:true});}
},120000);

test("an edit of an input while the read runs is seen by the child that is asked next",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"adopted-children-edit-")),input=join(root,"input.txt"),child=spawnChild();
 try{
  writeFileSync(input,"one");
  await new Promise(done=>setTimeout(done,2200));
  const cachePath=join(root,"np","owner.json");
  await observation.observeInference({cachePath,key:["owner"]},async()=>({text:observation.observedFs.readFileSync(input,"utf8")}));
  expect((await child.ask({cachePath,key:["owner"]})).verdict).toBe("holds");
  writeFileSync(input,"two, longer");
  let computed=0;
  const value=await observation.observeInference({cachePath,key:["owner"],verdict:()=>child.ask({cachePath,key:["owner"]})},async()=>{computed++;return{text:observation.observedFs.readFileSync(input,"utf8")};});
  expect(computed).toBe(1);
  expect(value.text).toBe("two, longer");
 }finally{child.stop();rmSync(root,{recursive:true,force:true});}
},120000);
