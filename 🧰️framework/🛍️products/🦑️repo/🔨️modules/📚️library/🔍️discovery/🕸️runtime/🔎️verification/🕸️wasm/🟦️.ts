import{isAbsolute,relative,resolve}from"node:path";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{renderCargoProfileConfigurationV1}from"../../../../🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import type{RuntimeWasmObservationV1,RuntimeWasmRostersV1,RuntimeGraphFindingV1}from"../../🧬️schema/🟦️.ts";
/** 📋️ Bounds complete physical browser output inventories and their publication marker. */
export const runtimeWasmProtocolV1=Object.freeze({...schema.$defs.RuntimeWasmProtocolV1.const});
const same=(left:unknown,right:unknown)=>JSON.stringify(left)===JSON.stringify(right);
const local=(root:string,path:string):string|undefined=>{const value=relative(root,path);return isAbsolute(path)&&path.length<=256&&value&&!isAbsolute(value)&&value!==".."&&!value.startsWith(".."+(process.platform==="win32"?"\\":"/"))?value.replaceAll("\\","/"):undefined;};
/** 🧬️ Admits only the closed current-format completed browser receipt. */
export function runtimeWasmObservationAdmittedV1(value:unknown):value is RuntimeWasmObservationV1{return validateJsonSchemaSubset(schema.$defs.RuntimeWasmObservationV1,value,schema).length===0;}
/** 🧾️ Matches the exact charged publication owner and complete canonical file marker. */
export function runtimeWasmPublicationMatchesV1(value:unknown,mode:"prototype"|"release",directory:string,outputs:readonly Readonly<{path:string;sha256:string}>[]):boolean{
 if(validateJsonSchemaSubset(schema.$defs.RuntimeWasmPublicationMarkerV1,value,schema).length)return false;const row=value as{owner:string;files:readonly string[]},paths=outputs.map(output=>local(directory,output.path));return row.owner===runtimeWasmProtocolV1.publicationOwnerPrefix+mode&&paths.every(path=>path!==undefined)&&same([...row.files].sort(),paths.sort());
}
/** 🎚️ Matches the actual single mode and complete canonical selected-profile selector tuple. */
export function runtimeWasmCargoProfileV1(args:readonly string[],mode:"prototype"|"release",owner:string,kind:"repository"|"isolated"):boolean{
 if(args[0]!=="build"||args.includes("--")||args.includes("--release"))return false;
 const profiles:string[]=[],selectors:string[]=[];
 for(let index=1;index<args.length;index++){const value=args[index]!;if(value==="--profile"||value.startsWith("--profile="))profiles.push(value==="--profile"?args[++index]??"":value.slice(10));else if(value==="--config"||value.startsWith("--config=")){const selected=value==="--config"?args[++index]??"":value.slice(9);if(/^profile(?:\.|\s|=|$)/u.test(selected))selectors.push(selected);}}
 return same(profiles,[mode])&&same(selectors,renderCargoProfileConfigurationV1({version:1,mode,target:"wasm32-unknown-unknown",producer:"browser-canvas",operation:"build"},owner,kind).selectors);
}
/** 🔏️ Binds every persistent transform and complete mounted file to the exact completed byte chain. */
export function runtimeWasmTransformationsV1(receipt:RuntimeWasmObservationV1,name:string,versions:Readonly<{bindgen:string;binaryen:string}>,digest:(path:string)=>string|undefined,rosters:RuntimeWasmRostersV1):readonly RuntimeGraphFindingV1[]{
 const findings:RuntimeGraphFindingV1[]=[],fail=(path:string,detail:string)=>findings.push({code:"runtime-input-mismatch",path,detail});
 if(!runtimeWasmObservationAdmittedV1(receipt)){fail("","Closed Wasm observation required");return findings;}
 if(validateJsonSchemaSubset(schema.$defs.RuntimeWasmRostersV1,rosters,schema).length){fail(receipt.outputDirectory,"Complete physical Wasm inventories required");return findings;}
 if(!/^[A-Za-z0-9_-]+$/u.test(name)){fail(receipt.manifest,"Exact Cargo package name required");return findings;}
 const stages=receipt.transformations,binding=stages[0],optimizer=stages[1],kinds=receipt.profile==="release"?["wasm-bindgen","binaryen"]:["wasm-bindgen"];
 if(!same(stages.map(stage=>stage.kind),kinds)){fail(receipt.manifest,"Wasm transformation order differs from selected profile");return findings;}
 const check=(file:Readonly<{path:string;sha256:string}>)=>{const actual=isAbsolute(file.path)&&file.path.length<=256?digest(file.path):undefined;if(actual!==file.sha256)fail(file.path,"Persistent Wasm bytes differ from their completed observation");};
 check(receipt.raw);for(const stage of stages){check(stage.tool);check(stage.input);for(const output of stage.outputs)check(output);if(new Set(stage.outputs.map(output=>resolve(output.path))).size!==stage.outputs.length)fail(stage.input.path,"Duplicate transformation outputs");}
 const bindingDirectory=binding!.arguments.at(-1)??"",raw=receipt.raw;
 if(!same(binding!.input,raw)||binding!.tool.version!=="wasm-bindgen "+versions.bindgen||!same(binding!.arguments,[raw.path,"--target","web","--no-typescript","--out-name",name,"--out-dir",bindingDirectory])||!isAbsolute(bindingDirectory))fail(raw.path,"Binding generator selection or input differs from canonical Cargo output");
 const bound=new Map<string,{path:string;sha256:string}>();for(const output of binding!.outputs){const path=local(bindingDirectory,output.path);if(!path||path!==name+".js"&&path!==name+"_bg.wasm"&&!path.startsWith("snippets/"))fail(output.path,"Binding output is outside the complete canonical package");else bound.set(path,output);}
 if(!bound.has(name+".js")||!bound.has(name+"_bg.wasm"))fail(bindingDirectory,"Canonical binding JavaScript and Wasm are required");
 if(optimizer){const input=bound.get(name+"_bg.wasm"),output=optimizer.outputs[0];if(!input||!output||optimizer.outputs.length!==1||!same(optimizer.input,input)||!same(optimizer.arguments,[input.path,"-Oz","-o",output.path])||optimizer.tool.version!=="wasm-opt version "+versions.binaryen+" (version_"+versions.binaryen+")"||resolve(output.path)===resolve(input.path))fail(optimizer.input.path,"Optimizer must consume retained binding Wasm and publish separate selected bytes");if(output)bound.set(name+"_bg.wasm",output);}
 if(!same([...rosters.binding].sort(),binding!.outputs.map(output=>output.path).sort())||!same([...rosters.published].sort(),receipt.outputs.map(output=>output.path).sort()))fail(receipt.outputDirectory,"Observed physical Wasm inventories differ from the completed receipt");
 const published=new Map<string,string>();for(const output of receipt.outputs){check(output);const path=local(receipt.outputDirectory,output.path);if(!path||published.has(path))fail(output.path,"Published Wasm file escapes its exact mount or is duplicated");else published.set(path,output.sha256);}
 if(published.size!==bound.size||[...bound].some(([path,file])=>published.get(path)!==file.sha256))fail(receipt.outputDirectory,"Mounted package is not the complete final transformation output");
 return findings;
}
