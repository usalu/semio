import {test,expect} from "bun:test";
import {build} from "esbuild";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {join} from "node:path";
import {mkdir,writeFile,rm} from "node:fs/promises";
import contract from "../🔣️contract.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {preflightNxAuthorizersV1} from "../🟦️.ts";
import {receiveScriptProcessInvocation,scriptInvocationEnvironment} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

test("actual Node preflight rejects unbundled imports and absent runtime hooks before activation",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(contract)).toBe(true);const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifacts required");const database=new Database(":memory:");database.exec("CREATE TABLE outcomes(id TEXT PRIMARY KEY,accepted INTEGER)");for(const row of fixture.cases)database.query("INSERT INTO outcomes VALUES(?1,?2)").run(row.id,Number(row.accepted));
 try{await receiveScriptProcessInvocation(process.env,async original=>{for(const row of fixture.cases){const directory=join(artifacts,"preflight",row.id),source=row.source==="valid"?contract.functions.map(name=>`export function ${name}(){}`).join("\n"):row.source==="number"?contract.functions.map(name=>`export const ${name}=1;`).join("\n"):row.source==="relative"?'import "./absent-original.mjs";export const value=1;':"export const value=1;",result=await build({stdin:{contents:source,resolveDir:directory,sourcefile:"📜️script.ts"},bundle:false,format:"esm",platform:"node",write:false,logLevel:"silent"});await mkdir(directory,{recursive:true});await writeFile(join(directory,"semio-authorizers.mjs"),result.outputFiles[0]!.contents);let accepted=false;try{await preflightNxAuthorizersV1(directory,scriptCompilerAdmissionControlV1(original,"nx:preflight:"+row.id,10000),scriptInvocationEnvironment(original,process.env));accepted=true;}catch{}const oracle=database.query("SELECT accepted FROM outcomes WHERE id=?1").get(row.id) as {accepted:number};expect(accepted,row.id).toBe(Boolean(oracle.accepted));await rm(directory,{recursive:true,force:true});}});}finally{database.close();}
});
