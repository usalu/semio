import {readFileSync} from "node:fs";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
/** 🗺️ Declares exact manifests whose distinct assertion policies accompany one owning command. */
export type NativeOwnerTestManifestRequestV1=Readonly<{version:1;manifest:string;cwd:string;testManifests:readonly string[];command:string;args:readonly string[]}>;
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🗺️owner-test-manifests/🔣️.json",import.meta.url),"utf8"));
/** 🚪️ Admits closed owner arguments before repository preparation or policy discovery. */
export function nativeOwnerTestManifestRequestV1(args:readonly string[]):NativeOwnerTestManifestRequestV1{
 if(args[0]!=="--manifest"||args[2]!=="--cwd")throw Error("Expected exact native owner manifest and working directory");
 const testManifests:string[]=[];let boundary=4;
 while(args[boundary]==="--test-manifest"){const path=args[boundary+1];if(!path||path.startsWith("--"))throw Error("Expected explicit test manifest");testManifests.push(path);boundary+=2;}
 if(args[boundary]!=="--")throw Error("Unknown native owner option or missing command boundary");
 const request={version:1,manifest:args[1],cwd:args[3],testManifests,command:args[boundary+1],args:args.slice(boundary+2)};
 const errors=validateJsonSchemaSubset(schema,request);if(errors.length)throw Error(`Invalid native owner test manifest request: ${errors.join("; ")}`);
 return request as NativeOwnerTestManifestRequestV1;
}
