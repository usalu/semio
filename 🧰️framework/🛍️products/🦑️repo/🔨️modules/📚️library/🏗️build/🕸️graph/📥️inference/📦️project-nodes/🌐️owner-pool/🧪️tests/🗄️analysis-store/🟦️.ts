/** 🗄️ Law: the source analysis store is derived only from an absolute closure store path of this machine, never resolved against the working directory (a pool child whose cwd was the replica member wrote 4,883 analyses into `🔁️replica/sa/` from a session a Windows publisher sent with `C:\semio\…\ce`), and a pool child keeps its analyses beside the shared closure store it binds, not beside the publisher's.
 * @see ../../🔬️inference/🟨️.mjs `ownerPoolSessionV1`
 * @see ../../../🟨️.mjs `analysisStoreDirectoryOfV1`
 * @see ./🐍️oracle.py */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{dirname,isAbsolute,join,normalize}from"node:path";
import{renderRepoErrorV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import fixture from"./🔣️.json" with{type:"json"};

const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const nodes=await import("../../../🟨️.mjs") as any,driver=await import("../../🔬️inference/🟨️.mjs") as any;
const refused=fixture.refused.filter(path=>process.platform!=="win32"||!path.startsWith("C:"));

test("an absolute closure store gives the analysis store beside it, as Python's os.path derives it",()=>{
 const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);
 const python=JSON.parse(oracle.stdout) as Record<string,string|null>;
 for(const row of fixture.derived){expect(nodes.analysisStoreDirectoryOfV1(row.closureStore)).toBe(normalize(row.analysisStore));expect(nodes.analysisStoreDirectoryOfV1(row.closureStore)).toBe(python[row.closureStore]);}
 for(const path of refused)expect(python[path]).toBeNull();
 expect(nodes.analysisStoreDirectoryOfV1(undefined)).toBeUndefined();
});

test("a relative, empty or foreign-platform closure store is refused in English and German instead of resolving against the working directory",()=>{
 for(const path of refused){
  let error:any;try{nodes.analysisStoreDirectoryOfV1(path);}catch(caught){error=caught;}
  expect(error?.code).toBe("invalid");
  expect(String(error?.key)).toEndWith("graph.analysis-store-closure-store-relative");
  expect(renderRepoErrorV1(error,"en")).toContain("absolute closure store path");
  expect(renderRepoErrorV1(error,"de")).toContain("absoluten Pfad des Closure-Speichers");
 }
});

test("a pool child's session puts the analysis store inside the parent of the shared closure store it binds, whatever closure store the publisher sent",()=>{
 const root=normalize("/srv/semio"),shared=driver.ownerPoolClosureStoreV1(root),base=join(root,".cache","f","owner-pool-staging","x");
 for(const sent of[{version:3,closureStore:fixture.sentFromWindows},{version:3,closureStore:"ce"},{version:3,closureStore:join(root,"elsewhere","ce")},{version:3}]){
  const session=driver.ownerPoolSessionV1({sent,root,shared,base});
  expect(session.closureStore).toBe(shared.directory);
  expect(isAbsolute(session.analysisStore)).toBe(true);
  expect(session.analysisStore).toBe(join(dirname(shared.directory),"sa"));
  expect(session.workspaceRoot).toBe(root);
  expect(nodes.analysisStoreDirectoryOfV1(session.closureStore)).toBe(session.analysisStore);
 }
});
