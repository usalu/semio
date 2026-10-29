import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{withCargoCompilerInputsV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️inputs/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{resolve,dirname,join}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import vectors from"./framework.json";
import schema from"./framework.schema.json";

declareBuildTestEnvelopeV1({owner:"test:framework-final-cargo-inputs",maximumElapsedMilliseconds:60000,capabilities:{}});
const workspace=(()=>{let p=import.meta.dir;while(p!==dirname(p)&&!require("node:fs").existsSync(join(p,"rust-toolchain.toml")))p=dirname(p);return p;})();
test("framework type generation acquires complete original inputs before offline metadata and retains them through consumption",async()=>{
 expect(new Ajv({strict:true}).validate(schema,vectors)).toBe(true);
 const path=join(workspace,"🧰️framework/📦️packages/🦀️rust/📜️script.ts"),tree=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),node=tree.statements.find(n=>ts.isFunctionDeclaration(n)&&n.name?.text==="captureCargo");
 expect(node).toBeDefined();
 const code=ts.transpileModule(node!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:");
 try{db.exec("create table events(id text,position integer,event text)");
 for(const row of vectors){
  const events:string[]=[],primary=Error(row.id),original={},control={remainingMilliseconds:()=>5000},manifest="/owned/framework/Cargo.toml",env={SEMIO_TYPEGEN_OUT:"/owned/mirror.ts",ORIGINAL_OFFLINE:String(row.offline)},admitted={...env,ADMITTED:"true"},actualArgs=["test","exports_typescript_bindings","--manifest-path",manifest,"--config","original-profile"];
  let cached=row.cached,held=false;
  const withCargoInputs=async(request:any,body:Function)=>{events.push("inputs");expect(request.arguments).not.toContain("--offline");expect(request.environment.ORIGINAL_OFFLINE).toBe(String(row.offline));expect(request.manifest).toBe(manifest);try{if(!cached&&row.offline)throw primary;cached=true;held=true;return await body({arguments:actualArgs,environment:admitted,control,withWriter:async(body:Function)=>{expect(held).toBe(true);events.push("writer");return body(control);}});}finally{held=false;events.push("retire");}};
  const storage={recheck:async()=>{}},store={environment:env,captureDirectory:"/owned/captures",storage,sourceBoundary:{},withCargoInputs};
  const bindings={join,readCargoTestPolicyV1:()=>({mode:"release",buildBudgetMs:5000,manifestPath:manifest,retainArtifacts:false}),scriptCompilerAdmissionControlV1:()=>control,
   withAdmittedCompilerStoreV1:async(_workspace:unknown,_mode:unknown,_control:unknown,_env:unknown,body:Function)=>body(store,control),
   withCargoCompilerInputsV1:async(request:any,port:Function,body:Function)=>port(request,body),
   captureCargoCompilationGraphV1:async(request:any)=>{events.push("metadata");expect(cached,"missing archive before offline metadata").toBe(true);expect(held,"registry custody before metadata").toBe(true);expect(request.args).toEqual(actualArgs);expect(request.environment.ADMITTED).toBe("true");return{};},
   actualCargoCompilerPlanningPortV1:()=>({}),admitCargoSourceBoundariesV1:async()=>{events.push("source");expect(held).toBe(true);},actualCargoSourceAdmissionPortV1:()=>({}),
   mkdtempSync:()=>"/owned/output",rmSync:()=>{},copyFileSync:()=>{events.push("consume");expect(held).toBe(true);},
   captureOwnedProcess:async(_program:unknown,args:readonly string[],options:any)=>{events.push("compile");expect(held).toBe(true);expect(args).toContain("--locked");expect(args).toContain("--offline");expect(options.env.ADMITTED).toBe("true");expect(options.invocation).toBe(original);if(row.compileFails)throw primary;return{reason:"exit",status:0,signal:null,stdout:"",stderr:""};},
   process:{stdout:{write:()=>{}},stderr:{write:()=>{}}}
  },run=new Function(...Object.keys(bindings),code+";return captureCargo;")(...Object.values(bindings));
  if(row.offline&&!row.cached||row.compileFails)await expect(run(original,"/owned","/owned/framework",["test","exports_typescript_bindings"],env)).rejects.toBe(primary);
  else expect(await run(original,"/owned","/owned/framework",["test","exports_typescript_bindings"],env)).toBe(0);
  expect(events).toEqual(row.events);expect(held).toBe(false);events.forEach((event,index)=>db.query("insert into events values(?,?,?)").run(row.id,index,event));
  if(row.events.includes("compile"))expect((db.query("select (select position from events where id=? and event='inputs') < (select position from events where id=? and event='metadata') and (select position from events where id=? and event='compile') < (select position from events where id=? and event='retire') accepted").get(row.id,row.id,row.id,row.id)as{accepted:number}).accepted).toBe(1);
 }
 }finally{db.close();}
});

test("generic compiler store binds dependency acquisition to the original repository network and final request",async()=>{
 const path=join(workspace,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🔌️provider/🟦️.ts"),tree=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
 const statement=tree.statements.find(n=>ts.isVariableStatement(n)&&n.declarationList.declarations.some(d=>ts.isIdentifier(d.name)&&d.name.text==="provider"))as ts.VariableStatement;
 const value=statement.declarationList.declarations[0]!.initializer as ts.ObjectLiteralExpression,property=value.properties.find(n=>ts.isPropertyAssignment(n)&&n.name.getText(tree)==="withCompilerStore")as ts.PropertyAssignment;
 const code=ts.transpileModule("const run="+property.initializer.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 for(const offline of[false,true]){
  const original={},parent={},phase={},storage={},environment={ORIGINAL:"held"},args=["test","exports_typescript_bindings"],request={manifest:"/repository/framework/Cargo.toml",cwd:"/repository/framework",arguments:args,environment,control:phase};let calls=0;
  const bindings={Object,receiveScriptProcessInvocation:async(env:unknown,body:Function)=>{expect(env).toBe(environment);return body(original);},
   withRepositoryCompilerStoreV1:async(root:string,mode:string,control:unknown,env:unknown,body:Function)=>{expect(root).toBe("/repository/framework");expect(mode).toBe("release");expect(control).toBe(parent);expect(env).toBe(environment);return body({storage,environment},phase);},
   readNativeCallerAuthorityV1:()=>({repositoryRoot:"/repository"}),repositoryCargoNetworkV1:(actual:unknown,root:string)=>{expect(actual).toBe(original);expect(root).toBe("/repository");return{offline};},
   repositoryCargoConsumptionV1:(actual:unknown,producer:string)=>{expect(actual).toBe(args);expect(producer).toBe("artifact");return{kind:"compiler",producer,operation:"test"};},
   repositoryCargoCompilerInputsV1:(options:any)=>{expect(options).toEqual({workspace:"/repository",invocation:original,control:phase,storage,network:{offline},consumption:{kind:"compiler",producer:"artifact",operation:"test"}});return(actual:unknown,body:Function)=>{expect(actual).toBe(request);calls++;return body({arguments:args,environment,control:phase});}}
  },run=new Function(...Object.keys(bindings),code+";return run;")(...Object.values(bindings));
  expect(await run("/repository/framework","release",parent,environment,async(store:any,control:unknown)=>{expect(control).toBe(phase);return store.withCargoInputs(request,async(view:any)=>{expect(view.arguments).toBe(args);return"captured";});})).toBe("captured");
  expect(calls).toBe(1);
 }
});

test("framework generator links the actual Cargo input continuation before loading its task router",async()=>{
 const path=join(workspace,"🧰️framework/📦️packages/🦀️rust/📜️script.ts"),tree=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
 const declaration=tree.statements.find(n=>ts.isImportDeclaration(n)&&n.importClause?.namedBindings&&ts.isNamedImports(n.importClause.namedBindings)&&n.importClause.namedBindings.elements.some(e=>(e.propertyName??e.name).text==="withCargoCompilerInputsV1"))as ts.ImportDeclaration;
 expect(declaration).toBeDefined();
 const target=resolve(dirname(path),(declaration.moduleSpecifier as ts.StringLiteral).text),implementation=await import(target);
 expect(typeof implementation.withCargoCompilerInputsV1).toBe("function");
 let calls=0;
 await expect(implementation.withCargoCompilerInputsV1({manifest:"/unissued/Cargo.toml",cwd:"/unissued",arguments:[],environment:{},control:{}},undefined,async()=>{calls++;})).rejects.toThrow();
 expect(calls).toBe(0);
 await withBuildTestInvocationV1(async original=>{
  const control=createIssuedResourceControlV1(original,"test:linked-cargo-inputs",5000),events:string[]=[],environment={ORIGINAL:"held"},arguments_=["test","exports_typescript_bindings"];
  const result=await implementation.withCargoCompilerInputsV1({manifest:"/source/Cargo.toml",cwd:"/source",arguments:arguments_,environment,control},async(request:any,consume:Function)=>{expect(request.control).toBe(control);events.push("inputs");try{return await consume({...request,writer:async(phase:unknown,body:Function)=>{events.push("writer");return body(phase);}});}finally{events.push("retire");}},async(actual:any)=>{expect(actual.arguments).toBe(arguments_);expect(actual.environment).toBe(environment);return actual.withWriter(async()=>{events.push("consume");return "complete";});});
  expect(result).toBe("complete");expect(events).toEqual(["inputs","writer","consume","retire"]);
  const db=new Database(":memory:");try{db.exec("create table phases(position integer primary key,event text unique)");events.forEach((event,index)=>db.query("insert into phases values(?,?)").run(index,event));expect((db.query("select (select position from phases where event='inputs') < (select position from phases where event='writer') and (select position from phases where event='writer') < (select position from phases where event='consume') and (select position from phases where event='consume') < (select position from phases where event='retire') accepted").get()as{accepted:number}).accepted).toBe(1);}finally{db.close();}
 });
});
