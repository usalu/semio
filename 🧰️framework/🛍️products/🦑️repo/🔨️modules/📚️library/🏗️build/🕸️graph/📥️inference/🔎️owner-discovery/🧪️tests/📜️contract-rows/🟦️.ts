import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {loadProductFunctionsSyncV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";

const observation=await import("../../../../../👁️observation/🟨️.mjs"),SOURCE=new URL("../../🟨️.mjs",import.meta.url);
type Contracts=Record<string,{ownership:string;note:string;outputRoots:{path:string;inclusion:string}[]}>;
const contracts=():Contracts=>({a:{ownership:"owned",note:"a",outputRoots:[{path:"gen/a",inclusion:"ignored"}]},b:{ownership:"owned",note:"b",outputRoots:[{path:"gen/b",inclusion:"ignored"}]},c:{ownership:"observed",note:"c",outputRoots:[]}});

function fixture(){
 const root=mkdtempSync(join(testScratchRootV1("contract-rows"),"ws-")),taxonomy=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
 let clock=1_700_000_000;
 const write=(value:Contracts)=>{mkdirSync(dirname(taxonomy),{recursive:true});writeFileSync(taxonomy,JSON.stringify({testDeliveryScopeDirectoryNames:["x"],generatorContracts:value}));clock+=10;utimesSync(taxonomy,clock,clock);};
 const functions=loadProductFunctionsSyncV1(SOURCE,["ownerGeneratorContractsV1","admissionGeneratorContractReadHoldsV1","ADMISSION_GENERATOR_CONTRACTS_FACT_V1"],{readTaxonomyProjectionV1:(path:string)=>({generatorContracts:JSON.parse(readFileSync(path,"utf8")).generatorContracts})}) as any;
 observation.registerGlobalFactV1(functions.ADMISSION_GENERATOR_CONTRACTS_FACT_V1,functions.admissionGeneratorContractReadHoldsV1);
 return {root,write,functions,cleanup:()=>rmSync(root,{recursive:true,force:true})};
}

const record=async(functions:any,root:string,read:(contracts:Contracts)=>void)=>(await observation.captureObservedSourceInputsV1(async()=>{read(functions.ownerGeneratorContractsV1(root));},undefined,false)).observations as {kind:string;path:string}[];

test("an owner admission that read one generator contract stays valid exactly while that read keeps its answer, as an independent oracle says",async()=>{
 const {root,write,functions,cleanup}=fixture();
 try{
  const base=contracts();write(base);
  const rows=await record(functions,root,view=>{void view.a.ownership;void view.a.ownership;void view.a.outputRoots[0]!.path;});
  expect(rows.filter(row=>row.kind==="analysis")).toEqual([]);
  expect(rows.filter(row=>row.kind==="global").length).toBe(2);
  expect(await observation.observationsHold(rows)).toBe(true);
  const edits:[string,(value:Contracts)=>void][]=[
   ["note of the read contract",value=>{value.a.note="changed";}],
   ["ownership of another contract",value=>{value.b.ownership="observed";}],
   ["a new contract",value=>{value.d={ownership:"owned",note:"d",outputRoots:[]};}],
   ["ownership of the read contract",value=>{value.a.ownership="observed";}],
   ["the read output root",value=>{value.a.outputRoots[0]!.path="gen/moved";}],
   ["the read contract removed",value=>{delete (value as Record<string,unknown>).a;}],
  ];
  const oracle=(value:Contracts)=>JSON.stringify([base.a.ownership,base.a.outputRoots[0]!.path])===JSON.stringify([value.a?.ownership,value.a?.outputRoots?.[0]?.path]);
  for(const [label,edit] of edits){
   const changed=contracts();edit(changed);write(changed);
   expect(await observation.observationsHold(rows),label).toBe(oracle(changed));
   write(base);expect(await observation.observationsHold(rows),label+" restored").toBe(true);
  }
 }finally{cleanup();}
});

test("a listing of the contracts is recorded as one read that moves with the keys only",async()=>{
 const {root,write,functions,cleanup}=fixture();
 try{
  write(contracts());
  const rows=await record(functions,root,view=>{void Object.keys(view);});
  expect(await observation.observationsHold(rows)).toBe(true);
  const changed=contracts();changed.a.note="edited";write(changed);
  expect(await observation.observationsHold(rows)).toBe(true);
  changed.z={ownership:"owned",note:"z",outputRoots:[]};write(changed);
  expect(await observation.observationsHold(rows)).toBe(false);
 }finally{cleanup();}
});

test("the owner discovery reads the contracts through the recorded view and never as one whole-projection row",()=>{
 const text=readFileSync(SOURCE,"utf8");
 expect(text).toContain("contracts=ownerGeneratorContractsV1(workspaceRoot)");
 expect(text).toContain("registerGlobalFactV1(ADMISSION_GENERATOR_CONTRACTS_FACT_V1,admissionGeneratorContractReadHoldsV1)");
 expect(text).not.toMatch(/contractsPath=join\(workspaceRoot,[^)]*taxonomy[^)]*\),contracts=existsSync/u);
});
