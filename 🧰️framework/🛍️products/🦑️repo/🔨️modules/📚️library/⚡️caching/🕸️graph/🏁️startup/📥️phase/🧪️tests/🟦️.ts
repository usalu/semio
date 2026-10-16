import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🧫️fixtures/🔣️.json"with{type:"json"};

const paths={bootstrap:resolve(import.meta.dir,"../../../../🚀️bootstrap/📜️script.ts"),service:resolve(import.meta.dir,"../../🚦️service/🟦️.ts"),startup:resolve(import.meta.dir,"../../🟦️.ts")};
function calls(path:string,scope:string,callee:string):ts.CallExpression[]{
 const source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),result:ts.CallExpression[]=[];
 const enclosing=(node:ts.Node):string|undefined=>{for(let current=node.parent;current;current=current.parent)if((ts.isFunctionDeclaration(current)||ts.isMethodDeclaration(current))&&current.name)return current.name.getText(source);return undefined;};
 const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(source)===callee&&enclosing(node)===scope)result.push(node);ts.forEachChild(node,visit);};visit(source);return result;
}
/** 🪢️ Closed Source edge DATA and independent SQLite set difference agree on every admitted carried phase. */
test("neutral startup carried phase follows actual returned owner callbacks",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const database=new Database(":memory:");database.exec("create table expected(edge integer,ordinal integer,phase text);create table observed(edge integer,ordinal integer,phase text)");
 try{for(const[index,row]of corpus.edges.entries()){const actual=calls(paths[row.owner as keyof typeof paths],row.scope,row.callee).map(call=>call.arguments[row.phaseIndex]?.getText()??"<missing>").sort(),expected=[...row.expected].sort();for(const[ordinal,phase]of expected.entries())database.query("insert into expected values(?,?,?)").run(index,ordinal,phase);for(const[ordinal,phase]of actual.entries())database.query("insert into observed values(?,?,?)").run(index,ordinal,phase);expect(actual,row.owner+":"+row.scope+"→"+row.callee).toEqual(expected);}expect(database.query("select count(*) mismatch from (select * from expected except select * from observed union all select * from observed except select * from expected)").get()).toEqual({mismatch:0});}finally{database.close();}
});
