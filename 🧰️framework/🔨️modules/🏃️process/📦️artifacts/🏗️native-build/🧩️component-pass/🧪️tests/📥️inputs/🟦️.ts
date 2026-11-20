import{withCargoCompilerInputsV1}from"../../../📥️inputs/🟦️.ts";
import{CargoGraphOperationsV1}from"../../📥️planning/🟦️.ts";
import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import*as routing from"../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../🔒️leases/🟦️.ts";
import corpus from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
test("component Cargo continuation consumes each exact live command view",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("driver.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="withNativeBuildCargoInputsV1");expect(node).toBeDefined();const code=ts.transpileModule(node!.getText(tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,database=new Database(":memory:");
 try{database.exec("CREATE TABLE args(position INTEGER PRIMARY KEY,value TEXT)");for(const row of corpus.cases){database.exec("DELETE FROM args");row.arguments.forEach((value,index)=>database.query("INSERT INTO args VALUES(?,?)").run(index,value));const reason=Object.freeze(Error("Component input cancelled"));const envelope=routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"component:inputs-law",maximumElapsedMilliseconds:5000},{fixture:"component-inputs"},Date.now()),async original=>{const parent=routing.createIssuedResourceControlV1(original,"component:parent",0),controller=new AbortController(),phase=routing.createRequestedIssuedResourceControlV1(parent,1000,controller.signal),args=[...row.arguments,"--config","owned"],environment={SENTINEL:row.kind},events:string[]=[],options={cwd:"/workspace",storage:{recheck:async()=>{events.push("storage");}},sourceBoundary:{},withCargoInputs:async(request:{arguments:readonly string[];control:unknown},body:(input:{arguments:readonly string[];environment:unknown;control:unknown})=>Promise<unknown>)=>{expect(request.arguments).toEqual(row.arguments);expect(request.control).toBe(parent);events.push("issued");return body({arguments:args,environment,control:phase});}},admitCargoSelectionV1=async(request:{args:readonly string[];environment:unknown},control:unknown)=>{expect(request.args).toEqual(args);expect(request.environment).toBe(environment);routing.requireIssuedResourceControlRefinementV1(phase,control as ResourceLeaseControl);events.push("source");};const run=Function("checkResourceLeaseControl","withCargoCompilerInputsV1","admitCargoSelectionV1",code+";return withNativeBuildCargoInputsV1;")(checkResourceLeaseControl,withCargoCompilerInputsV1,admitCargoSelectionV1);let entered=false;const work=run(options,{manifest:"/workspace/Cargo.toml",cwd:"/workspace",arguments:row.arguments,environment:{SENTINEL:"old"},control:parent},async(input:{arguments:readonly string[];environment:unknown;control:ResourceLeaseControl})=>{entered=true;expect(input.arguments).toEqual([...database.query("SELECT value FROM args ORDER BY position").all().map(value=>(value as {value:string}).value),"--config","owned"]);expect(input.environment).toBe(environment);routing.requireIssuedResourceControlRefinementV1(phase,input.control);if(row.cancel)controller.abort(reason);return row.kind;});if(row.cancel){await expect(work).rejects.toBe(reason);expect(original.control.signal.aborted).toBe(false);}else expect(await work).toBe(row.kind);expect(entered).toBe(true);expect(events).toEqual(["issued","source","storage"]);});if(row.cancel)await expect(envelope).rejects.toBe(reason);else await envelope;}}finally{database.close();}
});

