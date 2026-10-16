import storageSchema from"../../📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json";
import{join,resolve,relative,sep,isAbsolute}from"node:path";
import{requireMaterializedPathV1}from"../../🔒️leases/🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
const mountDirectory=storageSchema.definitions.ImageProtocol.const.mountDirectory;
const policy=Object.freeze({...schema.definitions.Policy.const});
/** 📏️ Captures the finite non-recomputable development envelope independently of public schema DATA. */
export function persistentStatePolicyV1():Readonly<typeof policy>{return policy;}
/** ⚖️ Leaves exact state backing and metadata charged inside the existing partition. */
export function persistentStateArtifactBudgetV1(filesystemBytes:number):number{if(!Number.isSafeInteger(filesystemBytes)||filesystemBytes<=policy.chargeBytes)throw Error("Persistent state requires a disjoint artifact budget");return filesystemBytes-policy.chargeBytes;}
/** 🗂️ Resolves one canonical development namespace across build intents without opening it. */
export function persistentStateLayoutV1(parentRoot:string):Readonly<{directory:string;root:string;binding:string;intent:string;data:string}>{const parent=requireMaterializedPathV1(resolve(parentRoot)),directory=requireMaterializedPathV1(join(parent,policy.directory)),root=requireMaterializedPathV1(join(directory,mountDirectory));const binding=requireMaterializedPathV1(join(parent,policy.binding)),intent=requireMaterializedPathV1(join(parent,policy.intent));requireMaterializedPathV1(binding+".new");requireMaterializedPathV1(intent+".new");return Object.freeze({directory,root,binding,intent,data:requireMaterializedPathV1(join(root,policy.dataDirectory))});}

/** 🛡️ Keeps non-recomputable state and its binding records outside recomputable publication or restore targets. */
export function requireRecomputableStatePathV1(root:string,path:string):string{root=resolve(root);path=resolve(path);const layout=persistentStateLayoutV1(root),reserved=[join(root,"persistent"),layout.binding,layout.intent,layout.binding+".new",layout.intent+".new"],contains=(parent:string,child:string)=>{const suffix=relative(parent.toLowerCase(),child.toLowerCase());return !suffix||!isAbsolute(suffix)&&suffix!==".."&&!suffix.startsWith(".."+sep);};if(reserved.some(item=>contains(item,path)||contains(path,item)))throw Error("Persistent state is excluded from recomputable artifacts");return path;}
