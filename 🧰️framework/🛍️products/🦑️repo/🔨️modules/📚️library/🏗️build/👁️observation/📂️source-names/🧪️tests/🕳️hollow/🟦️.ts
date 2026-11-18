import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{directoryFactOfV1}from"../../../🟨️.mjs";
import{sourceTreeFilterV1}from"../../🟨️.mjs";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(names:readonly string[])=>createHash("sha256").update(JSON.stringify([...names].sort())).digest("hex");
const write=(base:string,path:string,text="x\n")=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);};

/** 🕳️ The tree of one machine as the replica leaves it: the same tracked files, and the directories that remain where tracked files were removed (empty, empty below, holding only noise or output). */
function replica(label:string,leftovers:boolean):string{
 const base=mkdtempSync(join(tmpdir(),`source-names-${label}-`));scratch.push(base);
 for(const path of["package.json","📋️project.json","📜️script.ts","src/🟦️.ts","deep/a/b/c.txt"])write(base,path);
 write(base,".gitignore",".DS_Store\nThumbs.db\n");
 if(leftovers){
  for(const path of["🧬️schema","🏗️builder/🌐️vite","src/🧪️old/🌐️vite/🧱️deeper","ghost/inner"])mkdirSync(join(base,path),{recursive:true});
  write(base,"ghost/.DS_Store");write(base,"ghost/inner/Thumbs.db");
 }
 return base;
}
const trackedNames=(base:string):string[]=>{
 const git=(...args:string[])=>Bun.spawnSync(["git","-C",base,"-c","core.quotepath=off","-c","user.email=a@b","-c","user.name=a",...args]);
 git("init","-q");git("add","-A");
 return[...new Set(new TextDecoder().decode(git("ls-files","-z").stdout).split("\0").filter(Boolean).map(path=>path.split("/")[0]!))];
};

test("a directory that holds no source file anywhere below is not part of the source tree: two replicas of one tracked tree give one fact, for every kind that lists names or typed entries",()=>{
 const clean=replica("hollow-clean",false),left=replica("hollow-left",true);
 for(const directory of["","src"])for(const kind of["readdir","readdirNames"] as const)expect(directoryFactOfV1(kind,join(left,directory)),kind+":"+directory).toBe(directoryFactOfV1(kind,join(clean,directory)));
});

test("the fact of a replica with leftovers equals the digest of the top-level names an independent reader (git, which cannot hold an empty directory) tracks",()=>{
 const left=replica("hollow-git",true),tracked=trackedNames(left);
 expect(tracked.sort()).toEqual([".gitignore","deep","package.json","src","📋️project.json","📜️script.ts"].sort());
 expect(directoryFactOfV1("readdirNames",left)).toBe(sha(tracked));
});

test("a directory with a file below it, a link to an empty directory and a file named like an empty directory all stay in the fact",()=>{
 const base=replica("hollow-keep",false),plain=directoryFactOfV1("readdirNames",base);
 mkdirSync(join(base,"hollow"));expect(directoryFactOfV1("readdirNames",base)).toBe(plain);
 write(base,"hollow/inner/file.ts");expect(directoryFactOfV1("readdirNames",base)).not.toBe(plain);
 const linked=replica("hollow-link",false);mkdirSync(join(linked,"empty"));symlinkSync(join(linked,"empty"),join(linked,"link"));
 expect(directoryFactOfV1("readdirNames",linked)).not.toBe(directoryFactOfV1("readdirNames",replica("hollow-link-plain",false)));
 const file=replica("hollow-file",false);write(file,"empty");expect(directoryFactOfV1("readdirNames",file)).not.toBe(directoryFactOfV1("readdirNames",replica("hollow-file-plain",false)));
});

test("the filter drops a hollow entry only when the caller says so",()=>{
 expect(sourceTreeFilterV1(["a","b"],name=>name,()=>false,name=>name==="b")).toEqual(["a"]);
 expect(sourceTreeFilterV1(["a","b"],name=>name,()=>false)).toEqual(["a","b"]);
});
