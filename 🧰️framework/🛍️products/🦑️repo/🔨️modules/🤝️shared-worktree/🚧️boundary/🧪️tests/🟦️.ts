import {describe,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {existsSync,readFileSync} from "node:fs";
import {chmod,mkdir,mkdtemp,readdir,rm,symlink,utimes,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {compileIgnoreV1,foldNameV1,hashFileV1,linkRefusalV1,refusePathV1,scanSharedV1,sharedV1,type FleetPathRefusalV1} from "../🟦️.ts";

type Group={name:string;lines?:string[];source?:"repository";cases:{path:string;directory:boolean;ignored:boolean}[]};
type PathCase={path:string;times?:number;suffix?:string;refusal:FleetPathRefusalV1|null};
type LinkCase={path:string;target:string;refusal:FleetPathRefusalV1|null};

const groups=fixture.ignore as unknown as Group[],paths=fixture.paths as unknown as PathCase[],links=fixture.links as unknown as LinkCase[];
const posix=process.platform!=="win32";
const repositoryText=():string=>{
 let directory=import.meta.dir;
 while(!existsSync(join(directory,".gitignore"))){
  if(dirname(directory)===directory)throw Error("The repository .gitignore was not found above the tests");
  directory=dirname(directory);
 }
 return readFileSync(join(directory,".gitignore"),"utf8");
};
const textOf=(group:Group):string=>group.source==="repository"?repositoryText():(group.lines??[]).join("\n");
const pathOf=(row:PathCase):string=>row.path.repeat(row.times??1)+(row.suffix??"");

test("the fixture satisfies the closed schema",()=>{
 const ajv=new Ajv({strict:false,allowUnionTypes:true}),check=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(check(fixture),JSON.stringify(check.errors?.[0])).toBe(true);
});

describe("gitignore vectors",()=>{
 for(const group of groups)test(group.name,()=>{
  const ignore=compileIgnoreV1(textOf(group));
  for(const row of group.cases)expect(ignore.ignores(row.path,row.directory),`${row.path} directory=${row.directory}`).toBe(row.ignored);
 });
});

test("the digest is the sha256 of the text and the verdicts do not depend on the call order",()=>{
 expect(compileIgnoreV1("").digest).toBe(createHash("sha256").update("").digest("hex"));
 expect(compileIgnoreV1("a\n").digest).not.toBe(compileIgnoreV1("a\r\n").digest);
 const group=groups.find(row=>row.source==="repository")!,ignore=compileIgnoreV1(textOf(group)),reversed=[...group.cases].reverse();
 for(const row of [...group.cases,...reversed,...group.cases])expect(ignore.ignores(row.path,row.directory),row.path).toBe(row.ignored);
});

test("a carriage return line ending and a trailing slash on the queried path change nothing",()=>{
 const ignore=compileIgnoreV1("build/\r\n*.log\r\n");
 expect(ignore.ignores("x/build",true)).toBe(true);
 expect(ignore.ignores("x/build/",true)).toBe(true);
 expect(ignore.ignores("a.log",false)).toBe(true);
 expect(ignore.ignores("a.txt",false)).toBe(false);
});

test("the matcher handles the whole repository file and stays fast on many paths",()=>{
 const ignore=compileIgnoreV1(repositoryText()),started=performance.now();
 let ignored=0;
 for(let i=0;i<20000;i++)if(ignore.ignores(`dir${i%97}/sub${i%13}/file${i}.ts`,false))ignored++;
 expect(ignored).toBe(0);
 expect(performance.now()-started).toBeLessThan(5000);
});

describe("path policy vectors",()=>{
 test("refusePathV1",()=>{for(const row of paths)expect(refusePathV1(pathOf(row))??null,JSON.stringify(row.path)).toBe(row.refusal);});
 test("linkRefusalV1",()=>{for(const row of links)expect(linkRefusalV1(row.path,row.target)??null,`${row.path} -> ${row.target}`).toBe(row.refusal);});
});

test("sharedV1 combines ignore, .git and the path policy",()=>{
 const ignore=compileIgnoreV1("*.log\n");
 expect(sharedV1("src/a.ts",false,ignore)).toBe(true);
 expect(sharedV1("src/a.log",false,ignore)).toBe(false);
 expect(sharedV1(".git/config",false,ignore)).toBe(false);
 expect(sharedV1("sub/.git/config",false,ignore)).toBe(false);
 expect(sharedV1("src/con.ts",false,ignore)).toBe(false);
 expect(sharedV1(".gitignore",false,ignore)).toBe(true);
});

test("hashFileV1 streams sha256 and reports a stable file",async()=>{
 const directory=await mkdtemp(join(tmpdir(),"boundary-hash-")),file=join(directory,"f.txt");
 try{
  await writeFile(file,"hello");
  expect(await hashFileV1(file)).toEqual({sha256:"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",bytes:5,stable:true});
  await writeFile(file,"");
  expect((await hashFileV1(file)).sha256).toBe("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
 }finally{await rm(directory,{recursive:true,force:true});}
});

describe("scanner",()=>{
 const withTree=async(build:(root:string)=>Promise<void>,run:(root:string)=>Promise<void>):Promise<void>=>{
  const root=await mkdtemp(join(tmpdir(),"boundary-scan-"));
  try{await build(root);await run(root);}finally{
   for(const locked of ["node_modules",".git"])await chmod(join(root,locked),0o755).catch(()=>undefined);
   await rm(root,{recursive:true,force:true});
  }
 };
 const put=async(root:string,relative:string,content="x",mode?:number):Promise<void>=>{
  await mkdir(dirname(join(root,relative)),{recursive:true});
  await writeFile(join(root,relative),content);
  if(mode!==undefined)await chmod(join(root,relative),mode);
 };

 test("lists shared files sorted with forward slashes and skips ignored trees and .git",async()=>{
  const ignore=compileIgnoreV1("node_modules\n*.log\n/out/\n");
  await withTree(async root=>{
   await put(root,"b.txt");await put(root,"a.txt","abc");await put(root,"src/z.ts");await put(root,"src/deep/y.ts");
   await put(root,"keep.log");await put(root,"out/o.txt");await put(root,"sub/out/o.txt");
   await put(root,"node_modules/x/index.js");await put(root,".git/config");await put(root,"sub/.git/config");
   await put(root,"run.sh","#!/bin/sh\n",0o755);
  },async root=>{
   const scan=await scanSharedV1(root,ignore);
   expect(scan.entries.map(entry=>entry.path)).toEqual(["a.txt","b.txt","run.sh","src/deep/y.ts","src/z.ts","sub/out/o.txt"]);
   expect(scan.refused).toEqual([]);
   const a=scan.entries.find(entry=>entry.path==="a.txt")!;
   expect(a.kind).toBe("file");expect(a.bytes).toBe(3);expect(a.mtimeMs).toBeGreaterThan(0);
   expect(scan.entries.find(entry=>entry.path==="run.sh")!.executable).toBe(posix);
   expect(scan.entries.find(entry=>entry.path==="b.txt")!.executable).toBe(false);
  });
 });

 test("hands entries to a visitor without keeping them, decides children without walking ancestors, and refuses only the leaf",async()=>{
  const ignore=compileIgnoreV1("node_modules\n*.log\n/out/\n");
  await withTree(async root=>{
   for(let index=0;index<40;index++)await put(root,`d${index%5}/e${index%3}/f${index}.txt`);
   await put(root,"d0/keep.log");await put(root,"out/o.txt");await put(root,"d1/bad:name.txt");await put(root,"d2/Same.txt");await put(root,"d2/same.txt");
  },async root=>{
   const kept=await scanSharedV1(root,ignore),seen:string[]=[],visited=await scanSharedV1(root,ignore,undefined,entry=>{seen.push(entry.path);});
   expect(visited.entries).toEqual([]);
   expect(seen.sort()).toEqual(kept.entries.map(entry=>entry.path));
   expect(visited.refused).toEqual(kept.refused);
   expect(kept.refused.map(item=>item.reason).sort()).toContain("illegal-character");
   expect(ignore.child!("d0/keep.log",false)).toBe(true);expect(ignore.child!("d0/keep.txt",false)).toBe(false);expect(ignore.child!("out",true)).toBe(true);expect(ignore.child!("sub/out",true)).toBe(false);
  });
 });

 test.skipIf(!posix||process.getuid?.()===0)("never descends into an ignored directory or .git",async()=>{
  const ignore=compileIgnoreV1("node_modules\n");
  await withTree(async root=>{
   await put(root,"a.txt");await put(root,"node_modules/x.js");await put(root,".git/config");
   await chmod(join(root,"node_modules"),0o000);await chmod(join(root,".git"),0o000);
  },async root=>{expect((await scanSharedV1(root,ignore)).entries.map(entry=>entry.path)).toEqual(["a.txt"]);});
 });

 test.skipIf(!posix)("reports safe links, refuses unsafe ones and never follows a link to a directory",async()=>{
  const ignore=compileIgnoreV1("");
  await withTree(async root=>{
   await put(root,"a.txt");await put(root,"src/b.ts");
   await symlink("a.txt",join(root,"ok"));
   await symlink("../outside",join(root,"escape"));
   await symlink("/etc/passwd",join(root,"absolute"));
   await symlink("src",join(root,"dirlink"));
  },async root=>{
   const scan=await scanSharedV1(root,ignore);
   expect(scan.entries.map(entry=>entry.path)).toEqual(["a.txt","dirlink","ok","src/b.ts"]);
   const ok=scan.entries.find(entry=>entry.path==="ok")!;
   expect(ok.kind).toBe("link");expect(ok.target).toBe("a.txt");expect(ok.executable).toBe(false);
   expect(scan.entries.find(entry=>entry.path==="dirlink")!.target).toBe("src");
   expect(scan.refused).toEqual([{path:"absolute",reason:"unsafe-link"},{path:"escape",reason:"unsafe-link"}]);
  });
 });

 test.skipIf(!posix)("refuses policy violations and does not descend into a refused directory",async()=>{
  const ignore=compileIgnoreV1("");
  await withTree(async root=>{
   await put(root,"ok.txt");await put(root,"bad:name.txt");await put(root,"trail.");await put(root,"aux.txt");await put(root,"con/inner.txt");
  },async root=>{
   const scan=await scanSharedV1(root,ignore);
   expect(scan.entries.map(entry=>entry.path)).toEqual(["ok.txt"]);
   expect(scan.refused).toEqual([{path:"aux.txt",reason:"reserved-name"},{path:"bad:name.txt",reason:"illegal-character"},{path:"con",reason:"reserved-name"},{path:"trail.",reason:"trailing-dot-or-space"}]);
  });
 });

 test("refuses the later of two names that differ only by case, the first in sorted order wins",async()=>{
  const ignore=compileIgnoreV1("");
  await withTree(async root=>{await put(root,"Case.txt","upper");await put(root,"case.txt","lower");await put(root,"other.txt");},async root=>{
   const present=await readdir(root);
   const scan=await scanSharedV1(root,ignore);
   if(present.includes("Case.txt")&&present.includes("case.txt")){
    expect(scan.entries.map(entry=>entry.path)).toEqual(["Case.txt","other.txt"]);
    expect(scan.refused).toEqual([{path:"case.txt",reason:"case-collision"}]);
   }else expect(scan.refused).toEqual([]);
  });
 });

 test("an ignored sibling does not collide with a shared one",async()=>{
  const ignore=compileIgnoreV1("Data.txt\n");
  await withTree(async root=>{await put(root,"Data.txt");await put(root,"data.txt");},async root=>{
   const scan=await scanSharedV1(root,ignore);
   expect(scan.refused).toEqual([]);
   expect(scan.entries.map(entry=>entry.path)).toEqual((await readdir(root)).includes("data.txt")?["data.txt"]:[]);
  });
 });

 test("honours an aborted signal and a missing worktree",async()=>{
  const ignore=compileIgnoreV1(""),controller=new AbortController();
  await withTree(async root=>{await put(root,"a.txt");},async root=>{
   controller.abort();
   await expect(scanSharedV1(root,ignore,controller.signal)).rejects.toBeDefined();
   await expect(scanSharedV1(join(root,"missing"),ignore)).rejects.toBeDefined();
  });
 });

 test("the output is deterministic across runs",async()=>{
  const ignore=compileIgnoreV1("*.log\n");
  await withTree(async root=>{for(let i=0;i<40;i++)await put(root,`d${i%5}/f${i}.txt`);},async root=>{
   const first=await scanSharedV1(root,ignore),second=await scanSharedV1(root,ignore);
   expect(second).toEqual(first);
   expect(first.entries.map(entry=>entry.path)).toEqual(first.entries.map(entry=>entry.path).sort());
   await utimes(join(root,"d0/f0.txt"),new Date(1000),new Date(1000));
   expect((await scanSharedV1(root,ignore)).entries.find(entry=>entry.path==="d0/f0.txt")!.mtimeMs).toBe(1000);
  });
 });
});

describe("Windows behaviour, simulated on any host",()=>{
 const asWindows=async<T>(run:()=>Promise<T>):Promise<T>=>{
  const original=Object.getOwnPropertyDescriptor(process,"platform")!;
  Object.defineProperty(process,"platform",{value:"win32",configurable:true});
  try{return await run();}finally{Object.defineProperty(process,"platform",original);}
 };
 const inTree=async(build:(root:string)=>Promise<void>,run:(root:string)=>Promise<void>):Promise<void>=>{
  const root=await mkdtemp(join(tmpdir(),"boundary-win-"));
  try{await build(root);await run(root);}finally{await rm(root,{recursive:true,force:true});}
 };

 test.skipIf(!posix)("the executable bit is never reported on Windows, where the mode carries no such bit",async()=>{
  await inTree(async root=>{await writeFile(join(root,"run.sh"),"#!/bin/sh\n");await chmod(join(root,"run.sh"),0o755);},async root=>{
   expect((await scanSharedV1(root,compileIgnoreV1(""))).entries[0]!.executable).toBe(true);
   expect((await asWindows(()=>scanSharedV1(root,compileIgnoreV1("")))).entries[0]!.executable).toBe(false);
  });
 });

 test.skipIf(!posix)("a link target with backslashes is reported with forward slashes on Windows only, so the stored target and the scanned target agree",async()=>{
  await inTree(async root=>{await symlink("sub\\inner\\x.txt",join(root,"link"));},async root=>{
   expect((await scanSharedV1(root,compileIgnoreV1(""))).entries[0]!.target).toBe("sub\\inner\\x.txt");
   const windows=await asWindows(()=>scanSharedV1(root,compileIgnoreV1("")));
   expect(windows.entries[0]!.target).toBe("sub/inner/x.txt");
   expect(windows.refused).toEqual([]);
  });
 });

 test.skipIf(!posix)("a link whose target is drive-qualified or a UNC path is refused whatever the host",async()=>{
  await inTree(async root=>{await symlink("C:\\Windows",join(root,"drive"));await symlink("\\\\host\\share\\x",join(root,"unc"));await symlink("..\\up",join(root,"up"));},async root=>{
   for(const scan of[await scanSharedV1(root,compileIgnoreV1("")),await asWindows(()=>scanSharedV1(root,compileIgnoreV1("")))]){
    expect(scan.entries).toEqual([]);
    expect(scan.refused).toEqual([{path:"drive",reason:"unsafe-link"},{path:"unc",reason:"unsafe-link"},{path:"up",reason:"unsafe-link"}]);
   }
  });
 });

 test("names that differ only by case, by the final sigma or by sharp s fold to one name, different names do not",()=>{
  for(const [left,right] of[["Case.txt","case.txt"],["ας.txt","ασ.txt"],["Σ.txt","ς.txt"],["Straße","STRASSE"],["É.md","é.md"]])expect(foldNameV1(left),`${left} ${right}`).toBe(foldNameV1(right));
  for(const [left,right] of[["a.txt","b.txt"],["a.txt","a.txt "],["i","İ"],["ab","a b"]])expect(foldNameV1(left),`${left} ${right}`).not.toBe(foldNameV1(right));
 });

 test("device names are refused with superscript digits, any extension and any case, and the component beside them is not",()=>{
  for(const name of["COM\u00b9","com\u00b2.txt","Lpt\u00b3.tar.gz","NUL.","aux","Con .txt"])expect(refusePathV1(`dir/${name}`),name).toBeDefined();
  for(const name of["COM\u2074","COM10","console","nul-device","aux_1"])expect(refusePathV1(`dir/${name}`),name).toBeUndefined();
 });

 test.skipIf(!posix||process.getuid?.()===0)("hashFileV1 rejects with the original errno of an unreadable or missing file, so that the caller can tell a file held by someone else from a vanished one",async()=>{
  await inTree(async root=>{await writeFile(join(root,"locked"),"x");await chmod(join(root,"locked"),0o000);},async root=>{
   const code=async(file:string):Promise<string|undefined>=>hashFileV1(join(root,file)).then(()=>undefined,error=>(error as {code?:string}).code);
   expect(await code("locked")).toBe("EACCES");
   expect(await code("missing")).toBe("ENOENT");
   await chmod(join(root,"locked"),0o644);
  });
 });
});

