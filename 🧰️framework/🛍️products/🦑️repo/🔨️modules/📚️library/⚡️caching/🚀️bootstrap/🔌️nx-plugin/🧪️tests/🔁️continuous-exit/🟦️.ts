import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import {applyPatch,parsePatch,reversePatch} from "diff";
import {readFileSync,realpathSync} from "node:fs";
import {join} from "node:path";
import {applyToolingPatchImagesV1} from "../../../🛠️tools/🩹️patch/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./📋️schema.json";
const root=process.cwd(),target="dist/src/tasks-runner/task-orchestrator.js",patchFile=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🩹️patches/nx@23.2.0.patch");
const condition="!this.isContinuousTaskNeeded(task.id) && (code === 0 || !this.initializingTaskIds.has(task.id))";
test("the continuous exit vectors are closed input data",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,[{...corpus[0],extra:true},...corpus.slice(1)])).toBe(false);
});
test("the real Nx handler patch preserves dependency cleanup and reports selected failures",async()=>{
 const patch=readFileSync(patchFile,"utf8"),parts=patch.split("diff --git "),section="diff --git "+parts.find(part=>part.startsWith("a/"+target+" b/"+target+"\n"))!;
 expect(section.startsWith("diff --git a/"+target)).toBe(true);
 const current=readFileSync(join(realpathSync(join(root,".nx/installation")),"node_modules/nx",target),"utf8"),parsed=parsePatch(section)[0]!;
 const installed={...parsed,hunks:parsed.hunks.filter(hunk=>current.includes(condition)||!hunk.lines.some(line=>line.includes(condition)))};
 const pristine=applyPatch(current,reversePatch(installed));
 expect(pristine).not.toBe(false);
 const owned=new TextDecoder().decode(applyToolingPatchImagesV1(new Map([[target,new TextEncoder().encode(pristine as string)]]),section).get(target)!);
 expect(owned).toBe(applyPatch(pristine as string,section));
 const source=ts.createSourceFile(target,owned,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
 let method="";
 const visit=(node:ts.Node):void=>{if(ts.isMethodDeclaration(node)&&node.name.getText(source)==="handleContinuousTaskExit")method=node.getText(source);ts.forEachChild(node,visit);};visit(source);
 expect(method.length).toBeGreaterThan(100);
 const handler=new Function("exit_codes_1","return ({"+method+"}).handleContinuousTaskExit;")({EXPECTED_TERMINATION_SIGNALS:new Set([130,143])});
 const database=new Database(":memory:");
 const oracle=database.query("SELECT CASE WHEN ?1 THEN 'none' WHEN ?2 IS NOT NULL OR ?3 IN (130,143) THEN CASE WHEN ?2='fulfilled' THEN 'fulfilled' ELSE 'interrupted' END WHEN ?4 THEN 'crashed' WHEN NOT ?5 OR ?3=0 THEN 'fulfilled' ELSE 'crashed' END AS result");
 try{
  for(const row of corpus){
   const calls:string[]=[],errors:string[]=[],prior=console.error;
   console.error=(...values:unknown[])=>{errors.push(values.join(" "));};
   try{await handler.call({completedTasks:new Set(row.completed?["course:dev"]:[]),runningContinuousTasks:new Map([["course:dev",{stoppingReason:row.stoppingReason}]]),initializingTaskIds:new Set(row.selected?["course:dev"]:[]),isContinuousTaskNeeded:()=>row.needed,completeContinuousTask:async(_task:unknown,_group:unknown,_owns:unknown,reason:string)=>{calls.push(reason);}},row.code,{id:"course:dev"},1,true);}finally{console.error=prior;}
   const actual=calls[0]??"none",independent=oracle.get(Number(row.completed),row.stoppingReason,row.code,Number(row.needed),Number(row.selected)) as {result:string};
   expect(actual,row.name).toBe(row.expected);
   expect(independent.result,row.name).toBe(row.expected);
   expect(calls.length,row.name).toBe(row.expected==="none"?0:1);
   expect(errors.length,row.name).toBe(row.expected==="crashed"?1:0);
  }
 }finally{database.close();}
});
