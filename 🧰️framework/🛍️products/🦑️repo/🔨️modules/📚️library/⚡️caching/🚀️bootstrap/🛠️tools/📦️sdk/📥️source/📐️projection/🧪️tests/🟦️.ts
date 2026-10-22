/** 📐️ Contract: the external dependency pins and the two captured installation inputs equal the unit's own corpus in `./🔣️.json`, the extra corpus `./📋️extra.json` and the bytes an independent Python projection writes, refuse unclosed recipes and locks, and ignore patch rows; ajv closes the recipe schema's verdicts. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxDependencyInstallFilesV1,nxExternalPinsV1} from "../🟦️.ts";
import unitSchema from "../🧬️schema/🔣️.json";
import own from "./🔣️.json";
import corpus from "./📋️extra.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),policy=unitSchema.definitions.Protocol.const;
const ask=(request:object):any=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify(request),encoding:"utf8",maxBuffer:256*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const refusal=(action:()=>unknown):string=>{try{action();return "";}catch(error){return (error as Error).message;}};
const blob=(value:unknown):unknown=>value==="@16MiB"?"x".repeat(16*1024*1024):Array.isArray(value)?value.map(blob):value&&typeof value==="object"?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,blob(item)])):value;
const text=(files:ReadonlyMap<string,Buffer>,name:string):string=>files.get(name)!.toString("utf8");

test("the extra corpus is closed by its schema and the recipe schema agrees between ajv and the repository validator",()=>{
 const ajv=new Ajv({strict:false}),recipeSchema={...unitSchema,$ref:"#/definitions/Recipe"};
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,pins:[{...corpus.pins[0],extra:1}]})).toBe(false);
 expect(policy).toEqual({version:1,maximumInputs:2,maximumInputBytes:16777216});
 for(const row of [...corpus.projections,{name:"own",recipe:own.recipe}]){
  const verdict=ajv.validate({...recipeSchema},row.recipe);
  expect(refusal(()=>nxDependencyInstallFilesV1(row.recipe,{}))!=="Closed pinned dependency recipe required",row.name).toBe(verdict);
 }
});

test("external pins drop exactly the workspace links in the Python reading and the corpus",()=>{
 const answers=ask({op:"pins",rows:corpus.pins.map(row=>row.dependencies)});
 for(const[index,row]of corpus.pins.entries()){
  const pins=nxExternalPinsV1(Object.fromEntries(Object.entries(row.dependencies).filter((entry):entry is[string,string]=>typeof entry[1]==="string")));
  expect<unknown>(pins,row.name).toEqual(row.expected);expect(answers[index],row.name).toEqual(row.expected);expect(Object.keys(pins),row.name).toEqual(Object.keys(row.expected));
 }
});

test("the captured inputs are byte-identical to the Python projection and refusals are worded as authored",()=>{
 const rows=corpus.projections.filter(row=>row.error===""),answers=ask({op:"project",rows:rows.map(row=>({recipe:row.recipe,lock:row.lock}))});
 for(const[index,row]of rows.entries()){
  const files=nxDependencyInstallFilesV1(row.recipe,row.lock);
  expect([...files.keys()],row.name).toEqual(["package.json","bun.lock"]);
  expect(text(files,"package.json"),row.name).toBe(answers[index]["package.json"]);expect(text(files,"bun.lock"),row.name).toBe(answers[index]["bun.lock"]);
  if(row.package)expect(text(files,"package.json"),row.name).toBe(row.package);if(row.lockText)expect(text(files,"bun.lock"),row.name).toBe(row.lockText);
  expect(text(files,"bun.lock"),row.name).not.toContain("patchedDependencies");
 }
 for(const row of corpus.projections.filter(item=>item.error!=="")){
  const lock=blob(row.lock);
  expect(refusal(()=>nxDependencyInstallFilesV1(row.recipe,lock)),row.name).toBe(row.error);
 }
});

test("the unit's own mutation cases change the captured inputs exactly when the pin, the integrity or the package manager changes and never for a patch",()=>{
 const base=nxDependencyInstallFilesV1(own.recipe,own.lock),clone=()=>({recipe:structuredClone(own.recipe) as any,lock:structuredClone(own.lock) as any});
 const mutate={
  patch:({recipe,lock}:any)=>{recipe.semio.toolPatches["nx@23.2.0"]="patches/other.patch";lock.patchedDependencies["nx@23.2.0"]="patches/other.patch";},
  dependency:({recipe,lock}:any)=>{recipe.dependencies.nx="23.2.1";lock.workspaces[""].dependencies.nx="23.2.1";},
  integrity:({lock}:any)=>{lock.packages.nx[3]="sha512-other";},
  bun:({recipe}:any)=>{recipe.packageManager="bun@1.3.15";}
 } as Record<string,(value:any)=>void>;
 for(const row of own.cases){
  const value=clone();mutate[row.field]!(value);const files=nxDependencyInstallFilesV1(value.recipe,value.lock);
  expect(text(files,"package.json")===text(base,"package.json")&&text(files,"bun.lock")===text(base,"bun.lock"),row.name).toBe(row.same);
 }
 expect(own.cases.map(row=>row.field).sort()).toEqual(Object.keys(mutate).sort());
});
