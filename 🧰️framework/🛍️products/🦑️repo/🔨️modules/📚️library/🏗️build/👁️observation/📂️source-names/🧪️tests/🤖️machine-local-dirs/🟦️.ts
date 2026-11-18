/** 🤖️ Contract: a directory that git ignores and a generator fills on the machines that ran it (`🤖️generated`) is no part of a directory fact, whatever it holds: a machine that built and a machine that did not give one fact for one tracked tree, and the root .gitignore agrees that the name is not source. A file of that name stays.
 * @see ../../🟨️.mjs */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import schema from"../../🧬️schema/🔣️.json";
import{directoryFactOfV1}from"../../../🟨️.mjs";

const scratch:string[]=[],root="/Users/ueli/Documents/semio";
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const write=(base:string,path:string,text="x\n")=>{mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);};
const tree=(label:string,generated:boolean)=>{const base=mkdtempSync(join(tmpdir(),`machine-local-${label}-`));scratch.push(base);for(const path of["📋️project.json","📜️script.ts","src/🟦️.ts"])write(base,path);if(generated){write(base,"🤖️generated/🎮️playground-session/🟦️.ts");write(base,"src/🤖️generated/🟦️.ts");}return base;};

test("a machine that ran the generators and a machine that did not give one fact, for the listings of the directory and of a subdirectory",()=>{
 const built=tree("built",true),plain=tree("plain",false);
 for(const directory of["","src"])for(const kind of["readdir","readdirNames"] as const)expect(directoryFactOfV1(kind,join(built,directory)),kind+":"+directory).toBe(directoryFactOfV1(kind,join(plain,directory)));
});

test("a file named like the machine-local directory is source and changes the fact",()=>{
 const plain=tree("file-plain",false),filed=tree("file",false);write(filed,"🤖️generated","a file\n");
 expect(directoryFactOfV1("readdirNames",filed)).not.toBe(directoryFactOfV1("readdirNames",plain));
});

test("every declared machine-local directory name is ignored by the root .gitignore, checked by git itself",()=>{
 const names=schema.$defs.rules.const.machineLocalDirectoryNames as string[];
 expect(names.length).toBeGreaterThan(0);
 for(const name of names){const ignored=Bun.spawnSync(["git","-C",root,"check-ignore","-q","--no-index","✏️s/x/"+name+"/y.ts"]);expect(ignored.exitCode,name).toBe(0);}
});
