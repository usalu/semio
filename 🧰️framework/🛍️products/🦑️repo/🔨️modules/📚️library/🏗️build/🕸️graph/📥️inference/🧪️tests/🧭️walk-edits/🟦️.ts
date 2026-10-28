/** 🧭️ Contract: an edit that changes what a command walk loads can never leave the observations of the walk standing. For real edits of a fixture workspace (a comment, an import of a reached file, a transitive import of a new file, the exports of a workspace package, the paths of a tsconfig, files outside the closure) the observations the walk recorded hold exactly when the walk answers the same way a fresh process would; whenever the roster moves the observations are stale, and the roster always equals what esbuild's bundle graph loads. The recording and the check run in separate processes, as the inference and the generation verifier do, because a process caches the package manifests it read. The scenario lives in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const library=fileURLToPath(new URL("../../",import.meta.url)),build=join(library,"../../../../../");
const phaseSource=`
const mode=process.env.WALK_MODE,root=process.env.WALK_ROOT,rows=process.env.WALK_ROWS,roster=process.env.WALK_ROSTER,library=process.env.WALK_LIBRARY;
const {readFileSync,writeFileSync}=await import("node:fs"),{join}=await import("node:path");
const inference=await import(join(library,"🟨️.mjs"));await inference.libraryBootstrap;
const observation=await import(join(library,"../../👁️observation/🟨️.mjs")),scriptInputs=await import(join(library,"🧭️script-inputs/🟨️.mjs"));
if(mode==="compute"){
 const seen=await observation.observeDetachedV1(async()=>(await scriptInputs.relativeScriptInputs([join(root,process.env.WALK_ENTRY)],root,inference.createScriptInputCache({workspaceRoot:root}))).map(path=>path.replace("{workspaceRoot}/","")).sort());
 writeFileSync(rows,JSON.stringify({rows:seen.rows,layers:seen.layers.map(layer=>[...layer])}));writeFileSync(roster,JSON.stringify(seen.value));
}else{
 inference.prepareSourceAnalysesV1();const {rows:recorded,layers}=JSON.parse(readFileSync(rows,"utf8"));
 writeFileSync(roster,JSON.stringify({holds:observation.observationRowsHoldV1(recorded)&&layers.every(layer=>observation.observationLayerHoldsV1(layer))}));
}`;
const phase=(mode:string,root:string,rows:string,roster:string)=>{
 const result=spawnSync(process.execPath,["-e",phaseSource],{encoding:"utf8",env:{...process.env,WALK_MODE:mode,WALK_ROOT:root,WALK_ROWS:rows,WALK_ROSTER:roster,WALK_LIBRARY:library,WALK_ENTRY:corpus.entry}});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(readFileSync(roster,"utf8"));
};
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const configs=(paths:readonly string[])=>paths.filter(path=>!path.endsWith("package.json")&&!path.endsWith("tsconfig.json"));
const esbuildOracle=async(root:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,corpus.entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map(path=>path.replace(/^\.\//u,"")).sort();};

test("walk edit fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);expect(build.length).toBeGreaterThan(0);});

for(const step of corpus.steps)test(`the observations of a command walk hold exactly when a fresh walk answers the same: ${step.name}`,async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"walk-edits-")),scratch=mkdtempSync(join(realpathSync(tmpdir()),"walk-edits-rows-"));
 try{
  write(root,corpus.files);for(const[path,target]of Object.entries(corpus.links)){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(target,join(root,path));}
  const rows=join(scratch,"rows.json"),before=phase("compute",root,rows,join(scratch,"before.json")) as string[];
  write(root,step.edit);
  const holds=(phase("check",root,rows,join(scratch,"holds.json")) as {holds:boolean}).holds,after=phase("compute",root,join(scratch,"rows-after.json"),join(scratch,"after.json")) as string[];
  const moved=JSON.stringify(before)!==JSON.stringify(after);
  expect(moved,"the roster of a fresh walk").toBe(step.rosterMoved);
  expect(holds,"the recorded observations").toBe(step.rowsHold);
  if(moved)expect(holds,"a moved roster must have stale observations").toBe(false);
  expect(configs(after)).toEqual(await esbuildOracle(root));
 }finally{rmSync(root,{recursive:true,force:true});rmSync(scratch,{recursive:true,force:true});}
},120000);
