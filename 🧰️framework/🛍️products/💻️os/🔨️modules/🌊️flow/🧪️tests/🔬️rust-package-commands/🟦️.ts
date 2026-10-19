import type { ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import Ajv from "ajv";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🏃️ Runs the `FormSourceScript` command under the caller's roots and invocation. */
export async function runFormSourceScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
if(args.length)throw new Error("test-form-source accepts no arguments");
const base=join(context.root,"../../📝️forms/🧬️schema");
const schema=JSON.parse(readFileSync(join(base,"🔣️.json"),"utf8"));const corpus=JSON.parse(readFileSync(join(base,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8"));
const validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.equal(validate(corpus),true,JSON.stringify(validate.errors));
const bad=structuredClone(corpus);bad.widgets[0].value="3";assert.equal(validate(bad),false);
const source=readFileSync(join(context.root,"../../🌿️vcs/🦀️.rs"),"utf8");assert.ok(source.includes("P:crate::forms::FlowFormProducer"));assert.ok(!source.includes("PlaybookBlock")&&!source.includes("PlaybookSpec"));
 }
