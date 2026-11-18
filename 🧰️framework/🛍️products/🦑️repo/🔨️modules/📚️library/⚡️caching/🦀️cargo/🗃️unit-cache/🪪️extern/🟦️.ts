import {readFileSync,renameSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {digestFileV1} from "../💾️store/🟦️.ts";
import {sha256V1} from "../🔏️key/🟦️.ts";

const DYNAMIC=/\.(?:dylib|so|dll)$/u;
/** 🪪️ The file that names the unit a dynamic library was built from, beside the library. */
export const externIdentityFileV1=(path:string):string=>path+".semio-unit";

/** 🪪️ The identity a dynamic extern (a proc-macro library) enters its dependants' keys by: the digest of the portable key of the unit that built it together with the crate hash (SVH) the compiler recorded in the library, instead of the digest of its bytes. A linked Mach-O, ELF or PE file is not guaranteed to be the same bytes on two machines (linker version, build id), but its behaviour is a function of the unit's inputs; keyed by those inputs, every machine computes the same key for a dependant whichever copy of the library it holds. The crate hash is part of the identity because every dependant's metadata records it: a dependant restored for a library of another crate hash cannot be loaded (`E0463 can't find crate`), so two builds of one unit that differ in their crate hash are two identities. Read from the file the wrapper wrote beside the library and trusted only while it still names the library's current bytes; `undefined` (the bytes count) for any other extern, a library the wrapper did not build and a library whose crate hash is unknown. */
export function externIdentityV1(path:string,sha256:string,stamps:string|undefined):string|undefined{
 if(!DYNAMIC.test(path))return undefined;
 try{const row=JSON.parse(readFileSync(externIdentityFileV1(path),"utf8")) as {version?:number;sha256?:string;identity?:string;svh?:string};return row.version===2&&row.sha256===sha256&&typeof row.identity==="string"&&/^[0-9a-f]{64}$/u.test(row.identity)&&typeof row.svh==="string"&&/^[0-9a-f]{32}$/u.test(row.svh)?"unit:"+row.identity+":"+row.svh:undefined;}catch{return undefined;}
}

/** 🪪️ Writes the identity files of the dynamic libraries among `outputs` (best effort, atomic); `crateHashOf` answers the crate hash the compiler recorded in a library, a library without one gets no identity file. */
export function markExternIdentityV1(outputs:readonly string[],portableKey:string,sensitive:readonly string[],stamps:string|undefined,crateHashOf:(path:string)=>string|undefined):void{
 const identity=sha256V1(JSON.stringify(["extern-identity",portableKey,sensitive]));
 for(const path of outputs){
  if(!DYNAMIC.test(path))continue;
  try{const sha=digestFileV1(path,stamps),svh=sha===null?undefined:crateHashOf(path);if(sha===null||svh===undefined)continue;const file=externIdentityFileV1(path),temporary=file+"."+randomBytes(3).toString("hex");writeFileSync(temporary,JSON.stringify({version:2,sha256:sha,identity,svh}));renameSync(temporary,file);}catch{}
 }
}
