import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import ts from"typescript";
import rows from"./📥️.json" with{type:"json"};
import schema from"./🔣️.json" with{type:"json"};
import caching from"../../../../⚡️caching/🗂️suites/🔣️.json" with{type:"json"};
import build from"../../🔣️.json" with{type:"json"};

test("actual Python oracle suites declare pinned preparation on their genuine canonical owner",()=>{
 expect(new Ajv({strict:true}).validate(schema,rows)).toBe(true);const db=new Database(":memory:");try{for(const row of rows){const suite=(row.owner==="cache"?caching:build).suites.find(s=>s.id===row.id)!;expect(suite.platforms).toContain("win32");const oracle=db.query("select exists(select 1 from json_each(?1) where value='python') bound").get(JSON.stringify((suite as{tools?:string[]}).tools??[]))as{bound:number};expect(Boolean(oracle.bound),row.id).toBe(true);}}finally{db.close();}
});

test("both canonical suite owners query the declared managed workspace before issuing children",()=>{
 for(const url of[new URL("../../../../⚡️caching/📜️script.ts",import.meta.url),new URL("../../📜️script.ts",import.meta.url)]){const text=readFileSync(url,"utf8"),tree=ts.createSourceFile(url.pathname,text,ts.ScriptTarget.Latest,true),calls:string[]=[],bindings:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(tree).endsWith("workspaceTestPythonEnvironmentV1"))calls.push(node.expression.getText(tree));if(ts.isCallExpression(node)&&node.expression.getText(tree)==="runProcessTestSuiteV1"){const arg=node.arguments[0];if(arg&&ts.isObjectLiteralExpression(arg))for(const property of arg.properties)if(property.name?.getText(tree)==="environment")bindings.push(ts.isShorthandPropertyAssignment(property)?property.name.getText(tree):ts.isPropertyAssignment(property)?property.initializer.getText(tree):"");}ts.forEachChild(node,visit);};visit(tree);expect((tree as unknown as{parseDiagnostics:readonly unknown[]}).parseDiagnostics,url.pathname).toHaveLength(0);expect(calls,url.pathname).toHaveLength(1);expect(bindings,url.pathname).toEqual(["environment"]);}
});

test("actual implicated oracle consumers require the canonical pinned port without host fallbacks",()=>{
 for(const row of rows)for(const file of row.consumers){const base=row.owner==="cache"?new URL("../../../../⚡️caching/",import.meta.url):new URL("../../../",import.meta.url),url=new URL(file,base),text=readFileSync(url,"utf8"),tree=ts.createSourceFile(file,text,ts.ScriptTarget.Latest,true);expect((tree as unknown as{parseDiagnostics:readonly unknown[]}).parseDiagnostics,file).toHaveLength(0);const calls:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(tree)==="requirePinnedTestToolV1"&&node.arguments[0]?.getText(tree)==='"python"')calls.push(node.expression.getText(tree));ts.forEachChild(node,visit);};visit(tree);expect(calls.length,file).toBeGreaterThan(0);expect(text,file).not.toMatch(/testPythonV1|SEMIO_PYTHON|requireTestToolV1\("python"\)|\?\?"python3"/);}
});

import queryRows from"./🧭️query.json" with{type:"json"};
import querySchema from"./🧭️schema.json" with{type:"json"};
import{workspaceTestPythonArgumentsV1,readWorkspaceTestPythonOutputV1,requireWorkspaceTestPythonSuccessV1,workspaceTestPythonVariablesV1}from"../../🐍️python/🟦️.ts";

test("managed standalone discovery uses a bounded read-only query",()=>{
 expect(workspaceTestPythonArgumentsV1("3.14")).toEqual(["python","find","3.14","--system","--managed-python","--no-project","--no-python-downloads"]);
 expect(new Ajv({strict:true}).validate(querySchema,queryRows)).toBe(true);const db=new Database(":memory:");try{for(const row of queryRows){const path=row.output.trim().replaceAll("\\","/");const base=row.directory.replaceAll("\\","/");const normalized=path.replace(/\/[^/]+\/\.\./gu,"");const oracle=db.query("select substr(?1,1,length(?2)+1)=?2||'/' valid").get(normalized,base)as{valid:number};expect(Boolean(oracle.valid)).toBe(row.valid);if(row.valid)expect(readWorkspaceTestPythonOutputV1(row.output,row.directory,row.platform)).toBe(row.output.trim());else expect(()=>readWorkspaceTestPythonOutputV1(row.output,row.directory,row.platform)).toThrow();}}finally{db.close();}
});

