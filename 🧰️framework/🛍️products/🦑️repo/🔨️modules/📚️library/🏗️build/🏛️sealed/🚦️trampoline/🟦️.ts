import{spawn}from"node:child_process";
import{closeSync,createReadStream,fstatSync,openSync}from"node:fs";
import{constants as osConstants}from"node:os";
import{relative}from"node:path";
import{fileURLToPath}from"node:url";
import{repositoryCommandKernelV1}from"../../⌨️command/🧭️selection/🟦️.ts";
import kernelSchema from"../../⌨️command/🧭️selection/🧬️schema/🔣️.json"with{type:"json"};
import{acquirePartitionGroupV1,adoptPartitionPinV1,partitionGroupV1,readPartitionPinV1,releasePartitionPinV1}from"../🔀️partition/📌️retention/🟦️.ts";
import door from"../../🚪️door/🔣️.json"with{type:"json"};
import stamps from"../../🧷️format-stamps/🧬️schema/🔣️.json"with{type:"json"};
import{implementationModeV1,readSealedBlockedV1,readSealedCheckedV1,readSealedLoaderV1,readSealedPointerV1,readSealedReceiptV1,sealedEntryOfV1,sealedExplicitActionV1,sealedLanguagesV1,sealedLightActionV1,sealedLightCommandV1,sealedPolicyV1,sealedTextV1,sealedTreeDriftV1,writeSealedCheckedV1,type ImplementationModeV1,type SealedEntryV1,type SealedPointerV1,type SealedPolicyV1}from"../🟦️.ts";

/** 🏷️ The role argument that makes the sealed entry run the front door (the same literal the entry module exports; a law compares the two). */
const doorRoleV1="--semio-role=door";
/** 🔁️ What a promotion answers the trampoline. */
export interface TrampolinePromotionV1{readonly status:string;readonly pointer:Readonly<{digest:string}>}
/** 🔌️ What the trampoline needs beyond Node: the entry of the store and the promotion of a drifted tree (live code, loaded only when the tree drifted). */
export interface TrampolineOptionsV1{readonly entry?:string;readonly promote?:(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void)=>Promise<TrampolinePromotionV1>;readonly refresh?:(argv:readonly string[],environment:NodeJS.ProcessEnv)=>number|undefined;readonly policy?:SealedPolicyV1}
/** ⏳️ What a start does with the pointer it found. Under explicit promotion (the policy's `promotion.rule`) every command runs the promoted snapshot and a drift is only announced (`changed`), unless the snapshot is aged or this start is the restart of a command whose implementation yielded to a moved format stamp, see `sealedExplicitActionV1`. Under `drift` a heavy command promotes a drifted tree (or a snapshot without a receipt); a light command never waits for the stat walk while its snapshot is fresh, checks the tree once it is older, and then promotes before it runs only when the snapshot is old enough that running it would be wrong, or starts a detached promotion while it runs the snapshot. The facts of a check are kept in the store, so the next light commands skip it.
 * @see ../🟦️.ts the policy `light` and the plan, ../🧪️tests/🗂️store/🟦️.ts the laws */
