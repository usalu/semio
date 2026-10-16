import{test,expect}from"bun:test";
import ts from"typescript";
import{Database}from"bun:sqlite";
import{validateAsyncTaskReturnLift,closedBrowserComponentFactory}from"../../📜️script.ts";
import corpus from"../🧫️fixtures/🪝️async-task-return/🔣️.json"with{type:"json"};

test("neutral browser Source guards preserve actual task-return corpus and indexed AST rejection",async()=>{
 const db=new Database(":memory:");try{db.exec("create table calls(kind text)");for(const source of corpus.admitted){const parsed=ts.createSourceFile("oracle.js",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);db.exec("delete from calls");function visit(node:ts.Node):void{if(ts.isCallExpression(node)&&node.expression.getText(parsed)==="taskReturn.bind")db.query("insert into calls values(?)").run("return");ts.forEachChild(node,visit);}visit(parsed);expect(validateAsyncTaskReturnLift(source)).toBe(corpus.bindings);expect(validateAsyncTaskReturnLift(source)).toBe((db.query("select count(*) count from calls").get()as{count:number}).count);}for(const source of corpus.denied)expect(()=>validateAsyncTaskReturnLift(source)).toThrow("unflattenable async result requires an indirect task.return");expect(validateAsyncTaskReturnLift("")).toBe(0);
 const core={name:"fixture.core.wasm",bytes:Uint8Array.from([0,97,115,109,1,0,0,0])};for(const source of["export function instantiate(){}","export function instantiate(getCoreModule,imports){}","export function instantiate(getCoreModule,imports,instantiateCore){getCoreModule();}"])await expect(closedBrowserComponentFactory(source,[core])).rejects.toThrow("browser component factory");console.log("[DEBUG] Existing neutral browser task-return corpus agrees with TypeScript AST and SQLite; missing indexed Source nodes refuse before any compiler or Native work");
 }finally{db.close();}
});
