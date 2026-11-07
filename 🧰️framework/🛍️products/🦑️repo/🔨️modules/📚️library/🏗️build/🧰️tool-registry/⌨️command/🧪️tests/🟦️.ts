import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import fixture from"../🧫️fixtures/🔣️.json" with{type:"json"};
import{requiresNativeToolchainScopeV1,runToolsCommandV1}from"../🟦️.ts";
import{findToolV1,toolCommandNameV1,toolForCommandV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import{buildCapacityAskV1}from"../../../🌡️host-capacity/🟦️.ts";

test("only Windows native compiler executions require held toolchain scope",()=>{
 const db=new Database(":memory:");try{for(const row of fixture.cases){const command=row.args[0],raw=row.args[1]==="--cwd"?row.args[3]:row.args[1],entry=raw?(findToolV1(raw)??toolForCommandV1(raw)):undefined,oracle=db.query("select ?1='win32' and ?2='exec' and (?3='cc' or (?3='cargo' and ?4='rustc')) accepted").get(row.platform,command,entry?.id??null,raw?toolCommandNameV1(raw):null) as{accepted:number};expect(Boolean(oracle.accepted)).toBe(row.scoped);expect(requiresNativeToolchainScopeV1(row.args,row.platform)).toBe(row.scoped);}}finally{db.close();}
});

test("captured compiler tool execution requests outer host capacity",()=>{
 const db=new Database(":memory:");try{for(const args of [["tools","exec","cc","--","--version"],["tools","exec","rustc","--","--version"],["preflight"]]){const ask=buildCapacityAskV1(args,{}),oracle=db.query("select 'command' operation").get() as{operation:string};expect(ask?.operation).toBe(oracle.operation);}}finally{db.close();}
});

test("an already cancelled tool command stops before toolchain effects",async()=>{
 const db=new Database(":memory:"),controller=new AbortController(),reason=Error("cancelled before admission");controller.abort(reason);try{const oracle=db.query("select 0 started").get() as{started:number};expect(oracle.started).toBe(0);await expect(runToolsCommandV1(["exec","cc","--","--version"],{environment:process.env,write:()=>{},signal:controller.signal})).rejects.toBe(reason);}finally{db.close();}
});
