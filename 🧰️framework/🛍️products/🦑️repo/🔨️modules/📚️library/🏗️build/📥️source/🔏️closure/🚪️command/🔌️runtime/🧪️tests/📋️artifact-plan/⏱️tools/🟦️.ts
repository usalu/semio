import {prepareRepoCpuProfileArgumentsV1} from "../../../../../../../⏱️profile/📊️cpu/🟨️.mjs";
import {test,expect} from "bun:test";
import _ from "lodash";
import {relative} from "node:path";
import {fileURLToPath} from "node:url";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {runOwnedCommand} from "../../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {captureCompilerBuildSourceClosureV1} from "../../../../../🏗️compiler/🟦️.ts";
import {buildSourceClosureTextV1,buildSourceClosureStreamLoaderV1} from "../../../../../🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-source-closure",maximumElapsedMilliseconds:180000,scratch:"checkout"});
/** ⏱️ Records actual read-only Native method costs without replacing complete artifact-plan acceptance. */
test("public tools partition read observations preserve zero-request semantics",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),control=createIssuedResourceControlV1(original,"test:tools-read-observation",160000),entry=relative(root,fileURLToPath(new URL("./📜️script.ts",import.meta.url))).replaceAll("\\","/"),snapshot=buildSourceClosureTextV1(await captureCompilerBuildSourceClosureV1(root,entry,original,control)),chunks:Buffer[]=[];
 await runOwnedCommand(process.execPath,[...prepareRepoCpuProfileArgumentsV1(process.env),"--eval",buildSourceClosureStreamLoaderV1,"--",snapshot.sha256],root,"test:tools-read-observation",0,{control,signal:control.signal,input:"",env:scriptResourceProcessEnvironmentV1(control,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"}),auxiliaryInput:{version:1,text:snapshot.text},output:{maximumBytes:1048576,maximumLines:8192},outputStyle:"static",outputConsumer:(bytes,stream)=>{if(stream==="stdout")chunks.push(Buffer.from(bytes));}});
 const text=Buffer.concat(chunks).toString("utf8"),lines=text.trim().split("\n"),result=JSON.parse(lines.at(-1)!);expect(result.version).toBe(1);expect(result.mode).toBe("tools-read-observation");expect(result.timings.map((row:{operation:string})=>row.operation)).toEqual(["observe","recheck","measure-first","measure-second","usage-row","journal-sh"]);for(const row of result.timings)expect(Number.isFinite(row.wallMilliseconds)&&row.wallMilliseconds>=0).toBe(true);expect(result.usage.entries).toBeGreaterThan(0);expect(Number.isSafeInteger(result.usage.allocatedBytes)).toBe(true);expect(result.measureRequest).toEqual({mode:"recheck",requestBytes:0});expect(_.sum(result.allocationBytes)).toBe(result.usedBytes);expect(result.journal.sha256).toMatch(/^[a-f0-9]{64}$/);console.log(JSON.stringify({kind:"tools-read-measurement",...result}));
}),170000);
