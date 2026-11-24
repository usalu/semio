import{createHash}from"node:crypto";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{profilePhaseV1}from"../../../⏱️profile/🟨️.mjs";
import{canonicalJsonV1}from"../🧮️fact-codec/🟨️.mjs";
import{P,aborted,fail,isObject,requireCount,requireHex,requireKeys,requireText,sha256}from"../🧱️guards/🟨️.mjs";
import{createRowLogV1}from"../🗃️row-log/🟨️.mjs";
import{readGenerationManifestV1}from"../📖️documents/🟨️.mjs";
import{derivedOwnerNameV1,isDerivedOwnerV1,isDerivedRowV1}from"../✂️cutoff/🟨️.mjs";

/** 📏️ The declared limits of the build journal: the directory under the store root, the version, the rows one line of a rows file carries at most and the bytes after which a wave starts its next rows file (a wave never holds more than one part of its rows). */
export const BUILD_JOURNAL_V1=Object.freeze({directory:"j",version:1,rowsPerLine:4096,partBytes:8388608,maximumOwners:200000});

/** 🔑️ The name of the journal of one build: the digest of what the build starts from (the generation it refines, the identity, the implementation, the membership and the owners it has to derive). */
export const buildJournalKeyV1=input=>createHash("sha256").update(canonicalJsonV1(input)).digest("hex");

/**
 * 🔖️ Admits the header of one build journal. `waves` holds the work of the finished waves of a delta build (the owner table after them, what they dropped and derived, the rows files that carry their facts) and `sealed` a build whose manifest is written and whose index is kept but not yet published. Every step that finished leaves a marker `{step, inputs, output}` with the digest of what it read and the digest of what it made.
 * [layout](./🧬️schema/🔣️.json)
 */
export function readBuildJournalV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","key","state","parent","generation","identity","implementation","steps"],["repair","names","done","owners","dropped","returned","derived","rows","manifest","env","membership"],"build journal");
 if(value.version!==BUILD_JOURNAL_V1.version)throw fail("Build journal version 1 required");
 requireHex(value.key,"key");if(value.parent!==null)requireHex(value.parent,"parent");requireCount(value.generation,"generation");
 if(value.membership!==undefined)requireHex(value.membership,"membership");
 if(value.repair!==undefined){requireKeys(value.repair,["parent","receipt"],[],"journal repair");requireHex(value.repair.parent,"repair parent");requireHex(value.repair.receipt,"repair receipt");}
 for(const key of["identity","implementation"])requireHex(value[key],key);
 if(!["waves","sealed"].includes(value.state))throw fail("Closed build journal state required");
 if(!Array.isArray(value.steps)||value.steps.length>16)throw fail("Bounded step marker list required");
 for(const step of value.steps){requireKeys(step,["step","inputs","output"],[],"step marker");requireText(step.step,"step name",32);requireHex(step.inputs,"step inputs");requireHex(step.output,"step output");}
 const names=list=>{if(!Array.isArray(list)||list.length>BUILD_JOURNAL_V1.maximumOwners||list.some(name=>typeof name!=="string"))throw fail("Owner name list required");};
 if(value.state==="sealed"){requireHex(value.manifest,"manifest");if(value.derived!==undefined)names(value.derived);return Object.freeze(value);}
 for(const key of["names","dropped","returned","derived"])names(value[key]);
 requireCount(value.done,"done owners");
 requireKeys(value.env,["digest"],["parts"],"journal environment");requireHex(value.env.digest,"environment digest");
 if(!isObject(value.owners)||Object.keys(value.owners).length>BUILD_JOURNAL_V1.maximumOwners)throw fail("Bounded owner table required");
 if(!Array.isArray(value.rows))throw fail("Rows file list required");
 for(const file of value.rows){requireKeys(file,["file","sha256","bytes"],[],"rows file");requireText(file.file,"rows file name",160);requireHex(file.sha256,"rows file digest");requireCount(file.bytes,"rows file bytes");}
 return Object.freeze(value);
}

