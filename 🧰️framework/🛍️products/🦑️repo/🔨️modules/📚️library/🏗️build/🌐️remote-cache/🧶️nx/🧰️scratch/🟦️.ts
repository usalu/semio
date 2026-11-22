import {spawn} from "node:child_process";
import {cpSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {selectPackageManagerV1} from "../../../../⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts";

/** 🧰️ Where a pristine Nx tree for one Nx version is kept between runs (deterministic, reused when its marker is present). */
export const scratchNxTreeV1=(version:string):string=>join(realpathSync(tmpdir()),`semio-pristine-nx-${version}`);

const bunCacheV1=(environment:Readonly<Record<string,string|undefined>>):string=>{const descriptor=selectPackageManagerV1("javascript",{}).descriptor,cacheVariable=Object.entries(descriptor.environment.set).filter(([,role])=>role==="role:cache").map(([name])=>name).find(name=>!name.startsWith("XDG_")),homeVariable=descriptor.runtime.hostEnvironment[0];return(cacheVariable===undefined?undefined:environment[cacheVariable])??join((homeVariable===undefined?undefined:environment[homeVariable])??join(homedir(),"."+descriptor.executable),"install","cache");};

/** 🌿️ Builds, without any network, the unpatched Nx tree a scratch workspace needs: the checkout's `node_modules` guards Nx behind the graph authority of the repo (`repo` issues it to its own children only), so a scratch workspace gets a `node_modules` of symlinks to every entry of the checkout except `nx`, plus a copy of the pristine package of the pinned version from the Bun install cache and the nested dependencies of the checkout's Nx. The tree is built once beside its final name and renamed into place. Answers the path of `nx.js`, or `undefined` when the pristine package is not in the cache. @see https://nx.dev */
export function provisionPristineNxV1(checkoutRoot:string,environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const modules=join(checkoutRoot,"node_modules");
 let version:string;
 try{version=(JSON.parse(readFileSync(join(modules,"nx","package.json"),"utf8")) as {version:string}).version;}catch{return undefined;}
 if(!/^\d+\.\d+\.\d+$/u.test(version))return undefined;
 const tree=scratchNxTreeV1(version),entry=join(tree,"node_modules","nx","dist","bin","nx.js"),source=join(bunCacheV1(environment),`nx@${version}@@@1`);
 if(existsSync(join(tree,".complete")))return entry;
 if(!existsSync(join(source,"dist","bin","nx.js")))return undefined;
 const staging=mkdtempSync(join(realpathSync(tmpdir()),"semio-pristine-nx-build-"));
 try{
  const target=join(staging,"node_modules");mkdirSync(target);
  for(const name of readdirSync(modules))if(name!=="nx"&&name!==".bin")symlinkSync(join(realpathSync(modules),name),join(target,name));
  cpSync(source,join(target,"nx"),{recursive:true});
  const nested=join(modules,"nx","node_modules");
  if(existsSync(nested)&&lstatSync(nested).isDirectory())cpSync(nested,join(target,"nx","node_modules"),{recursive:true,dereference:true});
  writeFileSync(join(staging,".complete"),version+"\n");
  rmSync(tree,{recursive:true,force:true});renameSync(staging,tree);
 }catch{rmSync(staging,{recursive:true,force:true});return existsSync(join(tree,".complete"))?entry:undefined;}
 return entry;
}

/** 🏗️ A scratch Nx workspace with two cacheable projects `a` and `b` (`b` depends on `a`); `nonce` makes every hash unique so a live cache never serves a result from an earlier run. */
export function createScratchNxWorkspaceV1(parent:string,pristineModules:string,label:string,nonce:string):string{
 const base=mkdtempSync(join(parent,`nx-${label}-`));
 for(const project of ["a","b"])mkdirSync(join(base,"libs",project),{recursive:true});
 symlinkSync(pristineModules,join(base,"node_modules"),"junction");
 writeFileSync(join(base,"nx.json"),JSON.stringify({cacheDirectory:".nx-cache",useDaemonProcess:false,analytics:false,targetDefaults:{build:{cache:true,inputs:["{projectRoot}/**/*"],outputs:["{projectRoot}/out"]}}}));
 writeFileSync(join(base,"libs/a/project.json"),JSON.stringify({name:"a",root:"libs/a",targets:{build:{executor:"nx:run-commands",options:{command:"mkdir -p out && echo built-a > out/a.txt",cwd:"libs/a"}}}}));
 writeFileSync(join(base,"libs/b/project.json"),JSON.stringify({name:"b",root:"libs/b",implicitDependencies:["a"],targets:{build:{dependsOn:["^build"],executor:"nx:run-commands",options:{command:"mkdir -p out && echo built-b > out/b.txt",cwd:"libs/b"}}}}));
 writeFileSync(join(base,"libs/a/src.txt"),`a ${nonce}\n`);writeFileSync(join(base,"libs/b/src.txt"),`b ${nonce}\n`);
 return base;
}

/** 🧹️ Forgets the local Nx cache and outputs of a scratch workspace so the next run can only be served by the remote cache. */
export function wipeScratchNxCacheV1(base:string):void{
 for(const path of [".nx-cache",".nx","libs/a/out","libs/b/out"])rmSync(join(base,path),{recursive:true,force:true});
}

/** ▶️ Runs `nx run-many -t build` in a scratch workspace as an asynchronous child (never `spawnSync`: a caller that serves the cache in its own process would block its event loop and the server could not answer), killed after `timeoutMilliseconds`. `hits` and `total` come from Nx's own summary line `Cache: <hits>/<total> hit`. */
export function runScratchNxV1(nx:string,base:string,environment:Readonly<Record<string,string>>,timeoutMilliseconds:number):Promise<Readonly<{status:number|null;text:string;hits:number;total:number}>>{
 return new Promise(accept=>{
  const child=spawn("node",[nx,"run-many","-t","build"],{cwd:base,env:{PATH:process.env.PATH??"",HOME:process.env.HOME??"",NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",CI:"1",NO_COLOR:"1",...environment}}),parts:Buffer[]=[],timer=setTimeout(()=>child.kill("SIGKILL"),timeoutMilliseconds);
  child.stdout.on("data",chunk=>parts.push(chunk));child.stderr.on("data",chunk=>parts.push(chunk));
  const finish=(status:number|null)=>{clearTimeout(timer);const text=Buffer.concat(parts).toString(),match=/Cache:\s*(\d+)\/(\d+)\s+hit/u.exec(text);accept({status,text,hits:match===null?0:Number(match[1]),total:match===null?0:Number(match[2])});};
  child.on("error",()=>finish(null));child.on("close",finish);
 });
}
