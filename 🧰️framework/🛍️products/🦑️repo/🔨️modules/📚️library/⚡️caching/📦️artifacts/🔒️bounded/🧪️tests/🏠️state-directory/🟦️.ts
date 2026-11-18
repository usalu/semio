import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {readAuthoredNxCacheDirectoryV1} from "../../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import cacheConfigurationSchema from "../../../../💾️storage/🔒️enforcement/🔌️nx/🧬️schema/📋️configuration.json" with {type:"json"};
import storagePolicy from "../../../../💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};

const checkoutRoot=(()=>{for(const start of[dirname(fileURLToPath(import.meta.url)),process.cwd()]){let directory=start;for(;;){if(existsSync(join(directory,"nx.json")))return directory;const parent=dirname(directory);if(parent===directory)break;directory=parent;}}throw Error("Original checkout root required");})();
const authored=JSON.parse(readFileSync(join(checkoutRoot,"nx.json"),"utf8")).cacheDirectory as string;
const withState=<T>(state:string|undefined,run:()=>T):T=>{
 const previous=process.env.SEMIO_MACHINE_STATE_DIRECTORY;
 if(state===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=state;
 try{return run();}finally{if(previous===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=previous;}
};

/** 🏠️ The authored Nx cache directory is a checkout-relative name that goes through the machine state seam like every other `.cache` join, so a machine with a declared state directory accepts it and a different name is still refused. */
test("the authored Nx cache directory follows the declared machine state directory",()=>{
 const workspace=mkdtempSync(join(tmpdir(),"nx-auth-w-")),state=mkdtempSync(join(tmpdir(),"nx-auth-s-"));
 try{
  expect(authored).toBe(cacheConfigurationSchema.const);
  const logical=storagePolicy.partitions.rust.directory.replace(/^\.cache\//u,"");
  expect(withState(state,()=>readAuthoredNxCacheDirectoryV1(workspace,authored))).toBe(join(state,logical,"m/nx/artifacts"));
  expect(withState(undefined,()=>readAuthoredNxCacheDirectoryV1(workspace,authored))).toBe(join(workspace,storagePolicy.partitions.rust.directory,"m/nx/artifacts"));
  for(const alias of[join(workspace,authored),".cache/b/r/m/nx/other","../"+authored,"",undefined,7])expect(()=>withState(state,()=>readAuthoredNxCacheDirectoryV1(workspace,alias))).toThrow("Authored bounded Nx cache directory");
 }finally{rmSync(workspace,{recursive:true,force:true});rmSync(state,{recursive:true,force:true});}
});
