import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {parse} from "yaml";
import fixture from "../../🧫️fixtures/🚀️readiness/🔣️.json" with {type:"json"};

/** 🩺️ Proves that release CI uses actual generic admission while course image/runtime guards retain their own contract. */
export function testCoursePreflightAlignmentV1():void{
 const course=resolve(dirname(fileURLToPath(import.meta.url)),"../.."),root=resolve(course,"../../.."),read=(path:string)=>readFileSync(join(root,path),"utf8"),require=createRequire(import.meta.url),compiler=require("typescript") as typeof import("typescript"),build="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build",schema=JSON.parse(read(build+"/🩺️preflight/🧬️schema/🔣️.json")),policy=JSON.parse(read(build+"/🩺️preflight/🔣️policy.json"));
 const parserSource=compiler.createSourceFile("preflight.ts",read(build+"/🩺️preflight/🟦️.ts"),compiler.ScriptTarget.Latest,true),declaration=parserSource.statements.find(node=>compiler.isFunctionDeclaration(node)&&node.name?.text==="readBuildPreflightArgumentsV1");assert.ok(declaration);const accepted=new Function(compiler.transpileModule(declaration.getText(parserSource).replace(/^export\s+/u,""),{compilerOptions:{target:compiler.ScriptTarget.ES2022}}).outputText+";return readBuildPreflightArgumentsV1;")();
 const validate=new Ajv({strict:true}).compile({$ref:"#/definitions/Policy",definitions:schema.definitions});assert.equal(validate(policy),true,JSON.stringify(validate.errors));assert.deepEqual(policy.platforms.linux,fixture.preflight.linux);
 const workflow=parse(read(".github/workflows/architecture-course.yml")),steps=Object.values(workflow.jobs as Record<string,{steps:{name?:string;run?:string}[]}>).flatMap(job=>job.steps).filter(step=>step.run?.includes("bun repo preflight"));assert.equal(steps.length,3);
 for(const step of steps){const arguments_=step.run!.slice(step.run!.indexOf("bun repo ")+9).trim().split(/\s+/u);assert.deepEqual(arguments_,fixture.preflight.arguments);accepted(arguments_);assert.equal(step.name,fixture.preflight.step);}
 const project=JSON.parse(readFileSync(join(course,"📦️packages/🟦️typescript/📋️project.json"),"utf8"));assert.equal(project.metadata.semio.preflight,undefined);assert.equal(project.targets.preflight.options.command,"bun ./📜️script.ts preflight");
 const source=compiler.createSourceFile("📜️script.ts",readFileSync(join(course,"📦️packages/🟦️typescript/📜️script.ts"),"utf8"),compiler.ScriptTarget.Latest,true),calls:unknown[]=[];const visit=(node:import("typescript").Node)=>{if(compiler.isCallExpression(node)&&node.expression.getText(source)==="runBuildPreflightV1")calls.push(node.arguments[1]?.getText(source));compiler.forEachChild(node,visit);};visit(source);assert.deepEqual(calls,[JSON.stringify(fixture.preflight.arguments)]);
 const readme=readFileSync(join(course,"README.md"),"utf8");for(const token of fixture.preflight.documentation)assert.ok(readme.includes(token),"Missing truthful preflight/runtime documentation: "+token);for(const token of fixture.preflight.obsolete)assert.ok(!readme.includes(token),"Obsolete course preflight claim: "+token);
 const deploy=readFileSync(join(course,"🚀️deploy/🟦️.ts"),"utf8"),image=readFileSync(join(course,"🚀️deploy/📦️image/🟦️.ts"),"utf8"),imageVerbs=readFileSync(join(course,"🚀️deploy/🐳️docker-image/🟦️.ts"),"utf8");for(const token of fixture.preflight.imageGuards)assert.ok((deploy+imageVerbs+image).includes(token),"Actual image guard retained: "+token);const dockerfile=readFileSync(join(course,"🚀️deploy/Dockerfile"),"utf8"),checks=[...dockerfile.matchAll(/^RUN (\[.*\])$/gmu)].map(row=>JSON.parse(row[1]!));assert.ok(checks.some(row=>JSON.stringify(row)===JSON.stringify(fixture.preflight.runtimeCheck)),"Actual pinned-runtime ABI and content check retained");
 console.log("Course preflight alignment: actual accepted CI arguments, Ajv policy, TypeScript owner and retained image/runtime guards PASS");
}
