import {expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join,parse,resolve} from "node:path";
import Ajv from "ajv";
import {build} from "esbuild";
import {cacheInternals} from "../../../../🟨️.mjs";

type PathVector={id:string;prefix:string;segment:string;count:number;suffix:string;utf16:number;admitted:boolean};
const caching=resolve(import.meta.dir,"../../.."),fixture=JSON.parse(readFileSync(join(caching,"🧫️fixtures/📁️source-facts/🔣️.json"),"utf8")) as {version:number;paths:PathVector[]};
const schema=JSON.parse(readFileSync(join(caching,"🧬️schema/📁️source-facts/🔣️.json"),"utf8"));

test("source-fact path corpus satisfies the independent strict schema oracle",()=>{
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
  expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.paths.map(row=>row.id)).size).toBe(fixture.paths.length);
});

for(const row of fixture.paths)test(`source-fact cache path authority: ${row.id}`,()=>{
  const previous=process.env.NX_WORKSPACE_DATA_DIRECTORY,base=parse(resolve("/")).root,remaining=row.prefix.length+row.segment.length*row.count-base.length,directory=base+row.segment.repeat(Math.floor(remaining/row.segment.length))+"a".repeat(remaining%row.segment.length)+row.suffix;
  process.env.NX_WORKSPACE_DATA_DIRECTORY=directory;
  try{
    const root=cacheInternals.moduleSourceFactCacheRoot("/unused");
    expect(root,row.id).toBe(row.admitted?join(directory,"mf"):undefined);
    const path=join(directory,"mf","0".repeat(64)+".json");
    expect(Buffer.byteLength(path,"utf16le")/2,row.id).toBe(row.utf16);
    expect(path.length<=256,row.id).toBe(row.admitted);
  }finally{
    if(previous===undefined)delete process.env.NX_WORKSPACE_DATA_DIRECTORY;
    else process.env.NX_WORKSPACE_DATA_DIRECTORY=previous;
  }
});

test("oversized cache authority still parses current source without creating cache files",async()=>{
  const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;
  if(!artifact)throw Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(artifact,{recursive:true});
  const root=mkdtempSync(join(artifact,"p-")),previous=process.env.NX_WORKSPACE_DATA_DIRECTORY;
  const source="import './leaf.ts';",file="index.ts",hash=createHash("sha256").update(source).digest("hex");
  const imports=new Set<string>();
  await build({stdin:{contents:source,sourcefile:file,loader:"js"},bundle:true,write:false,format:"esm",logLevel:"silent",plugins:[{name:"source-fact-oracle",setup(builder){builder.onResolve({filter:/.*/},args=>{imports.add(args.path);return{path:args.path,external:true};});}}]});
  expect([...imports]).toEqual(["./leaf.ts"]);
  writeFileSync(join(root,file),source);
  process.env.NX_WORKSPACE_DATA_DIRECTORY=join(root,"a".repeat(180));
  try{
    expect(cacheInternals.moduleSourceFactCacheRoot(root)).toBeUndefined();
    for(const temperature of ["cold","warm"]){
      const targets=new Set<string>();
      await cacheInternals.collectImportEdges(root,{caller:[{file,hash}]},{caller:{root:""},leaf:{root:"leaf.ts"}},new Map(),undefined,(_from:string,target:string)=>targets.add(target));
      expect([...targets],temperature).toEqual(["leaf"]);
    }
    expect(existsSync(process.env.NX_WORKSPACE_DATA_DIRECTORY)).toBe(false);
    expect(readdirSync(root)).toEqual([file]);
    console.log("[DEBUG] oversized source-fact cache skipped; original import parsed twice with esbuild parity");
  }finally{
    if(previous===undefined)delete process.env.NX_WORKSPACE_DATA_DIRECTORY;
    else process.env.NX_WORKSPACE_DATA_DIRECTORY=previous;
    rmSync(root,{recursive:true,force:true});
  }
});
