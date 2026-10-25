/** 🪢️ Contract: the repository's own Cargo profile policy renders `trim-paths="object"` into every profile, the direct rlib mapper refuses a profile that trims paths, so every workspace library is a typed `effective-profile` unit and every project that declares a bounded Cargo fallback must declare that adapter too. Independent oracle: `git ls-files` lists the declarations, `JSON.parse` reads them. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{renderCargoProfileConfigurationV1}from"../../../../../🎚️mode/🦀️profile/🟦️.ts";
import{projectCargoRlibProfileV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🕸️units/📐️profile/🟦️.ts";
import{readRustBuildIntentV1}from"../../🟦️.ts";

const profile={name:"prototype",opt_level:"0",lto:"off",codegen_backend:null,codegen_units:256,debuginfo:1,split_debuginfo:"unpacked",debug_assertions:true,overflow_checks:true,rpath:false,incremental:true,panic:"unwind" as const,strip:{resolved:"None" as const}},context={edition:"2021",kind:"library",operation:"build",roles:["target"],triple:"aarch64-apple-darwin"};

test("the repository policy trims object paths and the direct rlib mapper refuses exactly that",()=>{
 const rendered=renderCargoProfileConfigurationV1({version:1,mode:"prototype",target:"aarch64-apple-darwin",producer:"artifact",operation:"build"},"🧰️framework/Cargo.toml","isolated");
 expect(rendered.selectors).toContain('profile.prototype.trim-paths="object"');
 expect(projectCargoRlibProfileV1({...profile,trim_paths:[]},context).arguments.length).toBeGreaterThan(0);
 expect(()=>projectCargoRlibProfileV1({...profile,trim_paths:["object"]},context)).toThrow("direct rlib adapter unavailable");
});

test("every declared bounded Cargo fallback admits the typed effective-profile refusal of a trimmed workspace library",()=>{
 const listed=Bun.spawnSync(["git","ls-files","--","*📋️project.json"],{cwd:process.cwd()}).stdout.toString().split("\n").filter(Boolean),declared:string[]=[];
 for(const path of listed){let text:string;try{text=readFileSync(path,"utf8");}catch{continue;}if(!text.includes('"bounded-cargo"'))continue;
  const project=JSON.parse(text) as {targets?:Record<string,{metadata?:{semio?:{rustBuildIntent?:unknown}}}>};
  for(const target of Object.values(project.targets??{})){const row=target.metadata?.semio?.rustBuildIntent;if(row===undefined)continue;const intent=readRustBuildIntentV1(row);if(intent.fallback){declared.push(path);expect(intent.fallback.adapters,path).toContain("effective-profile");}}
 }
 expect(declared.length).toBeGreaterThanOrEqual(24);
});
