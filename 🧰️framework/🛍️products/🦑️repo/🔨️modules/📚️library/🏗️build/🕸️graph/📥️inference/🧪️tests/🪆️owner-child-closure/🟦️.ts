/** 🪆️ Contract: a process-owner wrap (`repo "<driver>" owner-command --cwd <dir> -- <child>`) only forwards to its child, so the closure of the wrapped target is the closure of the driver together with the closure of the child script, exactly what esbuild and Bun's own bundler load for the two entries; a wrap without a child script keeps the driver closure alone. The scenario lives in `./🔣️.json` as language-agnostic data checked by `./🧬️schema/🔣️.json`. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),observation=await import("../../../../👁️observation/🟨️.mjs") as any,{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();
const build=(root:string)=>{for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const closureOf=async(root:string,command:string,cwd:string)=>observation.observeInferenceSnapshotV1(async()=>{
 const sources=await cacheInternals.targetScriptSources({options:{command:command.replaceAll("{ws}",root),cwd,forwardAllArgs:false,ownerArguments:true}},root,createScriptInputCache({workspaceRoot:root}),corpus.projectRoot);
 return{fallback:sources.fallback,closure:strip(sources.closure)};
});
const esbuildOracle=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map(path=>relative(root,resolve(root,path))).sort();};
const bunOracle=(root:string,entry:string)=>{const meta=join(root,"meta.json"),result=spawnSync("bun",["build","--target=bun","--outfile=/dev/null","--metafile="+meta,join(root,entry)],{cwd:root,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).map(path=>relative(root,resolve(root,path))).sort();};
const workspace=async(body:(root:string)=>Promise<void>)=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"owner-child-"));try{build(root);await body(root);}finally{rmSync(root,{recursive:true,force:true});}};

test("owner child fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("the declared closures of the driver and of the child agree with esbuild and Bun's bundler",async()=>workspace(async root=>{
 expect(await esbuildOracle(root,corpus.driver)).toEqual(corpus.driverClosure);expect(bunOracle(root,corpus.driver)).toEqual(corpus.driverClosure);
 expect(await esbuildOracle(root,corpus.child)).toEqual(corpus.childClosure);expect(bunOracle(root,corpus.child)).toEqual(corpus.childClosure);
}));

for(const wrapped of corpus.wrapped)test(`a wrapped target keys the driver and the child script: ${wrapped.name}`,async()=>workspace(async root=>{
 const expected=[...new Set([...corpus.driverClosure,...corpus.childClosure])].sort(),got=await closureOf(root,wrapped.command,wrapped.cwd);
 expect(got.fallback).toBe(false);
 expect(got.closure).toEqual(expected);
 expect(got.closure).toEqual([...new Set([...await esbuildOracle(root,corpus.driver),...await esbuildOracle(root,corpus.child)])].sort());
 expect(got.closure).not.toContain("pkg/unrelated.ts");
 const direct=await closureOf(root,`repo "./${corpus.child.split("/").pop()}" test`,corpus.projectRoot);
 expect(direct.closure).toEqual(corpus.childClosure);
}));

for(const plain of corpus.notWrapped)test(`a wrap without a child script keeps the driver closure alone: ${plain.name}`,async()=>workspace(async root=>{
 const got=await closureOf(root,plain.command,".");
 expect(got.closure).toEqual(corpus.driverClosure);
}));
