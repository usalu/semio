import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{nativeCreatorClaimAbsentV1}from"../🟦️.ts";

/** 🕳️ Contract: a funded child is claimless exactly when its creator sidecar `.native-birth-<allocation>.jsonl` beside the child directory is missing; python `os.path.lexists` is the independent oracle. */
test("a creator claim is absent exactly when its sidecar is missing beside the child directory",async()=>{
 const root=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"claimless-"))),directory=join(root,"child","native"),allocation="dd1cd41bf0ad6d0951dde251a6ff0fba",sidecar=join(root,"child",".native-birth-"+allocation+".jsonl"),request={directory,allocation} as never;
 const lexists=(path:string)=>{const run=Bun.spawnSync(["python3","-I","-c","import os,sys;print(int(os.path.lexists(sys.argv[1])))",path]);expect(run.exitCode).toBe(0);return run.stdout.toString().trim()==="1";};
 try{
  mkdirSync(directory,{recursive:true});
  expect(await nativeCreatorClaimAbsentV1(request)).toBe(!lexists(sidecar));expect(await nativeCreatorClaimAbsentV1(request)).toBe(true);
  writeFileSync(sidecar,"{}\n");
  expect(await nativeCreatorClaimAbsentV1(request)).toBe(!lexists(sidecar));expect(await nativeCreatorClaimAbsentV1(request)).toBe(false);
  expect(await nativeCreatorClaimAbsentV1({directory,allocation:"0".repeat(32)} as never)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