function startVerdictV1(root:string,store:SealedEntryV1,mode:ImplementationModeV1,pointer:SealedPointerV1|undefined,light:boolean,policy:SealedPolicyV1,restarted:boolean,now=Date.now()):Readonly<{action:"run"|"promote"|"refresh";start:boolean;changed:readonly string[]}>{
 if(mode==="promote"||!pointer)return{action:"promote",start:false,changed:[]};
 if(policy.promotion.rule==="explicit"){
  let changed:readonly string[]=[];
  const checked=readSealedCheckedV1(store,pointer.digest),plan=sealedExplicitActionV1({now,promotedAtMs:pointer.promotedAtMs,checked,restarted,drifted:()=>{const receipt=readSealedReceiptV1(store,pointer.digest);changed=receipt?.tree?sealedTreeDriftV1(root,receipt.tree).changed:["."];return changed.length>0;},blocked:()=>{const known=readSealedBlockedV1(store);return known?.tree!=null&&sealedTreeDriftV1(root,known.tree).fresh;}},policy);
  if(plan.probed)writeSealedCheckedV1(store,{version:1,entry:store.entry,digest:pointer.digest,atMs:now,drifted:plan.drifted,refreshedAtMs:checked?.refreshedAtMs??null});
  return{action:plan.action,start:false,changed:plan.notice?changed:[]};
 }
 if(!light){const receipt=readSealedReceiptV1(store,pointer.digest);return{action:!receipt?.tree||now-pointer.promotedAtMs>=policy.coalesceMilliseconds&&!sealedTreeDriftV1(root,receipt.tree).fresh?"promote":"run",start:false,changed:[]};}
 const checked=readSealedCheckedV1(store,pointer.digest),plan=sealedLightActionV1({now,promotedAtMs:pointer.promotedAtMs,checked,drifted:()=>{const receipt=readSealedReceiptV1(store,pointer.digest);return!receipt?.tree||!sealedTreeDriftV1(root,receipt.tree).fresh;},blocked:()=>{const known=readSealedBlockedV1(store);return known?.tree!=null&&sealedTreeDriftV1(root,known.tree).fresh;}},policy);
 if(plan.probed)writeSealedCheckedV1(store,{version:1,entry:store.entry,digest:pointer.digest,atMs:now,drifted:plan.drifted,refreshedAtMs:plan.start?now:checked?.refreshedAtMs??null});
 return{action:plan.action,start:plan.start,changed:[]};
}
/** 🔄️ Starts the detached process that promotes the live tree while a light command runs the snapshot it has (the same door script in `promote` mode, which runs nothing); the process id, or nothing when it could not be started. */
function refreshV1(argv:readonly string[],environment:NodeJS.ProcessEnv,policy:SealedPolicyV1=sealedPolicyV1):number|undefined{
 try{const script=process.argv[1];if(!script)return undefined;const child=spawn(process.execPath,[script,...argv],{detached:true,stdio:"ignore",env:{...environment,[policy.modeVariable]:"promote"}});child.unref();return child.pid;}catch{return undefined;}
}

const promoteLiveV1=async(root:string,entry:string,environment:NodeJS.ProcessEnv,write:(text:string)=>void):Promise<TrampolinePromotionV1>=>(await import("../🔁️promoter/🟦️.ts")).promoteLiveTreeV1(root,entry,environment,write);

/** 🚦️ Starts the build tool's front door from the sealed snapshot. This is the only code of the tool that runs from the live tree on every start, so it imports Node built-ins and the store reader only; everything heavier is loaded when it is needed. In `sealed` mode it compares the live tree with the tree the current snapshot was captured from (a stat fingerprint, no file read) and, when the tree drifted, promotes it from live code first (the store promotes only a tree that parses and boots, under one lock for all processes); a promotion that is blocked or fails says so in English and German and the last good snapshot runs, so a half-finished edit anywhere in the tool cannot stop a command. `live` (the declared opt-in of the agent that edits the tool) runs the live front door, `promote` promotes and runs nothing, a store without a snapshot runs the live door, and a sealed door that ends before it reported that it started (a snapshot that no longer loads here; not on Windows, where the report cannot be trusted and a command must never run twice) says so and runs the live door too. Signals reach the sealed door, its exit code is ours. A light command checks the live tree only once its snapshot is older than `light.freshMilliseconds`: a drifted tree older than `light.promoteAfterMilliseconds` is promoted before the command runs, a younger one is promoted by a detached process while the command runs the snapshot (one notice, debounced), and a snapshot still older than `light.refuseAfterMilliseconds` after the promotion attempt is refused with exit 75 (under the `drift` rule; under explicit promotion a light command whose aged promotion did not move the snapshot runs the last good one, as a heavy command does). On POSIX the snapshot reaches the sealed door as an inherited read-only descriptor of its stored file (one read instead of a pipe through this process).
 * @see ../🔁️promoter/🟦️.ts the promoter, ../🚪️entry/🟦️.ts the entry the snapshot runs, ../🟦️.ts the policy and the store */
