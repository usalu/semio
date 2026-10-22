/** 🔢️ How many file operations of the generation documents pass between two physical identity rechecks of the store. */
export const generationRecheckWindowV1=64;

type FilePortV1={read(path:string):Promise<unknown>;write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}):Promise<unknown>;names(directory:string):Promise<unknown>;list(directory:string):Promise<unknown>;remove(path:string):Promise<unknown>};

/** 🗂️ Wraps a file port so every path stays inside the original store root and the store is rechecked once per window of operations; at most one recheck is in flight and operations that fall on the same window join it instead of starting another. The recheck itself is the store's constant-time identity check. */
export function createGenerationFilesV1<Base extends FilePortV1>(input:Readonly<{store:Readonly<{root:string;recheck:()=>Promise<void>}>;base:Base;requirePath:(root:string,path:string)=>string;window?:number}>):Readonly<{[Key in keyof FilePortV1]:Base[Key]}>{
 const{store,base,requirePath}=input,window=input.window??generationRecheckWindowV1;let operations=0,rechecking:Promise<void>|undefined;
 const recheck=()=>rechecking??=store.recheck().finally(()=>{rechecking=undefined;});
 const admit=async(path:string)=>{if(++operations%window===0)await recheck();return requirePath(store.root,path);};
 return Object.freeze({read:async(path:string)=>base.read(await admit(path)),write:async(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean})=>base.write(await admit(path),bytes,options),names:async(directory:string)=>base.names(await admit(directory)),list:async(directory:string)=>base.list(await admit(directory)),remove:async(path:string)=>base.remove(await admit(path))}) as Readonly<{[Key in keyof FilePortV1]:Base[Key]}>;
}
