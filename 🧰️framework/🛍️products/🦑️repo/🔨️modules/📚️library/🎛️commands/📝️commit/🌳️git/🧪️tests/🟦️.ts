/** 🪢️ `gitShallow` agrees with git about a full repository, a depth-one clone and a boundary written over a full history. */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {gitShallow} from "../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const gitTest=Bun.spawnSync(["git","--version"]).exitCode===0?test:test.skip;
const run=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-C",cwd,...args],{stdout:"pipe",stderr:"pipe",env:{...process.env,GIT_CONFIG_GLOBAL:process.platform==="win32"?"NUL":"/dev/null",GIT_CONFIG_NOSYSTEM:"1",GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@t.invalid",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@t.invalid"}});if(result.exitCode!==0)throw Error(result.stderr.toString());return result.stdout.toString().trim();};

gitTest("a full history is not shallow, a depth-one clone and a boundary over a full history are",()=>{
 const base=mkdtempSync(join(tmpdir(),"commit-shallow-"));scratch.push(base);
 const source=join(base,"source"),clone=join(base,"clone");
 run(base,"init","-q","-b","main","source");
 for(const name of ["a","b"]){writeFileSync(join(source,`${name}.txt`),`${name}\n`);run(source,"add","-A");run(source,"commit","-q","-m",name);}
 expect(gitShallow(source)).toBe(false);
 expect(run(source,"rev-parse","--is-shallow-repository")).toBe("false");
 run(base,"clone","-q","--depth","1",`file://${source}`,"clone");
 expect(gitShallow(clone)).toBe(true);
 expect(run(clone,"rev-parse","--is-shallow-repository")).toBe("true");
 writeFileSync(join(source,".git","shallow"),`${run(source,"rev-parse","HEAD")}\n`);
 expect(gitShallow(source)).toBe(true);
 expect(run(source,"rev-list","--count","HEAD")).toBe("1");
});
