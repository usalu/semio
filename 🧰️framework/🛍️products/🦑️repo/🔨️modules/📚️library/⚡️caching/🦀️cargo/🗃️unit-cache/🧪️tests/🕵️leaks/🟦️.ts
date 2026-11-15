import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {leakedPathsV1} from "../../📜️script.ts";

/** 🕵️ A compiled output that holds a path of this machine is never shared: the roots the scan names are the checkout, the manifest directory, the workspace, the lane, the Cargo home and the Rust home, each as spelled and as resolved. */
const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"leaks-")));
afterAll(()=>{rmSync(base,{recursive:true,force:true});});
const write=(name:string,text:string)=>{const path=join(base,name);writeFileSync(path,text);return path;};

test("a root inside an output is found; a dependency file and an output without a root are not",()=>{
 const clean=write("clean.rlib","no path here"),dirty=write("dirty.rlib",`prefix ${base}/cargo-home/registry/src/x suffix`),depinfo=write("dirty.d",`${base}/cargo-home`);
 expect(leakedPathsV1([clean],[`${base}/cargo-home`])).toBeUndefined();
 expect(leakedPathsV1([dirty],[`${base}/cargo-home`])).toBe(`dirty.rlib: ${base}/cargo-home`);
 expect(leakedPathsV1([depinfo],[`${base}/cargo-home`])).toBeUndefined();
 expect(leakedPathsV1([dirty],[undefined,"/x","",`${base}/other`])).toBeUndefined();
});

test("a root is found under its resolved spelling too: the output holds the real path, the root is given through a link",()=>{
 const real=join(base,"real-rustup");mkdirSync(real);const alias=join(base,"alias-rustup");symlinkSync(real,alias);
 const output=write("tool.rlib",`path ${real}/toolchains/t`);
 expect(leakedPathsV1([output],[alias])).toBe(`tool.rlib: ${real}`);
 expect(leakedPathsV1([output],[join(base,"missing-root-dir")])).toBeUndefined();
});
