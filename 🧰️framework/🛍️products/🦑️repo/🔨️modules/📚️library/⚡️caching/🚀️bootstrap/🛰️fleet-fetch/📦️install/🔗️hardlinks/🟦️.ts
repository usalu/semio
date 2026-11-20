import {createHash} from "node:crypto";
import {constants,type BigIntStats} from "node:fs";
import {link,lstat,mkdir,open} from "node:fs/promises";
import {join} from "node:path";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import installSchema from "../../../📦️dependencies/💾️storage/🧬️schema/📥️install.json" with {type:"json"};

/** 🔗️ A second directory entry for an authenticated regular file in this install scope. */
export type FleetInstallHardlinkV1=Readonly<{path:string;target:string;bytes:number;sha256:string}>;

const bound=schema.definitions.InstallHardlinks.maxItems;
const maximumBytes=BigInt(schema.definitions.InstallHardlinks.items.properties.bytes.maximum);
const maximumPathUnits=Math.min(installSchema.$defs.InstalledPathProtocol.const.maximumPathUnits,schema.definitions.InstallRelativePath.maxLength);
const relativeName=new RegExp(schema.definitions.InstallRelativePath.pattern,"u");
export const fleetInstallRelativeNameV1=(name:string):boolean=>relativeName.test(name)&&name.length<=schema.definitions.InstallRelativePath.maxLength&&
 name.split("/").every(part=>!/[. ]$/u.test(part)&&!/^(?:con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(?:\.|$)/iu.test(part));

/** 📐️ Applies the installed-path budget to this physical scope and a relative archive path or path width. */
export function fleetInstallPathFitsV1(root:string,path:string|number):boolean{
 const prefix=join(root,"_").length-1;
 return typeof path==="string"?fleetInstallRelativeNameV1(path)&&prefix+path.length<=maximumPathUnits:prefix+path<=maximumPathUnits;
}

const identity=(facts:BigIntStats):string=>`${facts.dev}:${facts.ino}`;
const contentStamp=(facts:BigIntStats):string=>`${identity(facts)}:${facts.mode}:${facts.size}:${facts.mtimeNs}`;
const exact=(facts:BigIntStats):string=>`${contentStamp(facts)}:${facts.ctimeNs}:${facts.nlink}`;

async function physicalPath(root:string,name:string,check:()=>void,createParents=false){
 if(!fleetInstallPathFitsV1(root,name))throw Error("Expected a bounded relative install path");
 check();
 const base=await lstat(root,{bigint:true});
 if(!base.isDirectory())throw Error("Expected a physical install scope directory");
 const components=name.split("/"),leaf=components.pop()!;
 let path=root;
 const parents=[{path,identity:identity(base)}];
 for(const component of components){
  check();path=join(path,component);
  if(createParents)await mkdir(path).catch(error=>{if(error.code!=="EEXIST")throw error;});
  const facts=await lstat(path,{bigint:true});
  if(!facts.isDirectory()||facts.dev!==base.dev)throw Error("Expected a same-device physical install path");
  parents.push({path,identity:identity(facts)});
 }
 const verify=async()=>{
  for(const parent of parents){
   check();const facts=await lstat(parent.path,{bigint:true});
   if(!facts.isDirectory()||identity(facts)!==parent.identity)throw Error("Install hard link parent changed");
  }
 };
 return {path:join(path,leaf),device:base.dev,verify};
}

async function content(path:Awaited<ReturnType<typeof physicalPath>>,check:()=>void,expected?:{stamp?:string;bytes?:number}){
 await path.verify();check();
 const before=await lstat(path.path,{bigint:true});
 if(!before.isFile()||before.dev!==path.device||before.size>maximumBytes||expected?.stamp!==undefined&&exact(before)!==expected.stamp||
  expected?.bytes!==undefined&&before.size!==BigInt(expected.bytes))throw Error("Install hard link requires its exact regular target");
 const handle=await open(path.path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));
 try{
  if(exact(await handle.stat({bigint:true}))!==exact(before))throw Error("Install hard link target changed during open");
  const hash=createHash("sha256"),buffer=Buffer.allocUnsafe(1<<20);
  let bytes=0;
  for(;;){
   check();const {bytesRead}=await handle.read(buffer,0,buffer.length,null);
   if(bytesRead===0)break;
   bytes+=bytesRead;
   if(BigInt(bytes)>before.size)throw Error("Install hard link target grew during read");
   hash.update(buffer.subarray(0,bytesRead));
  }
  await path.verify();check();
  if(exact(await handle.stat({bigint:true}))!==exact(before)||exact(await lstat(path.path,{bigint:true}))!==exact(before)||BigInt(bytes)!==before.size)throw Error("Install hard link target changed during read");
  return {bytes,sha256:hash.digest("hex"),facts:before};
 }finally{await handle.close();}
}

