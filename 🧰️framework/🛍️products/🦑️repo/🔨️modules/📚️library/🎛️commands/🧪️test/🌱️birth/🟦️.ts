import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {playgroundActivationTargetV1} from "../../../../../../💻️os/🔨️modules/🧑‍💻dev/♻️activation/🎯️target/🟦️.ts";
import {loadFrameworkOsPlaygroundCatalog} from "../../../🎮️playground/📚️catalog/🟦️.ts";
const protocol=schema.$defs.Protocol.const;

/** 🎯️ Selects only the closed infrastructure request whose initial issuer may declare Native build resources. */
export function rootTestBuildArgumentsV1(argv:readonly string[],declared:readonly string[],environment:Record<string,string|undefined>={}):readonly string[]|null{
 const testing=argv.length===protocol.operands&&protocol.prefix.every((word,index)=>argv[index]===word)&&declared.includes(argv[protocol.prefix.length]!);
 const schema=argv.length>=protocol.schemaMinimumOperands&&protocol.schemaPrefix.every((word,index)=>argv[index]===word)&&protocol.schemaCommands.includes(argv[protocol.schemaPrefix.length]!)&&!argv.some(word=>protocol.helpFlags.includes(word));
 const native=protocol.nativeRequests.some(request=>(request.exact?argv.length===request.prefix.length:argv.length>=request.prefix.length)&&request.prefix.every((word,index)=>argv[index]===word))&&!argv.some(word=>protocol.helpFlags.includes(word));
 const activation=protocol.activationRequest,selection=(flag:string)=>argv.filter(word=>word===flag).length===1?argv[argv.indexOf(flag)+1]:undefined,profile=environment.SEMIO_BUILD_MODE,variant=selection("--variant"),renderer=selection("--renderer");
 if(activation.prefix.every((word,index)=>argv[index]===word)&&!argv.some(word=>protocol.helpFlags.includes(word))&&variant&&(renderer==="react"||renderer==="wgpu")&&(profile==="prototype"||profile==="release")){
  const catalog=environment.SEMIO_PLUGIN_CATALOG===undefined?[]:loadFrameworkOsPlaygroundCatalog(environment.SEMIO_PLUGIN_CATALOG);
  return catalog.some(row=>row.variant===variant)?Object.freeze(["run",playgroundActivationTargetV1(variant,profile,renderer,catalog)]):null;
 }
 return testing||schema||native?Object.freeze([...argv]):null;
}

/** 🌱️ Designates owned providers and builds fresh capability DATA before issuance; received Originals never enter this factory. */
export async function rootTestFreshCapabilitiesV1(root:string,argv:readonly string[],environment:Record<string,string|undefined>):Promise<Readonly<Record<string,unknown>>>{
 const{readCachingTestTableV1}=await import("../../../⚡️caching/🗂️suites/🟦️.ts"),arguments_=rootTestBuildArgumentsV1(argv,readCachingTestTableV1().codeSuites,environment);
 if(!arguments_)return{workspaceRoot:root};
 const{issueRepositoryScriptProvidersV1}=await import("../../../🏃️process/🔌️providers/🟦️.ts");issueRepositoryScriptProvidersV1(environment);
 const{createBuildBootstrapCapabilitiesV1}=await import("../../../🏗️build/📥️caller/📤️child/🟦️.ts");return{...await createBuildBootstrapCapabilitiesV1(root,arguments_,environment)};
}

