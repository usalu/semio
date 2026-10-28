/** 🧪️ Tests for bazel execution toolchain-link. */
import {expect,test} from "bun:test";
import {copyFileSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as api from "../../🟦️.ts";

/** 🪟️ Contract: the toolchain entry of the generated Bazel package never needs the symbolic-link privilege on Windows; it is the provisioned file itself (hard link) or an up-to-date copy. The Windows rules run on any host through the injected platform. */
const fresh=()=>mkdtempSync(join(tmpdir(),"bun-win-link-"));
const entry=(workspace:string)=>join(workspace,api.bazelBunToolPathV1(api.bazelExecutionPlatformByIdV1("linux-x64")));

test("on Windows the entry is a hard link to the provisioned file, kept when current and replaced when it moves, even over a symbolic link of an older run", async () => {
 const directory=fresh(),first=join(directory,"first"),second=join(directory,"second"),workspace=join(directory,"ws");
 try{
  writeFileSync(first,"1");writeFileSync(second,"22");const path=entry(workspace);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":first},"win32");
  expect(lstatSync(path).isSymbolicLink()).toBe(false);expect(lstatSync(path).ino).toBe(lstatSync(first).ino);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":first},"win32");expect(lstatSync(path).ino).toBe(lstatSync(first).ino);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":second},"win32");expect(readFileSync(path,"utf8")).toBe("22");expect(lstatSync(path).ino).toBe(lstatSync(second).ino);
  rmSync(path);symlinkSync(first,path);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":first},"win32");expect(lstatSync(path).isSymbolicLink(),"a symbolic link is replaced by the file").toBe(false);expect(readFileSync(path,"utf8")).toBe("1");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("when the volumes differ the file is copied, a current copy is kept and a stale copy is refreshed", async () => {
 const directory=fresh(),target=join(directory,"bun"),workspace=join(directory,"ws"),denied=async()=>{throw Object.assign(new Error("EXDEV"),{code:"EXDEV"});};
 let copies=0;const copy=async(from:string,to:string)=>{copies++;copyFileSync(from,to);};
 try{
  writeFileSync(target,"v1");const path=entry(workspace);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":target},"win32",{link:denied as never,copy:copy as never});
  expect(copies).toBe(1);expect(readFileSync(path,"utf8")).toBe("v1");expect(lstatSync(path).mode&0o111).toBeGreaterThan(0);
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":target},"win32",{link:denied as never,copy:copy as never});expect(copies,"a current copy is kept").toBe(1);
  writeFileSync(target,"version two");utimesSync(target,new Date(Date.now()+5000),new Date(Date.now()+5000));
  await api.linkBazelToolchainsV1(workspace,{"linux-x64":target},"win32",{link:denied as never,copy:copy as never});expect(copies).toBe(2);expect(readFileSync(path,"utf8")).toBe("version two");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("POSIX hosts keep the symbolic link", async () => {
 const directory=fresh(),target=join(directory,"bun");
 try{writeFileSync(target,"x");mkdirSync(join(directory,"ws"));await api.linkBazelToolchainsV1(join(directory,"ws"),{"linux-x64":target},"linux");expect(readlinkSync(entry(join(directory,"ws")))).toBe(target);}
 finally{rmSync(directory,{recursive:true,force:true});}
});