/** 🧾️ Writes the rows of the owners of one wave as lines `{o: owner, m: "u" | "a", r: rows}` (`u` replaces the rows of the owner, `a` adds to them), at most `rowsPerLine` rows to a line. */
export function encodeJournalRowsV1(entries){
 const lines=[];
 for(const entry of entries){
  if(entry.rows.length===0){lines.push(JSON.stringify({o:entry.name,m:entry.mode,r:[]}));continue;}
  for(let at=0;at<entry.rows.length;at+=BUILD_JOURNAL_V1.rowsPerLine)lines.push(JSON.stringify({o:entry.name,m:entry.mode,r:entry.rows.slice(at,at+BUILD_JOURNAL_V1.rowsPerLine)}));
 }
 return Buffer.from(lines.join("\n")+"\n","utf8");
}

/** 🧾️ Reads the lines of a rows file back into entries `{name, mode, rows}`; the lines of one owner follow each other. */
export function decodeJournalRowsV1(bytes){
 const entries=[];
 for(const line of Buffer.from(bytes).toString("utf8").split("\n")){
  if(!line)continue;
  const value=JSON.parse(line);
  if(!isObject(value)||typeof value.o!=="string"||!["u","a"].includes(value.m)||!Array.isArray(value.r))throw fail("Closed rows line required");
  const last=entries.at(-1);
  if(last&&last.name===value.o&&last.mode===value.m)last.rows.push(...value.r);else entries.push({name:value.o,mode:value.m,rows:value.r});
 }
 return entries;
}

/**
 * 🔖️ The journal of the build that is under way, on the file port of the store: `wave` keeps what a finished wave of a delta build derived (its rows go to a rows file, the owner table and the names to the header), `sealed` keeps a build whose manifest is written and whose index is kept, `resume` takes up a build that was cut off and `clear` removes what a build left. A sealed build is published and then verified against the tree (it answers `published: true` with `resumed: true` and the caller plans again); one that finished some waves continues with the owners they did not derive. Anything that does not fit the build any more (another generation, other owners, a missing or altered file) is dropped.
 * `manifestPath`, `publish` and `settler` are the store's own; `say` and `stepped` are its optional hooks.
 */
