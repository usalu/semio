import {createHash} from "node:crypto";
import {createReadStream,readFileSync,rmSync} from "node:fs";
import {lstat,mkdir,readdir,readFile,realpath,rename,rm,writeFile} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";

const schema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8")),policy=schema.definitions.Policy.const,repair=schema.definitions.Repair.const,entryPath=new RegExp(schema.definitions.Entry.properties.path.pattern,"u"),targetName=new RegExp(schema.definitions.Target.pattern,"u"),repairTask=new RegExp(repair.task,"u"),digestPattern=new RegExp(schema.definitions.Entry.properties.sha256.pattern,"u");
const posix=value=>value.split(sep).join("/"),text=value=>Buffer.from(value,"utf8");

/** 🧾️ Names the declared receipt output of one target, relative to its project root. */
export function storeReceiptOutputV1(target){
 if(typeof target!=="string"||!targetName.test(target))throw Error("Closed store receipt target required");
 return policy.projectRootToken+"/"+policy.directory+"/"+target+policy.extension;
}

/** 🔎️ Resolves the absolute receipt path a task declares among its outputs, or nothing when it declares none. */
export function declaredStoreReceiptV1({workspaceRoot,projectRoot,target,outputs}){
 if(!Array.isArray(outputs)||!outputs.includes(storeReceiptOutputV1(target)))return undefined;
 if(typeof workspaceRoot!=="string"||!isAbsolute(workspaceRoot)||typeof projectRoot!=="string"||isAbsolute(projectRoot))throw Error("Workspace root and relative project root required");
 const path=resolve(workspaceRoot,projectRoot,policy.directory,target+policy.extension),inside=relative(workspaceRoot,path);
 if(!inside||inside===".."||inside.startsWith(".."+sep)||isAbsolute(inside)||inside.length>policy.maximumPathLength)throw Error("Store receipt must lie inside the workspace");
 return path;
}

/** 🎟️ Starts a task that declares a receipt from no receipt and tells its handler where to write it; any other task is told of none, so a nested task never records into its parent's receipt. */
export function issueStoreReceiptV1(declaration){
 const path=declaredStoreReceiptV1(declaration);if(path===undefined)return {[policy.environment]:undefined};
 rmSync(path,{force:true});
 return {[policy.environment]:path};
}

/** 🔧️ Admits the single-task uncached Nx child launch that repairs a receipt whose store content is gone. */
export function isStoreRepairSelectionV1(selection){
 const head=2+repair.arguments.length;
 return Array.isArray(selection)&&selection.length>=head&&selection.every(value=>typeof value==="string")&&selection[0]===repair.command&&repairTask.test(selection[1])&&repair.arguments.every((argument,index)=>selection[2+index]===argument)&&(selection.length===head||selection[head]==="--");
}

/** 🛠️ Projects the Nx arguments that re-execute exactly one task uncached. */
export function storeRepairNxArgumentsV1(task,forwarded=[]){
 if(typeof task!=="string"||!repairTask.test(task)||!Array.isArray(forwarded)||forwarded.some(value=>typeof value!=="string"))throw Error("Closed store repair task required");
 return [repair.command,task,...repair.arguments,...forwarded.length?["--",...forwarded]:[]];
}

async function fileDigest(path){
 const hash=createHash("sha256");
 await new Promise((done,fail)=>{const stream=createReadStream(path);stream.on("data",chunk=>hash.update(chunk));stream.on("error",fail);stream.on("end",done);});
 return hash.digest("hex");
}

async function walk(root,prefix,rows){
 for(const name of (await readdir(join(root,prefix))).sort()){
  const relativePath=prefix?prefix+"/"+name:name,facts=await lstat(join(root,relativePath));
  if(facts.isDirectory())await walk(root,relativePath,rows);
  else if(facts.isFile())rows.push({path:relativePath,bytes:facts.size});
  else throw Error("Store content holds regular files and directories only: "+relativePath);
 }
}

/** 🧮️ Digests a directory by its sorted relative paths, sizes and file digests. */
export async function storeTreeDigestV1(directory,hashes=true){
 const rows=[];await walk(directory,"",rows);rows.sort((left,right)=>Buffer.compare(text(left.path),text(right.path)));
 const hash=createHash("sha256");let bytes=0;
 for(const row of rows){bytes+=row.bytes;hash.update(text(row.path+"\n"+row.bytes+"\n"+(hashes?await fileDigest(join(directory,row.path)):"")+"\n"));}
 return {sha256:hash.digest("hex"),bytes,files:rows.length};
}

