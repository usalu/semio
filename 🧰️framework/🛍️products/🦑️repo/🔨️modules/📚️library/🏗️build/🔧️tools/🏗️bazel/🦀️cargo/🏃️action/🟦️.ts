import {copyFile,mkdir,readFile,realpath,writeFile,appendFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {dirname,join} from "node:path";
import { isMainModuleV1 } from "../../../../../../../../../🔨️modules/⚙️runtime/🏁️main/🟦️.ts";

/** 📦️ Selects the published files of one package from Cargo's JSON messages; the typed module renders the same selection and a shared corpus keeps both equal. */
export function selectCargoArtifactsV1(lines:readonly string[],pack:string,suffixes:readonly string[]):string[]{
 const found=new Map<string,string>();
 for(const line of lines){
  if(!line.startsWith('{"reason":"compiler-artifact"'))continue;
  const message=JSON.parse(line) as {package_id:string;filenames:string[]};
  if(!message.package_id.includes("#"+pack+"@"))continue;
  for(const file of message.filenames)if(suffixes.some(suffix=>file.endsWith(suffix)))found.set(file.slice(file.lastIndexOf("/")+1),file);
 }
 return[...found.entries()].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([,file])=>file);
}

/** 🦀️ Runs one declared Cargo action through the repository ingress and publishes only its verdict and artifacts; the workspace, environment and journal come from the runtime channel beside the manifest, outside every action key. */
async function main(manifestPath:string,out:string):Promise<void>{
 const manifest=JSON.parse(await readFile(manifestPath,"utf8")),runtime=JSON.parse(await readFile(join(dirname(await realpath(manifestPath)),"runtime.json"),"utf8")),lines:string[]=[];
 if(runtime.journal)await appendFile(runtime.journal,manifest.id+" start "+Date.now()+"\n");
 const status=await new Promise<number>(accept=>{
  const child=spawn(process.execPath,["repo",...manifest.argv],{cwd:runtime.workspace,env:runtime.environment,stdio:["ignore","pipe","pipe"]});
  for(const stream of [child.stdout,child.stderr]){let rest="";stream.on("data",chunk=>{const parts=(rest+chunk.toString()).split("\n");rest=parts.pop()!;for(const line of parts){lines.push(line);if(!line.startsWith("[script]"))process.stderr.write(line+"\n");}});}
  child.on("close",code=>accept(code??1));
 });
 if(runtime.journal)await appendFile(runtime.journal,manifest.id+" end "+Date.now()+"\n");
 if(status!==0)process.exit(status);
 const artifacts:{name:string;sha256:string;bytes:number}[]=[];
 await mkdir(join(out,"artifacts"),{recursive:true});
 if(manifest.artifacts==="files")for(const file of selectCargoArtifactsV1(lines,manifest.package,manifest.suffixes)){const bytes=await readFile(file),name=file.slice(file.lastIndexOf("/")+1);await copyFile(file,join(out,"artifacts",name));artifacts.push({name,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.length});}
 await writeFile(join(out,"receipt.json"),JSON.stringify({version:1,id:manifest.id,status:0,artifacts}));
}

if(isMainModuleV1(import.meta.url))await main(process.argv[2]!,process.argv[3]!);
