import {expect,test} from "bun:test";
import {join} from "node:path";
import {vitestRunEnvironmentV1} from "../🟦️.ts";
import {repoPlaywrightBrowsersPath} from "../../../🏃️process/🌿️environment/🟦️.ts";
import {findToolV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {toolEnvironmentV1} from "../../../../../../../🔨️modules/🏃️process/🧰️tools/🌿️environment/🟦️.ts";

test("a vitest run carries its owned policy and the repository's provisioned Playwright browsers",()=>{
 const root=process.cwd(),before=process.env.PLAYWRIGHT_BROWSERS_PATH;
 try{
  delete process.env.PLAYWRIGHT_BROWSERS_PATH;
  const policy={version:1},env=vitestRunEnvironmentV1(root,policy);
  expect(JSON.parse(env.SEMIO_VITEST_POLICY!)).toEqual(policy);
  expect(env.PLAYWRIGHT_BROWSERS_PATH?.endsWith(join("tools","ms-playwright"))).toBe(true);
  process.env.PLAYWRIGHT_BROWSERS_PATH="/explicit/browsers";
  expect(vitestRunEnvironmentV1(root,policy).PLAYWRIGHT_BROWSERS_PATH).toBe("/explicit/browsers");
 }finally{if(before===undefined)delete process.env.PLAYWRIGHT_BROWSERS_PATH;else process.env.PLAYWRIGHT_BROWSERS_PATH=before;}
});

test("vitest, the e2e playwright tool run and the setup verifier share one browser cache path",()=>{
 const root=process.cwd(),registry=toolEnvironmentV1(findToolV1("playwright")!,{},{},{root}).PLAYWRIGHT_BROWSERS_PATH;
 expect(registry).toBeDefined();
 expect(repoPlaywrightBrowsersPath(root)).toBe(registry!);
});
