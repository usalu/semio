import {expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {delimiter,join} from "node:path";
import {cargoNextestAvailableV1} from "../../🟦️.ts";

/** 🪟️ The Cargo test runner lookup follows each host's rules: `cargo-nextest.exe` on Windows, found through a `Path` variable of any case and `;` separated directories with drive letters; the bare name through `PATH` and `:` on POSIX. The Windows rule runs here against a table of existing executables; the POSIX rule runs against real files. */
const exists=(...paths:string[])=>(path:string)=>paths.includes(path);

test("Windows: the .exe is found through a variable of any case and semicolon separated directories, the bare name and other extensions are not",()=>{
 const tool="C:\\Tools\\bin\\cargo-nextest.exe";
 for(const name of ["Path","PATH","path","pAtH"])expect(cargoNextestAvailableV1({[name]:"C:\\Windows;C:\\Tools\\bin;D:\\x"},"win32",exists(tool)),name).toBe(true);
 expect(cargoNextestAvailableV1({Path:"C:\\Windows;;C:\\Tools\\bin\\"},"win32",exists(tool)),"empty entries and a trailing separator").toBe(true);
 expect(cargoNextestAvailableV1({Path:"C:\\Windows"},"win32",exists(tool))).toBe(false);
 expect(cargoNextestAvailableV1({Path:"C:\\Tools\\bin"},"win32",exists("C:\\Tools\\bin\\cargo-nextest"))).toBe(false);
 expect(cargoNextestAvailableV1({Path:"C:\\Tools\\bin"},"win32",exists("C:\\Tools\\bin\\cargo-nextest.cmd"))).toBe(false);
 expect(cargoNextestAvailableV1({},"win32",exists(tool))).toBe(false);expect(cargoNextestAvailableV1({PATH:undefined},"win32",()=>true)).toBe(false);
 expect(cargoNextestAvailableV1({PATH:"/usr/bin:/opt/tools"},"win32",exists("/opt/tools/cargo-nextest.exe")),"a POSIX PATH is not split on colons under Windows rules").toBe(false);
});

test("POSIX: only PATH, only the bare name, colon separated; a lower case variable is not PATH",()=>{
 expect(cargoNextestAvailableV1({PATH:"/usr/bin:/opt/tools"},"linux",exists("/opt/tools/cargo-nextest"))).toBe(true);
 expect(cargoNextestAvailableV1({PATH:"/usr/bin:/opt/tools"},"darwin",exists("/opt/tools/cargo-nextest.exe"))).toBe(false);
 expect(cargoNextestAvailableV1({path:"/opt/tools"},"linux",()=>true)).toBe(false);expect(cargoNextestAvailableV1({PATH:""},"linux",()=>true)).toBe(false);
});

test("the real probe: an executable regular file in a PATH directory counts, a plain file or a directory of that name does not",()=>{
 if(process.platform==="win32")return;
 const root=mkdtempSync(join(tmpdir(),"semio-nextest-")),plain=join(root,"plain"),real=join(root,"real"),directoryNamed=join(root,"directory");
 try{
  for(const directory of [plain,real,directoryNamed])mkdirSync(directory);
  writeFileSync(join(plain,"cargo-nextest"),"");chmodSync(join(plain,"cargo-nextest"),0o644);writeFileSync(join(real,"cargo-nextest"),"#!/bin/sh\n");chmodSync(join(real,"cargo-nextest"),0o755);mkdirSync(join(directoryNamed,"cargo-nextest"));
  expect(cargoNextestAvailableV1({PATH:[plain,directoryNamed].join(delimiter)})).toBe(false);
  expect(cargoNextestAvailableV1({PATH:[plain,directoryNamed,real].join(delimiter)})).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
