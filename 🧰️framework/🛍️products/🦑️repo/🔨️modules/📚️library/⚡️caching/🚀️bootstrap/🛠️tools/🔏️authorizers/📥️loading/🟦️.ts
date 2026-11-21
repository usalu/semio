import {nxSdkCompilerEnvironmentV1} from "../../📦️sdk/📥️source/🔗️derivative/🎛️environment/🟦️.ts";
import {join} from "node:path";
import {pathToFileURL} from "node:url";
import contract from "./🔣️contract.json" with { type: "json" };
import {observeNxAuthorizerBundleV1} from "../📤️receipt/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 📥️ Executes the exact frozen production module in Node before it can become the active toolkit. */
export async function preflightNxAuthorizersV1(directory:string,control:ResourceLeaseControl,environment:Readonly<Record<string,string|undefined>>):Promise<void>{
 const check=()=>checkResourceLeaseControl(control);check();if(validateJsonSchemaSubset(schema,contract).length)throw Error("Closed authorizer runtime contract required");
 const path=join(directory,"semio-authorizers.mjs"),before=observeNxAuthorizerBundleV1(directory,check),probe='const[url,encoded]=process.argv.slice(1),required=JSON.parse(encoded),body=await import(url);for(const name of required)if(typeof body[name]!=="function")throw Error("Required immutable authorizer function missing: "+name);console.log(JSON.stringify({version:1,functions:required}));',output=await captureOwnedCommandV1("node",["--input-type=module","--eval",probe,pathToFileURL(path).href,JSON.stringify(contract.functions)],directory,{control,env:nxSdkCompilerEnvironmentV1(environment)}),row=JSON.parse(output.trim().split(/\r?\n/u).reverse().find(line=>line.startsWith('{"version":1'))??"null");
 const after=observeNxAuthorizerBundleV1(directory,check);if(validateJsonSchemaSubset(schema,row).length||JSON.stringify(row)!==JSON.stringify(contract)||before.stamp!==after.stamp||before.sha256!==after.sha256)throw Error("Original immutable authorizer preflight changed");
}
