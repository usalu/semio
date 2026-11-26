/** 🧲️ Contract: a graph read never gives up because the owner registry or the implementation moves while it is read (200 seeded worlds, the real `loadBuildGraphUnpinnedV1` over a model whose registry changes on a logical clock): a pinned read completes on the registry it loaded in at most two attempts, an unpinned read adopts every moved registry and ends with the first candidate that holds, and no count of attempts ends a read.
 * @see ../🌊️seal-under-churn */
import{withOwnerRegistryCustodyV1}from"../../../🕸️graph/📥️load/🧭️registry-follow/🟦️.ts";
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{join,dirname}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import authoritySchema from"../../../🧬️schema/📥️authority.json";
import retrySchema from"../../../🕸️graph/🔁retry/🧬️schema/🔣️.json";
import{sealVerdictV1}from"../../../🕸️graph/🔁retry/🟨️.mjs";

const LOAD=process.env.R167_LOAD_SOURCE??fileURLToPath(new URL("../../../🕸️graph/📥️load/🟦️.ts",import.meta.url));
const source=readFileSync(LOAD,"utf8"),tree=ts.createSourceFile("load.ts",source,ts.ScriptTarget.Latest,true);
const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text==="loadBuildGraphUnpinnedV1");if(!node)throw Error("Actual graph loader required");
const moved=ts.transform(node,[context=>{const visit:ts.Visitor=each=>ts.isPropertyAccessExpression(each)&&ts.isMetaProperty(each.expression)?ts.factory.createStringLiteral("file:///owned/graph.ts"):ts.isMetaProperty(each)?ts.factory.createIdentifier("importMetaV1"):ts.isCallExpression(each)&&each.expression.kind===ts.SyntaxKind.ImportKeyword?ts.factory.createCallExpression(ts.factory.createIdentifier("importModuleV1"),undefined,ts.visitNodes(each.arguments,visit,ts.isExpression)):ts.visitEachChild(each,visit,context);return each=>ts.visitNode(each,visit)as ts.FunctionDeclaration;}]);
const code=ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,moved.transformed[0]!,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
const generator=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const PERIODS=[1,3,7,20,90,400],WORK=[5,17,60,250];

