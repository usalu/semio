import{createHash}from"node:crypto";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{profilePhaseV1}from"../../../⏱️profile/🟨️.mjs";
import{canonicalJsonV1}from"../🧮️fact-codec/🟨️.mjs";
import{P,fail,isObject,requireCount,requireHex,requireKeys,requireText,sha256}from"../🧱️guards/🟨️.mjs";
import{createRowLogV1}from"../🗃️row-log/🟨️.mjs";
import{readGenerationManifestV1}from"../📖️documents/🟨️.mjs";

/** 📏️ The declared limits of the build journal: the directory under the store root, the version and the rows one line of a rows file carries at most. */
export const BUILD_JOURNAL_V1=Object.freeze({directory:"j",version:1,rowsPerLine:4096,maximumOwners:200000});

/** 🔑️ The name of the journal of one build: the digest of what the build starts from (the generation it refines, the identity, the implementation, the membership and the owners it has to derive). */
export const buildJournalKeyV1=input=>createHash("sha256").update(canonicalJsonV1(input)).digest("hex");

/**
 * 🔖️ Admits the header of one build journal. `waves` holds the work of the finished waves of a delta build (the owner table after them, what they dropped and derived, the rows files that carry their facts) and `sealed` a build whose manifest is written and whose index is kept but not yet published. Every step that finished leaves a marker `{step, inputs, output}` with the digest of what it read and the digest of what it made.
 * [layout](./🧬️schema/🔣️.json)
 */
export function readBuildJournalV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","key","state","parent","generation","identity","implementation","steps"],["names","done","owners","dropped","returned","derived","rows","manifest","env"],"build journal");
 if(value.version!==BUILD_JOURNAL_V1.version)throw fail("Build journal version 1 required");
 requireHex(value.key,"key");if(value.parent!==null)requireHex(value.parent,"parent");requireCount(value.generation,"generation");
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
 const rowsOf=async list=>{if(Array.isArray(list))return list;const rows=[];for await(const batch of list.batches())rows.push(...batch);return rows;};
 const header=({key,current,identity,implementation,steps})=>({version:BUILD_JOURNAL_V1.version,key,parent:current?.pointer.manifest??null,generation:(current?.pointer.generation??-1)+1,identity,implementation,steps});
 const save=header=>fs.write(path(header.key),Buffer.from(canonicalJsonV1(header)));
 const filesHold=async manifest=>{
  const[fragments,indexes,frontiers]=(await Promise.all([fs.names(join(root,P.fragments)),fs.names(join(root,P.indexes)),fs.names(join(root,P.frontiers))])).map(list=>new Set(list));
  return fragments.has(manifest.global+".json.br")&&indexes.has(manifest.factIndex+".facts.br")&&(manifest.frontier===null||frontiers.has(manifest.frontier+".json.br"))&&Object.values(manifest.owners).every(owner=>fragments.has(owner.fragment+".json.br"));
 };
 return Object.freeze({
  clear,
  sealed:input=>save({...header(input),state:"sealed",manifest:input.manifest,derived:input.derived}),
  async wave({key,current,identity,implementation,carry,steps,upserts,append,owners,env}){
   const entries=[];
   for(const[name,list]of upserts)if(name!==P.envOwner)entries.push({name,mode:"u",rows:await rowsOf(list)});
   for(const[name,list]of append)entries.push({name,mode:"a",rows:await rowsOf(list)});
   const bytes=encodeJournalRowsV1(entries),file=key+"."+carry.waves+".rows";
   await fs.write(join(directory,file),bytes);carry.files.push({file,sha256:sha256(bytes),bytes:bytes.length});
   await save({...header({key,current,identity,implementation,steps}),state:"waves",names:carry.names,done:carry.done,owners,dropped:[...carry.dropped],returned:[...carry.returned],derived:[...carry.derived],rows:carry.files,env});
  },
  async resume({key,current,identity,implementation,found,signal,claim}){
   const saved=await load(key),generation=(current?.pointer.generation??-1)+1;
   if(!saved)return undefined;
   if(saved.parent!==(current?.pointer.manifest??null)||saved.generation!==generation||saved.identity!==identity||saved.state==="sealed"&&saved.implementation!==implementation){await clear(key);return undefined;}
   if(saved.state==="sealed"){
    const bytes=await fs.read(manifestPath(saved.manifest));
    let manifest;try{manifest=bytes&&sha256(bytes)===saved.manifest?readGenerationManifestV1(bytes):undefined;}catch{manifest=undefined;}
    if(!manifest||!await filesHold(manifest)){await clear(key);return undefined;}
    const outcome=await profilePhaseV1("graph.build.publish",()=>publish({manifestDigest:saved.manifest,expectedGeneration:generation-1,signal,claim}));
    if(!outcome.published){await clear(key);return undefined;}
    await profilePhaseV1("graph.build.indexed",()=>settler.markIndexed({held:found.refresh,dropped:[...(saved.derived??[])]}).catch(()=>undefined));
    await clear(key);say?.("resumed",{generation,owners:Object.keys(manifest.owners).length});await stepped?.("sealing");
    return{result:{current:{pointer:outcome.pointer,manifest},built:true,published:true,plan:found,computed:0,resumed:true}};
   }
   const present=new Set(await fs.names(join(root,P.fragments)));
   if(Object.values(saved.owners).some(owner=>!present.has(owner.fragment+".json.br"))){await clear(key);return undefined;}
   const log=createRowLogV1({path:join(tmpdir(),"semio-row-logs","resume-"+key.slice(0,16)+"-"+process.pid+".rows")}),upserts=new Map(),append=new Map();
   for(const file of saved.rows){
    const bytes=await fs.read(join(directory,file.file));
    if(!bytes||sha256(bytes)!==file.sha256){log.close();await clear(key);return undefined;}
    for(const entry of decodeJournalRowsV1(bytes)){const target=entry.mode==="u"?upserts:append,known=target.get(entry.name);target.set(entry.name,known?known.concat(entry.rows):log.append(entry.rows));}
   }
   say?.("resumedWaves",{done:saved.done,total:saved.names.length});
   return{names:saved.names.slice(saved.done),carry:{owners:saved.owners,env:saved.env.digest,envParts:saved.env.parts,upserts,append,dropped:new Set(saved.dropped),returned:new Set(saved.returned),derived:new Set(saved.derived),names:saved.names,done:saved.done,waves:saved.rows.length,files:[...saved.rows],disposers:[()=>log.close()],resumedFrom:true}};
  },
 });
}