/** 🔭️ Observes one published file or directory into a receipt entry. */
export async function observeStoreContentV1(workspaceRoot,path){
 const absolute=resolve(path),inside=posix(relative(workspaceRoot,absolute));
 if(!inside||inside===".."||inside.startsWith("../")||isAbsolute(inside)||!entryPath.test(inside)||inside.length>policy.maximumPathLength)throw Error("Store content must lie inside the workspace: "+path);
 const facts=await lstat(absolute);
 if(facts.isFile())return {path:inside,kind:"file",bytes:facts.size,files:1,sha256:await fileDigest(absolute)};
 if(facts.isDirectory()){const tree=await storeTreeDigestV1(absolute);return {path:inside,kind:"directory",bytes:tree.bytes,files:tree.files,sha256:tree.sha256};}
 throw Error("Store content must be a regular file or directory: "+path);
}

/** 📜️ Admits exactly the closed receipt shape the schema declares. */
export function requireStoreReceiptV1(value){
 const object=value!==null&&typeof value==="object"&&!Array.isArray(value),keys=(row,expected)=>row!==null&&typeof row==="object"&&Object.keys(row).sort().join()===[...expected].sort().join();
 if(!object||!keys(value,["version","project","target","entries","links"])||value.version!==1||typeof value.project!=="string"||!value.project||value.project.length>256||typeof value.target!=="string"||!targetName.test(value.target)||!Array.isArray(value.entries)||!value.entries.length||value.entries.length>policy.maximumEntries||!Array.isArray(value.links)||value.links.length>policy.maximumLinks)throw Error("Closed store receipt required");
 const seen=new Set();
 for(const entry of value.entries){
  if(!keys(entry,["path","kind","bytes","files","sha256"])||typeof entry.path!=="string"||!entryPath.test(entry.path)||entry.path.length>policy.maximumPathLength||!["file","directory"].includes(entry.kind)||!Number.isSafeInteger(entry.bytes)||entry.bytes<0||!Number.isSafeInteger(entry.files)||entry.files<0||typeof entry.sha256!=="string"||!digestPattern.test(entry.sha256)||entry.kind==="file"&&entry.files!==1||seen.has(entry.path))throw Error("Closed store receipt entry required");
  seen.add(entry.path);
 }
 const bound=new Set();
 for(const link of value.links){
  if(!keys(link,["path","target"])||[link.path,link.target].some(path=>typeof path!=="string"||!entryPath.test(path)||path.length>policy.maximumPathLength)||bound.has(link.path))throw Error("Closed store receipt link required");
  bound.add(link.path);
 }
 return value;
}

/** 📖️ Reads one receipt from its text. */
export function parseStoreReceiptV1(source){
 if(typeof source!=="string"||Buffer.byteLength(source)>policy.maximumReceiptBytes)throw Error("Bounded store receipt text required");
 return requireStoreReceiptV1(JSON.parse(source));
}

/** 🖋️ Encodes a receipt canonically: entries sorted by path, two-space indent, trailing newline. */
export function encodeStoreReceiptV1(receipt){
 const sorted={version:1,project:receipt.project,target:receipt.target,links:[...receipt.links].sort((left,right)=>Buffer.compare(text(left.path),text(right.path))).map(({path,target})=>({path,target})),entries:[...receipt.entries].sort((left,right)=>Buffer.compare(text(left.path),text(right.path))).map(({path,kind,bytes,files,sha256})=>({path,kind,bytes,files,sha256}))};
 requireStoreReceiptV1(sorted);return JSON.stringify(sorted,null,2)+"\n";
}

async function withReceiptLock(receipt,body){
 const lock=receipt+".lock",deadline=Date.now()+60000;await mkdir(dirname(receipt),{recursive:true});
 for(;;){
  try{await mkdir(lock);break;}catch(error){
   if(error?.code!=="EEXIST")throw error;
   const facts=await lstat(lock).catch(()=>undefined);
   if(facts&&Date.now()-facts.mtimeMs>30000){await rm(lock,{recursive:true,force:true});continue;}
   if(Date.now()>deadline)throw Error("Store receipt lock timed out: "+receipt);
   await new Promise(done=>setTimeout(done,25));
  }
 }
 try{return await body();}finally{await rm(lock,{recursive:true,force:true});}
}

