import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import _ from "lodash";
import {Database} from "bun:sqlite";
import {cargoArtifactPlanIdentityV1,cargoArtifactAcquisitionStepV1} from "../🟦️.ts";
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
   for(const row of corpus.lifecycle){const {id,expected,...observation}=row;expect(cargoArtifactAcquisitionStepV1(observation)).toBe(expected);expect((statement.get({$cancel:Number(row.consumerCancelled),$publication:row.publication,$rechecked:Number(row.publicationRechecked),$plan:row.producerPlan,$settled:Number(row.producerSettled),$released:Number(row.custodyReleased)}) as {action:string}).action).toBe(row.expected);}
  }finally{database.close();}
 });
});
