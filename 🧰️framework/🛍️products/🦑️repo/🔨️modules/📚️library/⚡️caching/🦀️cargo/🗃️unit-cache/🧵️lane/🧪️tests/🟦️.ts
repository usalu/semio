import {existsSync,mkdtempSync,readFileSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {compilerTemporaryArgumentsV1,enterLaneV1,imageFaultV1,lanePathsV1,removeInImageV1} from "../🟦️.ts";
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
test("an image that failed under a running command answers a fault the release and the removal survive, any other error is never swallowed",()=>{
 const fault=(code:string)=>Object.assign(Error(code+": rm"),{code});
 for(const code of["EFAULT","EIO","ENXIO","ENODEV","ENOTCONN","ESTALE"])expect(imageFaultV1(fault(code)),code).toBe(true);
 for(const code of["ENOENT","EACCES","ENOSPC","EBUSY"])expect(imageFaultV1(fault(code)),code).toBe(false);
 expect(imageFaultV1(undefined)).toBe(false);expect(imageFaultV1(Error("plain"))).toBe(false);
 expect(removeInImageV1(()=>{})).toBe(true);
 expect(removeInImageV1(()=>{throw fault("EFAULT");})).toBe(false);
 expect(()=>removeInImageV1(()=>{throw fault("EACCES");})).toThrow("EACCES");
});
test("a command releases its lane once, its marker goes and a second release changes nothing",()=>{
 const root=mkdtempSync(join(tmpdir(),"lane-release-"));
 try{
  const entered=enterLaneV1(root,"p000000"),before=existsSync(entered.paths.live)?readFileSync(join(entered.paths.metadata)).length:0;
  expect(before).toBeGreaterThan(0);
  entered.release();entered.release();
  expect(readdirSync(entered.paths.live)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
