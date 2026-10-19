import {expect,test} from "bun:test";
import {mkdir,mkdtemp,readFile,writeFile,rm,link,readdir} from "node:fs/promises";
import {join,resolve} from "node:path";
import {sha256} from "@noble/hashes/sha2.js";
import Ajv from "ajv";
import {withArtifactRelativeDirectoryV1} from "../../📁️descriptor/🟦️.ts";
import schema from "../../📁️descriptor/🧬️schema/🔣️.json";

test("conditional rooted file publication preserves prior bytes and matches independent SHA256",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit publication test output required");await mkdir(output,{recursive:true});
 const validate=new Ajv({strict:true}).compile(schema.definitions.RelativeFilePublicationCase),cases=schema.definitions.RelativeFilePublicationCases.const.filter(row=>row.effect==="none"&&row.accepted&&(row.before==="missing"||row.before==="file"));
 for(const row of cases){expect(validate(row)).toBe(true);const root=await mkdtemp(join(resolve(output),"publication-")),name="dependencies.json";try{if(row.before==="file")await writeFile(join(root,name),row.prior!);
  await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.captureFilePublication(name,row.maximumBytes),result=await directory.publishFile(expected,Buffer.from(row.bytes));expect(result.authority).toBe(false);expect(result.sha256).toBe(Buffer.from(sha256(Buffer.from(row.bytes))).toString("hex"));expect(await readFile(join(root,name),"utf8")).toBe(row.bytes);if(row.before==="file"){expect(result.retained).not.toBeNull();expect(await readFile(join(root,result.retained!.name),"utf8")).toBe(row.prior!);expect(result.retained!.sha256).toBe(Buffer.from(sha256(Buffer.from(row.prior!))).toString("hex"));}else expect(result.retained).toBeNull();});
 }finally{await rm(root,{recursive:true,force:true});}}
 console.log(`[DEBUG] conditional publication verified ${cases.length} missing and replacement byte cases`);
},120_000);

test("conditional publication rejects a copied expectation before changing files",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit publication test output required");await mkdir(output,{recursive:true});const root=await mkdtemp(join(resolve(output),"publication-"));try{await writeFile(join(root,"dependencies.json"),"old");await expect(withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.captureFilePublication("dependencies.json",64);await directory.publishFile({...expected},Buffer.from("new"));})).rejects.toThrow();expect(await readFile(join(root,"dependencies.json"),"utf8")).toBe("old");}finally{await rm(root,{recursive:true,force:true});}
},30_000);

test("conditional file publication refuses hostile source and caller changes before effects",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit publication test output required");await mkdir(output,{recursive:true});
 const cases=schema.definitions.RelativeFilePublicationCases.const.filter(row=>!row.accepted&&["missing","file","hardlink","directory"].includes(row.before)&&["none","copied","changed","parent","expected-proxy","bytes-proxy","cancel"].includes(row.effect));
 for(const row of cases){const root=await mkdtemp(join(resolve(output),"publication-")),name="dependencies.json";try{if(row.before==="directory")await mkdir(join(root,name));else if(row.before!=="missing"){await writeFile(join(root,name),row.prior??"");if(row.before==="hardlink")await link(join(root,name),join(root,"other.json"));}let stopped=false;
  await expect(withArtifactRelativeDirectoryV1(root,()=>{if(stopped)throw Error("Cancelled publication");},async directory=>{const expected=await directory.captureFilePublication(name,row.maximumBytes);if(row.effect==="changed")await writeFile(join(root,name),"external");if(row.effect==="parent")await writeFile(join(root,"external.json"),"external");if(row.effect==="cancel")stopped=true;const input=row.effect==="copied"?{...expected}:row.effect==="expected-proxy"?new Proxy(expected,{}):expected,bytes=row.effect==="bytes-proxy"?new Proxy(Buffer.from(row.bytes),{}):Buffer.from(row.bytes);await directory.publishFile(input,bytes);})).rejects.toThrow();
  if(row.before==="file"||row.before==="hardlink")expect(await readFile(join(root,name),"utf8")).toBe(row.effect==="changed"?"external":row.prior??"");expect((await readdir(root)).every(entry=>[name,"other.json","external.json"].includes(entry))).toBe(true);
 }finally{await rm(root,{recursive:true,force:true});}}
 console.log(`[DEBUG] conditional publication refused ${cases.length} hostile source and caller cases before publication`);
},120_000);
