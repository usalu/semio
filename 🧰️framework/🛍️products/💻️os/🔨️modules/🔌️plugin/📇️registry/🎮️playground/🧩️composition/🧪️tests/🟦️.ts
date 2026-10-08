import assert from "node:assert/strict";
import {readFileSync,mkdirSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";

/** 🧭️ Joins both path implementations and actual discovery/projection to the portable authority. */
export async function provePlaygroundCompositionV1(root:string,output:string,receivers:boolean):Promise<void>{
 const require=createRequire(join(root,"package.json")),Ajv=require("ajv"),TOML=require("@iarna/toml"),directory=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition"),schema=JSON.parse(readFileSync(join(directory,"🧬️schema/🔣️.json"),"utf8")),fixture=JSON.parse(readFileSync(join(directory,"🧫️fixtures/🔣️.json"),"utf8"));
 const validate=new Ajv({strict:true}).addKeyword({keyword:"x-semio-nfc",type:"string",schemaType:"boolean",validate:(enabled:boolean,value:string)=>!enabled||value===value.normalize("NFC")}).compile(schema);
 for(const row of fixture.cases)assert.equal(validate(row.input),row.accepted,row.id+": AJV");
 const implementations=await Promise.all([import(join(directory,"🟦️.ts")),import(join(directory,"🟨️.mjs"))]);
 for(const implementation of implementations)for(const row of fixture.cases){
  let accepted=true,actual:unknown;try{actual=implementation.playgroundCompositionPathV1(row.input);}catch{accepted=false;}
  assert.equal(accepted,row.accepted,row.id+": implementation");if(accepted)assert.equal(actual,row.input,row.id+": retained path");
 }
 if(receivers){
  const {parsePlaygroundBlock}=await import(join(directory,"../🔎️discovery/🟦️.ts")),{emitPlaygroundsTypeScript}=await import(join(directory,"../../📽️projection/🟦️.ts"));
  for(const row of fixture.cases.filter((entry:any)=>entry.accepted)){
   const block='variant = "future"\nports = { react = 6061, wgpu = 6161 }\ncompositionConfigPath = '+JSON.stringify(row.input),oracle=TOML.parse(block);
   const parsed=parsePlaygroundBlock(block,"future","future/owner");assert.equal(parsed?.compositionConfigPath,oracle.compositionConfigPath,row.id+": actual discovery");
   const source=emitPlaygroundsTypeScript([parsed]);const ts=require("typescript"),vm=require("node:vm"),transpiled=ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.CommonJS}}).outputText,context={exports:{}};
   vm.runInNewContext(transpiled,context);assert.equal((context.exports as any).PLAYGROUND_BUILD_TARGETS[0].compositionConfigPath,row.input,row.id+": emitted actual catalog");
  }
  const path="future/🎨️config/🟦️.ts",workspace=join(output,"workspace"),owner="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",put=(file:string,value:string)=>{mkdirSync(dirname(join(workspace,file)),{recursive:true});writeFileSync(join(workspace,file),value);};
  for(const [directory,name]of [["🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript","renderer"],["🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust","flow"]])put(directory+"/📋️project.json",JSON.stringify({name,targets:{wasm:{},"wasm-release":{}}}));
  put(path,"export default {};");put("future/owner/🦀️.rs","");put("future/owner/Cargo.toml",'[package]\nname = "future"\nversion = "0.1.0"\nedition = "2021"\n[lib]\npath = "🦀️.rs"\n[package.metadata.component]\npackage = "semio:future"\n[package.metadata.semio]\ncomponent-kind = "plugin"\n[[package.metadata.semio.playground]]\nvariant = "future"\nports = { react = 6061, wgpu = 6161 }\ncompositionConfigPath = '+JSON.stringify(path));
  const {cacheInternals}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"));await cacheInternals.playgroundSessionTargets([],workspace);
  const targets=cacheInternals.playgroundPreparationTargets(["future/owner/Cargo.toml"],workspace,owner);
  for(const profile of ["dev","release"])for(const operation of ["serve","dev"]){
   const target=targets[operation+"-future-wgpu-"+profile];assert.equal(target.options.command,'bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve future '+profile+' --port 6161');assert.deepEqual(target.options.env,{SEMIO_WGPU_COMPOSITION_PATH:path});
   assert.deepEqual(target.dependsOn,["activate-future-wgpu-"+profile]);assert.equal(target.cache,false);assert.equal(target.continuous,true);
  }
 }
 console.log("[DEBUG] playground composition portable="+fixture.cases.length+" implementations=2 AJV=1 receiving="+receivers);
}
