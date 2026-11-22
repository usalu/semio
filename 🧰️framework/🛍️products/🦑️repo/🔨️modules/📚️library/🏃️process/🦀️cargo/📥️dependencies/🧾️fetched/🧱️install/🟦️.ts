import {randomBytes} from "node:crypto";
import {lstat,mkdir,readFile,rename,rm,writeFile} from "node:fs/promises";
import {dirname,join} from "node:path";

const marker=".cargo-ok",lost=new Set(["EEXIST","ENOTEMPTY","EPERM","EACCES"]);
const code=(error:unknown)=>(error as NodeJS.ErrnoException).code??"";
const same=async(path:string,body:Uint8Array)=>{try{return Buffer.from(body).equals(await readFile(path));}catch{return false;}};

/** 🧱️ Installs one registry package's files without ever making an extracted file, or the package directory, disappear for a concurrent Cargo reader or extractor: an absent directory is fully staged beside it and renamed into place once, and a present directory is completed file by file through same-directory renames with the `.cargo-ok` completion marker last, so the directory is never moved aside, truncated or removed. See [Cargo registry source layout](https://doc.rust-lang.org/cargo/guide/cargo-home.html). */
export async function installUnpackedV1(path:string,files:ReadonlyMap<string,Uint8Array>):Promise<void>{
 const suffix=randomBytes(6).toString("hex"),ordered=[...files].sort(([left],[right])=>Number(left===marker)-Number(right===marker)||(left<right?-1:left>right?1:0));
 if(!(await lstat(path).catch(()=>undefined))){
  const staged=path+".repair-"+suffix;
  try{for(const[name,body]of ordered){const target=join(staged,name);await mkdir(dirname(target),{recursive:true});await writeFile(target,body);}await rename(staged,path);return;}
  catch(error){await rm(staged,{recursive:true,force:true});if(!lost.has(code(error)))throw error;}
 }
 for(const[name,body]of ordered){
  const target=join(path,name);if(await same(target,body))continue;
  const temporary=target+".repair-"+suffix;
  try{await mkdir(dirname(target),{recursive:true});await writeFile(temporary,body);await rename(temporary,target);}
  catch(error){if(!await same(target,body))throw error;}
  finally{await rm(temporary,{force:true});}
 }
}
