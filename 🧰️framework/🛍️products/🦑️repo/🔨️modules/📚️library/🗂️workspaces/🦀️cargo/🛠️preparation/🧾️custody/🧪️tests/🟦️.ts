import {test,expect} from "bun:test";
import {mkdirSync,readFileSync,writeFileSync} from "node:fs";
import {resolve,join,dirname,relative} from "node:path";
import {fileURLToPath} from "node:url";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {buildSync} from "esbuild";
import {cargoPreparationProgramSourcesV1,admitCargoPreparationObservationV1,assertCargoPreparationObservationCurrentV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("actual preparation program custody binds distinct fixture and tracked defining roots without source copies",()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Explicit custody artifact root required");const root=join(artifacts,"r"),sourceRoot=process.cwd(),custody=fileURLToPath(new URL("../🟦️.ts",import.meta.url)),producer=fileURLToPath(new URL("../../📜️script.ts",import.meta.url)),entry=join(root,"📜️script.ts");mkdirSync(root,{recursive:true});writeFileSync(entry,fixture.program.replace("{CUSTODY}",JSON.stringify(custody.replaceAll("\\","/"))));
 const bytes=readFileSync(entry),row={version:1,root,sourceRoot,script:entry,command:fixture.command,sources:[{path:entry,kind:"file",sha256:createHash("sha256").update(bytes).digest("hex")}],resolverInputs:[],inputs:[],outputs:[],resolutions:[]},validate=new Ajv({strict:true}).addSchema(schema).getSchema(`${schema.$id}#/$defs/CargoPreparationObservationV1`)!;expect(validate(row),JSON.stringify(validate.errors)).toBe(true);for(const key of fixture.missing){const changed={...row};delete changed[key];expect(validate(changed)).toBe(false);}
 const oracle=buildSync({absWorkingDir:sourceRoot,entryPoints:[entry,producer],outdir:join(root,"esbuild"),bundle:true,write:false,metafile:true,platform:"node",packages:"external",format:"esm",logLevel:"silent"}).metafile!,queue=[relative(sourceRoot,entry),relative(sourceRoot,producer)],reached=new Set<string>();for(const path of queue){if(reached.has(path))continue;reached.add(path);for(const edge of oracle.inputs[path]!.imports)if(!edge.external&&["import-statement","require-call"].includes(edge.kind))queue.push(edge.path);}const program=cargoPreparationProgramSourcesV1(root,entry);expect(program.sourceRoot).toBe(sourceRoot);expect(program.sources.map(source=>source.path).sort()).toEqual([...reached].map(path=>resolve(sourceRoot,path)).sort());for(const source of program.sources)expect(source.sha256).toBe(new Bun.CryptoHasher("sha256").update(readFileSync(source.path)).digest("hex"));const observed=admitCargoPreparationObservationV1({...row,...program});expect(()=>assertCargoPreparationObservationCurrentV1(observed)).not.toThrow();expect(program.resolverInputs.some(input=>dirname(input.path)===sourceRoot)).toBe(true);expect(program.sources.some(source=>source.path===custody)).toBe(true);console.log("[DEBUG] dual-root original program sources match independent esbuild and Bun hashes without copying tracked files");
});