/** 🧷️ Records published store content into the receipt the running task declared; a task without a declared receipt records nothing. */
export async function recordStoreContentV1({environment=process.env,workspaceRoot,paths=[],links=[]}){
 const receipt=environment[policy.environment];if(receipt===undefined)return;
 const project=environment.SEMIO_TASK_PROJECT,target=environment.SEMIO_TASK_TARGET;
 if(typeof project!=="string"||typeof target!=="string"||!isAbsolute(receipt))throw Error("Declared store receipt requires its task identity");
 const observed=[];for(const path of paths)observed.push(await observeStoreContentV1(workspaceRoot,path));
 await withReceiptLock(receipt,async()=>{
  let entries=observed,bound=links;
  try{const existing=parseStoreReceiptV1(await readFile(receipt,"utf8"));if(existing.project===project&&existing.target===target){entries=[...existing.entries.filter(entry=>!observed.some(row=>row.path===entry.path)),...observed];bound=[...existing.links.filter(link=>!links.some(row=>row.path===link.path)),...links];}}catch(error){if(error?.code!=="ENOENT"&&!(error instanceof SyntaxError)&&!String(error?.message).startsWith("Closed store receipt"))throw error;}
  const temporary=receipt+"."+process.pid+".tmp";await writeFile(temporary,encodeStoreReceiptV1({project,target,entries,links:bound}));await rename(temporary,receipt);
 });
}

async function verifyLink(workspaceRoot,link){
 const absolute=resolve(workspaceRoot,link.path);
 try{
  if(!(await lstat(absolute)).isSymbolicLink())return {path:link.path,reason:"link"};
  return await realpath(absolute)===await realpath(resolve(workspaceRoot,link.target))?undefined:{path:link.path,reason:"link"};
 }catch(error){return {path:link.path,reason:error?.code==="ENOENT"||error?.code==="ENOTDIR"?"absent":"link"};}
}

async function verifyEntry(workspaceRoot,entry,mode){
 const absolute=resolve(workspaceRoot,entry.path);let facts;
 try{facts=await lstat(absolute);}catch(error){return {path:entry.path,reason:error?.code==="ENOENT"||error?.code==="ENOTDIR"?"absent":"unreadable"};}
 try{
  if(entry.kind==="file"){
   if(!facts.isFile())return {path:entry.path,reason:"kind"};
   if(facts.size!==entry.bytes)return {path:entry.path,reason:"size"};
   if(mode==="digest"&&await fileDigest(absolute)!==entry.sha256)return {path:entry.path,reason:"digest"};
   return undefined;
  }
  if(!facts.isDirectory())return {path:entry.path,reason:"kind"};
  const tree=await storeTreeDigestV1(absolute,mode==="digest");
  if(tree.bytes!==entry.bytes||tree.files!==entry.files)return {path:entry.path,reason:"size"};
  if(mode==="digest"&&tree.sha256!==entry.sha256)return {path:entry.path,reason:"digest"};
  return undefined;
 }catch{return {path:entry.path,reason:"unreadable"};}
}

/** ✅️ Checks every receipt entry against the store: `stat` compares kind and size, `digest` also compares content. */
export async function verifyStoreReceiptV1(workspaceRoot,receipt,mode="digest"){
 if(!["stat","digest"].includes(mode))throw Error("Store receipt verification mode required");
 requireStoreReceiptV1(receipt);const findings=[];
 for(const entry of receipt.entries){const finding=await verifyEntry(workspaceRoot,entry,mode);if(finding)findings.push(finding);}
 for(const link of receipt.links){const finding=await verifyLink(workspaceRoot,link);if(finding)findings.push(finding);}
 return {state:findings.length?"invalid":"valid",findings};
}

/** 🗂️ Verifies the receipt file one task left behind: missing is `unwritten`, unreadable is `malformed`. */
export async function checkStoreReceiptFileV1(workspaceRoot,file,mode="digest"){
 let source;
 try{source=await readFile(file,"utf8");}catch(error){if(error?.code==="ENOENT")return {state:"unwritten",findings:[]};throw error;}
 let receipt;try{receipt=parseStoreReceiptV1(source);}catch{return {state:"malformed",findings:[]};}
 return verifyStoreReceiptV1(workspaceRoot,receipt,mode);
}

