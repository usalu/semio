import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{createProjectFleetPortV1,runTakeLookV1,type ProjectClaimV1,type ProjectRecordInfoV1}from"../../🟦️.ts";
import{OWNER_POOL_POLICY_V1,pullOwnerPoolV1}from"../../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import{observationLayerHoldsV1,observationRowsHoldV1}from"../../../../👁️observation/🟨️.mjs";
import{bindClosureStoreV1,closureRecipeKeyV1,closureTravelV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs";
import{createClosureStoreV1}from"../../../../🕸️graph/📥️inference/🧭️script-inputs/💾️entries/🟨️.mjs";

/** 🤝️ Law: owners are inferred by the machines that have capacity and read by the machine that needs them. Two machines of one fleet (two checkouts of the same tree, one cache server) pull the owners of a read through the pool, claim each owner before they infer it and publish the record; a third machine reads the same owners and finds every record in the fleet: it infers nothing itself and no owner is inferred twice. Selection owners (tier 0) are pulled before the lookahead owners (tier 1).
 * @see ../🤝️pool-reader/🟦️.ts */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`pool-serve-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(bytes:Buffer|string)=>createHash("sha256").update(bytes).digest("hex");
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const commandRecipe={fn:"native-command"};
const rowOf=(root:string,name:string)=>{const path=`${root}/${name}`,status=lstatSync(path);return{kind:"file",path,fact:{sha256:sha(readFileSync(path)),size:status.size,mtimeMs:status.mtimeMs,ctimeMs:status.ctimeMs,ino:status.ino}};};
const tree=(root:string,names:readonly string[])=>{mkdirSync(join(root,"ce"),{recursive:true});writeFileSync(join(root,"e.ts"),"export const e=1;\n");for(const name of names){mkdirSync(join(root,"libs",name),{recursive:true});writeFileSync(join(root,"libs",name,"a.ts"),`export const ${name.replace(/\W/gu,"_")}=1;\n`);}bindClosureStoreV1(root,join(root,"ce"),true);};
const walk=async(root:string)=>{const store=createClosureStoreV1({directory:join(root,"ce")}),row=rowOf(root,"e.ts");await store.save(`${root}\0${closureRecipeKeyV1(commandRecipe)}`,{recipe:commandRecipe,value:["{workspaceRoot}/e.ts"],rows:[row],layers:[[row]],sidecar:{facts:[null],members:[null]}});};
const infoOf=(root:string,name:string):ProjectRecordInfoV1=>({cachePath:join(root,"np",`${name}.json`),fleet:`libs/${name}`,keyText:JSON.stringify(["rev",null,[["authority"],`${root}/libs/${name}`,{name}]]),exportRecord:()=>({digest:sha(name),authority:[],observations:[rowOf(root,`libs/${name}/a.ts`)],image:JSON.stringify({name,closureRecipes:{[closureRecipeKeyV1(commandRecipe)]:commandRecipe}})}),async import(){return true;},closures:closureTravelV1});
const taken=async(root:string,environment:Record<string,string>,name:string):Promise<boolean>=>{
 const port=createProjectFleetPortV1({root,environment,owner:`reader:${process.pid}:${Math.random()}`})!,gate=await runTakeLookV1(()=>port.gate(infoOf(root,name)));
 if(!gate.seeded)return false;
 const entry=await createClosureStoreV1({directory:join(root,"ce")}).bundle(`${root}\0${closureRecipeKeyV1(commandRecipe)}`);
 return entry!==undefined&&observationRowsHoldV1(entry.rows)&&[...entry.layers].every(layer=>observationLayerHoldsV1(layer));
};
type Unit={name:string;tier:number};

test("two machines serve the owners of a read through the pool and a third machine reads them all from the fleet with no local inference, every owner inferred exactly once, selection owners first",async()=>{
 const cache=temp("cache"),server=await startRemoteCacheServerV1({directory:cache,host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 try{
  const units:Unit[]=[...Array.from({length:4},(_all,index)=>({name:`selection-${index}`,tier:0})),...Array.from({length:8},(_all,index)=>({name:`ahead-${index}`,tier:1}))],names=units.map(unit=>unit.name);
  const [one,two,reader]=["one","two","reader"].map(label=>{const root=temp(label);tree(root,names);return root;}) as [string,string,string];
  await walk(one);await walk(two);
  const inferred=new Map<string,string[]>([["one",[]],["two",[]]]),claimed:string[]=[],policy={...OWNER_POOL_POLICY_V1,sweepPollMilliseconds:5,heldRetryMilliseconds:200,faultRetryMilliseconds:200};
  const serve=(machine:"one"|"two",root:string)=>{
   const port=createProjectFleetPortV1({root,environment,owner:`${machine}:${process.pid}`})!;
   return pullOwnerPoolV1<Unit>({units:()=>units,puller:`${machine}:1`,policy,slots:()=>2,
    claim:async unit=>{const claim=await port.claim(infoOf(root,unit.name));if(claim.state!=="granted")return claim;claimed.push(unit.name);const granted=claim as Extract<ProjectClaimV1,{state:"granted"}>;return{...granted,publish:async()=>{const stored=await granted.publish();await port.flush();return stored;}};},
    run:async unit=>{inferred.get(machine)!.push(unit.name);await pause(60);return true;}});
  };
  const servers=Promise.all([serve("one",one),serve("two",two)]);
  let local=0;const arrived:string[]=[];
  for(const name of names){
   const until=Date.now()+60000;let found=false;
   while(!found&&Date.now()<until){found=await taken(reader,environment,name);if(!found)await pause(100);}
   if(found)arrived.push(name);else local++;
  }
  const stats=await servers;
  expect(local,"the reading machine inferred nothing itself").toBe(0);
  expect(arrived).toEqual(names);
  const all=[...inferred.get("one")!,...inferred.get("two")!];
  expect([...all].sort(),"every owner is inferred, and by exactly one machine").toEqual([...names].sort());
  expect(stats.reduce((sum,row)=>sum+row.published,0)).toBe(names.length);
  const firstAhead=claimed.findIndex(name=>name.startsWith("ahead-")),lastSelection=Math.max(...claimed.map((name,at)=>name.startsWith("selection-")?at:-1));
  expect(firstAhead===-1||lastSelection<firstAhead+4,"the selection owners are claimed before the lookahead owners (apart from the claims granted ahead)").toBe(true);
 }finally{await server.stop();}
},180000);
