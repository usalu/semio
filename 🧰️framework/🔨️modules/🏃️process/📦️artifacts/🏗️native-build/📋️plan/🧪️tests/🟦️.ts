import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import _ from "lodash";
import {Database} from "bun:sqlite";
import {cargoArtifactPlanDifferenceV1,cargoArtifactPlanIdentityV1,cargoArtifactAcquisitionStepV1,cargoSourceDigestMismatchV1,type CargoArtifactPlanFactsV1} from "../🟦️.ts";
import {cargoTargetRecipeCanonicalV1} from "../../🎯️recipe/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";

type Plan=typeof corpus.identity[number]["left"];
const root=(plan:Plan):string=>plan.platform==="win32"?plan.root.replaceAll("\\","/"):plan.root;
const recipe=(plan:Plan)=>({version:1,cwd:root(plan),manifest:root(plan)+"/Cargo.toml",command:"cargo",args:["build","--bin",plan.product,...plan.mode==="release"?["--release"]:[]],buildDirectory:root(plan)+"/target",compilerIdentity:plan.compiler,toolchainIdentity:plan.toolchain,inputs:[{path:root(plan)+"/Cargo.toml",sha256:plan.configuration}],environment:[]});

/** 🧪️ These DATA laws never issue a plan, Original, control or lease. */
describe("admitted artifact producer identity",()=>{
 test("neutral rows validate the production plan and acquisition DTOs with Ajv",()=>{const ajv=new Ajv({strict:true}),plan=ajv.compile(schema),observation=ajv.compile(schema.definitions.AcquisitionObservation);for(const row of corpus.identity)for(const value of[row.left,row.right])expect(plan({version:1,...value,root:root(value)})).toBe(true);for(const {id,expected,...row}of corpus.lifecycle)expect(observation(row)).toBe(true);});
 for(const row of corpus.identity)test(row.id,()=>{
  const normalize=(plan:Plan)=>({...plan,root:root(plan)}),oracle=_.isEqual(_.pick(normalize(row.left),Object.keys(row.left)),_.pick(normalize(row.right),Object.keys(row.right)));
  expect(oracle).toBe(row.equal);
  expect(cargoArtifactPlanIdentityV1({version:1,...normalize(row.left)})===cargoArtifactPlanIdentityV1({version:1,...normalize(row.right)})).toBe(row.equal);
 });
 test("recipe exclusion alone intentionally does not claim Source identity",()=>{const row=corpus.identity.find(row=>row.id==="changed-source")!;expect(cargoTargetRecipeCanonicalV1(recipe(row.left))).toBe(cargoTargetRecipeCanonicalV1(recipe(row.right)));});
 test("SQLite independently distinguishes cancellation settlement and rechecked publication",()=>{
  const database=new Database(":memory:");
  try{
   const statement=database.query("SELECT CASE WHEN $cancel THEN 'cancel' WHEN $publication='foreign' THEN 'refuse' WHEN $publication='matching' AND $rechecked THEN 'consume' WHEN $publication='matching' THEN 'recheck' WHEN $plan='same' AND (NOT $settled OR NOT $released) THEN 'join' ELSE 'produce' END AS action");
   for(const row of corpus.lifecycle){const {id,expected,...observation}=row;expect<unknown>(cargoArtifactAcquisitionStepV1(observation)).toBe(expected);expect((statement.get({$cancel:Number(row.consumerCancelled),$publication:row.publication,$rechecked:Number(row.publicationRechecked),$plan:row.producerPlan,$settled:Number(row.producerSettled),$released:Number(row.custodyReleased)}) as {action:string}).action).toBe(row.expected);}
  }finally{database.close();}
 });
});

