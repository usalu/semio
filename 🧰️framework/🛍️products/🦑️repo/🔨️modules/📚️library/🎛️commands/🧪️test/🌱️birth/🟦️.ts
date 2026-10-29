import schema from "./🧬️schema/🔣️.json" with {type:"json"};
const protocol=schema.$defs.Protocol.const;

/** 🎯️ Selects only the closed infrastructure request whose initial issuer may declare Native build resources. */
export function rootTestBuildArgumentsV1(argv:readonly string[],declared:readonly string[]):readonly string[]|null{return argv.length===protocol.operands&&protocol.prefix.every((word,index)=>argv[index]===word)&&declared.includes(argv[protocol.prefix.length]!)?Object.freeze([...argv]):null;}

/** 🌱️ Designates owned providers and builds fresh capability DATA before issuance; received Originals never enter this factory. */
export async function rootTestFreshCapabilitiesV1(root:string,argv:readonly string[],environment:Record<string,string|undefined>):Promise<Readonly<Record<string,unknown>>>{
 const{readCachingTestTableV1}=await import("../../../⚡️caching/🗂️suites/🟦️.ts"),arguments_=rootTestBuildArgumentsV1(argv,readCachingTestTableV1().codeSuites);
 if(!arguments_)return{workspaceRoot:root};
 const{issueRepositoryScriptProvidersV1}=await import("../../../🏃️process/🔌️providers/🟦️.ts");issueRepositoryScriptProvidersV1(environment);
 const{createBuildBootstrapCapabilitiesV1}=await import("../../../🏗️build/📥️caller/📤️child/🟦️.ts");return{...await createBuildBootstrapCapabilitiesV1(root,arguments_,environment)};
}

