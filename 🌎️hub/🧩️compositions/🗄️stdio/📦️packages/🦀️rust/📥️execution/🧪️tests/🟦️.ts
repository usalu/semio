import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
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
 const source=readFileSync(new URL("../../📜️script.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("stdio.ts",source,ts.ScriptTarget.Latest,true);let raw=0,owned=0;
 const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)){if(node.expression.text==="spawn")raw++;if(node.expression.text==="runStdioOwnedCommandV1")owned++;}ts.forEachChild(node,visit);};visit(ast);expect(raw).toBe(0);expect(owned).toBeGreaterThan(0);expect(source).toContain("withCapturedCargoArtifactsTaskV1");expect(source).not.toContain("const started = Date.now()");
});
