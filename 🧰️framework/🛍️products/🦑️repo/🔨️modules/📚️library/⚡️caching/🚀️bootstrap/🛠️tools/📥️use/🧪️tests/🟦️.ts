import{withBuildTestInvocationV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import "./📥️purpose/🧪️tests/🟦️.ts";
import{captureNxToolingUseRefusalV1}from"./📥️purpose/🟦️.ts";
import {test,expect} from "bun:test";
import {connect} from "node:net";
import {Database} from "bun:sqlite";
import {mkdtempSync,mkdirSync,rmSync,readdirSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import {withNxToolingUseV1,type NxToolingUseWitnessV1} from "../🟦️.ts";
import {createRequestedIssuedResourceControlV1,scriptInvocationEnvironment} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import witnessSchema from "../🧬️schema/📥️witness.json";
import fixture from "./🧫️fixtures/🔣️.json";

/** 📡️ Independently challenges the exact finite original owner through its bounded advertised socket. */
function challengeWitness(witness:NxToolingUseWitnessV1,nonce=witness.nonce):Promise<string>{
 return new Promise((accept,reject)=>{let image="";const socket=connect(witness.endpoint,()=>socket.write(nonce+"\n"));socket.setTimeout(1000,()=>socket.destroy(Error("Original finite witness challenge timeout")));socket.on("data",bytes=>image+=bytes);socket.once("error",reject);socket.once("end",()=>accept(image));});
}

test("finite actual Nx toolkit use refuses physical SQLite retirement until its owned body settles",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR,identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!artifacts||!identity)throw Error("Original artifacts and immutable SDK identity required");const witnessValid=new Ajv({strict:true}).compile(witnessSchema);const root=mkdtempSync(join(artifacts,"tu-")),directory=root,sdk=createRequire(join(process.cwd(),".cache/b/t/m/nx",identity,"package.json"));mkdirSync(directory,{recursive:true});
 const oracle=()=>{const database=new Database(join(directory,createHash("sha256").update(fixture.resource+identity).digest("hex")+".sqlite"),{create:true});try{database.exec("PRAGMA busy_timeout=0");database.exec("BEGIN EXCLUSIVE");database.exec("ROLLBACK");return "allowed";}catch(error){if(!/locked|busy/i.test(String(error)))throw error;return "refused";}finally{database.close();}};
 try{await withBuildTestInvocationV1(async original=>{const control=scriptCompilerAdmissionControlV1(original,"nx:toolkit-use:actual",15000);for(const outcome of fixture.cases){if(outcome==="refused"){const directory=mkdtempSync(join(root,"nr-"));try{const{result,receipt}=await captureNxToolingUseRefusalV1(process.cwd(),directory,original,control,"toolkit-refusal");expect(result.status,result.stderr).toBe(1);expect(result.signal).toBe(null);expect(result.stderr).toContain("Original finite body refused");expect(result.stderr).not.toContain("acknowledgement deadline");expect(receipt.nodeVersion).toBe(true);expect(receipt.witnessAfter).toBe(0);expect(original.control.signal.aborted).toBe(false);expect(oracle()).toBe(fixture.after);}finally{rmSync(directory,{recursive:true,force:true});}continue;}const observed:{value?:string}={};let receipt:NxToolingUseWitnessV1|undefined;await withNxToolingUseV1({directory,witnessDirectory:directory},{identity,toolkitRoot:join(process.cwd(),".cache/b/t/m/nx",identity),invocation:original},control,async(_witness,usePhase)=>{observed.value=oracle();const receipts=readdirSync(directory).filter(path=>path.endsWith(".json"));expect(receipts.length).toBe(fixture.witness.during);receipt=JSON.parse(readFileSync(join(directory,receipts[0]!),"utf8"));expect(witnessValid(receipt),JSON.stringify(witnessValid.errors)).toBe(true);expect(receipt!.identity).toBe(identity);expect(receipt!.processId).toBe(process.pid);expect(receipt!.toolkitRoot).toBe(join(process.cwd(),".cache/b/t/m/nx",identity));expect(receipt!.original.capabilities).toEqual(original.capabilities);expect(JSON.parse(await challengeWitness(receipt!))).toEqual(receipt);expect(await challengeWitness(receipt!,"invalid-original-nonce")).toBe("");expect(receipt!.original.policy).toEqual(original.policy);const output=await captureOwnedCommandV1("node",[sdk.resolve("nx/bin/nx.js"),"--version"],process.cwd(),{control:createRequestedIssuedResourceControlV1(usePhase,15000),env:scriptInvocationEnvironment(original,process.env)});expect(output).toContain("23.2.0");});expect(outcome).toBe("complete");expect(observed.value,outcome).toBe(fixture.during);expect(oracle(),outcome).toBe(fixture.after);expect(readdirSync(directory).filter(path=>path.endsWith(".json")).length).toBe(fixture.witness.after);expect(await challengeWitness(receipt!).catch(error=>error.code)).toBe("ECONNREFUSED");}});}finally{rmSync(root,{recursive:true,force:true});}
},20_000);
