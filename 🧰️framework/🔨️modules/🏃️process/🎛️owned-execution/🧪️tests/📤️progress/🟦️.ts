import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve,join} from "node:path";
import Ajv from "ajv";
import JSON5 from "json5";
import {build} from "esbuild";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/🟦️.ts";
const owner=resolve(import.meta.dir,"../.."),fixture=JSON.parse(readFileSync(join(owner,"🧫️fixtures/📤️progress/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(join(owner,"🧬️schema/📤️progress/🔣️.json"),"utf8"));
test("explicit progress output preserves canonical stdout against the independent process oracle",async()=>{
 expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const source=`setTimeout(()=>process.stdout.write(${JSON.stringify(fixture.expectedStdout)}),${fixture.delayMs})`,invocation=`try{await runOwnedCommand("node",["-e",${JSON.stringify(source)}],process.cwd(),${JSON.stringify(fixture.label)},${fixture.budgetMs},{onProgress:line=>process.stderr.write(line+"\\n")});}catch(error){console.error(error);process.exit(1)}`;
 let code=`const{runOwnedCommand}=await import(${JSON.stringify(join(owner,"🟦️.ts"))});${invocation}`;
 if(process.env.SEMIO_OWNED_PROGRESS_SOURCE_PAIRS){const pair=JSON.parse(readFileSync(process.env.SEMIO_OWNED_PROGRESS_SOURCE_PAIRS,"utf8")).sources[0],bundle=await build({stdin:{contents:pair.before,loader:"ts",resolveDir:owner},bundle:true,write:false,platform:"node",format:"iife",globalName:"progressModule",logLevel:"silent"});code=bundle.outputFiles![0]!.text+`\nconst{runOwnedCommand}=progressModule;${invocation}`;}
 const independent=Bun.spawn(["node","-e",source],{stdout:"pipe",stderr:"pipe"}),owned=Bun.spawn([process.execPath,"--eval",code],{stdout:"pipe",stderr:"pipe"});
 const [oracleStatus,status,oracleStdout,stdout,stderr]=await Promise.all([independent.exited,owned.exited,new Response(independent.stdout).text(),new Response(owned.stdout).text(),new Response(owned.stderr).text()]);
 expect(oracleStatus).toBe(0);expect(status).toBe(0);expect(stdout).toBe(oracleStdout);expect(stdout).toBe(fixture.expectedStdout);expect(JSON5.parse(stdout)).toEqual(JSON.parse(fixture.expectedStdout));expect(stderr).toContain(`[${fixture.label}] running elapsedMs=`);
 console.log(`[DEBUG] neutral progress stdout=canonical output=${fixture.expectedChannel}`);
},20000);
