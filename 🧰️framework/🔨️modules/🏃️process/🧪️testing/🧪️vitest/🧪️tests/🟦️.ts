import { test, expect } from "bun:test";
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { createRequire } from "node:module";

test("Vitest execution requires owner tool/cache policy and preserves exact budgets", async () => {
  const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")), schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")), validate=new (createRequire(import.meta.url)("ajv").default)({strict:true}).compile(schema),api=await import("../🟦️.ts");
  expect(validate(fixture.policy)).toBe(true);
  for(const key of ["cwd","toolPath","cacheRoot","coverageDirectory"])fixture.policy[key]=resolve(fixture.policy[key]);
  fixture.expected[0]=fixture.policy.toolPath;fixture.expected[3]=resolve(fixture.expected[3]);fixture.coverage[3]=`--coverage.reportsDirectory=${fixture.policy.coverageDirectory}`;
  expect(()=>api.readVitestPolicyV1({SEMIO_VITEST_POLICY:JSON.stringify(fixture.policy)},"/foreign")).toThrow("another owner");
  expect(api.vitestArgumentsV1(fixture.policy,fixture.arguments,fixture.config,false)).toEqual(fixture.expected);
  expect(api.vitestArgumentsV1(fixture.policy,fixture.arguments,fixture.config,true)).toEqual([...fixture.expected.slice(0,10),...fixture.coverage,...fixture.arguments]);
  expect(api.testCacheDirectoryV1({SEMIO_VITEST_POLICY:JSON.stringify(fixture.policy)},"kernel")).toBe(join(fixture.policy.cacheRoot,"kernel"));
  expect(()=>api.readVitestPolicyV1({})).toThrow("Explicit Vitest policy required");
  for(const policy of [{...fixture.policy,extra:1},{...fixture.policy,budgetMs:-1},{...fixture.policy,cacheRoot:"relative"}]){expect(()=>api.readVitestPolicyV1({SEMIO_VITEST_POLICY:JSON.stringify(policy)})).toThrow();}
  expect(validate({...fixture.policy,budgetMs:-1})).toBe(false);
  expect(()=>api.testCacheDirectoryV1({SEMIO_VITEST_POLICY:JSON.stringify(fixture.policy)},"../foreign")).toThrow();
  expect(api.readVitestPolicyV1({SEMIO_VITEST_POLICY:JSON.stringify(fixture.policy),SEMIO_TEST_LEVEL:"quick"}).budgetMs).toBe(300000);
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(output).toBeTruthy();const temp=mkdtempSync(join(output,"vitest-port-"));
  try{
    const log=join(temp,"argv.json"),tool=join(temp,"tool.cjs");writeFileSync(tool,"require('node:fs').writeFileSync(process.env.VITEST_PORT_LOG,JSON.stringify(process.argv.slice(2)));");
    const policy={...fixture.policy,cwd:temp,toolPath:tool,runtime:"node",coverageRuntime:"node",cacheRoot:temp,coverageDirectory:temp};
    for(const coverage of [false,true]){await api.runVitestV1(policy,fixture.arguments,fixture.config,{...process.env,VITEST_PORT_LOG:log,SEMIO_COVERAGE:coverage?"1":"0"});expect(JSON.parse(readFileSync(log,"utf8"))).toEqual(api.vitestArgumentsV1(policy,fixture.arguments,fixture.config,coverage).slice(1));}
    const cancelled=new AbortController();cancelled.abort();await expect(api.runVitestV1(policy,[],fixture.config,process.env,cancelled.signal)).rejects.toThrow("cancelled");
    const esbuild=createRequire(import.meta.url)("esbuild"), bundle=await esbuild.build({entryPoints:[resolve(import.meta.dir,"../🟦️.ts")],bundle:true,write:false,platform:"node",format:"esm",external:["bun:sqlite"]});expect(bundle.outputFiles[0].text).not.toContain("🛍️products");
  }finally{rmSync(temp,{recursive:true,force:true});}
});