export function createBuildJournalV1({fs,root,manifestPath,publish,settler,say,stepped}){
 const directory=join(root,BUILD_JOURNAL_V1.directory),path=key=>join(directory,key+".json");
 const load=async key=>{const bytes=await fs.read(path(key));if(!bytes)return undefined;try{const header=readBuildJournalV1(bytes);return header.key===key?header:undefined;}catch{await fs.remove(path(key));return undefined;}};
 const clear=async key=>{for(const name of await fs.names(directory))if(name.startsWith(key))await fs.remove(join(directory,name));};
 const batchesOf=async function*(list){if(Array.isArray(list)){for(let at=0;at<list.length;at+=BUILD_JOURNAL_V1.rowsPerLine)yield list.slice(at,at+BUILD_JOURNAL_V1.rowsPerLine);return;}yield*list.batches();};
 const header=({key,current,repair,identity,implementation,steps})=>({version:BUILD_JOURNAL_V1.version,key,parent:current?.pointer.manifest??null,generation:(current?.pointer.generation??-1)+1,...repair?{repair}:{},identity,implementation,steps});
 const save=header=>fs.write(path(header.key),Buffer.from(canonicalJsonV1(header)));
 const filesHold=async manifest=>{
  const[fragments,indexes,frontiers]=(await Promise.all([fs.names(join(root,P.fragments)),fs.names(join(root,P.indexes)),fs.names(join(root,P.frontiers))])).map(list=>new Set(list));
  return fragments.has(manifest.global+".json.br")&&indexes.has(manifest.factIndex+".facts.br")&&(manifest.frontier===null||frontiers.has(manifest.frontier+".json.br"))&&Object.values(manifest.owners).every(owner=>fragments.has(owner.fragment+".json.br"));
 };

 const reconcileWaves=async({saved,current,found,ordered,hold,signal,membership})=>{
  const parent=current.manifest;
  if(saved.env.digest!==parent.env||found.envStale||found.added.length||found.removed.length||saved.dropped.length)return undefined;
  const raw=rows=>rows.filter(row=>!isDerivedRowV1(row)),holds=async rows=>{const list=raw(rows);return list.length===0||await hold(list,signal);};
  const returned=new Set(saved.returned),verdict=new Map(),refs=new Map(),pseudo=new Map();
  for(const file of saved.rows){
   const bytes=await fs.read(join(directory,file.file));
   if(!bytes||sha256(bytes)!==file.sha256)return undefined;
   for(const entry of decodeJournalRowsV1(bytes)){
    aborted(signal);
    if(entry.mode==="a")return undefined;
    if(isDerivedOwnerV1(entry.name)){pseudo.set(entry.name,(pseudo.get(entry.name)??true)&&await holds(entry.rows));continue;}
    if(!returned.has(entry.name))continue;
    verdict.set(entry.name,(verdict.get(entry.name)??true)&&await holds(entry.rows));
    const named=refs.get(entry.name)??new Set();for(const row of entry.rows)if(isDerivedRowV1(row))named.add(derivedOwnerNameV1(row));refs.set(entry.name,named);
   }
  }
  const reusable=saved.returned.filter(name=>verdict.get(name)===true&&[...refs.get(name)??[]].every(derived=>pseudo.get(derived)===true));
  if(reusable.length===0)return undefined;
  const reused=new Set(reusable),needed=new Set(reusable.flatMap(name=>[...refs.get(name)??[]])),keepEntry=entry=>isDerivedOwnerV1(entry.name)?needed.has(entry.name):reused.has(entry.name);
  const log=createRowLogV1({path:join(tmpdir(),"semio-row-logs","resume-"+saved.key.slice(0,16)+"-"+process.pid+".rows")}),upserts=new Map(),rewritten=[];
  for(let at=0;at<saved.rows.length;at++){
   const kept=decodeJournalRowsV1(await fs.read(join(directory,saved.rows[at].file))).filter(keepEntry),bytes=encodeJournalRowsV1(kept.map(entry=>({name:entry.name,mode:entry.mode,rows:entry.rows})));
   await fs.write(join(directory,saved.rows[at].file),bytes);rewritten.push({file:saved.rows[at].file,sha256:sha256(bytes),bytes:bytes.length});
   for(const entry of kept){const known=upserts.get(entry.name);upserts.set(entry.name,known?known.concat(entry.rows):log.append(entry.rows));}
  }
  const owners={...saved.owners};
  for(const name of saved.returned)if(!reused.has(name)){if(parent.owners[name])owners[name]=parent.owners[name];else delete owners[name];}
  const remaining=ordered.filter(name=>!reused.has(name)),names=[...reusable,...remaining];
  return{names:remaining,carry:{owners,env:saved.env.digest,envParts:saved.env.parts,upserts,append:new Map(),dropped:new Set(),returned:new Set(reusable),derived:new Set(saved.derived),names,done:reusable.length,waves:saved.rows.length,files:rewritten,disposers:[()=>log.close()],resumedFrom:true,membership}};
 };
 return Object.freeze({
  clear,
  sealed:input=>save({...header(input),state:"sealed",manifest:input.manifest,derived:input.derived}),
  async wave({key,current,repair,identity,implementation,carry,steps,upserts,append,owners,env}){
   const first=carry.files.length;let pending=[],pendingBytes=0;
   const flush=async()=>{const bytes=Buffer.from(pending.length?pending.join(""):"\n","utf8"),file=key+"."+carry.files.length+".rows";pending=[];pendingBytes=0;await fs.write(join(directory,file),bytes);carry.files.push({file,sha256:sha256(bytes),bytes:bytes.length});};
   const put=async line=>{pending.push(line);pendingBytes+=line.length;if(pendingBytes>=BUILD_JOURNAL_V1.partBytes)await flush();};
   const putOwner=async(name,mode,list)=>{let any=false;for await(const batch of batchesOf(list)){if(!batch.length)continue;any=true;await put(JSON.stringify({o:name,m:mode,r:batch})+"\n");}if(!any)await put(JSON.stringify({o:name,m:mode,r:[]})+"\n");};
   for(const[name,list]of upserts)if(name!==P.envOwner)await putOwner(name,"u",list);
   for(const[name,list]of append)await putOwner(name,"a",list);
   if(pending.length||carry.files.length===first)await flush();
   await save({...header({key,current,repair,identity,implementation,steps}),state:"waves",names:carry.names,done:carry.done,owners,dropped:[...carry.dropped],returned:[...carry.returned],derived:[...carry.derived],rows:carry.files,env,...carry.membership===undefined?{}:{membership:carry.membership}});
  },
  async resume({key,current,repair,identity,implementation,found,signal,claim,qualify,verify,ordered=[],membership,hold}){
   const saved=await load(key),generation=(current?.pointer.generation??-1)+1;
   if(!saved)return undefined;
   if(saved.parent!==(current?.pointer.manifest??null)||saved.generation!==generation||saved.identity!==identity||(repair||saved.state==="sealed")&&saved.implementation!==implementation||saved.repair?.receipt!==repair?.receipt||saved.repair?.parent!==repair?.parent){await clear(key);return undefined;}
   if(saved.state==="sealed"){
    const bytes=await fs.read(manifestPath(saved.manifest));
    let manifest;try{manifest=bytes&&sha256(bytes)===saved.manifest?readGenerationManifestV1(bytes):undefined;}catch{manifest=undefined;}
    if(!manifest||manifest.generation!==generation||manifest.parent!==saved.parent||manifest.identity!==identity||manifest.implementation!==implementation||!await filesHold(manifest)){await clear(key);return undefined;}
    if(verify&&!await verify(manifest)){await clear(key);return undefined;}
    try{await qualify?.(manifest,signal);}catch(error){aborted(signal);await clear(key);throw error;}
    const outcome=await profilePhaseV1("graph.build.publish",()=>publish({manifestDigest:saved.manifest,expectedGeneration:generation-1,...repair?{expectedManifest:null,expectedRepairParent:repair.parent}:{},signal,claim}));
    if(!outcome.published){await clear(key);return repair?{superseded:true}:undefined;}
    await profilePhaseV1("graph.build.indexed",()=>settler.markIndexed({held:found.refresh,dropped:[...(saved.derived??[])]}).catch(()=>undefined));
    await clear(key);say?.("resumed",{generation,owners:Object.keys(manifest.owners).length});await stepped?.("sealing");
    return{result:{current:{pointer:outcome.pointer,manifest},built:true,published:true,plan:found,computed:0,resumed:true}};
   }
   if(repair){await clear(key);return undefined;}
   const present=new Set(await fs.names(join(root,P.fragments)));
   if(Object.values(saved.owners).some(owner=>!present.has(owner.fragment+".json.br"))){await clear(key);return undefined;}
   const log=createRowLogV1({path:join(tmpdir(),"semio-row-logs","resume-"+key.slice(0,16)+"-"+process.pid+".rows")}),upserts=new Map(),append=new Map();
   for(const file of saved.rows){
    const bytes=await fs.read(join(directory,file.file));
    if(!bytes||sha256(bytes)!==file.sha256){log.close();await clear(key);return undefined;}
    for(const entry of decodeJournalRowsV1(bytes)){const target=entry.mode==="u"?upserts:append,known=target.get(entry.name);target.set(entry.name,known?known.concat(entry.rows):log.append(entry.rows));}
   }
   if(!(saved.membership===membership&&membership!==undefined&&canonicalJsonV1(saved.names)===canonicalJsonV1(ordered))){
    const kept=hold&&current?.manifest?await reconcileWaves({saved,current,found,ordered,hold,signal,membership}):undefined;
    if(!kept){await clear(key);return undefined;}
    say?.("resumedWaves",{done:kept.carry.done,total:kept.carry.names.length});
    return kept;
   }
   say?.("resumedWaves",{done:saved.done,total:saved.names.length});
   return{names:saved.names.slice(saved.done),carry:{owners:saved.owners,env:saved.env.digest,envParts:saved.env.parts,upserts,append,dropped:new Set(saved.dropped),returned:new Set(saved.returned),derived:new Set(saved.derived),names:saved.names,done:saved.done,waves:saved.rows.length,files:[...saved.rows],disposers:[()=>log.close()],resumedFrom:true}};
  },
 });
}
