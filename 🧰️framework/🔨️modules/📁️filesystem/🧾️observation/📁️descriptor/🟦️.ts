import {constants} from "node:fs";
/** 📁️ Opens a final component without following links where the platform exposes that flag. */
export function artifactReadFlagsV1():number{return constants.O_RDONLY|(constants.O_NOFOLLOW??0);}
/** 🔒️ Retains an accepted descriptor through its body and asynchronous close. */
export async function withArtifactDescriptorV1<T>(descriptor:{close():Promise<void>},body:()=>Promise<T>):Promise<T>{let outcome:{value:T}|{error:unknown};try{outcome={value:await body()};}catch(error){outcome={error};}try{await descriptor.close();}catch(error){if("value"in outcome)throw error;}if("error"in outcome)throw outcome.error;return outcome.value;}
