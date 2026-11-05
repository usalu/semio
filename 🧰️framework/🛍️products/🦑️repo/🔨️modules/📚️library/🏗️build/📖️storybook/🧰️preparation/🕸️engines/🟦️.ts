import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import catalog from "../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json" with {type:"json"};
import type {ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
export type StorybookEngineScopeV1=Readonly<{id:string;wasmPackages?:readonly string[]}>;

/** 🧩️ Selects only distinct package requirements declared by actual active story and preview consumers. */
export function storybookEnginePackagesV1(scopes:readonly StorybookEngineScopeV1[],declared:readonly string[]=catalog.sourcePackages.map(row=>row.packageName)):readonly string[]{
 const packages=[...new Set(scopes.flatMap(scope=>scope.wasmPackages??[]))];
 if(packages.length>schema.definitions.Protocol.const.maximumPackages||packages.some(name=>!declared.includes(name)))throw Error("Finite declared Storybook engine packages required");
 return Object.freeze(packages);
}

/** 🏗️ Resolves each selected package through its canonical authored Source producer, never a published byte probe. */
export async function storybookEngineBuildArgumentsV1(workspace:string,mode:string,scope:string):Promise<readonly (readonly string[])[]>{
 if(mode!=="prototype"&&mode!=="release")throw Error("Explicit Storybook engine build mode required");
 const {resolveActiveScopes}=await import("../../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts"),packages=storybookEnginePackagesV1(resolveActiveScopes(scope));
 return packages.map(name=>{
  const owner=catalog.sourcePackages.find(row=>row.packageName===name)!;
  const project=JSON.parse(readFileSync(join(workspace,owner.sourceOwner,"📋️project.json"),"utf8")) as {name?:string;targets?:Record<string,unknown>};
  if(!project.name||!project.targets?.wasm)throw Error("Storybook engine must name an authored wasm producer: "+name);
  return Object.freeze(["run",project.name+":wasm"]);
 });
}

/** 🌱️ Declares exact Storybook producer descendants before the fresh Original is issued. */
export async function storybookBuildDescendantsV1(workspace:string,args:readonly string[],environment:Readonly<Record<string,string|undefined>>):Promise<readonly (readonly string[])[]>{
 if(args.some(word=>schema.definitions.Protocol.const.helpFlags.includes(word)))return [];
 let request=args;
 if(args[0]==="run"){
  if(!args[1]?.startsWith(schema.definitions.Protocol.const.workspaceProject+":"))return [];
  const root=JSON.parse(readFileSync(join(workspace,"📋️project.json"),"utf8")) as {name:string;targets:Record<string,{options?:{command?:string;env?:Record<string,string>;forwardAllArgs?:boolean}}>};
  const [project,target]=args[1]?.split(":")??[];
  if(project!==root.name||!target)return [];
  const declaration=root.targets[target],words=declaration?.options?.command?.split(/\s+/u)??[],entry=words.findIndex(word=>word==="storybook");
  if(entry<1||declaration===undefined)return [];
  const boundary=args.indexOf("--");
  request=[...words.slice(entry-1),...(declaration.options?.forwardAllArgs===true&&boundary>=0?args.slice(boundary+1):[])];
  environment={...environment,...declaration.options?.env};
 }
 if(!["build","test","dev"].includes(request[0]??"")||request[1]!=="storybook")return [];
 const tail=request.slice(2),boundary=tail.findIndex(word=>word.startsWith("-")),positional=request[0]==="dev"?tail.slice(0,boundary<0?undefined:boundary):[];
 const scope=request[0]==="dev"?positional.join("/"):environment.STORYBOOK_SCOPE??"";
 return storybookEngineBuildArgumentsV1(workspace,environment.SEMIO_BUILD_MODE??"",scope);
}

/** 🔒️ Prepares current engine artifacts serially before any Storybook compiler or store hold begins. */
export async function prepareStorybookEnginesV1(workspace:string,mode:string,scope:string,invocation:ScriptInvocation):Promise<void>{
 const requests=await storybookEngineBuildArgumentsV1(workspace,mode,scope);
 if(!requests.length)return;
 const {scriptCompilerAdmissionControlV1}=await import("../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"),{runOwnedBuildChildInvocationV1}=await import("../../../📥️caller/📤️child/🟦️.ts");
 const control=scriptCompilerAdmissionControlV1(invocation,"storybook:engines",schema.definitions.Protocol.const.enginePhaseAdditionalMilliseconds);
 for(const args of requests)await runOwnedBuildChildInvocationV1(workspace,args,invocation,control);
}
