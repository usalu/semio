
/** 📄️ Reads opaque current capsule metadata without inventing historical intent authority. */
import {expect,test} from "bun:test";
import {mkdir,mkdtemp,writeFile,rm,link,symlink,readFile} from "node:fs/promises";
import {writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {sha256} from "@noble/hashes/sha2.js";
import ts from "typescript";
import {fileURLToPath} from "node:url";
import {artifactDescriptorCustodyV1} from "../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";

test("opaque capsule metadata agrees with Python SHA256 fixtures and independent SQLite bytes",async()=>{
 const api=await import("../🟦️.ts"),validate=new Ajv({strict:true}).compile(schema),output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit opaque metadata test output required");expect(validate(corpus)).toBe(true);const observations=new Ajv({strict:true});observations.addSchema(schema,"opaque");const observation=observations.getSchema("opaque#/$defs/Observation")!;await mkdir(output,{recursive:true});const db=new Database(":memory:");
 try{for(const row of corpus.cases){const root=await mkdtemp(join(resolve(output),"opaque-")),path=join(root,row.effect==="wrong-name"?"other":"i"),bytes=Buffer.from(row.hex,"hex"),marker=Object.freeze(Error("actual metadata cancellation"));let checkpoint=0,failed=false,intervened=false;const active=artifactDescriptorCustodyV1().active;
  try{
   if(row.effect==="directory")await mkdir(path);else await writeFile(path,bytes);
   if(row.effect==="multilink")await link(path,join(root,"alias"));
   let actual=path;
   if(row.effect==="link"){if(process.platform==="win32"){await mkdir(join(root,"physical"));await symlink(join(root,"physical"),join(root,"r"),"junction");}else await symlink(path,join(root,"r"));actual=join(root,"r");}
   if(row.effect==="ancestor-link"){await mkdir(join(root,"physical"));await writeFile(join(root,"physical","i"),bytes);await symlink(join(root,"physical"),join(root,"linked"),process.platform==="win32"?"junction":"dir");actual=join(root,"linked","i");}
   const check=()=>{checkpoint++;if(!intervened&&artifactDescriptorCustodyV1().active>active){if(row.effect==="cancel"){intervened=true;throw marker;}if(row.effect==="drift"){intervened=true;writeFileSync(path,"foreign");}}};
   try{const result=api.readJavascriptDependencyUnreadyMetadataV1(actual,4096,check);expect(row.accepted,row.name).toBe(true);expect(Object.isFrozen(result)).toBe(true);expect(Object.isFrozen(result.stamp)).toBe(true);expect(result.before).toEqual(result.after);expect(result.stamp).toBe(result.before);expect(result.sha256,row.name).toBe(row.sha256);expect(result.text,row.name).toBe(row.text);expect(result.authority).toBe(false);expect(observation(result)).toBe(true);expect(Buffer.from(sha256(bytes)).toString("hex")).toBe(row.sha256);expect(result.path).toBe(actual);
    const physical=await readFile(actual),reference=db.query("SELECT length(?1) bytes,lower(hex(?1)) hex").get(physical)as{bytes:number;hex:string};expect(result.stamp.size).toBe(reference.bytes);expect(reference.hex).toBe(row.hex);expect(result.stamp.nlink).toBe(1);
   }catch(error){failed=true;if(row.accepted)throw error;if(row.effect==="cancel")expect(error).toBe(marker);}
   expect(!failed,row.name).toBe(row.accepted);if(row.effect==="cancel"||row.effect==="drift")expect(intervened).toBe(true);expect(artifactDescriptorCustodyV1().active).toBe(active);
  }finally{await rm(root,{recursive:true,force:true});}
 }}finally{db.close();}
},120_000);

test("opaque metadata defining owner passes strict semantic TypeScript",()=>{
 const path=fileURLToPath(new URL("../🟦️.ts",import.meta.url)),program=ts.createProgram([path,fileURLToPath(import.meta.url)],{noEmit:true,strict:true,noUncheckedIndexedAccess:true,skipLibCheck:true,target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext,moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true,resolveJsonModule:true,allowSyntheticDefaultImports:true}),diagnostics=ts.getPreEmitDiagnostics(program);expect(ts.formatDiagnostics(diagnostics,{getCanonicalFileName:path=>path,getCurrentDirectory:()=>process.cwd(),getNewLine:()=>"\n"})).toBe("");
},120_000);
