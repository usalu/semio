import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📐️ The authored platform-class policy.
 * @see ./🧬️schema/🔣️.json */
export const PLATFORM_CLASS_POLICY_V1=schema.definitions.Policy.const;
const policy=PLATFORM_CLASS_POLICY_V1,allowedRuntime=policy.allowedRuntimePatterns.map(source=>new RegExp(source,"u")),boundEnvironment=policy.boundEnvironmentPatterns.map(source=>new RegExp(source,"u")),boundCommand=policy.boundCommandPatterns.map(source=>new RegExp(source,"u"));

/** 🧭️ Whether a task hash may be shared across platforms (`independent`) or carries the platform (`platform`), and why. */
export type PlatformVerdictV1=Readonly<{class:"independent"|"platform";reason:string}>;
/** 🔌️ The parts of a projected Nx target and project the class is judged from. */
export type PlatformTargetV1={inputs?:readonly unknown[];options?:Readonly<Record<string,unknown>>;metadata?:unknown;[key:string]:unknown};
export type PlatformProjectV1={namedInputs?:Readonly<Record<string,readonly unknown[]>>;targets?:Record<string,PlatformTargetV1>;[key:string]:unknown};
type Summary={bound:string|undefined;platform:boolean};

/** 🔎️ Whether an input row is the platform runtime row. */
export const isPlatformRowV1=(item:unknown):boolean=>item!==null&&typeof item==="object"&&!Array.isArray(item)&&Object.keys(item).length===1&&typeof (item as {runtime?:unknown}).runtime==="string"&&(policy.platformRuntimes as readonly string[]).includes((item as {runtime:string}).runtime);

const bare=(name:string)=>name.replace(/^[^A-Za-z0-9]+/u,"");
const metadataOf=(target:PlatformTargetV1):Record<string,unknown>=>{const semio=(target.metadata as {semio?:Record<string,unknown>}|undefined)?.semio;return semio!==null&&typeof semio==="object"?semio:{};};

/** 🧭️ Judges every input group once: whether it reaches a native toolchain input, a runtime, an environment variable or a command that binds a task to its platform, and whether it carries the platform row. */
function summariser(projects:Readonly<Record<string,PlatformProjectV1>>){
 const memo=new Map<string,Summary>();
 const group=(owner:string,name:string,trail:Set<string>):Summary=>{
  const key=`${owner}\0${name}`,known=memo.get(key);if(known)return known;
  if(trail.has(key))return {bound:undefined,platform:false};
  trail.add(key);
  const list=projects[owner]?.namedInputs?.[name],result=list===undefined?{bound:undefined,platform:false}:rows(owner,list,trail);
  trail.delete(key);memo.set(key,result);return result;
 };
 const rows=(owner:string,list:readonly unknown[],trail:Set<string>):Summary=>{
  let bound:string|undefined,platform=false;
  for(const item of list){
   let part:Summary|undefined;
   if(typeof item==="string"){
    const name=item.replace(/^[!^]+/u,"");
    if((policy.boundInputs as readonly string[]).includes(name))part={bound:`input ${name}`,platform:false};
    else if(!name.startsWith("{")&&projects[owner]?.namedInputs&&Object.hasOwn(projects[owner]!.namedInputs!,name))part=group(owner,name,trail);
   }else if(item!==null&&typeof item==="object"&&!Array.isArray(item)){
    const row=item as Record<string,unknown>;
    if(isPlatformRowV1(row))platform=true;
    else if(typeof row.runtime==="string"){if(!allowedRuntime.some(pattern=>pattern.test(row.runtime as string)))part={bound:`runtime ${row.runtime}`,platform:false};}
    else if(typeof row.env==="string"){if(boundEnvironment.some(pattern=>pattern.test(row.env as string)))part={bound:`env ${row.env}`,platform:false};}
    else if(typeof row.input==="string"){
     if((policy.boundInputs as readonly string[]).includes(row.input))part={bound:`input ${row.input}`,platform:false};
     else for(const other of typeof row.projects==="string"?[row.projects]:Array.isArray(row.projects)?row.projects as string[]:[]){const found=group(other,row.input,trail);if(found.bound)part=found;if(found.platform)platform=true;}
    }
   }
   if(part){platform||=part.platform;bound??=part.bound;}
  }
  return {bound,platform};
 };
 return {rows,group};
}

const commandText=(target:PlatformTargetV1):string=>{const options=target.options;if(options===undefined)return "";return JSON.stringify([options.command,options.commands,options.nativeOwnerCommand===undefined?undefined:"native"]);};

/** 🧭️ The platform class of one projected target: `independent` only when the target is native-free, reaches no native toolchain input and no toolchain runtime, environment or command, declares no store-bound output kind, is not declared `bound` in its metadata and is of an independent kind (the authored list of names, or the declaration `metadata.semio.platform = "independent"`); `platform` in every other case, with the first reason found. */
export function platformClassV1(name:string,target:PlatformTargetV1,project:string,projects:Readonly<Record<string,PlatformProjectV1>>,summary=summariser(projects)):PlatformVerdictV1&{reachesPlatform:boolean}{
 const semio=metadataOf(target),declared=semio[policy.metadataKey],inputs=target.inputs??["default","^default"],reached=summary.rows(project,inputs,new Set());
 const verdict=(klass:"independent"|"platform",reason:string):PlatformVerdictV1&{reachesPlatform:boolean}=>({class:klass,reason,reachesPlatform:reached.platform});
 if(declared==="bound")return verdict("platform","declared bound");
 if(target.options?.nativeOwnerCommand!==undefined)return verdict("platform","native owner command");
 for(const key of policy.boundMetadata)if(semio[key]!==undefined)return verdict("platform",`metadata ${key}`);
 if(reached.bound)return verdict("platform",reached.bound);
 const text=commandText(target),hit=boundCommand.find(pattern=>pattern.test(text));if(hit)return verdict("platform",`command ${hit.source}`);
 if(declared==="independent")return verdict("independent","declared independent");
 if((policy.independentTargets as readonly string[]).includes(bare(name)))return verdict("independent",`kind ${bare(name)}`);
 return verdict("platform","kind not listed");
}

/** ✂️ Takes the platform row out of every input group and target of the projects and puts it back, as one row of the target's own inputs, on every target whose hash reached it and whose class is `platform`. A target of class `independent` carries no platform, so its hash is the same on every operating system and architecture; every other target keeps exactly the platform dependence it had. The projects that need no change keep their identity. */
export function splitNxPlatformInputsV1<T extends Record<string,PlatformProjectV1>>(projects:T):T{
 const summary=summariser(projects),out:Record<string,PlatformProjectV1>={};
 for(const [owner,project] of Object.entries(projects)){
  const named=project.namedInputs===undefined?undefined:Object.fromEntries(Object.entries(project.namedInputs).map(([key,list])=>[key,list.some(isPlatformRowV1)?list.filter(item=>!isPlatformRowV1(item)):list]));
  const targets=project.targets===undefined?undefined:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>{
   const verdict=platformClassV1(name,target,owner,projects,summary),own=target.inputs?.some(isPlatformRowV1)===true;
   if(!verdict.reachesPlatform&&!own)return [name,target];
   const kept=(target.inputs??["default","^default"]).filter(item=>!isPlatformRowV1(item));
   return [name,{...target,inputs:verdict.class==="platform"?[...kept,{runtime:policy.canonicalRuntime}]:kept}];
  }));
  out[owner]={...project,...(named===undefined?{}:{namedInputs:named}),...(targets===undefined?{}:{targets})};
 }
 return out as T;
}
