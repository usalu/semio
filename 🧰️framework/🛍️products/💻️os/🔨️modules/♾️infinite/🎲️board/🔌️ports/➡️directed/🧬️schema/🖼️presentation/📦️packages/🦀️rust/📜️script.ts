#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedCargoPackageTestsV1 } from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import { runHostKindAdmissionChecks } from "../../🧪️tests/📥️host-kind-admission/🟦️.ts";

/** 🧬️ Validates the portable corpus with the independent JSON schema oracle. */
class SourceScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-source accepts no arguments");
  const schema=JSON.parse(readFileSync(resolve(this.root,"../../🔣️.json"),"utf8"));
  const corpus=JSON.parse(readFileSync(resolve(this.root,"../../../../🧪️tests/🧫️fixtures/🖼️presentation/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/IoPortSpec"});
  for(const port of corpus.ports){if(this.invocation.control.signal.aborted)throw Error("presentation source check cancelled");if(!validate(port))throw Error(JSON.stringify(validate.errors));}
  const snapshot=new Ajv({strict:false}).compile({...schema,$ref:"#/$defs/DagHostSnapshot"});
  if(!snapshot(corpus.emptySnapshot))throw Error(JSON.stringify(snapshot.errors));
  const source=readFileSync(resolve(this.root,"../../🦀️.rs"),"utf8");
  if(source.includes("semio_framework_artifact_")||source.includes("crate::vcs"))throw Error("presentation defining owner requires a child artifact");
  runHostKindAdmissionChecks(this.invocation.control.signal);
  console.log(`[directed-presentation] ${corpus.ports.length} portable port vectors admitted by Ajv; native implementation remains separately qualified`);
 }
}

/** 🦀️ Runs the defining owner's portable law through original repository execution. */
class NativeScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-native accepts no arguments");
  await runAdmittedCargoPackageTestsV1(["semio-framework-os-infinite-directed-presentation"],this.root,this.invocation,["--lib"]);
 }
}
const router=new ScriptRouter(import.meta.dir).register("test-source",SourceScript).register("test-native",NativeScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
