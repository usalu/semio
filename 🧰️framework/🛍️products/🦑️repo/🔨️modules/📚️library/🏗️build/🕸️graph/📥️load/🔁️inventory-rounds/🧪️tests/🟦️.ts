/** 🔁️ Contract: an inventory that follows a live tree pays for the dirty owners only, and a path that keeps changing seals as volatile within a bound instead of repeating forever. */
import {expect,test} from "bun:test";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {settleInventoryRoundsV1} from "../🟦️.ts";
import {buildGraphMembershipV1,workspaceBuildFilesV1} from "../../../📁️workspace/🟦️.ts";
import {graphDirtyTextV1,graphVolatileTextV1} from "../../🧲️membership-follow/🟦️.ts";

const limits={repairRounds:4,volatileRounds:3,inventoryRounds:12},OWNERS=60;
const names=Array.from({length:OWNERS},(_,at)=>"owner-"+at);

/** 🌍️ A tree of OWNERS owners where `readers` read the file `moving`; every attempt observes the owners the memo forgot and fails with the churn error while the moving path is not tolerated and `churn` says it moved again. */
function world(readers:Record<string,string[]>,churn:(round:number)=>string[]){
  const memo=new Set<string>(),state={checks:0,rounds:0,lines:[] as unknown[]};
  const ports={
    attempt:async(round:number,volatile:ReadonlySet<string>)=>{
      state.rounds=round;
      for(const name of names)if(!memo.has(name)){state.checks++;memo.add(name);}
      const moved=churn(round).filter(path=>!volatile.has(path));
      if(moved.length>0)throw Object.assign(Error("moved"),{code:"BUILD_MEMBERSHIP_CHANGED",paths:moved});
      return "sealed";
    },
    follow:(error:unknown)=>({membership:true,paths:(error as {paths:string[]}).paths}),
    forget:(paths:readonly string[])=>{const gone=names.filter(name=>paths.some(path=>readers[path]?.includes(name)));for(const name of gone)memo.delete(name);return gone;},
    limits,orNothing:false,dirty:(event:unknown)=>state.lines.push(event),
  };
  return {state,ports};
}

test("a path that changes once is followed with only its readers observed again",async()=>{
  const {state,ports}=world({"noise.txt":["owner-2","owner-5"]},round=>round===1?["noise.txt"]:[]);
  const outcome=await settleInventoryRoundsV1(ports);
  expect(outcome.value).toBe("sealed");
  expect(outcome.volatile).toEqual({paths:[],owners:[]});
  expect(state.checks).toBe(OWNERS+2);
});

test("a path that changes in every round seals as volatile with bounded owner checks",async()=>{
  const readers=["owner-1","owner-7","owner-9"],{state,ports}=world({"churn.txt":readers},()=>["churn.txt"]);
  const outcome=await settleInventoryRoundsV1(ports);
  expect(outcome.value).toBe("sealed");
  expect(outcome.volatile).toEqual({paths:["churn.txt"],owners:readers});
  expect(state.rounds).toBe(limits.volatileRounds+1);
  expect(state.checks).toBeLessThanOrEqual(OWNERS+limits.volatileRounds*readers.length);
  expect(state.lines.map(line=>(line as {streak:number}).streak)).toEqual([1,2,3]);
});

test("a fresh path in every round ends the read after twice the round bound with every dirty owner checked once per round",async()=>{
  const readers:Record<string,string[]>={};for(let at=0;at<40;at++)readers["p"+at+".txt"]=[names[at]!];
  const {state,ports}=world(readers,round=>["p"+(round-1)+".txt"]);
  await expect(settleInventoryRoundsV1(ports)).rejects.toThrow("moved");
  expect(state.rounds).toBe(2*limits.inventoryRounds);
  expect(state.checks).toBe(OWNERS+2*limits.inventoryRounds-1);
});

test("a failure that names no path stays fatal and an unreadable change without paths gives up after the repair rounds",async()=>{
  const {ports}=world({},()=>[]);
  await expect(settleInventoryRoundsV1({...ports,attempt:async()=>{throw Error("boom");},follow:()=>({membership:false,paths:[]})})).rejects.toThrow("boom");
  let rounds=0;
  const outcome=await settleInventoryRoundsV1({...ports,orNothing:true,attempt:async()=>{rounds++;throw Error("moved");},follow:()=>({membership:true,paths:[]})});
  expect(outcome.value).toBeUndefined();
  expect(rounds).toBe(limits.repairRounds);
});

test("a volatile directory leaves the walk and the membership hash of a real tree",async()=>{
  const root=await mkdtemp(join(tmpdir(),"inventory-volatile-"));
  try{
    await mkdir(join(root,"stable"),{recursive:true});await mkdir(join(root,"gone"),{recursive:true});
    await writeFile(join(root,"stable","package.json"),"{}");await writeFile(join(root,"gone","package.json"),"{}");
    const files=await workspaceBuildFilesV1(root);
    expect(files).toContain("gone/package.json");
    await rm(join(root,"gone"),{recursive:true,force:true});
    await expect(buildGraphMembershipV1(root,files)).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED",paths:["gone","gone/package.json"]});
    const kept=await buildGraphMembershipV1(root,files,undefined,new Set(["gone"])),only=await buildGraphMembershipV1(root,["stable/package.json"]);
    expect(kept).toBe(only);
    expect(await workspaceBuildFilesV1(root)).not.toContain("gone/package.json");
  }finally{await rm(root,{recursive:true,force:true});}
});

test("the dirty and volatile lines name the owners and paths in English and German",()=>{
  const dirty=graphDirtyTextV1(2,["a.txt","b.txt"],["owner-1","owner-2"],2),volatile=graphVolatileTextV1(["a.txt"],["owner-1"]);
  expect(dirty.en).toContain("owner-1, owner-2");expect(dirty.en).toContain("a.txt, b.txt");expect(dirty.de).toContain("Eigentümer");
  expect(volatile.en).toContain("volatile");expect(volatile.de).toContain("flüchtig");expect(volatile.de).toContain("owner-1");
});
