/** 🎚️ Contract: the Nx rendering of a host capacity grant sets exactly one variable, NX_PARALLEL, to the granted jobs and reports every other contract field honestly, as the corpus in `./🔣️.json` and an independent Python rendering state. */
import {expect,test,setDefaultTimeout} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {nxHostResourceSettingsV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(plans:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({plans}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,plans:[{...corpus.plans[0],parallel:"01"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,plans:[{...corpus.plans[0],plan:{...corpus.plans[0]!.plan,jobs:1025}}]})).toBe(false);
});

test("every plan is rendered like the Python rendering with the authored parallelism and a frozen result",()=>{
 const answers=ask(corpus.plans.map(row=>row.plan));
 for(const[index,row]of corpus.plans.entries()){
  const settings=nxHostResourceSettingsV1(row.plan as never);
  expect(settings.environment,row.name).toEqual({NX_PARALLEL:row.parallel});expect(JSON.parse(JSON.stringify(settings)),row.name).toEqual(answers[index]);
  expect(Object.keys(settings.environment),row.name).toEqual(["NX_PARALLEL"]);expect(Object.isFrozen(settings)&&Object.isFrozen(settings.environment)&&Object.isFrozen(settings.mapping),row.name).toBe(true);
  expect(settings.mapping.filter(item=>item.support==="exact").map(item=>item.field),row.name).toEqual(["jobs"]);
 }
});