export async function startImplementationV1(root:string,argv:readonly string[],environment:NodeJS.ProcessEnv,live:()=>Promise<void>,options:TrampolineOptionsV1={}):Promise<void>{
 const policy=options.policy??sealedPolicyV1;
 let mode;
 try{mode=implementationModeV1(environment,policy);}catch(error){process.stderr.write((error as Error).message+"\n");process.exitCode=2;return;}
 if(mode==="live")return live();
 const entry=options.entry??relative(root,fileURLToPath(new URL("../🚪️entry/🟦️.ts",import.meta.url))).split("\\").join("/"),store=sealedEntryOfV1(root,entry,policy),languages=sealedLanguagesV1(environment),promote=options.promote??promoteLiveV1,say=(text:string)=>{try{process.stderr.write(text);}catch{}};
 const kernelArgs=argv[0]==="repo"?argv.slice(1):argv,kernel=options.entry?undefined:repositoryCommandKernelV1(kernelArgs,environment),kernelEntry=kernel&&relative(root,kernel.entry).split("\\").join("/"),allKernels=kernelEntry&&(kernel?.kind==="utility"||argv[0]==="test"||argv[0]==="build")?Object.values(kernelSchema.properties.kernels.const).map(path=>relative(root,fileURLToPath(new URL("../../⌨️command/"+path,import.meta.url))).split("\\").join("/")):kernelEntry?[kernelEntry]:[];
 let promotionAccepted=true;const promotePartition=async(path:string)=>{const result=await promote(root,path,environment,say);if(!["fresh","refreshed","promoted","served"].includes(result.status))promotionAccepted=false;return result;};
 const inheritedToken=mode!=="promote"&&allKernels.length?environment.SEMIO_SEALED_PARTITION_PIN:undefined,inherited=inheritedToken?readPartitionPinV1(root,inheritedToken,policy):undefined,inheritedGroup=inherited&&inherited.door.entry===entry&&inherited.door.digest===environment.SEMIO_SEALED_DOOR_DIGEST&&allKernels.every(path=>inherited.kernels.some(value=>value.entry===path))?inherited:undefined;
 let pin:string|undefined,pointer=inheritedGroup?.door??readSealedPointerV1(store);
 const light=sealedLightCommandV1(argv,door.light.map(row=>row.operation)),verdict=startVerdictV1(root,store,mode,pointer,light,policy,(Number.parseInt(environment[stamps.$defs.Protocol.const.reexecutionEnvironment]??"0",10)||0)>0);
 if(!inheritedGroup&&verdict.action==="promote"){
  try{
   const done=await promotePartition(entry);
   for(const selectedEntry of allKernels)await promotePartition(selectedEntry);
   if(mode==="promote"&&!allKernels.length){say(sealedTextV1(languages,"promoteDone",{digest:done.pointer.digest.slice(0,12),status:done.status})+"\n");process.exitCode=["fresh","refreshed","promoted","served"].includes(done.status)?0:1;return;}
  }catch(error){
   const message=String((error as Error).message??error);
   say((error as{code?:string}).code==="SEMIO_IMPLEMENTATION_NONE"?message+"\n":sealedTextV1(languages,"promoter",{reason:message.replace(/\s+/gu," ").slice(0,200)})+"\n");
   if(mode==="promote"){process.exitCode=1;return;}
  }
  pointer=readSealedPointerV1(store);
  if(light&&pointer&&policy.promotion.rule!=="explicit"&&Date.now()-pointer.promotedAtMs>=policy.light.refuseAfterMilliseconds){
   const rows=readSealedBlockedV1(store)?.rows??[],shown=rows.slice(0,policy.shownBlockedRows).map(row=>sealedTextV1(["en"],"blockedRow",{path:row.path,line:row.line,column:row.column,message:row.message})).join("; ")||"-";
   say(sealedTextV1(languages,"refused",{digest:pointer.digest.slice(0,12),hours:Math.floor((Date.now()-pointer.promotedAtMs)/3600000),rows:shown,variable:policy.modeVariable},policy)+"\n");process.exitCode=75;return;
  }
 }else if(!inheritedGroup&&verdict.action==="refresh"&&verdict.start&&pointer){
  const pid=(options.refresh??refreshV1)(argv,environment,policy);
  if(pid!==undefined)say(sealedTextV1(languages,"stale",{digest:pointer.digest.slice(0,12),minutes:Math.floor((Date.now()-pointer.promotedAtMs)/60000),pid},policy)+"\n");
 }else if(!inheritedGroup&&verdict.changed.length&&pointer)say(sealedTextV1(languages,"unpromoted",{count:verdict.changed.length,first:verdict.changed[0]!,digest:pointer.digest.slice(0,12),promoted:new Date(pointer.promotedAtMs).toTimeString().slice(0,8),variable:policy.modeVariable},policy)+"\n");
 const selectedPointers:SealedPointerV1[]=[];
 for(const selectedEntry of inheritedGroup?[]:allKernels){
  const kernelStore=sealedEntryOfV1(root,selectedEntry,policy);let selected=readSealedPointerV1(kernelStore);
  const kernelVerdict=startVerdictV1(root,kernelStore,mode,selected,light,policy,(Number.parseInt(environment[stamps.$defs.Protocol.const.reexecutionEnvironment]??"0",10)||0)>0);
  if(kernelVerdict.action!=="run"){try{await promotePartition(selectedEntry);}catch(error){promotionAccepted=false;say(String((error as Error).message??error)+"\n");}selected=readSealedPointerV1(kernelStore);}
  if(selected)selectedPointers.push(selected);
 }
 if(inheritedGroup&&inheritedToken){try{pin=adoptPartitionPinV1(root,inheritedToken,policy);}catch(error){say(String((error as Error).message??error)+"\n");process.exitCode=1;return;}}
 else if(allKernels.length){try{const candidate=promotionAccepted&&pointer&&selectedPointers.length===allKernels.length?partitionGroupV1(pointer,selectedPointers):undefined,admission=acquirePartitionGroupV1(root,entry,allKernels,candidate,policy);pin=admission.token;pointer=admission.group.door;if(mode==="promote"){releasePartitionPinV1(root,pin,policy);process.exitCode=admission.committed?0:1;return;}}catch(error){say(String((error as Error).message??error)+"\n");process.exitCode=1;return;}}
 const loader=pointer&&readSealedLoaderV1(store,pointer);
 if(!pointer||loader===undefined){if(pin){releasePartitionPinV1(root,pin,policy);say("Pinned authenticated door is unavailable.\n");process.exitCode=1;return;}return live();}
 try{
 let snapshot:number|undefined;
 if(process.platform!=="win32")try{snapshot=openSync(store.snapshot(pointer.digest),"r");if(fstatSync(snapshot).size!==pointer.bytes){closeSync(snapshot);if(pin){say("Pinned authenticated door changed.\n");process.exitCode=1;return;}return live();}}catch{if(snapshot!==undefined)try{closeSync(snapshot);}catch{}if(pin){say("Pinned authenticated door is unavailable.\n");process.exitCode=1;return;}return live();}
 const code=await new Promise<number|undefined>(accept=>{
  const child=spawn(process.execPath,["--eval",loader,"--",pointer.digest,doorRoleV1,...argv],{stdio:["inherit","inherit","inherit",snapshot??"pipe","pipe"],env:{...environment,...(pin?{SEMIO_SEALED_PARTITION_PIN:pin}:{}),BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}}),forwards=(["SIGINT","SIGTERM","SIGHUP"] as const).map(signal=>[signal,()=>{stopped=true;try{child.kill(signal);}catch{}}] as const);
  let ready=false,stopped=false;
  for(const[signal,forward] of forwards)process.on(signal,forward);
  const finish=(value:number|undefined)=>{for(const[signal,forward] of forwards)process.off(signal,forward);accept(value);};
  if(snapshot!==undefined)try{closeSync(snapshot);}catch{}
  const feed=child.stdio[3] as NodeJS.WritableStream|null;
  if(feed){feed.on("error",()=>{});createReadStream(store.snapshot(pointer.digest)).on("error",()=>{try{feed.end();}catch{}}).pipe(feed);}
  child.stdio[4]?.on("data",()=>{ready=true;});child.stdio[4]?.on("error",()=>{});
  child.once("error",()=>finish(undefined));
  child.once("close",(status,signal)=>{const exit=status??(signal?128+(osConstants.signals[signal]??0):1);finish(ready||stopped||process.platform==="win32"||status===0?exit:undefined);});
 });
 if(code===undefined){say(sealedTextV1(languages,"startup",{code:"-"})+"\n");if(pin){process.exitCode=1;return;}return live();}
 process.exitCode=code;
 }finally{if(pin)releasePartitionPinV1(root,pin,policy);}
}
