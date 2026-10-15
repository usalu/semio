import{lstatSync}from"node:fs";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
import{checkResourceLeaseControl,requireMaterializedPathV1,type ResourceLeaseControl}from"../../🟦️.ts";
/** 🧱️ Kernel creation and rename refusal is a mechanism; the caller must still hold original backing and namespace custody. */
export async function withResourceLeaseDirectorySealV1<T>(directory:string,control:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>{
 checkResourceLeaseControl(control);directory=requireMaterializedPathV1(directory);if(process.platform!=="darwin")throw Error("Enforcing lease directory creation and rename provider unavailable on this platform");const before=lstatSync(directory);if(!before.isDirectory()||before.isSymbolicLink())throw Error("Original physical lease directory required");const invoke=(command:string,args:string[])=>captureOwnedCommandV1(command,args,directory,{control,output:{maximumBytes:4096,maximumLines:32}}),flags=()=>invoke("stat",["-f","%f",directory]);if((await flags()).trim()!=="0")throw Error("Fresh original directory flags required");await invoke("chflags",["uchg",directory]);
 try{const current=lstatSync(directory);if(current.dev!==before.dev||current.ino!==before.ino||(await flags()).trim()!=="2")throw Error("Actual immutable directory ingress refused");return await body();}finally{await invoke("chflags",["nouchg",directory]);}
}
