import{join,sep}from"node:path";
import{createProjectFileIndexV1}from"../../🧭️file-owner/🟨️.mjs";
import{stampOfV1}from"../../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{profilePhaseV1}from"../../../⏱️profile/🟨️.mjs";
import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{rowOwnerNamesV1,selectFactClassesV1,staleFactRowsV1}from"../🧮️fact-rows/🟨️.mjs";
import{holdersOfDerivedV1,isDerivedOwnerV1,scopeWithDerivedV1,splitStaleConsumersV1,suspectRowsV1,suspectsWithCleanHolderV1,suspectsWithinV1,verifiableRowIdsV1}from"../✂️cutoff/🟨️.mjs";
import{selectDoubtfulRowsV1}from"../🔎️doubt/🟨️.mjs";
import{staleConfigOwnersV1}from"../🏷️config-owners/🟨️.mjs";
import{pendingCratePathsV1}from"../🕓️pending-crates/🟨️.mjs";
import{P}from"../🧱️guards/🟨️.mjs";
import{revalidatedWithheldOwnersV1}from"../../📥️load/🛟️owner-isolation/🔁️revalidation/🟨️.mjs";

/** 🚧️ The withheld owners of a generation that do not hold the source that withholds them: an untagged `unavailable` owner names the file that does not parse (`config`), and it stays valid only while the bytes of that file are the ones it was withheld for, so a generation that carries no fact about that file (a withholding recorded before its facts were kept) can never notice the repair and its owner must be inferred again. Sources outside the workspace and tagged owners (rediscovered as a group) are not judged. The judgement is global: a repair is bounded by the withheld owners and one build heals all of them, instead of one build per layer of the closure they open. */
export function unfoundedWithheldOwnersV1(index,owners,separator=sep){
 const open=new Map(),kinds=new Set(["file","analysis","digest"]);
 for(const[name,row]of Object.entries(owners)){
  if(row.kind!=="unavailable"||row.tag||typeof row.config!=="string"||row.config.startsWith(".."))continue;
  const at=index.owners.indexOf(name);if(at>=0)open.set(at,[name,row.config]);
 }
 for(const cls of index.classes){
  if(!open.size)break;
  const here=cls.o.filter(at=>open.has(at));if(!here.length)continue;
  const configs=new Set(here.map(at=>open.get(at)[1]));
  for(const id of cls.r){
   const row=index.rows[id];if(!kinds.has(row.kind))continue;
   const path=separator==="/"?row.path:row.path.split(separator).join("/");
   for(const config of configs)if(path===config||path.endsWith("/"+config)){for(const at of here)if(open.get(at)?.[1]===config)open.delete(at);configs.delete(config);}
   if(!configs.size)break;
  }
 }
 return[...open.values()].map(([name])=>name);
}

