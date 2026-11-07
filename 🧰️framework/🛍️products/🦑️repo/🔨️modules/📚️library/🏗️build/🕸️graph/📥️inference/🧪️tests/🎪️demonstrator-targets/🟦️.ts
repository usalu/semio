/** 🎪️ Contract: every playground target the mit-bestand demonstrator depends on is produced by a declared playground row, and every tracked Cargo manifest parses, because one unparsable manifest withholds the whole `@semio-tech/framework-os-dev` preparation family. The independent oracle is the third-party `@iarna/toml` parser. */
import{expect,test}from"bun:test";
import{execFileSync}from"node:child_process";
import{existsSync,readFileSync}from"node:fs";
import{join}from"node:path";
import iarna from"@iarna/toml";

const workspace=join(import.meta.dir,"../../../../../../../../../..");
const tracked=(glob:string)=>execFileSync("git",["ls-files","-z","--",glob],{cwd:workspace,maxBuffer:1<<28}).toString().split("\0").filter(path=>path&&!path.includes(".🧬semio")&&existsSync(join(workspace,path)));
const demonstrator=JSON.parse(readFileSync(join(workspace,"♻️mit-bestand/🧺️demonstrator/📋️project.json"),"utf8"));
const wasmProducers=["@semio-tech/framework-surface-rs","@semio-tech/framework-editor-rs","@semio-tech/framework-os-node-graph-rs","@semio-tech/puzzle-plugin"];
const variantsOf=(profile:string)=>(demonstrator.targets[`prepare-${profile}`].dependsOn as string[]).flatMap(id=>{const match=/^@semio-tech\/framework-os-dev:prepare-(.+)-react-(prototype|release)$/u.exec(id);return match?.[2]===profile?[match[1]]:[];}).sort();
const rowsOf=(manifests:string[])=>manifests.flatMap(path=>{const metadata=(Bun.TOML.parse(readFileSync(join(workspace,path),"utf8")) as any).package?.metadata?.semio;return(metadata?.playground??[]).map((row:{variant:string})=>row.variant);}).sort();

test("every tracked Cargo manifest parses",()=>{
 const manifests=tracked("*Cargo.toml"),failures:string[]=[];
 for(const path of manifests)try{Bun.TOML.parse(readFileSync(join(workspace,path),"utf8"));}catch(error){failures.push(`${path}: ${(error as Error).message}`);}
 expect(failures).toEqual([]);
 const oracle=manifests.filter(path=>{try{iarna.parse(readFileSync(join(workspace,path),"utf8"));return false;}catch{return true;}});
 expect(oracle).toEqual([]);
});

test("the demonstrator prototype and release preparation name the same declared playground rows",()=>{
 const declared=rowsOf(tracked("🌎️hub/*Cargo.toml")),prototype=variantsOf("prototype");
 expect(prototype).toEqual(["aggregator","aussuchen","bearbeiten","energy","fem3d","generation3d","generator","koordinator","verfolgen"]);
 expect(variantsOf("release")).toEqual(prototype);
 expect(prototype.filter(variant=>!declared.includes(variant))).toEqual([]);
 const oracle=tracked("🌎️hub/*Cargo.toml").flatMap(path=>((iarna.parse(readFileSync(join(workspace,path),"utf8")) as any).package?.metadata?.semio?.playground??[]).map((row:{variant:string})=>row.variant)).sort();
 expect(oracle).toEqual(declared);
});

test("the activation lanes depend on activate targets of declared rows and the wasm producers are authored",()=>{
 const lanes=(demonstrator.targets["activate-lanes-prototype"].dependsOn as string[]).map(id=>/^@semio-tech\/framework-os-dev:activate-(.+)-react-prototype$/u.exec(id)?.[1]);
 expect(lanes.sort()).toEqual(["energy","fem3d","generator"]);
 const authored=new Map<string,string[]>();
 for(const path of tracked("*📋️project.json")){const project=JSON.parse(readFileSync(join(workspace,path),"utf8"));authored.set(project.name,Object.keys(project.targets??{}));}
 for(const producer of wasmProducers)expect(authored.get(producer)).toContain("wasm");
 for(const profile of["prototype","release"])for(const producer of wasmProducers)expect(demonstrator.targets[`prepare-${profile}`].dependsOn).toContain(`${producer}:wasm`);
});
