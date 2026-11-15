/** 🧭️ A refused relative filesystem call names what was refused: the message and its code stay as they were (the hashed reference of the service log does not move), the error carries the root and the entry name as data. A directory without search permission makes `fstatat` of an entry fail with EACCES, like macOS does with EPERM below a protected folder. */
import {expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {withArtifactRelativeDirectoryV1} from "../../📁️descriptor/🟦️.ts";

test.skipIf(process.platform==="win32"||process.getuid?.()===0)("the error of a refused entry carries the root and the entry name and keeps its message",async()=>{
  const root=mkdtempSync(join(tmpdir(),"denied-entry-"));
  mkdirSync(join(root,"inner"));writeFileSync(join(root,"victim"),"x");
  try{
    chmodSync(root,0o600);
    let caught:(Error&{code?:string;root?:string;entry?:string})|undefined;
    await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{await directory.entry("victim");}).catch(error=>{caught=error;});
    expect(caught).toBeDefined();
    expect(caught!.message).toMatch(/^Relative filesystem fstatat failed \((EACCES|EPERM)\)$/u);
    expect(["EACCES","EPERM"]).toContain(caught!.code);
    expect(caught!.root).toBe(root);expect(caught!.entry).toBe("victim");
  }finally{chmodSync(root,0o700);rmSync(root,{recursive:true,force:true});}
});
