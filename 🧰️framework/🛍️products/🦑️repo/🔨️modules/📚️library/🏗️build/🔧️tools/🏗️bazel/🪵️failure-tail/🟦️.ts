import {mkdir,readFile,readdir,rename,rm,writeFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {retryOnConcurrentEditV1,type ChurnOptionsV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {failureChurnOfTailsV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧾️refusals/🟦️.ts";

const policy=schema.definitions.Policy.const;
/** 🪵️ The authored numbers of the failed action output tail. */
export const BAZEL_FAILURE_TAIL_POLICY_V1=policy;
export type BazelFailureTailLanguageV1="en"|"de";
export type BazelFailureTailV1=Readonly<{push(line:string):void;result():Readonly<{text:string;bytes:number;droppedBytes:number}>}>;
export type BazelFailureTailRowV1=Readonly<{task:string;bytes:number;droppedBytes:number;file:string;text:string}>;

/** ✂️ Keeps the last `maximumBytes` bytes of every line pushed, newline-terminated, in arrival order; a cut never lands inside a UTF-8 sequence, and `droppedBytes` counts everything that fell off the front. */
export function createBazelFailureTailV1(maximumBytes:number=policy.maximumTailBytes):BazelFailureTailV1{
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>policy.maximumTailBytes)throw Error("Bounded Bazel failure tail size required");
 let pieces:Buffer[]=[],kept=0,dropped=0;
 return Object.freeze({
  push(line:string):void{
   let piece=Buffer.from(line+"\n","utf8");pieces.push(piece);kept+=piece.length;
   while(kept>maximumBytes){
    const head=pieces[0]!,excess=kept-maximumBytes;
    if(excess>=head.length){pieces.shift();kept-=head.length;dropped+=head.length;continue;}
    let cut=excess;while(cut<head.length&&(head[cut]!&0xc0)===0x80)cut++;
    pieces[0]=head.subarray(cut);kept-=cut;dropped+=cut;
   }
  },
  result(){const bytes=Buffer.concat(pieces);return Object.freeze({text:bytes.toString("utf8"),bytes:bytes.length,droppedBytes:dropped});}
 });
}

/** 🏷️ Names the tail file of one task: a short ASCII stem plus the digest of the task id, so no id can leave the directory or collide. */
export function bazelFailureTailFileV1(task:string):string{
 return task.replace(/[^A-Za-z0-9._-]/gu,"_").slice(0,80)+"-"+createHash("sha256").update(task).digest("hex").slice(0,12)+policy.fileSuffix;
}

/** 📁️ Names the failure directory inside a generated package of a selection. */
export function bazelFailureTailDirectoryV1(workspace:string):string{return join(workspace,policy.directory);}

/** 💾️ Persists the tail of one failed task: a JSON header line, then the tail bytes; written beside and renamed, so a reader never sees half a file. */
export async function writeBazelFailureTailV1(directory:string,task:string,tail:BazelFailureTailV1):Promise<string>{
 const result=tail.result(),header={version:1 as const,task,bytes:result.bytes,droppedBytes:result.droppedBytes};
 if(validateJsonSchemaSubset(schema.definitions.Header,header,schema).length)throw Error("Closed Bazel failure tail header required");
 const file=join(directory,bazelFailureTailFileV1(task)),temporary=file+"."+process.pid+".tmp";
 await mkdir(directory,{recursive:true});await writeFile(temporary,JSON.stringify(header)+"\n"+result.text,{mode:0o600});await rename(temporary,file);return file;
}

/** 🔎️ Parses one tail file; anything that is not exactly a header line plus the declared bytes is refused. */
export function parseBazelFailureTailV1(file:string,content:Buffer):BazelFailureTailRowV1{
 const end=content.indexOf(10);if(end<0||end>policy.maximumHeaderBytes)throw Error(`Bazel failure tail header required: ${file}`);
 const header=JSON.parse(content.subarray(0,end).toString("utf8"));if(validateJsonSchemaSubset(schema.definitions.Header,header,schema).length)throw Error(`Closed Bazel failure tail header required: ${file}`);
 const body=content.subarray(end+1);if(body.length!==header.bytes)throw Error(`Bazel failure tail length mismatch: ${file}`);
 return Object.freeze({task:header.task,bytes:header.bytes,droppedBytes:header.droppedBytes,file,text:body.toString("utf8")});
}

/** 📚️ Reads every tail a failed build left in the directory, ordered by task. */
export async function readBazelFailureTailsV1(directory:string):Promise<readonly BazelFailureTailRowV1[]>{
 let names:string[];try{names=(await readdir(directory)).filter(name=>name.endsWith(policy.fileSuffix)).sort().slice(0,policy.maximumFiles);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return [];throw error;}
 const rows:BazelFailureTailRowV1[]=[];for(const name of names){const file=join(directory,name);try{rows.push(parseBazelFailureTailV1(file,await readFile(file)));}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}}
 return Object.freeze(rows.sort((left,right)=>left.task<right.task?-1:left.task>right.task?1:0));
}

/** 🧹️ Empties the failure directory before a build, so only this run's failures are ever reported. */
export async function clearBazelFailureTailsV1(directory:string):Promise<void>{await rm(directory,{recursive:true,force:true});}

/** 💬️ Renders the report of one failed task in the requested language: the authored heading, then the tail. */
export function renderBazelFailureTailV1(row:BazelFailureTailRowV1,language:BazelFailureTailLanguageV1):string{
 const text=(schema.definitions.Messages.const[language] as Readonly<Record<string,string>>)[row.droppedBytes===0?"complete":"failed"]!,values:Readonly<Record<string,string|number>>={task:row.task,bytes:row.bytes,dropped:row.droppedBytes,file:row.file};
 const heading=text.replace(/\{([a-z]+)\}/gu,(_,name:string)=>String(values[name]));
 return heading+"\n"+row.text+(row.text.endsWith("\n")||row.text===""?"":"\n");
}

/** 📣️ Prints the tails a failed build left, each preceded by its authored heading; returns how many were reported. */
export async function reportBazelFailureTailsV1(directory:string,language:BazelFailureTailLanguageV1,write:(text:string)=>void):Promise<number>{
 const rows=await readBazelFailureTailsV1(directory);for(const row of rows)write(renderBazelFailureTailV1(row,language));return rows.length;
}

/** 🔁️ Classifies a failed build by the output of its failed tasks: a task that yielded because its implementation is outdated makes the outermost command start again with the current one (`reexec`), a task whose input an edit kept moving makes the lane run the build again (`reobserve`, the `build` layer of the churn policy); `undefined` when no failed task names such a class, so the failure stands. */
export async function bazelFailureChurnV1(directory:string):Promise<Error|undefined>{
 return failureChurnOfTailsV1(await readBazelFailureTailsV1(directory).catch(()=>[] as readonly BazelFailureTailRowV1[]));
}

/** 🏃️ Runs one Bazel build batch under the churn policy: before every attempt the failure directory is emptied, and a batch that fails because tasks yielded for an outdated implementation ends with the stale implementation error (the outermost command starts again), one that fails because task inputs were edited meanwhile runs again, bounded by the `build` layer, with the announcements and cancellation of the policy; any other failure is the original error. */
export function runBazelBatchWithChurnV1<T>(directory:string,run:()=>Promise<T>,options:ChurnOptionsV1={}):Promise<T>{
 return retryOnConcurrentEditV1("build",async()=>{await clearBazelFailureTailsV1(directory);try{return await run();}catch(error){throw await bazelFailureChurnV1(directory)??error;}},options);
}
