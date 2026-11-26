import{readdir,rm,stat}from"node:fs/promises";
import{join}from"node:path";
import{processPidExistsV1}from"../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The authored numbers of the origin file retention: the age of a temporary file and of a published-name file at which it is retired, and how often a pool looks.
 * @see ./🧬️schema/🔣️.json */
export const ORIGIN_FILES_POLICY_V1=schema.definitions.Policy.const;
const P=ORIGIN_FILES_POLICY_V1,TEMPORARY=new RegExp(P.temporaryPattern,"u"),PUBLISHED=new RegExp(P.publishedPattern,"u"),APPLE_DOUBLE=new RegExp(P.appleDoublePattern,"u"),LANES=16;

/** 🧹️ What one retirement removed, by kind. */
export type OriginFilesRetiredV1=Readonly<{temporaries:readonly string[];published:readonly string[];appleDouble:readonly string[]}>;

/** 🧹️ Retires the files of one machine's directory of the shared store that nobody reads or finishes any more: a temporary file `<name>.<pid>.tmp` of a process that is gone or older than `temporaryMaximumAgeMilliseconds` (never the caller's own), a `pool-published-<wave>[-<key>].txt` older than `publishedRetentionMilliseconds`, every AppleDouble stub `._<name>` of the origin directory and of its `metrics` directory at once (nobody reads them; a macOS copy with extended attributes onto the network store leaves them). Only the directory of the caller's own origin may be given (the pids are those of this machine). Never throws; a file that cannot be removed stays for the next round.
 * @see ./🧪️tests/🟦️.ts */
export async function retireOriginFilesV1(input:Readonly<{directory:string;now?:number;alive?:(pid:number)=>boolean;own?:number}>):Promise<OriginFilesRetiredV1>{
 const now=input.now??Date.now(),alive=input.alive??processPidExistsV1,own=input.own??process.pid,temporaries:string[]=[],published:string[]=[],appleDouble:string[]=[];
 let names:string[]=[];
 try{names=await readdir(input.directory);}catch{return Object.freeze({temporaries,published,appleDouble});}
 const work=names.filter(name=>PUBLISHED.test(name)||TEMPORARY.test(name));
 let next=0;
 const lane=async():Promise<void>=>{
  while(next<work.length){
   const name=work[next++]!,file=join(input.directory,name),found=TEMPORARY.exec(name);
   try{
    if(found!==null){
     const pid=Number(found[2]);if(pid===own)continue;
     if(alive(pid)&&now-(await stat(file)).mtimeMs<=P.temporaryMaximumAgeMilliseconds)continue;
     await rm(file,{force:true});temporaries.push(name);
    }else if(now-(await stat(file)).mtimeMs>P.publishedRetentionMilliseconds){await rm(file,{force:true});published.push(name);}
   }catch{}
  }
 };
 await Promise.all(Array.from({length:Math.min(LANES,work.length)},lane));
 for(const sub of P.appleDoubleDirectories){
  let listed:string[]=[];
  try{listed=await readdir(join(input.directory,sub));}catch{continue;}
  for(const name of listed)if(APPLE_DOUBLE.test(name))try{await rm(join(input.directory,sub,name),{force:true});appleDouble.push(sub===""?name:`${sub}/${name}`);}catch{}
 }
 return Object.freeze({temporaries:temporaries.sort(),published:published.sort(),appleDouble:appleDouble.sort()});
}
