/** 🔗️ Contract: the binding plan of a TypeScript package names the owner, the two fixed links, the generated output and the receipt exactly like an independent Python derivation, for the unit's own fixture in `./🔣️.json`, the cases in `./📋️plans.json` and every package directory length around the path capacity; the receipt schema is the one ajv reads. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {typeScriptPackageBindingPlanV1} from "../🟦️.ts";
import receiptSchema from "../🧬️schema/🔣️.json";
import layoutSchema from "../../../💾️storage/🧬️schema/🔣️.json";
import own from "./🔣️.json";
import corpus from "./📋️plans.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),protocol=layoutSchema.definitions.Protocol.const,posix=process.platform!=="win32";
const ask=(rows:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({protocol,rows}),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const outcome=(row:any):{value?:any;error?:string}=>{try{return{value:typeScriptPackageBindingPlanV1(row.workspace,row.package,row.packageName,row.mode,row.products)};}catch(error){return{error:(error as Error).message};}};

test("the corpus is closed by its schema and the receipt schema agrees with the authored receipts",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[{...corpus.cases[0],posixOnly:"yes"}]})).toBe(false);
 for(const row of corpus.receipts)expect(ajv.validate(receiptSchema,row.value),row.name).toBe(row.valid);
 expect(protocol.maximumPathUnits).toBe(256);expect(protocol.namespace).toBe("a/ts");
});

test.skipIf(!posix)("every authored case is planned like the Python derivation, with the authored refusal",()=>{
 const answers=ask(corpus.cases);
 for(const[index,row]of corpus.cases.entries()){
  const result=outcome(row);
  expect(result.error??"",row.name).toBe(row.error);expect(answers[index].error??"",row.name).toBe(row.error);
  if(row.error==="")expect(JSON.parse(JSON.stringify(result.value)),row.name).toEqual(answers[index].value);
 }
});

test("the unit's own fixture is planned for both modes with its fixed links, and plans are frozen and never alias the input",()=>{
 const workspace=resolve(own.workspace),products=resolve(own.products),root=resolve(own.package);
 for(const mode of own.modes){
  const plan=typeScriptPackageBindingPlanV1(workspace,root,own.name,mode,products);
  expect(plan.links.map(link=>link.path.slice(root.length+1).split("\\").join("/"))).toEqual(own.fixedBindings.filter(path=>path==="dist"||path.endsWith("/"+mode)));
  expect(plan.links.every(link=>link.target===plan.output)).toBe(true);expect(plan.output.startsWith(products)).toBe(true);expect(plan.receipt.endsWith("binding.json")).toBe(true);
  expect(Object.isFrozen(plan)).toBe(true);expect(Object.isFrozen(plan.links)).toBe(true);expect(plan.links.every(link=>Object.isFrozen(link))).toBe(true);
 }
 const prototype=typeScriptPackageBindingPlanV1(workspace,root,own.name,"prototype",products),release=typeScriptPackageBindingPlanV1(workspace,root,own.name,"release",products);
 expect(prototype.output).not.toBe(release.output);expect(prototype.owner).toBe(release.owner);expect(prototype.owner.startsWith("/")).toBe(false);
 expect(own.foreignPackage).toBeDefined();expect(()=>typeScriptPackageBindingPlanV1(workspace,resolve(own.foreignPackage),own.name,"release",products)).toThrow("Actual repository package owner required");
});

test.skipIf(!posix)("the path capacity boundary of every package directory length equals the Python derivation, refusing exactly where the fixed links would not fit",()=>{
 const sweep=corpus.capacity,rows=[];
 for(let length=sweep.from;length<=sweep.to;length++)rows.push({name:"length "+length,workspace:sweep.workspace,package:sweep.workspace+"/"+"x".repeat(length),packageName:"n",mode:"release",products:sweep.products});
 const answers=ask(rows),refused=new Set<string>();
 for(const[index,row]of rows.entries()){
  const result=outcome(row);expect(result.error??"",row.name).toBe(answers[index].error??"");if(result.error)refused.add(result.error);
 }
 expect([...refused].sort()).toEqual(["Closed TypeScript artifact layout required","Package binding path capacity exceeded"]);
 const first=rows.findIndex((_,index)=>answers[index].error),last=rows.length-1;
 expect(first).toBeGreaterThan(0);for(let index=first;index<=last;index++)expect(answers[index].error,"monotonic "+index).toBeDefined();
});
