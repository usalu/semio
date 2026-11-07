import {access,stat} from "node:fs/promises";
import {constants} from "node:fs";
import {delimiter,dirname,join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import policy from "./🔣️policy.json" with { type: "json" };
import {buildPreflightContainerV1,buildPreflightRemediesV1} from "./💊️remedies/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {defaultNativeToolchainPortsV1,nativeToolchainHostV1,probeNativeToolV1} from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🧰️toolchain/🟦️.ts";
import {nativeDirectoryHostAvailableBytesV1,nativeDirectoryProtocolV1,nativeDirectorySystemPortV1,probeNativeDirectoryCaseCapabilityV1,probeNativeDirectoryExchangeV1,type NativeDirectoryPlatformPortV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import {checkScriptInvocation,requireIssuedScriptInvocationV1,advanceScriptInvocation,createIssuedResourceControlV1,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withRepositoryNativeToolchainV1} from "../../⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🛠️native-toolchain/🟦️.ts";

/** 🩺️ Declares the physical host and managed tool requirements of the whole workspace. */
export type BuildPreflightPlatformV1 = Readonly<{architectures:readonly("x64"|"arm64")[];minimumAvailableBytes:number;tools:readonly string[];exchange:boolean;apt?:Readonly<Record<string,readonly string[]>>}>;
/** 🔬️ Captures read-only facts from the actual invoking process and the filesystem that will hold the stores. */
export type BuildPreflightFactsV1 = Readonly<{platform:string;architecture:string;availableBytes:number;tools:readonly string[];exchange:"supported"|"unsupported"|"unprobed";caseSensitive:"yes"|"no"|"unprobed"|"fold-safe"}>;

/** 🔠️ Observes the exact journaled Windows fallback already supported by directory storage; other failures retain their identity. */
export function probeBuildPreflightCaseV1(platform:string,kind:NativeDirectoryPlatformPortV1["kind"],probe:()=>boolean):BuildPreflightFactsV1["caseSensitive"] {
 try{return probe()?"yes":"no";}catch(error){const protocol=nativeDirectoryProtocolV1();if(platform==="win32"&&kind==="journaled"&&error instanceof Error&&protocol.foldSafeRefusals.some(key=>(protocol.refusals as Readonly<Record<string,string>>)[key]===error.message))return"fold-safe";throw error;}
}
/** 🧱️ Observes physical capabilities in order without converting probe or cancellation failures. */
export function probeBuildPreflightFilesystemV1(platform:string,kind:NativeDirectoryPlatformPortV1["kind"],port:Readonly<{exchange():void;caseSensitive():boolean;check():void}>):Readonly<{exchange:"supported";caseSensitive:BuildPreflightFactsV1["caseSensitive"]}>{
 port.check();port.exchange();port.check();const caseSensitive=probeBuildPreflightCaseV1(platform,kind,port.caseSensitive);port.check();return{exchange:"supported",caseSensitive};
}

/** 🎯️ Accepts only the bare command; preflight has no scope, bypass or provisioning options. */
export function readBuildPreflightArgumentsV1(args:readonly string[]):void {
 if(args.length!==1||args[0]!=="preflight")throw Error("Expected repo preflight without arguments");
}
/** 📋️ Resolves the authored closed requirement metadata without invoking discovery or opening stores. */
export function readBuildPreflightPolicyV1():{version:1;platforms:Record<"linux"|"darwin"|"win32",BuildPreflightPlatformV1>} {
 if(validateJsonSchemaSubset(schema.definitions.Policy,policy,schema).length)throw Error("Closed build preflight policy required");return policy as never;
}
/** ⚖️ Reports every unmet authored requirement without pretending that observations grant compiler authority. */
export function buildPreflightProblemsV1(requirements:unknown,facts:unknown):string[] {
 if(validateJsonSchemaSubset(schema.definitions.Platform,requirements,schema).length)throw Error("Closed build preflight requirements required");if(validateJsonSchemaSubset(schema.definitions.Facts,facts,schema).length)throw Error("Closed build preflight facts required");
 const required=requirements as BuildPreflightPlatformV1,actual=facts as BuildPreflightFactsV1,problems:string[]=[];
 if(!(required.architectures as readonly string[]).includes(actual.architecture))problems.push(`architecture ${actual.architecture} is not one of ${required.architectures.join(", ")}`);
 if(actual.availableBytes<required.minimumAvailableBytes)problems.push(`available bytes ${actual.availableBytes} are below required ${required.minimumAvailableBytes}`);
 if(required.exchange&&actual.exchange!=="supported")problems.push("atomic or journaled directory swap is unsupported by the host or the workspace filesystem");
 if(required.exchange&&actual.caseSensitive!=="yes"&&!(actual.platform==="win32"&&actual.caseSensitive==="fold-safe"))problems.push("a case-sensitive workspace filesystem is required for directory-backed partitions");
 for(const tool of required.tools)if(!actual.tools.includes(tool))problems.push(`missing executable: ${tool}`);return problems;
}
/** 🚧️ Carries observation failures without changing cancellation or tool-scope errors. */
export class BuildPreflightRefusalV1 extends Error {
 constructor(readonly problems:readonly string[]){super("Repository preflight refused:\n"+problems.join("\n"));}
}
/** 🪜️ Admits physical host facts before entering a tool scope; observations remain DATA, never compiler authority. */
export async function runBuildPreflightPhasesV1(requirements:unknown,facts:unknown,withTools:(observe:(tools:readonly string[])=>void)=>Promise<void>,check:()=>void):Promise<void>{
 check();buildPreflightProblemsV1(requirements,facts);const required=requirements as BuildPreflightPlatformV1,observed=facts as BuildPreflightFactsV1,physical=buildPreflightProblemsV1({...required,tools:[]},observed);
 if(physical.length)throw new BuildPreflightRefusalV1(physical);check();let live=true,calls=0,accepted=false;
 try{await withTools(tools=>{if(!live||++calls!==1)throw Error("One live preflight tool observation required");check();const problems=buildPreflightProblemsV1(required,{...observed,tools:[...tools]});if(problems.length)throw new BuildPreflightRefusalV1(problems);accepted=true;});check();if(!accepted||calls!==1)throw Error("Complete scoped preflight tool observation required");}finally{live=false;}
}

/** 🔭️ Observes declared tools only through the environment whose managed custody the caller is retaining. */
async function observeBuildPreflightToolsV1(required:readonly string[],environment:Readonly<Record<string,string|undefined>>,invocation:ScriptInvocation):Promise<string[]>{
 const platform=process.platform,toolchainHost=nativeToolchainHostV1(environment),toolchainPorts=defaultNativeToolchainPortsV1(),tools:string[]=[],pathName=platform==="win32"?Object.keys(environment).find(name=>name.toLowerCase()==="path")??"PATH":"PATH",extensions=platform==="win32"?(environment.PATHEXT??".EXE").split(";"):[""];
 for(const tool of required){checkScriptInvocation(invocation);const probe=probeNativeToolV1(tool,toolchainHost,toolchainPorts);if(probe!==undefined){if(probe.found)tools.push(tool);checkScriptInvocation(invocation);continue;}search:for(const directory of(environment[pathName]??"").split(delimiter)){if(!directory)continue;for(const extension of extensions){try{const path=join(directory,tool+extension);if(!(await stat(path)).isFile())continue;await access(path,constants.X_OK);tools.push(tool);break search;}catch{}}checkScriptInvocation(invocation);}checkScriptInvocation(invocation);}
 return tools;
}
/** 🧭️ Admits physical host capacity before provisioning and observes tools under genuine managed custody. */
export async function runBuildPreflightV1(root:string,args:readonly string[],invocation:ScriptInvocation):Promise<void> {
 requireIssuedScriptInvocationV1(invocation);checkScriptInvocation(invocation);readBuildPreflightArgumentsV1(args);
 const platform=process.platform,rows=readBuildPreflightPolicyV1().platforms as Record<string,BuildPreflightPlatformV1>,required=rows[platform];
 if(!required)throw Error(`Repository preflight refused before store allocation:\nplatform ${platform} is not supported`);
 await advanceScriptInvocation(invocation,"preflight","running");
 const cache=resolve(root,".cache");let host=cache;for(;;){try{await stat(host);break;}catch{const parent=dirname(host);if(parent===host)break;host=parent;}}
 const availableBytes=nativeDirectoryHostAvailableBytesV1(host);let exchange:BuildPreflightFactsV1["exchange"]="unprobed",caseSensitive:BuildPreflightFactsV1["caseSensitive"]="unprobed";
 if(required.exchange){const port=nativeDirectorySystemPortV1(),observed=probeBuildPreflightFilesystemV1(platform,port.kind,{exchange:()=>probeNativeDirectoryExchangeV1(host,port),caseSensitive:()=>probeNativeDirectoryCaseCapabilityV1(host,port),check:()=>checkScriptInvocation(invocation)});exchange=observed.exchange;caseSensitive=observed.caseSensitive;}
 checkScriptInvocation(invocation);const facts:BuildPreflightFactsV1={platform,architecture:process.arch,availableBytes,tools:[],exchange,caseSensitive};let reportedFacts=facts;
 try{await runBuildPreflightPhasesV1(required,facts,observe=>withRepositoryNativeToolchainV1(root,invocation,createIssuedResourceControlV1(invocation,"preflight:toolchain",0),process.env,async scope=>{const tools=await observeBuildPreflightToolsV1(required.tools,scope.environment,invocation);reportedFacts={...facts,tools};observe(tools);}),()=>checkScriptInvocation(invocation));}
 catch(error){if(error instanceof BuildPreflightRefusalV1)throw Error(`Repository preflight refused:\n${[...error.problems,...buildPreflightRemediesV1(reportedFacts,required.tools,required.exchange,{container:buildPreflightContainerV1()})].join("\n")}`);throw error;}
 await advanceScriptInvocation(invocation,"preflight","complete");
}
