/** ✍️ Tests for mounted-image sealing. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{copyFileSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{sealMacExecutableV1}from"../🟦️.ts";
test("sealing is a no-op off macOS and never reads the file",()=>{
 expect(()=>sealMacExecutableV1("/does/not/exist","linux")).not.toThrow();expect(()=>sealMacExecutableV1("/does/not/exist","win32")).not.toThrow();
});
test.skipIf(process.platform!=="darwin")("a copied executable is ad-hoc signed through the pinned tool and an independent verification accepts it, even with a hostile PATH",()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("sealing"),"seal-"))),copy=join(directory,"true"),bin=join(directory,"bin"),before=process.env.PATH;
 try{
  copyFileSync("/usr/bin/true",copy);spawnSync("/usr/bin/codesign",["--remove-signature",copy]);
  process.env.PATH=bin;sealMacExecutableV1(copy);
  const verify=spawnSync("/usr/bin/codesign",["--verify","--strict",copy],{encoding:"utf8"});expect(verify.status,verify.stderr).toBe(0);
  const detail=spawnSync("/usr/bin/codesign",["-dv",copy],{encoding:"utf8"});expect(detail.stderr).toContain("Signature=adhoc");
  expect(spawnSync(copy,[]).status).toBe(0);
 }finally{process.env.PATH=before;rmSync(directory,{recursive:true,force:true});}
});
test.skipIf(process.platform!=="darwin")("a path codesign refuses is reported with its status",()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("sealing"),"seal-")));
 try{expect(()=>sealMacExecutableV1(join(directory,"missing"))).toThrow("codesign refused");}finally{rmSync(directory,{recursive:true,force:true});}
});
