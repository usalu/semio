/** 🚫️ Contract: paths that can never be graph sources (tickets under `.🧬semio`, caches, generated output, dependency and build directories) are not entered by the membership walk, so a peer writing there never moves the membership of a read and never starts a repair round.
 * @see ../../../📁️workspace/🟦️.ts */
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{afterAll,expect,test}from"bun:test";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{workspaceBuildFilesV1}from"../../../📁️workspace/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("files below tickets, caches, generated output and dependency directories are not members of the walk",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"excluded-membership-")));scratch.push(root);
 const kept=["src/📋️project.json","src/deep/Cargo.toml"],dropped=[".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/T/📋️project.json",".cache/b/Cargo.toml","src/🗑️generated/Cargo.toml","node_modules/x/📋️project.json","target/debug/Cargo.toml",".git/Cargo.toml"];
 for(const file of[...kept,...dropped]){mkdirSync(join(root,file,".."),{recursive:true});writeFileSync(join(root,file),"x\n");}
 expect(await workspaceBuildFilesV1(root)).toEqual([...kept].sort());
});