test("completed Source diagnostics agree with independent SQLite and preserve missing-record identity",()=>{
 const database=new Database(":memory:"),valid=new Ajv({strict:true}).compile(schema.definitions.SourceDigestMismatch);
 try{
  database.exec("CREATE TABLE observed(path TEXT PRIMARY KEY,digest TEXT)");
  const lookup=database.query("SELECT $path AS path,$expected AS expected,observed.digest AS observed,CASE WHEN observed.path IS NULL THEN 1 ELSE 0 END AS missingRecord FROM (SELECT 1) LEFT JOIN observed ON observed.path=$path WHERE observed.digest IS NOT $expected");
  for(const row of corpus.sourceDigests){
   database.exec("DELETE FROM observed");for(const item of row.observations)database.query("INSERT INTO observed VALUES(?,?)").run(item.path,item.sha256);
   const values=new Map(row.observations.map(item=>[item.path,item.sha256])),actual=cargoSourceDigestMismatchV1(row.path,row.expected,values.get(row.path)),sql=lookup.get({$path:row.path,$expected:row.expected}) as {path:string;expected:string;observed:string|null;missingRecord:number}|null,oracle=sql?{...sql,missingRecord:Boolean(sql.missingRecord)}:null;
   expect(actual,row.id).toEqual(row.result);expect(actual,row.id).toEqual(oracle);if(actual){expect(valid(actual),row.id).toBe(true);expect(JSON.stringify(actual).length).toBeLessThan(600);}
  }
 }finally{database.close();}
});

test("pre-spawn plan differences report the first changed facts and only existing source digests",()=>{
 const {baseline,cases}=corpus.planDifferences,hash=(character:string)=>character.repeat(64);
 const facts:CargoArtifactPlanFactsV1={version:1,root:"/w",mode:"release",platform:"linux",product:hash("1"),compiler:"cargo",toolchain:"rust",configuration:hash("2"),source:hash("3")};
 const graph={roots:["unit"],units:[{id:"unit",sourceInputs:[{path:baseline.path,digest:hash(baseline.character),bytes:baseline.bytes}],planningDigest:hash("4"),compilationDigest:hash("5"),configurationDigest:null}]};
 const database=new Database(":memory:");
 try{
  const oracle=database.query(`SELECT CASE WHEN $before IS $after THEN NULL WHEN $configured THEN 'plan-fact' WHEN $roots THEN 'graph-roots' WHEN $unit THEN 'graph-units' WHEN $path THEN 'source-roster' WHEN $bytes THEN 'source-file' WHEN $metadata THEN 'unit-metadata' ELSE 'graph-other' END AS category`);
  for(const row of cases){
   const actual={...facts},current=structuredClone(graph),old=graph.units[0]!,next=current.units[0]!,input=next.sourceInputs[0]!;
   if(row.change!=="none")actual.source=hash("9");
   switch(row.change){
    case "configuration": actual.configuration=hash("8"); break;
    case "source-file": input.digest=hash("b"); break;
    case "source-roster": input.path="/w/renamed.rs"; break;
    case "unit-metadata": next.planningDigest=hash("7"); break;
    case "graph-roots": current.roots.push("extra"); break;
    case "graph-units": next.id="replacement"; break;
   }
   const difference=cargoArtifactPlanDifferenceV1(facts,actual,graph,current,()=>{});
   const sql=oracle.get({$before:facts.source,$after:actual.source,$configured:Number(facts.configuration!==actual.configuration),$roots:Number(!_.isEqual(graph.roots,current.roots)),$unit:Number(old.id!==next.id),$path:Number(old.sourceInputs[0]!.path!==input.path),$bytes:Number(old.sourceInputs[0]!.digest!==input.digest),$metadata:Number(old.planningDigest!==next.planningDigest)}) as {category:string|null};
   expect(difference?.category??null,row.id).toBe(row.category);expect(sql.category,row.id).toBe(row.category);
   if(row.change==="source-file")expect(difference).toMatchObject({path:baseline.path,expectedSha256:hash(baseline.character),observedSha256:hash("b")});
   if(difference)expect(JSON.stringify(difference).length).toBeLessThan(1024);
  }
  const stop=Error("cancelled");expect(()=>cargoArtifactPlanDifferenceV1(facts,{...facts,source:hash("9")},graph,graph,()=>{throw stop;})).toThrow(stop);
 }finally{database.close();}
});