const world=async(seed:number,pinned:boolean)=>{
 const pick=generator(seed),period=PERIODS[Math.floor(pick()*PERIODS.length)]!,work=WORK[Math.floor(pick()*WORK.length)]!,limit=pinned?Infinity:Math.floor(pick()*120);
 let clock=0,version=0,moves=0,snapshot:number|undefined,attempts=0,clears=0,implementationVersion=0;
 const advance=(milliseconds:number)=>{clock+=milliseconds;while(moves<limit&&clock>=(moves+1)*period){moves++;version++;implementationVersion++;}};
 const readRegistry=()=>{if(pinned){snapshot??=version;return{version:1,owners:[snapshot]};}return{version:1,owners:[version]};};
 const graph={version:1,projects:{},dependencies:[]},invalidated=Object.freeze({kind:"invalidated",reason:"owner-registry"});
 const bindings={
  withOwnerRegistryCustodyV1,
  sliceReadStepV1:(_step:string,run:()=>Promise<unknown>)=>run(),sliceReadWaitStepV1:(_step:string,run:(leave:()=>void)=>Promise<unknown>)=>run(()=>{}),
  requireIssuedResourceControlInvocationV1:()=>{},checkResourceLeaseControl:()=>{},checkScriptInvocation:()=>{},
  readBuildGraphSelectionV1:(value:unknown)=>value,readBuildOwnerRegistryV1:(value:unknown)=>value,
  importModuleV1:async()=>({readBuildOwnerRegistrySourceV1:()=>JSON.stringify(readRegistry())}),
  readSealedBuildGraphV1:async()=>undefined,buildGraphScopeKeyV1:()=>"scope",ownerRegistryDigestV1:(registry:{owners:number[]})=>String(registry.owners[0]),askOwnerRegistryV1:()=>"registry",
  isOwnerRegistryInvalidationV1:(value:unknown)=>value===invalidated,ownerRegistryInvalidationV1:invalidated,
  clearPinnedAuthorityV1:()=>{clears++;snapshot=undefined;},
  process:{stderr:{write:()=>true}},
  requireNativeStoragePathV1:(_root:string,path:string)=>path,mkdir:async()=>{},readdir:async()=>[],lstat:async()=>{throw Error("Unexpected staging file");},rm:async()=>{},join,dirname,buildGraphPathV1:()=>"/owned/cache/canonical.json",
  selectedThroughFleetV1:({attempt}:{attempt():unknown})=>attempt(),
  withBuildGraphStateV1:async(_root:string,_invocation:unknown,actual:unknown,body:(store:unknown,phase:unknown,critical:(work:()=>Promise<unknown>)=>Promise<unknown>)=>Promise<unknown>)=>body({root:"/owned/cache",recheck:async()=>{}},actual,work=>work()),
  loadBuildGraphAttemptV1:async(_root:string,_invocation:unknown,_actual:unknown,_options:unknown,registry:{owners:number[]},scope?:string)=>{
   attempts++;expect(scope).toBe("scope");advance(work);
   return JSON.stringify(readRegistry())===JSON.stringify(registry)?{...graph,version:registry.owners[0]!}:invalidated;
  },
  advanceScriptInvocation:async()=>{advance(1);},
 };
 const execute=Function(...Object.keys(bindings),code+";return loadBuildGraphUnpinnedV1;")(...Object.values(bindings));
 const result=await execute("/owned",{},{signal:new AbortController().signal},{selection:{kind:"tasks",tasks:[{project:"one",target:"build"}]}});
 return{result,attempts,clears,moves,period,work,version};
};

test("a pinned read completes on the registry it loaded, however fast the registry and the implementation move",async()=>{
 for(let seed=1;seed<=200;seed++){const row=await world(seed,true);expect(row.attempts,"world "+seed).toBe(1);expect(row.clears).toBe(0);expect((row.result as{version:number}).version).toBe(0);}
});

test("an unpinned read adopts every moved registry and ends with the first candidate that holds (no count of attempts gives up)",async()=>{
 let beyond=0;
 for(let seed=1;seed<=200;seed++){const row=await world(seed,false);expect(row.clears,"world "+seed).toBe(row.attempts-1);expect(row.attempts).toBeLessThanOrEqual(row.moves+1);expect((row.result as{version:number}).version).toBe(row.moves);if(row.attempts>9)beyond++;}
 expect(beyond,"worlds that needed more attempts than the old bound of nine").toBeGreaterThan(0);
});

test("a seal names the owner of every stale row it lists (at most the stale paths it keeps), not 64, and the authority receipt admits the list",()=>{
 const owners=retrySchema.$defs.SealPolicy.const.maximumStalePaths,rows=Array.from({length:owners},(_,index)=>({kind:"file",path:"owner-"+index+"/src",fact:{sha256:"a".repeat(64)}})),projects=Object.fromEntries(rows.map((_,index)=>["owner-"+index,{root:"owner-"+index}]));
 const verdict=sealVerdictV1({rounds:4,staleCount:owners,stale:rows,membershipMoved:false,admissionMoved:false,projects,rows});
 expect(verdict.laterChangedOwners.length).toBe(owners);expect(Object.keys(verdict.owners).length).toBe(owners);expect(owners).toBeGreaterThan(64);
 const seal=(authoritySchema as{properties:{seal:unknown}}).properties.seal,ajv=new Ajv({strict:false});
 expect(ajv.validate(seal as object,verdict),JSON.stringify(ajv.errors)).toBe(true);
 expect(retrySchema.$defs.SealPolicy.const.maximumOwners).toBeGreaterThanOrEqual(4774*2);
});
