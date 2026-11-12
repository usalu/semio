import{registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import{nativeCallerPhaseV1}from"../../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import{cargoPreparationControlV1}from"../../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureOwnedCommandResultV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{cargoPreparationProgramSourcesV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import fixture from"../🧫️fixtures/🔣️.json";
import{cargoPreparationSourceOracleV1}from"../🔭️oracle/🟦️.ts";
import{expect}from"bun:test";
import{mkdirSync,readFileSync}from"node:fs";
import{resolve,join}from"node:path";
import{fileURLToPath}from"node:url";
import{createRequire}from"node:module";
import{isUtf8}from"node:buffer";
/** 🌊️ Compares the actual preparation producer's complete metadata and backpressure delivery. */
registerBuildTestLawV1("Cargo capture-program delivers the actual large preparation observation","caller-birth",async original=>{
 const control=cargoPreparationControlV1(nativeCallerPhaseV1(original,process.cwd(),createIssuedResourceControlV1(original,"custody:large-output",0))),root=process.cwd(),artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Explicit custody artifact root required");const directory=join(artifacts,"l");mkdirSync(directory,{recursive:true});
 const entry=resolve(root,fixture.actualProgram),producer=fileURLToPath(new URL("../../../📜️script.ts",import.meta.url));
 const program=await cargoPreparationProgramSourcesV1(root,entry,control),serialized=Buffer.from(JSON.stringify(program)+"\n");expect(program.sourceRoot).toBe(root);for(const source of program.sources)expect(source.sha256).toBe(new Bun.CryptoHasher("sha256").update(readFileSync(source.path)).digest("hex"));expect(serialized.length).toBeGreaterThanOrEqual(fixture.minimumObservationBytes);
 const crossSpawn=createRequire(import.meta.url)("cross-spawn"),independent=crossSpawn.sync(process.execPath,["-e",'const value=await Bun.stdin.json();await new Promise((accept,reject)=>process.stdout.write(JSON.stringify(value)+"\\n",error=>error?reject(error):accept()));'],{input:serialized,encoding:null,timeout:Math.max(1,Math.floor(control.childBudget(10000))),maxBuffer:schema.$defs.CargoPreparationControlProtocolV1.const.maximumCaptureBytes});expect(independent.status).toBe(0);expect(independent.signal).toBeNull();expect(isUtf8(independent.stdout)).toBe(true);expect(Buffer.from(independent.stdout)).toEqual(serialized);
 const wrapper=fileURLToPath(new URL("./👷️child/📜️script.ts",import.meta.url));
 const delivered=await captureOwnedCommandResultV1(process.execPath,[wrapper,"capture-program",root,entry],root,{control:control.leaseControl,output:{maximumBytes:schema.$defs.CargoPreparationControlProtocolV1.const.maximumCaptureBytes,maximumLines:65536}});expect(delivered.status).toBe(0);expect(delivered.signal).toBeNull();expect(Buffer.from(delivered.stdout)).toEqual(serialized);expect(delivered.stderr).toContain("custody stdout backpressure=true completed=true");expect(program.sources.map(source=>source.path).sort()).toEqual(await cargoPreparationSourceOracleV1(root,[entry,producer],join(directory,"esbuild")));
});