test("canonical query binds the planned managed interpreter without running a project",()=>{
 const text=readFileSync(new URL("../../🐍️python/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("python.ts",text,ts.ScriptTarget.Latest,true),calls:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node))calls.push(node.getText(tree));ts.forEachChild(node,visit);};visit(tree);expect((tree as unknown as{parseDiagnostics:readonly unknown[]}).parseDiagnostics).toHaveLength(0);expect(calls.some(call=>call.startsWith("workspaceTestPythonArgumentsV1(plan.series)"))).toBe(true);expect(calls.some(call=>call.startsWith("readWorkspaceTestPythonOutputV1(result.stdout,plan.interpreters)"))).toBe(true);expect(text).not.toContain("plan.runArguments");
});

test("the registered manager is selected from the verified native uv receipt",()=>{
 const text=readFileSync(new URL("../../🐍️python/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("python.ts",text,ts.ScriptTarget.Latest,true),calls:string[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node))calls.push(node.getText(tree));ts.forEachChild(node,visit);};visit(tree);expect(calls.some(call=>call.startsWith("preparedNativeToolV1(protocol.manager"))).toBe(true);expect(calls.some(call=>call.startsWith("nativeDistributionV1(protocol.manager"))).toBe(true);expect(text).toContain("workspaceTestPythonVariablesV1(environment,plan.variables,managerLocator.name,manager)");expect(text).toContain(".binaries.find(");
});

test("manager discovery failure preserves its bounded diagnostic",()=>{
 const text=readFileSync(new URL("../../🐍️python/🟦️.ts",import.meta.url),"utf8");expect(text).toContain("check:false");expect(text).toContain("requireWorkspaceTestPythonSuccessV1(result,signal)");
});

test("failed and cancelled discovery never issues an interpreter binding",()=>{
 const signal=new AbortController(),result={status:2,signal:null,stderr:"bad\u001b[31m input"};expect(()=>requireWorkspaceTestPythonSuccessV1(result,signal.signal)).toThrow("bad[31m input");signal.abort();expect(()=>requireWorkspaceTestPythonSuccessV1(result,signal.signal)).toThrow();expect(()=>requireWorkspaceTestPythonSuccessV1({status:0,signal:null,stderr:""},new AbortController().signal)).not.toThrow();
});

test("the explicit managed flag does not conflict with an inherited uv preference",()=>{
 const text=readFileSync(new URL("../../🐍️python/🟦️.ts",import.meta.url),"utf8");expect(text).toContain("UV_PYTHON_PREFERENCE:undefined");
});

import{findToolV1}from"../../../../../../../../🔨️modules/🏃️process/🧰️tools/📖️registry/🟦️.ts";
import{toolEnvironmentV1}from"../../../../../../../../🔨️modules/🏃️process/🧰️tools/🌿️environment/🟦️.ts";
test("child environment removes planned and inherited preference",()=>{
 const entry=findToolV1("uv")!;for(const inherited of[undefined,"only-system"]){const ambient={UV_PYTHON_PREFERENCE:inherited},variables=workspaceTestPythonVariablesV1(ambient,{UV_PYTHON_PREFERENCE:"only-managed"},"SEMIO_TOOL_UV","/managed/uv"),child=toolEnvironmentV1(entry,variables,{}, {root:"/workspace",platform:"linux"});expect(Object.hasOwn(child,"UV_PYTHON_PREFERENCE")).toBe(false);expect(child.SEMIO_TOOL_UV).toBe("/managed/uv");}
});

import environmentRows from"./🌿️cases.json" with{type:"json"};
import environmentSchema from"./🌿️schema.json" with{type:"json"};
test("platform environment projection preserves authoritative managed settings",()=>{
 expect(new Ajv({strict:true}).validate(environmentSchema,environmentRows)).toBe(true);const entry=findToolV1("uv")!,db=new Database(":memory:");try{for(const row of environmentRows){const original={...row.ambient},variables=workspaceTestPythonVariablesV1(row.ambient,{UV_PYTHON_PREFERENCE:"only-managed",UV_PYTHON_INSTALL_DIR:"C:/managed"},"SEMIO_TOOL_UV","C:/managed/uv",row.platform),child=toolEnvironmentV1(entry,variables,{}, {root:"/workspace",platform:row.platform as NodeJS.Platform});const keys=Object.keys(child);const oracle=db.query("select exists(select 1 from json_each(?1) where value=?2) found").get(JSON.stringify(keys),"Uv_Python_Preference") as{found:number};expect(Boolean(oracle.found)).toBe(row.preference);expect(child.UV_PYTHON_INSTALL_DIR).toBe(row.root);expect(Object.hasOwn(child,"uv_python_install_dir")).toBe(row.alias);expect(Object.hasOwn(child,"sEmIo_ToOl_Uv")).toBe(row.alias);expect(row.ambient).toEqual(original);}}finally{db.close();}
});
