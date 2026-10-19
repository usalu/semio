import type { ScriptInvocation } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { runHostKindAdmissionChecks } from "../📥️host-kind-admission/🟦️.ts";

/** 🏃️ Runs the `SourceScript` command under the caller's roots and invocation. */
export async function runSourceScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
if(args.length)throw new Error("test-source accepts no arguments");
const schema=JSON.parse(readFileSync(resolve(context.root,"../../🔣️.json"),"utf8"));
const corpus=JSON.parse(readFileSync(resolve(context.root,"../../../../🧪️tests/🧫️fixtures/🖼️presentation/🔣️.json"),"utf8"));
const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/IoPortSpec"});
for(const port of corpus.ports){if(context.invocation.control.signal.aborted)throw new Error("presentation source check cancelled");if(!validate(port))throw new Error(JSON.stringify(validate.errors));}
const snapshot=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/DagHostSnapshot"});
if(!snapshot(corpus.emptySnapshot))throw new Error(JSON.stringify(snapshot.errors));
const source=readFileSync(resolve(context.root,"../../🦀️.rs"),"utf8");
if(source.includes("semio_framework_artifact_")||source.includes("crate::vcs"))throw new Error("presentation defining owner requires a child artifact");
runHostKindAdmissionChecks(context.invocation.control.signal);
console.log(`[directed-presentation] ${corpus.ports.length} portable port vectors admitted by Ajv; native implementation remains separately qualified`);
 }
