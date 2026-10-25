import{test,expect}from"bun:test";
import{cargoPlanningOutputBytesV1}from"../../../🕸️units/📥️capture/🟦️.ts";
import{readFileSync}from"node:fs";
import{join,resolve,relative}from"node:path";
import{StringDecoder}from"node:string_decoder";
import{isDeepStrictEqual}from"node:util";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import schema from"./🧬️schema.json";
import corpus from"./🔣️.json";
import{withCargoCompilerInputsV1,type CargoCompilerInputsPortV1,type CargoCompilerInputsViewV1}from"../../🟦️.ts";
import{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,requireIssuedResourceControlV1,withIssuedResourceOperationV1}from"../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../🔒️leases/🟦️.ts";
const processRoot=resolve(import.meta.dir,"../../../../.."),cargoPath=join(processRoot,"🧪️testing/🦀️cargo/🟦️.ts"),wasmPath=join(processRoot,"📦️artifacts/🕸️wasm-build/🟦️.ts");
type Row=(typeof corpus.cases)[number];
type Invocation=(...values:unknown[])=>Promise<void>;
type Settlement=Readonly<{status:"settled"}|{status:"rejected";reason:unknown}>;
type Execution=Readonly<{trace:readonly string[];failed:{error:unknown}|undefined;settlement:Settlement;primary:unknown;releaseFailure:unknown;occurrences:number}>;
class ConsumptionRefusal extends Error{readonly port:string;readonly received:readonly string[];readonly expected:readonly string[];
                                       constructor(port:string,received:readonly string[],expected:readonly string[]){super("Model refused unconsumed arguments at "+port);this.port=port;this.received=received;this.expected=expected;}}
