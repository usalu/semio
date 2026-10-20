import{isAbsolute,join,resolve}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{cargoConfigurationArgumentsV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import{nativeBuildPassArgsV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import{renderCargoProfileConfigurationV1,appendCargoProfileSelectorsV1}from"../../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import profileSchema from"../../../../🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json"with{type:"json"};
import{descriptorEmitterCargoArgumentsV1}from"../../../../../../../💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🖨️describe/🛠️emitter/📋️contract/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
export interface RuntimeActorCargoArgumentsRequestV1{readonly version:1;readonly kind:"component"|"descriptor-emitter";readonly mode:"release";readonly root:string;readonly workspaceOwner:string;readonly workspaceKind:"repository"|"isolated";readonly packageManifest:string;readonly packageName:string;readonly buildDirectory:string}
const hosts=Object.freeze(profileSchema.definitions.Request.properties.target.enum.filter(value=>!value.startsWith("wasm")));
/** 🎭️ Recomputes current compiler argv as finite DATA; physical callers must retain actual Source and artifact owners. */
export function runtimeActorCargoArgumentsV1(value:unknown):readonly string[]{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed actual actor compiler expectation required");const row=value as RuntimeActorCargoArgumentsRequestV1;if([row.root,row.packageManifest,row.buildDirectory].some(path=>!isAbsolute(path)||resolve(path)!==path)||isAbsolute(row.workspaceOwner)||row.workspaceOwner.split(/[\\/]/u).some(part=>!part||part==="."||part==="..")||!row.workspaceOwner.endsWith("Cargo.toml"))throw Error("Canonical actor compiler Source and writer coordinates required");
 const target=row.kind==="component"?"wasm32-wasip2":hosts[0]!,producer=row.kind==="component"?"artifact":"descriptor-emitter",rendered=renderCargoProfileConfigurationV1({version:1,mode:row.mode,target,producer,operation:"build"},row.workspaceOwner,row.workspaceKind);
 if(row.kind==="descriptor-emitter")for(const host of hosts){const candidate=renderCargoProfileConfigurationV1({version:1,mode:row.mode,target:host,producer,operation:"build"},row.workspaceOwner,row.workspaceKind);if(JSON.stringify(candidate.selectors)!==JSON.stringify(rendered.selectors))throw Error("Observed compiler host required for target-dependent descriptor policy");}
 const baseline=row.kind==="component"?nativeBuildPassArgsV1({id:"expected",manifest:join(row.root,row.workspaceOwner),packages:[{name:row.packageName,manifest:row.packageManifest}],target:"wasm32-wasip2",profile:row.mode}):["build","--locked","--manifest-path",row.packageManifest,...descriptorEmitterCargoArgumentsV1(row.mode)],configured=cargoConfigurationArgumentsV1(baseline,join(row.root,".cargo/config.toml"),row.buildDirectory),arguments_=appendCargoProfileSelectorsV1(configured,rendered);
 return Object.freeze([...arguments_,...(row.kind==="descriptor-emitter"?["--message-format=json-render-diagnostics"]:[])]);
}
