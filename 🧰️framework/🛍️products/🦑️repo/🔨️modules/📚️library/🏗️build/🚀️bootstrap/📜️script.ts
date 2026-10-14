import {createRequire} from "node:module";
import {readFile,writeFile,mkdir,rename} from "node:fs/promises";
import {readFileSync,lstatSync} from "node:fs";
import {join,dirname,sep,resolve} from "node:path";
import {createHash,randomUUID} from "node:crypto";
import {checkScriptInvocation,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🌱️ Publishes system-only source prerequisites before any graph or application import. */
export function publishBuildBootstrapSourcesV1(root:string):void {
  const declaration=JSON.parse(readFileSync(new URL("./🌱️sources/🔣️.json",import.meta.url),"utf8")) as {sources:{module:string;export:string}[]};
  for(const source of declaration.sources)createRequire(import.meta.url)(join(root,source.module))[source.export](root);
}

/** 📦️ Supplies the only dependency preparation operation before build inference can run. */
export type BuildEnvironmentInstallerV1=(root:string,invocation:ScriptInvocation)=>Promise<void>;

/** 🥾️ Acquires missing or changed locked parser dependencies through the dependency-free owner. */
export async function ensureBuildEnvironmentV1(root:string,invocation:ScriptInvocation,install:BuildEnvironmentInstallerV1=async(root,original)=>{
  const {prepareJavascriptDependencies}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts");
  const {withJavascriptDependencyStorageV1}=await import("../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts");await withJavascriptDependencyStorageV1(root,original,store=>prepareJavascriptDependencies("sync",root,store));
},publish:(root:string)=>void=publishBuildBootstrapSourcesV1):Promise<void> {
  checkScriptInvocation(invocation);
  publish(root);
  const manifest=JSON.parse(await readFile(join(root,"package.json"),"utf8")),lock=await readFile(join(root,"bun.lock")),key=createHash("sha256").update(JSON.stringify([manifest.dependencies,manifest.devDependencies,manifest.optionalDependencies,manifest.patchedDependencies,process.versions.bun,process.platform,process.arch])).update(lock).digest("hex"),path=join(root,".cache/b/d/m/bootstrap/dependencies.json");
  const ready=()=>{try{
    if(!lstatSync(join(root,"node_modules"),{throwIfNoEntry:false})?.isSymbolicLink())return false;
    const require=createRequire(join(root,"package.json"));
    for(const dependency of ["typescript","@iarna/toml"]){
      const manifestPath=require.resolve(dependency+"/package.json"),entry=require.resolve(dependency),owned=resolve(root,".cache/b/d/m")+sep;
      if(!manifestPath.startsWith(owned)||!entry.startsWith(owned)||!lstatSync(entry,{throwIfNoEntry:false})?.isFile())return false;
      const installed=JSON.parse(readFileSync(manifestPath,"utf8")),escaped=dependency.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"),locked=new RegExp('"'+escaped+'"\\s*:\\s*\\[\\s*"'+escaped+'@([^"\\s]+)"').exec(lock.toString("utf8"))?.[1],declared=manifest.devDependencies?.[dependency]??manifest.dependencies?.[dependency];
      if(typeof installed.version!=="string"||(locked?installed.version!==locked:typeof declared!=="string"||!Bun.semver.satisfies(installed.version,declared)))return false;
    }
    return true;
  }catch{return false;}};
  const publishReceipt=async()=>{await mkdir(dirname(path),{recursive:true});const temporary=path+"."+randomUUID();await writeFile(temporary,JSON.stringify({version:1,key}));await rename(temporary,path);};
  await invocation.control.publish({owner:invocation.policy.owner,command:"build:dependencies","stage":"running"});
  await install(root,invocation);checkScriptInvocation(invocation);
  if(!ready())throw Error("Build inference dependencies are unavailable after synchronization");
  await publishReceipt();
}
