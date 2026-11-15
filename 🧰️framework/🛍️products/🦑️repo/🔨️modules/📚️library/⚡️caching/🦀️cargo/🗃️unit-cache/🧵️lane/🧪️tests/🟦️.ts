import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {compilerTemporaryArgumentsV1,lanePathsV1} from "../🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import envelope from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📏️path-envelope/🧬️schema/🔣️.json" with {type:"json"};
const layout=UNIT_CACHE_POLICY_V1.layout,budget=envelope.definitions.Budget.const,policy=envelope.definitions.Policy.const;
const workspace=(()=>{let directory=dirname(fileURLToPath(import.meta.url));while(directory!==dirname(directory)&&!existsSync(join(directory,".cargo","config.toml")))directory=dirname(directory);return directory;})();
test("rustc gets the short lane temporary directory for compilations and nothing for probes",()=>{
 const root=mkdtempSync(join(tmpdir(),"lane-temporaries-"));
 try{
  const paths=lanePathsV1(root,"p000000"),compile=["--crate-name","a","--out-dir","o","-Z","threads=0"],args=compilerTemporaryArgumentsV1(paths.temporaries,compile);
  expect(compilerTemporaryArgumentsV1(join(root,"stable"),["--crate-name","a","--out-dir","o"])).toEqual([]);expect(compilerTemporaryArgumentsV1(join(root,"probe"),["--crate-name","___","--print=file-names","-Z","threads=0"])).toEqual([]);
  expect(args).toEqual(["-Z","temps-dir="+join(root,layout.lanes,"p000000",layout.compilerTemporaries)]);
  expect(existsSync(paths.temporaries)).toBe(true);
  expect(compilerTemporaryArgumentsV1(join(root,"unused"),["-vV"])).toEqual([]);expect(existsSync(join(root,"unused"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the layout the path envelope budgets for equals the layout the unit cache creates",()=>{
 expect(layout.compilerTemporaries).toBe(policy.temporaryDirectory);
 expect(budget.laneDirectory).toBe(layout.lanes);
 expect(UNIT_CACHE_POLICY_V1.pool.product.prefix.length+UNIT_CACHE_POLICY_V1.pool.product.digestDigits).toBe(budget.laneCharacters);
 expect(()=>lanePathsV1("/r","q"+"0".repeat(budget.laneCharacters-1))).not.toThrow();
 expect(budget.cacheDirectory.split("/").at(-1)).toBe("u");
 expect(readFileSync(join(workspace,".cargo","config.toml"),"utf8")).toContain("build-dir = \""+budget.cacheDirectory+"/"+budget.laneDirectory+"/");
});
