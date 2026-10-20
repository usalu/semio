/** 📐️ Contract: a target that declares `metadata.semio.closure` joins the module files its globs match, with their import closure, and the exact files the TypeScript compiler's own program reads for its declared tsconfig, to its command closure. The scenario lives in `./🔣️.json` as language-agnostic data; esbuild, Bun's bundler and `tsc --listFilesOnly` are the independent oracles. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const workspace=resolve(import.meta.dirname,"../../../../../../../../../..");
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();
const build=(root:string)=>{for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const sources=async(root:string,scenario:"modules"|"typescript")=>cacheInternals.targetScriptSources({metadata:corpus[scenario].metadata,options:{cwd:corpus.projectRoot,command:corpus.command}},root,createScriptInputCache(),corpus.projectRoot);
const digest=(root:string,files:readonly string[])=>{const hash=createHash("sha256");for(const name of files)hash.update(name).update("\0").update(createHash("sha256").update(readFileSync(join(root,name))).digest("hex")).update("\0");return hash.digest("hex");};
const esbuildOracle=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js",loader:{".json":"json"}});return Object.keys(result.metafile.inputs).map(path=>relative(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};
const bunOracle=(root:string,entry:string)=>{const meta=join(root,"meta.json"),result=spawnSync("bun",["build","--target=bun","--outfile=/dev/null","--metafile="+meta,join(root,entry)],{cwd:root,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).map(path=>relative(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};
const tscOracle=(root:string)=>{const tsc=join(workspace,"node_modules/typescript/bin/tsc"),result=spawnSync(process.execPath,[tsc,"-p",join(root,corpus.projectRoot,"tsconfig.json"),"--listFilesOnly"],{cwd:root,encoding:"utf8"});expect(result.status,result.stdout+result.stderr).toBe(0);const files=result.stdout.split("\n").filter(Boolean).map(path=>relative(root,realpathSync(path)));return{workspace:files.filter(path=>!path.startsWith("..")&&!path.split("/").includes("node_modules")).sort(),packages:[...new Set(files.flatMap(path=>{const at=path.lastIndexOf("node_modules/");return at<0?[]:[path.slice(at+13).split("/")[0]!]; }))].sort()};};
const withFixture=async(run:(root:string)=>Promise<void>)=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"declared-inputs-"));try{build(root);await run(root);}finally{rmSync(root,{recursive:true,force:true});}};

test("declared input fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("declared module globs join the matched modules and their import closure, agreeing with esbuild and Bun's bundler",()=>withFixture(async root=>{
 const got=await sources(root,"modules");
 expect(strip(got.closure)).toEqual([...corpus.modules.closure].sort());
 for(const entry of corpus.modules.oracleEntries){
  expect((await esbuildOracle(root,entry)).every(path=>strip(got.closure).includes(path))).toBe(true);
  expect(bunOracle(root,entry).every(path=>strip(got.closure).includes(path))).toBe(true);
 }
 expect(got.packages).toBeUndefined();
}));

test("a declared TypeScript project joins exactly the files and packages `tsc --listFilesOnly` reads",()=>withFixture(async root=>{
 const got=await sources(root,"typescript"),oracle=tscOracle(root);
 expect(strip(got.closure)).toEqual([...corpus.typescript.closure].sort());
 expect(got.packages).toEqual(corpus.typescript.packages);
 expect(oracle.workspace.filter(path=>!strip(got.closure).includes(path))).toEqual([]);
 expect(oracle.packages).toEqual(expect.arrayContaining(corpus.typescript.packages!));
}));

test("a target that declares no closure metadata is unchanged",()=>withFixture(async root=>{
 const got=await cacheInternals.targetScriptSources({inputs:["{projectRoot}/test/**/*.ts"],options:{cwd:corpus.projectRoot,command:corpus.command}},root,createScriptInputCache(),corpus.projectRoot);
 expect(strip(got.closure)).toEqual(["app/src/a.ts","app/📜️script.ts"]);
}));

test("closure and digest move exactly with the declared sources",async()=>{
 for(const mutation of corpus.mutations){
  await withFixture(async root=>{
   const scenario=mutation.target as "modules"|"typescript",before=strip((await sources(root,scenario)).closure),beforeDigest=digest(root,before);
   mkdirSync(dirname(join(root,mutation.file)),{recursive:true});writeFileSync(join(root,mutation.file),mutation.content);
   const after=strip((await sources(root,scenario)).closure);
   expect(after,mutation.name).toEqual(mutation.closure==="same"?before:[...mutation.closure].sort());
   expect(digest(root,after)===beforeDigest,mutation.name).toBe(mutation.digest==="same");
  });
 }
});
