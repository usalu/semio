import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{nativeCallerPhaseV1}from"../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{cargoPreparationControlV1}from"../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {resolve,join,dirname,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import{cargoPreparationSourceOracleV1}from"./🔭️oracle/🟦️.ts";
import {cargoPreparationInputV1,cargoPreparationProgramSourcesV1,admitCargoPreparationObservationV1,assertCargoPreparationObservationCurrentV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json";

test("directory custody records link membership without admitting the linked owner",()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"cu-"));
 try{
  const source=join(root,"source"),dependency=join(root,"dependency"),link=join(source,"node_modules");
  mkdirSync(source);mkdirSync(dependency);writeFileSync(join(source,"manifest.json"),"{}");
  const before=cargoPreparationInputV1(root,source,"directory");
  symlinkSync(dependency,link,process.platform==="win32"?"junction":"dir");
  const observed=cargoPreparationInputV1(root,source,"directory");
  const expected=new Bun.CryptoHasher("sha256").update(JSON.stringify([["manifest.json","file"],["node_modules","symlink"]])).digest("hex");
  expect(observed.sha256).toBe(expected);expect(observed.sha256).not.toBe(before.sha256);
  writeFileSync(join(dependency,"manifest.json"),"{}");
  expect(cargoPreparationInputV1(root,source,"directory")).toEqual(observed);
  expect(()=>cargoPreparationInputV1(root,join(link,"manifest.json"),"file")).toThrow("follows a symlink");
 }finally{rmSync(root,{recursive:true,force:true});}
});

registerBuildTestLawV1("Cargo custody actual preparation program binds distinct fixture and tracked defining roots without source copies","caller-birth",async original=>{
 const control=cargoPreparationControlV1(nativeCallerPhaseV1(original,process.cwd(),createIssuedResourceControlV1(original,"custody:actual",0)));
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Explicit custody artifact root required");const root=join(artifacts,"r"),sourceRoot=process.cwd(),custody=fileURLToPath(new URL("../🟦️.ts",import.meta.url)),producer=fileURLToPath(new URL("../../📜️script.ts",import.meta.url)),entry=join(root,"📜️script.ts");mkdirSync(root,{recursive:true});writeFileSync(entry,fixture.program.replace("{CUSTODY}",JSON.stringify(custody.replaceAll("\\","/"))));
 const bytes=readFileSync(entry),row={version:1,root,sourceRoot,script:entry,command:fixture.command,sources:[{path:entry,kind:"file",sha256:createHash("sha256").update(bytes).digest("hex")}],resolverInputs:[],inputs:[],outputs:[],resolutions:[]},validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/CargoPreparationObservationV1`)!;expect(validate(row),JSON.stringify(validate.errors)).toBe(true);for(const key of fixture.missing){const changed:Record<string,unknown>={...row};delete changed[key];expect(validate(changed)).toBe(false);}
 const expected=await cargoPreparationSourceOracleV1(sourceRoot,[entry,producer],join(root,"esbuild"));const program=await cargoPreparationProgramSourcesV1(root,entry,control);expect(program.sourceRoot).toBe(sourceRoot);expect(program.sources.map(source=>source.path).sort()).toEqual(expected);for(const source of program.sources)expect(source.sha256).toBe(new Bun.CryptoHasher("sha256").update(readFileSync(source.path)).digest("hex"));const observed=admitCargoPreparationObservationV1({...row,...program});await assertCargoPreparationObservationCurrentV1(observed,control);expect(program.resolverInputs.some(input=>dirname(input.path)===sourceRoot)).toBe(true);expect(program.sources.some(source=>source.path===custody)).toBe(true);
});