/** 🧾️ Captures file identity while discovering units and proves each linked entry again before signing. */
export function captureFleetInstallHardlinksV1(root:string,check:()=>void){
 const originals=new Map<string,{path:string;stamp:string}>(),aliases:{path:string;target:string;stamp:string}[]=[];
 return {
  omit(path:string,facts:BigIntStats):boolean{
   if(!facts.isFile()||facts.nlink<2n)return false;
   const key=identity(facts),first=originals.get(key),stamp=exact(facts);
   if(!first){
    if(originals.size>=bound)throw Error("Install hard link observation exceeds its bound");
    originals.set(key,{path,stamp});return false;
   }
   if(first.stamp!==stamp||aliases.length>=bound)throw Error("Install hard link observation changed or exceeded its bound");
   aliases.push({path,target:first.path,stamp});return true;
  },
  async prove(){
   const targets=new Map<string,{bytes:number;sha256:string}>();
   const verify=async(item:{path:string;stamp:string})=>{
    check();const path=await physicalPath(root,item.path,check),facts=await lstat(path.path,{bigint:true});
    if(!facts.isFile()||facts.dev!==path.device||exact(facts)!==item.stamp)throw Error("Published install hard link identity changed");
    await path.verify();
   };
   const rows:FleetInstallHardlinkV1[]=[];
   for(const alias of aliases){
    await verify(alias);
    let target=targets.get(alias.target);
    if(!target){
     const path=await physicalPath(root,alias.target,check);
     const {bytes,sha256}=await content(path,check,{stamp:alias.stamp});
     target={bytes,sha256};targets.set(alias.target,target);
    }
    rows.push({path:alias.path,target:alias.target,...target});
   }
   const recheck=async()=>{
    const verified=new Set<string>();
    for(const alias of aliases){
     await verify(alias);
     if(!verified.has(alias.target)){await verify({path:alias.target,stamp:alias.stamp});verified.add(alias.target);}
    }
   };
   return {rows,targets,recheck};
  },
 };
}

/** 🔒️ Checks the signed hard-link data before extraction and binds each destination to a package file within these roots. */
export function planFleetInstallHardlinkRestoreV1(directory:string,roots:readonly string[],index:Readonly<{
 directories:readonly string[];links:readonly {path:string}[];hardlinks:readonly FleetInstallHardlinkV1[];units:readonly {path:string;maximumPathUnits:number}[];
}>){
 const normalize=process.platform==="win32"?(name:string)=>name.toLowerCase():(name:string)=>name;
 const below=(name:string,base:string)=>name.startsWith(base+"/");
 const contained=(name:string,allowRoot:boolean)=>fleetInstallPathFitsV1(directory,name)&&roots.some(root=>below(name,root)||allowRoot&&name===root);
 const opaque=new Set([...index.directories,...index.units.map(row=>row.path),...index.links.map(row=>row.path)].map(normalize));
 const aliases=new Set(index.hardlinks.map(row=>normalize(row.path))),bodies=new Set(index.units.map(row=>row.path));
 const symlinks=new Set(index.links.map(row=>normalize(row.path)));
 const memberOrAncestor=(name:string,entries:ReadonlySet<string>)=>{
  const pieces=name.split("/");
  while(pieces.length){if(entries.has(pieces.join("/")))return true;pieces.pop();}
  return false;
 };
 if(aliases.size!==index.hardlinks.length||index.directories.some(path=>!contained(path,true))||
  index.links.some(row=>!contained(row.path,true))||index.units.some(row=>!contained(row.path,false)||row.maximumPathUnits<row.path.length||
   !fleetInstallPathFitsV1(directory,row.maximumPathUnits)))throw Error("Install index lies outside its requested roots or path budget");
 const grouped=new Map<string,{proof:FleetInstallHardlinkV1;paths:string[]}>();
 for(const row of index.hardlinks){
  const path=normalize(row.path),target=normalize(row.target);
  if(!contained(row.path,false)||!contained(row.target,false)||memberOrAncestor(target,aliases)||opaque.has(path)||
   memberOrAncestor(path.split("/").slice(0,-1).join("/"),aliases)||
   memberOrAncestor(path,symlinks)||memberOrAncestor(target,symlinks))throw Error("Install hard link lies outside its owned files");
  if(!memberOrAncestor(row.target,bodies))throw Error("Install hard link target has no authenticated archive unit");
  const group=grouped.get(target);
  if(group){
   if(group.proof.target!==row.target||group.proof.bytes!==row.bytes||group.proof.sha256!==row.sha256)throw Error("Conflicting install hard link proofs");
   group.paths.push(row.path);
  }else grouped.set(target,{proof:row,paths:[row.path]});
 }
 return async(check:()=>void)=>{
  for(const {proof,paths} of grouped.values()){
   const source=await physicalPath(directory,proof.target,check),observed=await content(source,check,{bytes:proof.bytes});
   if(observed.bytes!==proof.bytes||observed.sha256!==proof.sha256||observed.facts.nlink!==1n)throw Error("Install hard link target differs from its authenticated file");
   const stamp=contentStamp(observed.facts);
   let nlink=1n;
   for(const path of paths){
    const destination=await physicalPath(directory,path,check,true);
    await destination.verify();await source.verify();check();
    const before=await lstat(source.path,{bigint:true});
    if(!before.isFile()||contentStamp(before)!==stamp||before.nlink!==nlink)throw Error("Install hard link target changed before linking");
    await link(source.path,destination.path);nlink++;
    await destination.verify();await source.verify();check();
    const after=await lstat(destination.path,{bigint:true});
    if(!after.isFile()||contentStamp(after)!==stamp||after.nlink!==nlink)throw Error("Install hard link did not preserve the authenticated file");
   }
  }
 };
}
