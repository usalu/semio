import{requireIssuedResourceControlInvocationV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {createRequire} from "node:module";
import {readFile,writeFile,mkdir,rename} from "node:fs/promises";
import {readFileSync,lstatSync,realpathSync,readdirSync,openSync,fstatSync,readSync,closeSync} from "node:fs";
import {join,dirname,sep,resolve,relative} from "node:path";
import {createHash,randomUUID} from "node:crypto";
import {buildParserReceiptMatchesV1,type BuildParserReceiptV1} from "./📦️parsers/🟦️.ts";
import bindingsSchema from "../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️bindings.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {checkScriptInvocation,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🌱️ Publishes system-only source prerequisites before any graph or application import. */
export function publishBuildBootstrapSourcesV1(root:string):void {
  const declaration=JSON.parse(readFileSync(new URL("./🌱️sources/🔣️.json",import.meta.url),"utf8")) as {sources:{module:string;export:string}[]};
  for(const source of declaration.sources)createRequire(import.meta.url)(join(root,source.module))[source.export](root);
}

/** 📦️ Supplies the only dependency preparation operation before build inference can run. */
export type BuildEnvironmentInstallerV1=(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl)=>Promise<void>;

/** 🥾️ Acquires missing or changed locked parser dependencies through the dependency-free owner. */
export async function ensureBuildEnvironmentV1(root:string,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,install:BuildEnvironmentInstallerV1=async(root,original,phase)=>{
  const {prepareJavascriptDependencies}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts");
  const {withJavascriptDependencyStorageV1}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");checkResourceLeaseControl(phase);await withJavascriptDependencyStorageV1(root,original,phase,(store,control)=>prepareJavascriptDependencies("sync",root,store,control));checkResourceLeaseControl(phase);
},publish:(root:string)=>void=publishBuildBootstrapSourcesV1):Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);const check=()=>{checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);};check();
  publish(root);check();
  const workspace=(invocation.capabilities as {workspaceRoot?:unknown}).workspaceRoot;
  if(typeof workspace!=="string"||resolve(workspace)!==resolve(root))throw Error("Original build bootstrap repository required");
  const {withJavascriptDependencyStorageV1,requireJavascriptDependencyStorageV1}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");
  const {requireNativeStoragePathV1}=await import("../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts");
  const bounded=(path:string,maximum:number)=>{check();const before=lstatSync(path);check();if(!before.isFile()||before.isSymbolicLink()||before.size>maximum)throw Error("Bounded regular bootstrap input required");const descriptor=openSync(path,"r");try{const opened=fstatSync(descriptor);check();if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size)throw Error("Bootstrap input changed before read");const bytes=Buffer.alloc(opened.size);let offset=0;while(offset<bytes.length){check();const count=readSync(descriptor,bytes,offset,bytes.length-offset,offset);if(!count)throw Error("Bootstrap input truncated");offset+=count;}const after=fstatSync(descriptor),current=lstatSync(path);check();if(after.size!==opened.size||after.mtimeMs!==opened.mtimeMs||after.ctimeMs!==opened.ctimeMs||current.ino!==opened.ino||current.dev!==opened.dev)throw Error("Bootstrap input changed during read");return bytes;}finally{closeSync(descriptor);}};
  const manifestBytes=bounded(join(root,"package.json"),8388608),manifest=JSON.parse(manifestBytes.toString()),lock=bounded(join(root,"bun.lock"),8388608),key=createHash("sha256").update(JSON.stringify([manifest.dependencies,manifest.devDependencies,manifest.optionalDependencies,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])).update(lock).digest("hex");
  const inspect=async(publishReceipt:boolean)=>withJavascriptDependencyStorageV1(workspace,invocation,phaseControl,async(store,actualControl)=>{
    const check=()=>{checkScriptInvocation(invocation);checkResourceLeaseControl(phaseControl);checkResourceLeaseControl(actualControl);};check();
    await requireJavascriptDependencyStorageV1(workspace,store);const path=requireNativeStoragePathV1(store.root,join(store.root,"bootstrap",createHash("sha256").update(resolve(root)).digest("hex").slice(0,24)+".json"));
    const witness=():BuildParserReceiptV1=>{
      if(!lstatSync(join(root,"node_modules")).isSymbolicLink())throw Error("Original parser dependency binding required");
      const target=requireNativeStoragePathV1(store.root,realpathSync(join(root,"node_modules"))),stage=dirname(target),bindingPath=requireNativeStoragePathV1(store.root,join(stage,"bindings.json")),bindingBytes=bounded(bindingPath,1048576),binding=JSON.parse(bindingBytes.toString());
      if(validateJsonSchemaSubset(bindingsSchema,binding).length||target!==join(store.root,"j",binding.generation.slice(0,20),"node_modules")||!binding.bindings.some((row:any)=>row.sourceDirectory===join(root,"node_modules")&&row.targetDirectory===target))throw Error("Original parser generation receipt required");
      const require=createRequire(join(root,"package.json")),parsers=["typescript","@iarna/toml"].map(name=>{
        const directory=requireNativeStoragePathV1(store.root,realpathSync(dirname(require.resolve(name+"/package.json")))) ,entry=realpathSync(require.resolve(name));
        if(directory!==join(target,...name.split("/"))||!entry.startsWith(directory+sep))throw Error("Original installed parser package required");
        const installed=JSON.parse(bounded(join(directory,"package.json"),1048576).toString()),escaped=name.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"),locked=new RegExp('"'+escaped+'"\\s*:\\s*\\[\\s*"'+escaped+'@([^"\\s]+)"').exec(lock.toString())?.[1];
        if(typeof installed.version!=="string"||!locked||installed.version!==locked)throw Error("Exact locked parser version required");
        const hash=createHash("sha256");let files=0,bytes=0,entries=0;const walk=(parent:string)=>{const state=lstatSync(parent);if(!state.isDirectory()||state.isSymbolicLink())throw Error("Physical installed parser directory required");for(const item of readdirSync(parent).sort()){check();if(++entries>2048)throw Error("Bounded installed parser entries required");const path=requireNativeStoragePathV1(store.root,join(parent,item)),state=lstatSync(path);if(state.isDirectory()&&!state.isSymbolicLink())walk(path);else{if(!state.isFile()||state.isSymbolicLink()||++files>1024||bytes+state.size>33554432)throw Error("Bounded installed parser package required");const content=bounded(path,33554432-bytes);bytes+=content.length;hash.update(JSON.stringify([relative(directory,path).split(sep).join("/"),content.length])).update(content);}}};walk(directory);
        return {name,version:installed.version,digest:hash.digest("hex"),files,bytes};
      });return {version:1,key,generation:binding.generation,bindingDigest:createHash("sha256").update(bindingBytes).digest("hex"),parsers};
    };
    let current:BuildParserReceiptV1;try{current=witness();}catch(error){if(publishReceipt)throw error;return false;}
    if(!bounded(join(root,"package.json"),8388608).equals(manifestBytes)||!bounded(join(root,"bun.lock"),8388608).equals(lock))throw Error("Build bootstrap defining Source changed");
    await store.recheck();check();
    if(!publishReceipt){try{return buildParserReceiptMatchesV1(JSON.parse(bounded(path,16384).toString()),current);}catch{return false;}}
    if(!buildParserReceiptMatchesV1(current,current))throw Error("Closed parser material receipt required");await mkdir(dirname(path),{recursive:true});check();const temporary=requireNativeStoragePathV1(store.root,path+"."+randomUUID().slice(0,8));await writeFile(temporary,JSON.stringify(current));check();await rename(temporary,path);check();return true;
  });
  if(await inspect(false)){check();return;}check();
  await invocation.control.publish({owner:invocation.policy.owner,command:"build:dependencies","stage":"running"});
  check();await install(root,invocation,phaseControl);check();await inspect(true);check();
}
