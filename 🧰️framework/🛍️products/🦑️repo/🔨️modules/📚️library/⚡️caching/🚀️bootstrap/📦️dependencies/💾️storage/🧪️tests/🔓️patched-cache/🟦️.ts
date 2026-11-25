import{expect,test}from"bun:test";
import{chmodSync,existsSync,linkSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

/** 🔓️ Contract: before an install the cache entries of the patched packages hold private writable files, because Bun copies them with their mode and patches the copies in place (a read-only shared Store object fails every patch with `EACCES` on `open()`); the shared object stays read-only and intact, other entries are untouched, and stale patched results of failed attempts are removed. */
test("patched package cache files are unshared and stale patch results removed",async()=>{
 const{unshareJavascriptDependencyPatchedCacheV1:unshare}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"unshare-"));
 try{
  const cache=join(root,"cache"),object=join(root,"object"),entry=join(cache,"nx@23.2.0@@host@@@1"),scoped=join(cache,"@s","p@1.0.0@@host@@@1"),other=join(cache,"left-pad@1.0.0@@host@@@1"),stale=join(cache,"nx@23.2.0@@host@@@1_patch_hash=abc");
  mkdirSync(join(entry,"dist"),{recursive:true});mkdirSync(scoped,{recursive:true});mkdirSync(other,{recursive:true});mkdirSync(stale,{recursive:true});
  writeFileSync(object,"shared");chmodSync(object,0o444);
  linkSync(object,join(entry,"dist","big.js"));linkSync(object,join(scoped,"index.js"));linkSync(object,join(other,"index.js"));writeFileSync(join(stale,"x"),"y");
  const manifest=(patched:Record<string,string>)=>({path:"package.json",bytes:Buffer.from(JSON.stringify({patchedDependencies:patched}))});
  const done=unshare(cache,[manifest({"nx@23.2.0":"patches/0.patch","@s/p@1.0.0":"patches/1.patch","../x@1":"patches/2.patch"})]);
  expect(done).toEqual({unshared:2,removed:1});
  for(const path of[join(entry,"dist","big.js"),join(scoped,"index.js")]){const facts=lstatSync(path);expect(facts.nlink).toBe(1);expect(readFileSync(path,"utf8")).toBe("shared");if(process.platform!=="win32")expect(facts.mode&0o200).toBe(0o200);}
  expect(lstatSync(join(other,"index.js")).nlink).toBe(2);
  expect(lstatSync(object).nlink).toBe(2);if(process.platform!=="win32")expect(lstatSync(object).mode&0o222).toBe(0);
  expect(existsSync(stale)).toBe(false);
  expect(unshare(cache,[manifest({"nx@23.2.0":"patches/0.patch"})])).toEqual({unshared:0,removed:0});
  if(process.platform!=="win32"){const probe=Bun.spawnSync(["python3","-c","import os,sys;print(os.access(sys.argv[1],os.W_OK),os.stat(sys.argv[1]).st_nlink)",join(entry,"dist","big.js")]);if(probe.exitCode===0)expect(probe.stdout.toString().trim()).toBe("True 1");}
 }finally{rmSync(root,{recursive:true,force:true});}
});