/** 🧭️ Verifies selected owners' facts and maps membership changes to their roots. An absent frontier diff requires a cold plan. */
export function createGenerationPlanV1({readIndexView,fragmentNames,now,doubt,settler,derive,hold,workspaceRoot,ledger,say,reasonOf}){
 return async function plan({current,identity,implementation,changes,journalDirty,only,repairBase,forcedOwners,verifyAll=false,skipEnvironment=false,signal,progress}){
  const cold=reason=>({kind:"cold",reason,dirty:new Set(),added:[],removed:[],pending:[],envStale:true,checked:0});
  const manifest=repairBase??current?.manifest;
  if(!manifest)return cold(current?(current.missing?"manifest":"invalidated"):"empty");
  if(manifest.identity!==identity)return cold("identity");
  if(changes===undefined)return cold("frontier");
  const times={},indexAt=now(),index=await profilePhaseV1("graph.plan.index",()=>readIndexView(manifest.factIndex));times.index=now()-indexAt;if(!index)return cold("index");
  const present=new Set(await fragmentNames());if(!present.has(manifest.global+".json.br"))return cold("global");
  const dirty=new Set(forcedOwners),added=[],removed=new Set(),reasons=[],owners=Object.entries(manifest.owners),ownerOf=createProjectFileIndexV1(Object.fromEntries(owners.map(([name,row])=>[name,{root:row.root}])));
  const configs=new Map(owners.filter(([,row])=>row.config!==undefined).map(([name,row])=>[row.config,name]));
  let envStale=false;
  const pending=pendingCratePathsV1(changes,ownerOf);
  for(const change of changes){
   const known=configs.get(change.path);
   if(pending.has(change.path))continue;
   if(change.config&&change.kind==="added"){added.push(change.path);const prior=ownerOf(change.path);if(prior!==undefined&&manifest.owners[prior].config!==change.path&&change.path.startsWith(manifest.owners[prior].root+"/")&&!change.path.slice(manifest.owners[prior].root.length+1).includes("/"))removed.add(prior);}
   else if(change.config&&change.kind==="removed"&&known)removed.add(known);
   else if(change.config&&known)dirty.add(known);
   else if(change.directory&&change.kind!=="removed")continue;
   else{const owner=ownerOf(change.path);if(owner!==undefined)dirty.add(owner);else{envStale=true;if(change.kind!=="removed")added.push(change.path);}}
  }
  for(const[name,row]of owners)if((!only||only.has(name))&&!present.has(row.fragment+".json.br"))dirty.add(name);
  const unfounded=unfoundedWithheldOwnersV1(index,manifest.owners);for(const name of unfounded)dirty.add(name);
  const revalidated=ledger?await revalidatedWithheldOwnersV1(manifest.owners,{ledger,now,include:name=>!only||only.has(name),say,implementation,reasonOf}):[];for(const name of revalidated)dirty.add(name);
  const scope=only?new Set(skipEnvironment?only:[...only,P.envOwner]):undefined,selection=selectFactClassesV1(index,scope?scopeWithDerivedV1(index,scope):undefined);
  const factsAt=now(),verifiable=verifiableRowIdsV1(index,selection.ids),candidates=doubt&&!verifyAll?await profilePhaseV1("graph.plan.doubt",()=>selectDoubtfulRowsV1(index,verifiable,doubt,{signal,progress,chunk:P.doubtChunkRows})):verifiable;
  const{stale}=await profilePhaseV1("graph.plan.facts",()=>staleFactRowsV1(index,candidates,hold,{signal,progress,...doubt?{budget:Infinity}:{}}));
  times.facts=now()-factsAt;await doubt?.settled?.({rows:index.rows,index,ids:verifiable,stale,signal});
  const settleAt=now();const{direct,suspects:reached}=splitStaleConsumersV1(index,selection.classes,stale),inScope=scope?suspectsWithinV1(index,reached,scope):reached,rebuilt=new Set([...dirty,...direct,...staleConfigOwnersV1({rows:index.rows,stale,configs,workspaceRoot})].filter(name=>manifest.owners[name]&&!removed.has(name)&&(!only||only.has(name)))),suspects=suspectsWithCleanHolderV1(index,inScope,rebuilt),settled=await profilePhaseV1("graph.plan.settle",()=>settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{staleFacts:[...stale].map(id=>index.rows[id])},...workspaceRoot!==undefined?{vouchStamp:(name,cheap)=>stampOfV1(join(workspaceRoot,name),undefined,cheap)}:{}})),changedHolders=holdersOfDerivedV1(index,settled.changed.keys());
  await settler.release?.();times.settle=now()-settleAt;
  for(const name of direct){if(name===P.envOwner)envStale=true;else if(manifest.owners[name])dirty.add(name);}
  for(const name of staleConfigOwnersV1({rows:index.rows,stale,configs,workspaceRoot}))if(manifest.owners[name])dirty.add(name);
  for(const holders of changedHolders.values())for(const name of holders)if(manifest.owners[name])dirty.add(name);
  for(const change of changes){
   if(reasons.length>=P.reasonRows)break;if(change.directory&&change.kind!=="removed"||pending.has(change.path))continue;
   const owner=configs.get(change.path)??ownerOf(change.path);reasons.push({change:{path:change.path,kind:change.kind},owners:[owner??P.envOwner]});
  }
  const staleIds=[...stale].sort((a,b)=>a-b),holders=reasons.length<P.reasonRows&&staleIds.length?rowOwnerNamesV1(index,selection.classes,staleIds,isDerivedOwnerV1):new Map();
  for(const id of staleIds){
   if(reasons.length>=P.reasonRows)break;
   const names=holders.get(id);
   if(!names?.length)continue;
   reasons.push({row:index.rows[id],owners:names});
  }
  for(const[name,row]of settled.changed){if(reasons.length>=P.reasonRows)break;const names=[...changedHolders.get(name)??[]].sort(compareCodePointsV1).slice(0,3);if(names.length)reasons.push({row,owners:names});}
  for(const name of removed)dirty.delete(name);
  const scoped=only?new Set([...[...dirty].filter(name=>only.has(name)),...unfounded,...revalidated]):dirty,clean=!scoped.size&&!added.length&&!removed.size&&!envStale;
  const staleKinds={};for(const id of stale){const kind=index.rows[id].kind;staleKinds[kind]=(staleKinds[kind]??0)+1;}
  return{kind:clean?"clean":"delta",times,dirty:scoped,revalidated,added,removed:[...removed],pending:[...pending].sort(compareCodePointsV1),envStale,checked:selection.ids.length,stale:stale.size,doubtful:candidates.length,staleKinds,reasons,cutoff:{questions:suspects.size,skipped:inScope.size-suspects.size,held:settled.held.size,kept:settled.kept.size,changed:settled.changed.size,failed:settled.failed,reused:settled.reused},refresh:settled.held};
 }
}
