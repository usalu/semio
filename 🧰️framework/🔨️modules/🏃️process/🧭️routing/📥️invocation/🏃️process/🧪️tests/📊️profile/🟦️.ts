import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{findWorkspaceRoot}from"../../../../🟦️.ts";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

test("shared build profiling uses the neutral environment contract",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const root=findWorkspaceRoot(import.meta.dir),source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("issued-process.ts",source,ts.ScriptTarget.Latest,true),conditions:ts.Expression[]=[];
 const visit=(node:ts.Node)=>{if(ts.isIfStatement(node)&&node.expression.getText(tree).includes("PROFILE_INPUTS"))conditions.push(node.expression);ts.forEachChild(node,visit);};visit(tree);expect(conditions).toHaveLength(1);
 const enabled=new Function("process","return Boolean("+conditions[0]!.getText(tree)+");")as(process:{env:Record<string,string|undefined>})=>boolean,db=new Database(":memory:");
 try{for(const row of corpus.cases){const environment:Record<string,string|undefined>={[corpus.neutralName]:row.neutral??undefined,[corpus.retiredName]:row.retired??undefined},oracle=db.query("SELECT CASE WHEN json_extract(?1,'$.neutral')='1' THEN 1 ELSE 0 END enabled").get(JSON.stringify(row))as{enabled:number};expect(Boolean(oracle.enabled),row.name).toBe(row.enabled);expect(enabled({env:environment}),row.name).toBe(row.enabled);}}finally{db.close();}
 const library=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),policy=JSON.parse(readFileSync(join(library,"🔌️nx-plugin/🌍️environment/📋️policy/🔣️.json"),"utf8"))as{taskOnly:string[]};expect(policy.taskOnly).toContain(corpus.neutralName);expect(policy.taskOnly).not.toContain(corpus.retiredName);
 for(const path of corpus.consumers){const bytes=readFileSync(join(root,path),"utf8");expect(bytes,path).toContain(corpus.neutralName);expect(bytes,path).not.toContain(corpus.retiredName);}
 console.log("[DEBUG] Neutral build profiling actual condition agrees with SQLite across "+corpus.cases.length+" portable cases and "+corpus.consumers.length+" consumer declarations");
});