test("component unit graph derives a step from the declared pass configuration",async()=>{
 const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8");
 const tree=ts.createSourceFile("driver.ts",source,ts.ScriptTarget.Latest,true);
 let declaration:ts.VariableDeclaration|undefined;
 const visit=(node:ts.Node)=>{if(ts.isVariableDeclaration(node)&&node.name.getText(tree)==="unitGraphs")declaration=node;ts.forEachChild(node,visit);};
 visit(tree);
 const code=ts.transpileModule("const "+declaration!.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const pass={id:"pass",manifest:"/work/Cargo.toml"},base={SENTINEL:"captured"},build=["build","--keep-going"],graph=["build","--unit-graph"],issued=[...graph,"--config","owned"];
 const inputEnvironment={...base,CARGO_TARGET_DIR:"/target",CARGO_BUILD_BUILD_DIR:"/build"};
 const values={
  plan:{passes:[pass]},options:{cwd:"/work",environment:{...base,SEMIO_COMPILER_PARALLELISM:"3"}},inputEnvironment,
  cpuControl:{},generation:{target:"/target",build:"/build"},policy:{featureUnification:"selected"},
  graphOperationsOwner:{start:(body:()=>unknown)=>body()},nativeBuildPassArgsV1:()=>build,nativeBuildUnitGraphArgsV1:()=>graph,
  cargoConfigurationArgumentsV1:(args:string[])=>args,checkResourceLeaseControl:()=>{},
  withRequestedIssuedResourceControlV1:(_scope:unknown,_deadline:number,body:(control:unknown)=>unknown)=>body({}),
  withNativeBuildCargoInputsV1:(_options:unknown,request:{arguments:string[];environment:unknown},body:(inputs:unknown)=>unknown)=>{
   expect(request.arguments).toEqual(build);expect(request.environment).toEqual(inputEnvironment);
   return body({control:{},environment:base,admitArguments:async(args:string[])=>{expect(args).toEqual(graph);return issued;}});
  },
  readUnitGraph:(_pass:unknown,args:string[],environment:Record<string,string>)=>{
   expect(args).toBe(issued);expect(environment.SENTINEL).toBe("captured");expect(environment.CARGO_RESOLVER_FEATURE_UNIFICATION).toBe("selected");return "captured graph";
  }
 };
 const result=Function(...Object.keys(values),code+";return unitGraphs;")(...Object.values(values));
 expect(await result.get("pass")).toBe("captured graph");
});

test("component batch effects receive the exact consumed view",async()=>{
 const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("driver.ts",source,ts.ScriptTarget.Latest,true);
 const declarations=new Map<string,ts.VariableDeclaration>();
 const collect=(node:ts.Node)=>{if(ts.isVariableDeclaration(node))declarations.set(node.name.getText(tree),node);ts.forEachChild(node,collect);};
 const findPass=(node:ts.Node)=>{if(ts.isVariableDeclaration(node)&&node.name.getText(tree)==="runAdmittedPass"){collect(node);return;}ts.forEachChild(node,findPass);};
 findPass(tree);
 const args=declarations.get("args")!,child=declarations.get("child")!;
 expect(child.initializer?.getText(tree).startsWith("spawn(")).toBe(true);
 const original={SENTINEL:"issued"},expected=["build","-p","subset","--config","owned"];
 const values={inputs:{arguments:expected,environment:original},pass:{id:"selected",profile:"release"},options:{cwd:"/workspace",environment:{SENTINEL:"stale"}},generation:{build:"/build",target:"/target"},effectControl:{},port:{command:"cargo",args:[]},tokens:3,profileBuildModeV1:()=>"release",jobserver:{environment:()=>({})},scheduler:"/scheduler",ADMISSION_POLICY_V1:{brokerEnvironment:"BROKER",passEnvironment:"PASS"},broker:{endpoint:"held"},policy:{featureUnification:"selected"},join:(...parts:string[])=>parts.join("/"),nativeBuildLaneRootV1:()=>"/lane",compilerWrapperEnvironmentV1:()=>({}),scriptResourceProcessEnvironmentV1:(_control:unknown,environment:unknown)=>environment,spawn:(command:string,argv:string[],options:{env:Record<string,string>})=>{expect(command).toBe("cargo");expect(argv).toEqual(expected);expect(options.env.SENTINEL).toBe("issued");expect(options.env.SEMIO_COMPILER_PARALLELISM).toBe("3");return "child";}};
 const code=ts.transpileModule("const "+args.getText(tree)+";const "+child.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 expect(Function(...Object.keys(values),code+";return child;")(...Object.values(values))).toBe("child");
});
