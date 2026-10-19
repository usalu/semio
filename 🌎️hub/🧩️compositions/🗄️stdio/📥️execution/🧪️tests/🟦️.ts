/** 🧪️ Execution: 1 test covering neutral Stdio commands retain original phase and bounded producer custody. */
import{test,expect}from"bun:test";
import{readdirSync,readFileSync}from"node:fs";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import policy from"../🔣️policy.json";
import schema from"../🧬️schema/🔣️.json";
import{runStdioOwnedCommandV1}from"../🟦️.ts";

test("neutral Stdio commands retain original phase and bounded producer custody",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(policy)).toBe(true);expect(validate({...policy,maximumBytes:policy.maximumBytes+1})).toBe(false);expect(validate({...policy,legacy:true})).toBe(false);
 const db=new Database(":memory:");try{expect(db.query("select json_extract(?, '$.maximumBytes') as bytes").get(JSON.stringify(policy))).toEqual({bytes:16777216});}finally{db.close();}
 await expect(Reflect.apply(runStdioOwnedCommandV1,undefined,[Object.freeze({}),Object.freeze({}),"cargo",["check"],"/unowned",{}])).rejects.toThrow();
 const scripts=fileURLToPath(new URL("../../📦️packages/🦀️rust/",import.meta.url)),walk=(directory:string):string[]=>readdirSync(directory,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?(["dist","node_modules","target"].includes(entry.name)?[]:walk(join(directory,entry.name))):entry.name.endsWith(".ts")?[join(directory,entry.name)]:[]);
 const source=walk(scripts).map(file=>readFileSync(file,"utf8")).join("\n");let raw=0,owned=0;
 for(const file of walk(scripts)){const ast=ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true);const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)){if(node.expression.text==="spawn")raw++;if(node.expression.text==="runStdioOwnedCommandV1")owned++;}ts.forEachChild(node,visit);};visit(ast);}
 expect(raw).toBe(0);expect(owned).toBeGreaterThan(0);expect(source).toContain("withCapturedCargoArtifactsTaskV1");expect(source).not.toContain("const started = Date.now()");
});
