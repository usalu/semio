#!/usr/bin/env bun
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import Ajv from "ajv";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runRepositoryCargoTests } from "../../../../../🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts";

/** 🧪️ Runs the original extension contribution laws with their defining Flow owner. */
class ExtensionRegistryScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-extension-registry accepts no arguments");
  await runRepositoryCargoTests(["semio-framework-os-flow-operators"],this.root,this.invocation,["--lib","registry::contribution_tests"]);
 }
}
/** 📝️ Runs original widget loans against the portable independent producer oracle. */
class FormProducerScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-form-producer accepts no arguments");
  await runRepositoryCargoTests(["semio-framework-os-flow-vcs"],this.root,this.invocation,["--lib","forms::tests"]);
 }
}
/** 🧬️ Validates the entire portable original field-loan corpus with the independent schema oracle. */
class FormSourceScript extends BundleScript {
 async run(args:string[]):Promise<void>{
  if(args.length)throw Error("test-form-source accepts no arguments");
  const base=join(import.meta.dir,"../../📝️forms/🧬️schema");
  const schema=JSON.parse(readFileSync(join(base,"🔣️.json"),"utf8"));const corpus=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.equal(validate(corpus),true,JSON.stringify(validate.errors));
  const bad=structuredClone(corpus);bad.widgets[0].value="3";assert.equal(validate(bad),false);
  const source=readFileSync(join(import.meta.dir,"../../🌿️vcs/🦀️.rs"),"utf8");assert.ok(source.includes("P:crate::forms::FlowFormProducer"));assert.ok(!source.includes("PlaybookBlock")&&!source.includes("PlaybookSpec"));
 }
}
const router=new ScriptRouter(import.meta.dir).register("test-extension-registry",ExtensionRegistryScript).register("test-form-producer",FormProducerScript).register("test-form-source",FormSourceScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));
