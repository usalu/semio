import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{dependencyReadmissionV1,dependencyReadmissionCodeV1}from"../../../../../🏗️build/📥️source/🔏️closure/🔁️dependencies/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const library=resolve(import.meta.dirname,"../../../../.."),bootstrap=resolve(library,"⚡️caching/🚀️bootstrap/📜️script.ts"),rootModule=resolve(library,"🟨️.mjs");
test("actual Nx source delegation requests authenticated readmission before preparation or launch and preserves stable identity refusal",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);const source=readFileSync(bootstrap,"utf8"),tree=ts.createSourceFile(bootstrap,source,ts.ScriptTarget.Latest,true),blocks:ts.Block[]=[];
 const visit=(node:ts.Node)=>{if(ts.isBlock(node)&&node.statements.some(statement=>ts.isVariableStatement(statement)&&statement.declarationList.declarations.some(declaration=>declaration.name.getText(tree)==="admitProjection")))blocks.push(node);ts.forEachChild(node,visit);};visit(tree);expect(blocks).toHaveLength(1);
 const statements=blocks[0]!.statements,start=statements.findIndex(statement=>ts.isVariableStatement(statement)&&statement.declarationList.declarations.some(declaration=>declaration.name.getText(tree)==="admitProjection")),selected=statements.slice(start,start+3).map(statement=>statement.getText(tree)).join("\n").replaceAll("import.meta.url","moduleURL"),code=ts.transpileModule(selected,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText;
 const plugin=ts.createSourceFile(rootModule,readFileSync(rootModule,"utf8"),ts.ScriptTarget.Latest,true),dependency=plugin.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="createDependencies")as ts.FunctionDeclaration,guard=dependency.body!.statements.slice(0,2).map(statement=>statement.getText(plugin)).join("\n"),guardCall=new Function("nxBuildDependencyImplementationV1","process",guard+";return true;");
 const database=new Database(":memory:");try{for(const row of fixture.cases){const oracle=database.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) batch,json_each(batch.value) changed) readmitted").get(JSON.stringify(row.changes))as{readmitted:number};expect(Boolean(oracle.readmitted),row.name).toBe(row.readmitted);let reads=0,identityCalls=0,readmitted=false,accepted=false;
  const api={nxBuildDependencyImplementationChangesV1:()=>row.changes[Math.min(reads++,row.changes.length-1)]!,nxBuildDependencyImplementationV1:()=>{identityCalls++;return"a".repeat(64);}};
  try{const gate=new Function("createRequire","moduleURL","check","dependencyReadmissionV1",code+";return{admitProjection,implementationIdentity};")(()=>()=>api,"file:///owned/module.ts",()=>{},dependencyReadmissionV1)as{admitProjection():void;implementationIdentity:string};gate.admitProjection();try{accepted=guardCall(()=>"a".repeat(64),{env:{SEMIO_NX_IMPLEMENTATION_IDENTITY:row.environment}});}catch(error){expect(String(error)).toContain("Original private Nx dependency implementation differs");}}catch(error){readmitted=dependencyReadmissionCodeV1(error)===76;expect(readmitted).toBe(true);}
  expect(readmitted,row.name).toBe(row.readmitted);expect(identityCalls,row.name).toBe(row.identityCalls);expect(accepted,row.name).toBe(row.guardAccepted);
 }}finally{database.close();}
 const launch=source.slice(source.indexOf("const launch ="),source.indexOf("const interrupt =",source.indexOf("const launch =")));expect(launch.indexOf("admitProjection()")).toBeGreaterThanOrEqual(0);expect(launch.indexOf("admitProjection()")).toBeLessThan(launch.indexOf("spawnNxProcess("));expect(source).toContain('output.text().includes("Original private Nx dependency implementation differs"))admitProjection()');
});