type LeaseRequest={args:readonly string[];control:ResourceLeaseControl;signal?:AbortSignal};
const consumedArguments=(kind:string)=>kind==="wasm"?"port.admit(actual.arguments,":"admitCargoExecutionV1(request, policy, admission, actual.arguments)",substitutedArguments=(marker:string)=>marker.replace("actual.arguments",'["unconsumed"]');
function extract(path:string,names:readonly string[],ports:Readonly<Record<string,unknown>>,kind:string,occurrence?:number,mutated:(occurrences:number)=>void=()=>{}):Invocation{
 const source=readFileSync(path,"utf8"),file=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true,ts.ScriptKind.TS),parts=names.map(name=>{const node=file.statements.find((item):item is ts.FunctionDeclaration=>ts.isFunctionDeclaration(item)&&item.name?.text===name);if(!node)throw Error("Actual Core function required: "+name);return node.getText(file);});
 let text=ts.transpileModule(parts.join("\n"),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText.replaceAll("export ","");if(occurrence!==undefined){const marker=consumedArguments(kind),occurrences=text.split(marker).length-1;mutated(occurrences);if(occurrence<occurrences){let at=-1;for(let seen=0;seen<=occurrence;seen++)at=text.indexOf(marker,at+1);text=text.slice(0,at)+substitutedArguments(marker)+text.slice(at+marker.length);}}
 return new Function(...Object.keys(ports),text+";return "+names.at(-1))(...Object.values(ports)) as Invocation;
}
async function execute(row:Row,occurrence?:number):Promise<Execution>{
 const trace:string[]=[];let occurrences=0,failed:{error:unknown}|undefined,primary:unknown,releaseFailure:unknown,settlement:Settlement={status:"settled"};
 try{await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"core-cargo-consumer-model",maximumElapsedMilliseconds:5000},{workspaceRoot:process.cwd()},Date.now()),async original=>{
  const parent=createIssuedResourceControlV1(original,"compiler:fixture",0),views=new Map<number,CargoCompilerInputsViewV1>(),numbers=new Map<ResourceLeaseControl,number>();let issued=0;
  primary=row.outcome==="undefined"?undefined:Object.freeze(Error("frozen compiler primary"));releaseFailure=Object.freeze(Error("modeled release failure"));
  const view=(index:number)=>{const actual=views.get(index);if(!actual)throw Error("Actual consumed input view required");return actual;};
  const withInputs:CargoCompilerInputsPortV1=async(request,body)=>{requireIssuedResourceControlRefinementV1(parent,request.control);expect(request.manifest).toBe("/model/source/Cargo.toml");expect(request.cwd).toBe("/model/source");if(issued)requireIssuedResourceControlRefinementV1(view(1).control,request.control);else expect(request.control).toBe(row.kind==="wasm"?request.control:parent);const number=++issued,stop=new AbortController();if(row.outcome==="cancelled")stop.abort(primary);const signal=stop.signal,control=createRequestedIssuedResourceControlV1(request.control,1000,signal),actual={arguments:[...request.arguments,"--config","owned="+number],environment:{...request.environment,ADMITTED:String(number)},control};views.set(number,actual);numbers.set(control,number);trace.push("request:"+number);return body(actual);};
  const number=(control:ResourceLeaseControl)=>{requireIssuedResourceControlV1(control);let result=numbers.get(control);if(!result)for(const[candidate,value]of numbers){try{requireIssuedResourceControlRefinementV1(candidate,control);result=value;break;}catch{}}if(!result)throw Error("Actual consumed control required");checkResourceLeaseControl(control);return result;};
  const exact=(port:string,args:readonly string[],control:ResourceLeaseControl)=>{const index=number(control),expected=view(index).arguments;if(!isDeepStrictEqual(args,expected))throw new ConsumptionRefusal(port,args,expected);return index;};
  const release=()=>{trace.push("release");if(row.releaseFails)throw releaseFailure;};
  const storage={allocation:{kind:"compiler-workspace"},backing:{root:"/model"},recheck:async()=>{}};
  const policy={mode:"prototype",targetDirectory:"/model/target",buildDirectory:"/model/build",artifactDirectory:"/model/artifacts",leaseDirectory:"/model/leases",rustMinStack:"8192",retainArtifacts:false,workspaceRoot:"/model",sourceOwner:"source",cwd:"/model/source",budgetMs:3000,searchPath:"/model/bin",wasmPack:{command:"wasm-pack",args:["prefix"]},bindgen:{command:"wasm-bindgen",args:[]}};
  const env={CARGO_HOME:"/model/home",TMPDIR:"/model/tmp",TMP:"/model/tmp",TEMP:"/model/tmp"};
  const ports:Record<string,unknown>={withIssuedResourceOperationV1,cargoPlanningOutputBytesV1,withCargoCompilerInputsV1,checkResourceLeaseControl,requireIssuedResourceControlV1,createRequestedIssuedResourceControlV1,join,resolve,relative,StringDecoder,console:{log:()=>{},error:()=>{}},
   requireNativeStorageReservationV1:(_:unknown,control:ResourceLeaseControl)=>{if(control!==parent)number(control);},requireNativeStoragePathV1:()=>{},admitPolicy:(value:unknown)=>value,
   cargoTestCompilerArgumentsV1:()=>["test","--no-run"],cargoTestPlanV1:()=>[{phase:"build",args:["test","--locked","--offline"],budgetMs:3000,capture:false}],cargoLintPlanV1:()=>({phase:"build",args:["clippy","--locked","--offline"],budgetMs:3000,capture:false}),cargoTestExecutionArgumentsV1:(step:{args:string[]})=>step.args,
   cargoBuildLeaseIdentityV1:(buildDirectory:string,args:readonly string[])=>({buildDirectory,args}),acquireCargoBuildLeaseV1:async(request:LeaseRequest)=>{requireIssuedResourceControlV1(request.control);expect(request.signal).toBe(request.control.signal);expect(request.args).toEqual(row.kind==="wasm"?["build","--lib","--profile","dev","--target","wasm32-unknown-unknown"]:row.kind==="tests"?["test","--no-run"]:["clippy","--locked","--offline"]);if(row.kind!=="wasm"){expect(request.control).toBe(parent);expect(request.signal).toBe(parent.signal);}trace.push("lease");return{release};},
   captureCargoCompilationGraphV1:async(request:{args:readonly string[];environment:Record<string,string|undefined>},control:ResourceLeaseControl)=>{const index=exact("planning",request.args,control);expect(request.environment.ADMITTED).toBe(String(index));trace.push("planning:"+index);return{};},admitCargoSourceBoundariesV1:async(_:unknown,_root:unknown,_boundary:unknown,control:ResourceLeaseControl)=>{trace.push("source:"+number(control));},actualCargoCompilerPlanningPortV1:()=>({}),actualCargoSourceAdmissionPortV1:()=>({}),
   mkdirSync:()=>{},mkdtempSync:(prefix:string)=>prefix+"owned",writeFileSync:()=>{},rmSync:()=>{trace.push("remove");},startNativeProgress:()=>()=>{trace.push("stop");},
   runBudgetedTestCommand:async(command:string,args:readonly string[],options:{env:Record<string,string|undefined>;signal:AbortSignal;budgetMs:number})=>{const index=issued,view=views.get(index);if(!view)throw Error("Actual input view required");expect(command).toBe("cargo");expect(args).toEqual(["prefix",...view.arguments]);expect(options.signal).toBe(view.control.signal);expect(options.env.ADMITTED).toBe(String(index));if(row.kind==="tests")expect(options.env).toBe(view.environment);else expect(options.env).toEqual({...view.environment,CARGO_TARGET_DIR:policy.targetDirectory,CARGO_BUILD_BUILD_DIR:policy.buildDirectory,RUST_MIN_STACK:policy.rustMinStack});expect(options.budgetMs).toBeGreaterThan(0);expect(options.budgetMs).toBeLessThanOrEqual(1000);trace.push("run:"+index);if(row.outcome==="undefined"||row.outcome==="frozen")throw primary;},
  };
  try{
   if(row.kind!=="wasm"){
    const run=extract(cargoPath,["admitCargoExecutionV1",row.kind==="tests"?"runCargoTestsV1":"runCargoLintV1"],ports,row.kind,occurrence,count=>{occurrences=count;});
    await run({manifestPath:"/model/source/Cargo.toml",cwd:"/model/source",environment:env,packages:["owned"]},policy,{workspaceRoot:"/model",sourceBoundary:{},control:parent,storage,withCargoInputs:withInputs},{command:"cargo",args:["prefix"]});
   }else{
    Object.assign(ports,{requireNativeStorageReservationV1:(_:unknown,control:ResourceLeaseControl)=>{requireIssuedResourceControlV1(control);},requestedCompilerAdmissionControlV1:(control:ResourceLeaseControl,budget:number)=>createRequestedIssuedResourceControlV1(control,budget),admitWasmPolicy:(value:unknown)=>value,sourceSchema:{properties:{maximumFiles:{const:16}},definitions:{SourceName:{}}},validateJsonSchemaSubset:()=>[],wasmCompilerSelectionArgumentsV1:()=>({cargo:["build","--lib","--profile","dev","--target","wasm32-unknown-unknown"]}),buildModePolicy:()=>({cargoProfile:"dev"}),wasmBuildArguments:()=>({pack:["--dev"]}),
     availableParallelism:()=>8,acquireCompilerCapacityLeaseV1:async(request:{control:ResourceLeaseControl;request:unknown})=>{expect(request.request).toEqual({kind:"available",maximumParallelism:4});trace.push("capacity:"+number(request.control));return{parallelism:2,release:()=>{trace.push("capacity-release");}};},compilerAdmissionBudgetV1:(control:ResourceLeaseControl)=>checkResourceLeaseControl(control),readFileSync:()=>'name = "owned-crate"',existsSync:()=>true,statSync:()=>({size:1}),collectArtifactFiles:async()=>new Map([["owned_bg.wasm","/model/wasm"]]),readWasmPackageOwnerV1:(value:unknown)=>value,consumeProducedWasmPackageV1:async(_:unknown,_owner:unknown,_options:unknown,stopped:()=>void)=>{trace.push("consume");stopped();},
    });
    const run=extract(wasmPath,["buildWasmWebV1"],ports,row.kind,occurrence,count=>{occurrences=count;});
    await run({control:parent,storage,rsDir:"/model/source",componentDir:"/model/source",logPrefix:"model",pkg:{name:"owned",files:["owned_bg.wasm"]},wasmBaseName:"owned",threads:row.threads,environment:env,cargoConfigurationSelectors:[],withCargoInputs:withInputs,consume:async()=>{}},policy,{
     admit:async(args:readonly string[],options:{control:ResourceLeaseControl;environment:Record<string,string|undefined>})=>{const index=exact("admit",args,options.control);expect(options.environment.ADMITTED).toBe(String(index));trace.push("admit:"+index);},
     run:async(command:string,args:readonly string[],options:{control:ResourceLeaseControl;signal:AbortSignal;environment:Record<string,string|undefined>;budgetMs:number})=>{const index=number(options.control);expect(options.signal).toBe(options.control.signal);expect(options.environment).toEqual({...view(index).environment,SEMIO_COMPILER_CAPACITY_HELD:"true",SEMIO_COMPILER_PARALLELISM:"2",CARGO_BUILD_JOBS:"2"});expect(options.environment.SEMIO_COMPILER_CAPACITY_HELD).toBe("true");expect(options.budgetMs).toBeGreaterThan(0);expect(options.budgetMs).toBeLessThanOrEqual(1000);if(command==="cargo")exact("run",args,options.control);else if(command==="wasm-pack"){const actual=views.get(index);if(!actual)throw Error("Actual view required");expect(args.slice(args.indexOf("--")+1)).toEqual(actual.arguments.slice(1).filter((arg,at,all)=>arg!=="--lib"&&arg!=="--profile"&&all[at-1]!=="--profile"));}else expect(command).toBe("wasm-bindgen");trace.push("run:"+index);if(row.outcome==="undefined"||row.outcome==="frozen")throw primary;},
    });
   }
  }catch(error){failed={error};}
  });}catch(reason){settlement={status:"rejected",reason};}
 return{trace,failed,settlement,primary,releaseFailure,occurrences};
}
function judge(row:Row,run:Execution):void{
 const{failed,settlement}=run;
 if(row.outcome==="success"){if(failed)throw failed.error;expect(settlement).toEqual({status:"settled"});return;}
 const expected=row.outcome==="cleanup"?run.releaseFailure:run.primary;
 if(!failed)throw Error("Expected "+row.name+" to fail with its modeled "+(row.outcome==="cleanup"?"release failure":"primary"));
 if(!Object.is(failed.error,expected))throw failed.error??Error("Unexpected undefined failure for "+row.name);
 if(row.settlement==="settles")expect(settlement).toEqual({status:"settled"});
 else{expect(settlement.status).toBe("rejected");expect(Object.is((settlement as{reason:unknown}).reason,expected)).toBe(true);}
}
test("Core compiler consumers retain actual consumed arguments, phases and cleanup",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");db.exec("create table expected(name text,ordinal integer,event text);create table actual(name text,ordinal integer,event text)");try{for(const row of corpus.cases){const run=await execute(row);judge(row,run);row.trace.forEach((event,index)=>db.query("insert into expected values(?,?,?)").run(row.name,index,event));run.trace.forEach((event,index)=>db.query("insert into actual values(?,?,?)").run(row.name,index,event));expect(run.trace).toEqual(row.trace);}expect(db.query("select * from expected except select * from actual").all()).toEqual([]);expect(db.query("select * from actual except select * from expected").all()).toEqual([]);}finally{db.close();}
});
test("actual Core lease boundary refuses each substituted unconsumed argument at its declared port",async()=>{
 for(const kind of["tests","lint","wasm"]){
  const row=corpus.cases.find(row=>row.kind===kind&&row.outcome==="success");if(!row)throw Error("Closed compiler consumer row required");
  judge(row,await execute(row));
  const declared=corpus.substitutions.filter(item=>item.kind===kind),probe=await execute(row,declared.length);
  expect(probe.occurrences,kind+" substitution sites").toBe(declared.length);
  for(const item of declared){
   const run=await execute(row,item.occurrence),error=run.failed?.error;
   expect(error,kind+"#"+item.occurrence+" refusal").toBeInstanceOf(ConsumptionRefusal);
   const refusal=error as ConsumptionRefusal;
   expect(refusal.port,kind+"#"+item.occurrence+" port").toBe(item.port);expect(refusal.received).toEqual(["unconsumed"]);expect(refusal.expected.length).toBeGreaterThan(0);expect(refusal.expected).not.toContain("unconsumed");
   expect(run.settlement.status).toBe("rejected");expect(Object.is((run.settlement as{reason:unknown}).reason,error)).toBe(true);
  }
 }
});
