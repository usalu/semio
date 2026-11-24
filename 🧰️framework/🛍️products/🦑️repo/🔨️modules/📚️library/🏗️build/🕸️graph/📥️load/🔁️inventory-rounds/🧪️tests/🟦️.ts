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

test("a fresh path in every round ends a read that cannot be lenient after twice the round bound with every dirty owner checked once per round",async()=>{
  const readers:Record<string,string[]>={};for(let at=0;at<40;at++)readers["p"+at+".txt"]=[names[at]!];
  const {state,ports}=world(readers,round=>["p"+(round-1)+".txt"]);
  await expect(settleInventoryRoundsV1(ports)).rejects.toThrow("moved");
  expect(state.rounds).toBe(2*limits.inventoryRounds);
  expect(state.checks).toBe(OWNERS+2*limits.inventoryRounds-1);
});

test("a fresh path in every round still seals: after twice the round bound the last attempt is lenient and names what vanished",async()=>{
  const readers:Record<string,string[]>={};for(let at=0;at<40;at++)readers["p"+at+".txt"]=[names[at]!];
  const {state,ports}=world(readers,round=>["p"+(round-1)+".txt"]),lenientRounds:number[]=[];
  const attempt=ports.attempt;
  const outcome=await settleInventoryRoundsV1({...ports,lenient:true,attempt:async(round,volatile,lenient)=>{
    if(lenient){lenientRounds.push(round);(volatile as Set<string>).add("late-"+round);state.rounds=round;return "sealed";}
    return attempt(round,volatile);
  }});
  expect(outcome.value).toBe("sealed");
  expect(lenientRounds).toEqual([2*limits.inventoryRounds+1]);
  expect(outcome.volatile.paths).toContain("late-"+(2*limits.inventoryRounds+1));
  expect(outcome.volatile.paths.filter(path=>path.startsWith("p")).length).toBeGreaterThanOrEqual(limits.inventoryRounds);
  expect(state.checks).toBeLessThanOrEqual(OWNERS+2*limits.inventoryRounds);
});

test("a tree whose members vanish while it is walked: the lenient attempt of the real walk and hash names them and does not fail",async()=>{
  const root=await mkdtemp(join(tmpdir(),"inventory-lenient-"));
  try{
    await mkdir(join(root,"stable"),{recursive:true});await mkdir(join(root,"flicker"),{recursive:true});
    await writeFile(join(root,"stable","package.json"),"{}");await writeFile(join(root,"flicker","package.json"),"{}");
    const files=await workspaceBuildFilesV1(root);
    await rm(join(root,"flicker"),{recursive:true,force:true});
    await expect(buildGraphMembershipV1(root,files)).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED"});
    const named=new Set<string>(),digest=await buildGraphMembershipV1(root,files,undefined,new Set(),path=>named.add(path));
    expect([...named].sort()).toEqual(["flicker","flicker/package.json"]);
    const walkedNamed=new Set<string>(),walked=await workspaceBuildFilesV1(root,undefined,new Set(),path=>walkedNamed.add(path));
    expect(walked).not.toContain("flicker/package.json");expect(walkedNamed.size).toBe(0);
    expect(digest).toBe(await buildGraphMembershipV1(root,walked));
    await mkdir(join(root,"stable","inner"),{recursive:true});
    const gone=new Set<string>(),partial=await workspaceBuildFilesV1(root,undefined,new Set(),path=>gone.add(path));
    expect(partial).toContain("stable/package.json");
  }finally{await rm(root,{recursive:true,force:true});}
});

test("a failure that names no path stays fatal and an unreadable change without paths gives up after the repair rounds",async()=>{
  const {ports}=world({},()=>[]);
  await expect(settleInventoryRoundsV1({...ports,attempt:async()=>{throw Error("boom");},follow:()=>({membership:false,paths:[]})})).rejects.toThrow("boom");
  let rounds=0;
  const outcome=await settleInventoryRoundsV1({...ports,orNothing:true,attempt:async()=>{rounds++;throw Error("moved");},follow:()=>({membership:true,paths:[]})});
  expect(outcome.value).toBeUndefined();
  expect(rounds).toBe(limits.repairRounds);
});

test("a failure that keeps naming only a tolerated path ends the read at once with a bounded number of owner checks",async()=>{
  const {state,ports}=world({"gone.txt":["owner-3"]},()=>[]);
  const refused=Object.assign(Error("owner reads a file that stays gone"),{code:"BUILD_SOURCE_UNPARSEABLE"});
  await expect(settleInventoryRoundsV1({...ports,attempt:async round=>{state.rounds=round;state.checks++;throw refused;},follow:()=>({membership:false,paths:["gone.txt"]})})).rejects.toBe(refused);
  expect(state.rounds).toBe(limits.volatileRounds+1);
  expect(state.checks).toBe(limits.volatileRounds+1);
  let rounds=0;
  const outcome=await settleInventoryRoundsV1({...ports,orNothing:true,attempt:async()=>{rounds++;throw Object.assign(Error("moved"),{code:"BUILD_MEMBERSHIP_CHANGED"});},follow:()=>({membership:true,paths:["gone.txt"]})});
  expect(outcome.value).toBeUndefined();
  expect(outcome.volatile.paths).toEqual(["gone.txt"]);
  expect(rounds).toBe(limits.volatileRounds+1);
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
    const kept=await buildGraphMembershipV1(root,files,undefined,new Set(["gone"])),walked=await workspaceBuildFilesV1(root);
    expect(walked).not.toContain("gone/package.json");
    expect(kept).toBe(await buildGraphMembershipV1(root,walked));
  }finally{await rm(root,{recursive:true,force:true});}
});

test("the dirty and volatile lines name the owners and paths in English and German",()=>{
  const dirty=graphDirtyTextV1(2,["a.txt","b.txt"],["owner-1","owner-2"],2),volatile=graphVolatileTextV1(["a.txt"],["owner-1"]);
  expect(dirty.en).toContain("owner-1, owner-2");expect(dirty.en).toContain("a.txt, b.txt");expect(dirty.de).toContain("Eigentümer");
  expect(volatile.en).toContain("volatile");expect(volatile.de).toContain("flüchtig");expect(volatile.de).toContain("owner-1");
});
