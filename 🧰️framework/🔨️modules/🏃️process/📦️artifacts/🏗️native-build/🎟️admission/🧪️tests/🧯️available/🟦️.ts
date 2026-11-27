/** 🧯️ Contract: a unit starts only when it fits the measured available memory (less the margin and the unrealized remainder of the running units), never while the host warden is at guard or danger, and a machine that never has the room still makes progress after the wait bound. The verdict of every corpus case equals an independent Python oracle that reads the policy from the schema file. */
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import {ADMISSION_POLICY_V1,availableAdmissionV1,formatAdmissionHeldV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const policy=ADMISSION_POLICY_V1.available,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),schemaFile=fileURLToPath(new URL("../../🧬️schema/🔣️.json",import.meta.url)),corpusFile=fileURLToPath(new URL("./🔣️.json",import.meta.url));
const wardenLevelOfV1=(level:string|null|undefined):"calm"|"guard"|"danger"|undefined=>level==="calm"||level==="guard"||level==="danger"?level:undefined;
const run=(input:(typeof corpus.cases)[number]["input"])=>availableAdmissionV1({...input,availableBytes:input.availableBytes??undefined,wardenLevel:wardenLevelOfV1(input.wardenLevel),policy});

test("the policy is the authored one and the corpus is closed by its own schema",()=>{
 expect(policy).toMatchObject({marginBasisPoints:1500,waitMilliseconds:180000,pollMilliseconds:1000,cancelledFactorBasisPoints:12500,cancelledMemoryMilliseconds:3600000});
 const ajv=new Ajv({strict:false}).addSchema(schema);
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(corpus.cases.length).toBeGreaterThanOrEqual(16);
});

test("every corpus case equals its expected verdict",()=>{
 for(const item of corpus.cases)expect<unknown>(run(item.input),item.id).toEqual(item.expected);
});

test("the verdict of every corpus case equals the independent Python oracle",()=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",oracle,schemaFile,corpusFile],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 expect(JSON.parse(result.stdout)).toEqual(corpus.cases.map(item=>run(item.input)));
});

test("the held-back lines exist in English and German for every kind and fill every placeholder",()=>{
 const values={unit:"semio_hub_draw_component",predicted:2500,available:1937,margin:2457,level:"calm",seconds:12};
 for(const language of ["en","de"] as const)for(const kind of ["available","warden","swap","cancelled"] as const){
  const line=formatAdmissionHeldV1(language,kind,values);
  expect(line).toContain("semio_hub_draw_component");
  expect(line).toContain("2500");
  expect(line).not.toMatch(/\{[a-z]+\}/u);
 }
 expect(formatAdmissionHeldV1("en","available",values)).not.toBe(formatAdmissionHeldV1("de","available",values));
});
