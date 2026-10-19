import {nxSdkCompilerEnvironmentV1} from "../../📦️sdk/📥️source/🔗️derivative/🎛️environment/🟦️.ts";
import {createHash} from "node:crypto";
import {lstat,readFile} from "node:fs/promises";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import contract from "./🔣️contract.json";
import schema from "./🧬️schema/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 📥️ Executes the exact frozen production module in Node before it can become the active toolkit. */
export async function preflightNxAuthorizersV1(directory:string,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Promise<void>{
 checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,contract).length)throw Error("Closed authorizer runtime contract required");const path=join(directory,"semio-authorizers.mjs"),before=await lstat(path,{bigint:true});if(!before.isFile()||before.isSymbolicLink()||before.size>8_388_608n)throw Error("Original bounded authorizer module required");const hash=()=>readFile(path).then(bytes=>createHash("sha256").update(bytes).digest("hex")),digest=await hash(),probe='const[url,encoded]=process.argv.slice(1),required=JSON.parse(encoded),body=await import(url);for(const name of required)if(typeof body[name]!=="function")throw Error("Required immutable authorizer function missing: "+name);console.log(JSON.stringify({version:1,functions:required}));',output=await captureOwnedCommandV1("node",["--input-type=module","--eval",probe,pathToFileURL(path).href,JSON.stringify(contract.functions)],directory,{control,env:nxSdkCompilerEnvironmentV1(environment)}),row=JSON.parse(output.trim().split(/\r?\n/u).reverse().find(line=>line.startsWith('{"version":1'))??"null"),after=await lstat(path,{bigint:true});
 checkResourceLeaseControl(control);if(validateJsonSchemaSubset(schema,row).length||JSON.stringify(row)!==JSON.stringify(contract)||before.dev!==after.dev||before.ino!==after.ino||before.size!==after.size||before.mtimeNs!==after.mtimeNs||before.ctimeNs!==after.ctimeNs||await hash()!==digest)throw Error("Original immutable authorizer preflight changed");
}
