import {afterAll,expect,test} from "bun:test";
import {copyFileSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1} from "../../🟨️.mjs";
import {carryEdgeStampsV1,restampComponentValidEdgesV1} from "../../🪡️edge-heal/🟨️.mjs";
import {componentEdgeValidityOnV1,createComponentValidityV1,indexEnvironmentV1,readsDigestV1,recordKeysV1} from "../🟨️.mjs";

const ORACLE=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url)),scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const worlds=async(seed:number,count:number):Promise<any[]>=>{
 const directory=mkdtempSync(join(tmpdir(),"component-oracle-"));scratch.push(directory);const file=join(directory,"worlds.json"),script=join(directory,"oracle.py");copyFileSync(ORACLE,script);
 const child=Bun.spawn(["python3","-I",script,file,String(seed),String(count)],{stdout:"pipe",stderr:"pipe"}),code=await child.exited;
 expect(code).toBe(0);return JSON.parse(readFileSync(file,"utf8"));
};
const closed=(reads:any)=>{const out:any={};for(const key of["owners","paths","packages","modules","workspaces"])if(reads[key].length)out[key]=reads[key];if(reads.lock)out.lock=true;return out;};

test("the component rule equals the CPython oracle on random worlds: keys, digests of the components an owner read, the owners an environment change re-stales; and it misses no owner whose edges truly changed",async()=>{
 let records=0,stale=0,truth=0;
 for(const world of await worlds(1127,60)){
  const first=createComponentValidityV1(world.env1),second=createComponentValidityV1(world.env2),index1=indexEnvironmentV1(world.env1),flagged:string[]=[];
  for(const record of world.records){
   const row1=world.env1.owners.find((owner:string[])=>owner[0]===record.name),row={root:row1[1],nativeRoot:row1[2],reads:closed(record.reads)};
   expect(recordKeysV1({name:record.name,...row},row.reads,index1)).toEqual(record.keys);
   expect(readsDigestV1(record.keys,index1)).toBe(record.readsDigest);
   expect(first.digestOf(record.name,row)).toBe(record.readsDigest);expect(first.holds(record.name,{...row,edgesReads:record.readsDigest})).toBe(true);
   if(!second.holds(record.name,{...row,edgesReads:record.readsDigest}))flagged.push(record.name);
  }
  expect(flagged.sort()).toEqual(world.flagged);
  for(const name of world.truth)expect(flagged.includes(name),`${name} changed but was not re-staled`).toBe(true);
  records+=world.records.length;stale+=flagged.length;truth+=world.truth.length;
 }
 expect(records).toBeGreaterThan(1000);expect(stale).toBeLessThan(records);expect(stale).toBeGreaterThanOrEqual(truth);
});

test("an environment change re-stales only the owners whose components changed: the others are restamped, not derived again, however many environments lie between",async()=>{
 const [world]=await worlds(7,1),second=createComponentValidityV1(world.env2),first=createComponentValidityV1(world.env1),owners:Record<string,any>={};
 for(const record of world.records){const row1=world.env1.owners.find((owner:string[])=>owner[0]===record.name);owners[record.name]={root:row1[1],nativeRoot:row1[2]??undefined,reads:closed(record.reads),edgesEnv:"old",edgesReads:first.digestOf(record.name,{root:row1[1],nativeRoot:row1[2],reads:closed(record.reads)}),kind:"project"};}
 owners[world.records[0].name].tag="tests";
 const result=restampComponentValidEdgesV1({owners,environment:"new",component:second,pending:[]});
 const kept=world.records.map((record:any)=>record.name).filter((name:string)=>!world.flagged.includes(name)&&name!==world.records[0].name).sort();
 expect(result.restamped).toEqual(kept);
 for(const name of Object.keys(owners))expect(result.owners[name].edgesEnv).toBe(kept.includes(name)?"new":"old");
 expect(restampComponentValidEdgesV1({owners:{x:{...owners[world.records[1].name],edgesReads:undefined}},environment:"new",component:second}).restamped,"an owner without a recorded digest is never restamped").toEqual([]);
 expect(restampComponentValidEdgesV1({owners:{[world.records[1].name]:owners[world.records[1].name]},environment:"new",component:second,pending:[world.records[1].name]}).restamped,"an owner pending for inference is left to its wave").toEqual([]);
});

test("the flag is off by default and is switched by the policy or by the environment",()=>{
 expect(GENERATION_POLICY_V1.componentEdgeValidity).toBe(false);
 expect(componentEdgeValidityOnV1(GENERATION_POLICY_V1,{})).toBe(false);
 expect(componentEdgeValidityOnV1({componentEdgeValidity:true},{})).toBe(true);
 expect(componentEdgeValidityOnV1(GENERATION_POLICY_V1,{SEMIO_GRAPH_COMPONENT_EDGES:"1"})).toBe(true);
});

test("a build stamps carried owners with the environment and, with a judge, the digest of the components they read; without a judge rows get no digest",async()=>{
 const [world]=await worlds(11,1),judge=createComponentValidityV1(world.env2),record=world.records[0],row1=world.env1.owners.find((owner:string[])=>owner[0]===record.name);
 const make=():Record<string,any>=>({[record.name]:{root:row1[1],nativeRoot:row1[2]??undefined,reads:closed(record.reads),edgesEnv:"old",kind:"project"}});
 const withJudge=make(),without=make(),said:string[]=[];
 carryEdgeStampsV1({owners:withJudge,carried:[record.name],returned:new Set(),environment:"new",component:judge,cold:false,say:(key)=>said.push(key)});
 carryEdgeStampsV1({owners:without,carried:[record.name],returned:new Set(),environment:"new",component:undefined,cold:false});
 expect(withJudge[record.name].edgesEnv).toBe("new");expect(withJudge[record.name].edgesReads).toBe(judge.digestOf(record.name,withJudge[record.name]));
 expect(without[record.name].edgesEnv).toBe("new");expect(without[record.name].edgesReads).toBeUndefined();expect(said).toEqual([]);
});
