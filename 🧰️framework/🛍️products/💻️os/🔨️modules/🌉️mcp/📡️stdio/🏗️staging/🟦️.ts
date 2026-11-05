import {resolve} from "node:path";
import {buildAdmittedCargoArtifactsV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import {verifyCapturedCargoArtifactsV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🟦️.ts";
import {scriptInvocationEnvironment,type ScriptInvocation} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import type {CompilerAdmissionControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
const producer=schema.definitions.Producer.const;
/** 📜️ Identifies the one authored manifest that owns the native MCP publication. */
export const MCP_MANIFEST_REL=producer.manifest;

/** 🏗️ Builds the fixed native MCP publication through its genuine Original compiler admission. */
export async function buildMcpArtifactV1(repoRoot:string,invocation:ScriptInvocation,control:CompilerAdmissionControl,diagnostic:(line:string)=>void=line=>void process.stderr.write(line+"\n")):Promise<void>{
 await buildAdmittedCargoArtifactsV1(resolve(repoRoot,producer.manifest),["--package",producer.package,"--bin",producer.binary],repoRoot,{producer:"artifact",invocation,control,diagnostic,environment:scriptInvocationEnvironment(invocation,process.env),consume:verifyCapturedCargoArtifactsV1,sourcesRecord:producer.sourcesRecord});
}
