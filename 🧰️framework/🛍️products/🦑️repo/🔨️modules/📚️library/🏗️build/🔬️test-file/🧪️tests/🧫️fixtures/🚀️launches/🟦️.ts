import * as bun from "bun:test";
import {join} from "node:path";
import {withBuildTestInvocationV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {readOwnedBuildInvocationV1} from "../../../../📥️caller/🟦️.ts";

const root=process.env.SEMIO_WORKSPACE_ROOT!,manifest=join(import.meta.dir,"Cargo.toml"),declared=["cargo","--store=shared","--configuration=release","--","check","--manifest-path",manifest],expectation=process.env.SEMIO_FIXTURE_LAUNCH;
const attempt=(args:readonly string[])=>withBuildTestInvocationV1(async original=>{try{readOwnedBuildInvocationV1(root,original,args);return undefined;}catch(error){return String((error as Error).message);}});
const refused=async(args:readonly string[])=>bun.expect(await attempt(args)).toContain("Undeclared original Repo build selection");

bun.test("the declared nested launch is admitted exactly when the suite row declares it", async()=>{
  if(expectation==="declared")bun.expect(await attempt(declared)).toBeUndefined();else await refused(declared);
});

bun.test("every other nested launch stays refused", async()=>{
  await refused(["cargo","--store=shared","--configuration=release","--","check","--manifest-path",join(import.meta.dir,"other","Cargo.toml")]);
  await refused(["cargo","--store=shared","--configuration=release","--","build","--manifest-path",manifest]);
  await refused(["cargo","--store=shared","--configuration=release","--","check","--manifest-path",manifest,"--workspace"]);
  await refused(["cargo","--","check"]);
  await refused(["run","@semio-tech/repo-lib:test-other"]);
});
