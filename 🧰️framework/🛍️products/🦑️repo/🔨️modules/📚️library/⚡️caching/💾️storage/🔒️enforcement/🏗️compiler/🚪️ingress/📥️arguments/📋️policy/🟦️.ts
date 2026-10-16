import {admitRepositoryCargoForwardingV1} from "../📤️forwarding/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";

/** 📋️ Names the exact configuration and writer paths already issued to the Cargo caller. */
export interface RepositoryCargoStorageArgumentsV1{readonly version:1;readonly compilerConfiguration:string;readonly buildDirectory:string;readonly targetDirectory:string}
/** 🚪️ Refuses storage overrides before Cargo can resolve or fetch into an unowned writer namespace. */
export function repositoryCargoStorageArgumentsV1(args:readonly string[],context:RepositoryCargoStorageArgumentsV1):readonly string[]{
 const protocol=schema.definitions.Protocol.const;if(validateJsonSchemaSubset(schema.definitions.Context,context).length||args.length>protocol.maximumArguments||args.some(arg=>typeof arg!=="string"||arg.length>4096||arg.includes("\0")))throw Error("Original Cargo storage argument declaration required");
 admitRepositoryCargoForwardingV1(args);const delimiter=args.indexOf(protocol.delimiter),head=delimiter<0?args:args.slice(0,delimiter),configurations=[context.compilerConfiguration,protocol.buildKey+"="+JSON.stringify(context.buildDirectory),protocol.targetKey+"="+JSON.stringify(context.targetDirectory)];
 for(let index=0;index<head.length;index++){const arg=head[index]!;if(protocol.forbidden.some(flag=>arg===flag||arg.startsWith(flag+"=")))throw Error("Cargo generated publication belongs to original repository admission");const target=arg===protocol.target?head[++index]:arg.startsWith(protocol.target+"=")?arg.slice(protocol.target.length+1):undefined;if(arg===protocol.target||arg.startsWith(protocol.target+"=")){if(target!==context.targetDirectory)throw Error("Cargo target storage belongs to original repository admission");continue;}const configuration=arg===protocol.configuration?head[++index]:arg.startsWith(protocol.configuration+"=")?arg.slice(protocol.configuration.length+1):undefined;if(arg===protocol.configuration||arg.startsWith(protocol.configuration+"=")){if(!configuration||!configurations.includes(configuration))throw Error("Cargo configuration belongs to original repository admission");}}
 return Object.freeze([...args]);
}

