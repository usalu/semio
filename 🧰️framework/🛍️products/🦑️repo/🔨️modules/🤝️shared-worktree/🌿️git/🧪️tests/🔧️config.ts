import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {gitBrokenV1,normaliseGitConfigV1,repairGitConfigV1,repairGitShallowV1} from "../🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const posix=process.platform==="win32"?test.skip:test;
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const run=(cwd:string,...args:string[]):string=>{const result=Bun.spawnSync(["git","-C",cwd,...args],{stdout:"pipe",stderr:"pipe",env:{...process.env,GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1",GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@t.invalid",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@t.invalid"}});if(result.exitCode!==0)throw Error(result.stderr.toString());return result.stdout.toString();};
const repository=():string=>{const root=mkdtempSync(join(tmpdir(),"git-config-"));scratch.push(root);run(root,"init","-q","-b","main");writeFileSync(join(root,"a.txt"),"a\n");run(root,"add","a.txt");run(root,"commit","-q","-m","one");return root;};

posix("a config git cannot parse is replaced by a minimal valid one, the broken file is kept in lost, HEAD resolves again and the pins are set",async()=>{
 const root=repository(),config=join(root,".git","config");
 writeFileSync(config,"[core\n\tbare = =\n");
 expect(await gitBrokenV1(root)).toBe("config-unreadable");
 expect(await repairGitConfigV1(root,1_700_000_000_123)).toBe(true);
 expect(await gitBrokenV1(root)).toBeNull();
 const lost=join(root,".cache","f","lost","1700000000123-git-config","config");
 expect(readFileSync(lost,"utf8")).toBe("[core\n\tbare = =\n");
 const text=readFileSync(config,"utf8");
 expect(text).toContain("repositoryformatversion = 0");expect(text).toContain("bare = false");expect(text).toMatch(/autocrlf\s*=\s*false/u);
 expect(run(root,"rev-parse","HEAD").trim()).toMatch(/^[0-9a-f]{40}$/u);
});

posix("a sound config, an absent config and a repository without git are left alone",async()=>{
 const root=repository(),config=join(root,".git","config"),before=readFileSync(config,"utf8");
 expect(await repairGitConfigV1(root,1)).toBe(false);
 expect(readFileSync(config,"utf8")).toBe(before);
 rmSync(config);
 expect(await repairGitConfigV1(root,2)).toBe(false);
 expect(existsSync(join(root,".cache","f","lost"))).toBe(false);
 expect(readdirSync(join(root,".git")).includes("config")).toBe(false);
});

posix("machine-local ignore rules in .git/info/exclude are emptied, kept once under lost, and a view no longer differs from a machine without them",async()=>{
 const root=repository(),exclude=join(root,".git","info","exclude");
 mkdirSync(join(root,".git","info"),{recursive:true});
 writeFileSync(exclude,"# comment\nscratch/\n*.sample\n");
 writeFileSync(join(root,"b.sample"),"x\n");mkdirSync(join(root,"scratch"));writeFileSync(join(root,"scratch","y.txt"),"y\n");
 const hidden=run(root,"status","--porcelain");
 expect(hidden).not.toContain("b.sample");
 expect(await normaliseGitConfigV1(root)).toContain("info/exclude");
 expect(run(root,"status","--porcelain")).toContain("b.sample");
 expect(run(root,"status","--porcelain")).toContain("scratch/");
 const kept=readdirSync(join(root,".cache","f","lost")).filter(name=>name.endsWith("-info-exclude"));
 expect(kept).toHaveLength(1);
 expect(readFileSync(join(root,".cache","f","lost",kept[0]!),"utf8")).toBe("# comment\nscratch/\n*.sample\n");
 expect(await normaliseGitConfigV1(root)).not.toContain("info/exclude");
 expect(readdirSync(join(root,".cache","f","lost")).filter(name=>name.endsWith("-info-exclude"))).toHaveLength(1);
});

posix("a shallow boundary over a history that is fully present is removed and kept in lost, and git reads every commit again",async()=>{
 const root=repository(),shallow=join(root,".git","shallow");
 for(const name of ["b","c"]){writeFileSync(join(root,`${name}.txt`),`${name}\n`);run(root,"add","-A");run(root,"commit","-q","-m",name);}
 const tip=run(root,"rev-parse","HEAD").trim(),parent=run(root,"rev-parse","HEAD~1").trim();
 writeFileSync(shallow,`${tip}\n`);
 expect(run(root,"rev-parse","--is-shallow-repository").trim()).toBe("true");
 expect(run(root,"rev-list","--count","HEAD").trim()).toBe("1");
 expect(run(root,"log","-1","--format=%P","HEAD").trim()).toBe("");
 expect(await repairGitShallowV1(root,1_700_000_000_456)).toBe(true);
 expect(existsSync(shallow)).toBe(false);
 expect(readFileSync(join(root,".cache","f","lost","1700000000456-git-shallow","shallow"),"utf8")).toBe(`${tip}\n`);
 expect(run(root,"rev-parse","--is-shallow-repository").trim()).toBe("false");
 expect(run(root,"rev-list","--count","HEAD").trim()).toBe("3");
 expect(run(root,"log","-1","--format=%P","HEAD").trim()).toBe(parent);
 expect(await repairGitShallowV1(root,2)).toBe(false);
});

posix("a shallow boundary whose history is absent, and one that names no commit of the repository, are left alone",async()=>{
 const source=repository();
 writeFileSync(join(source,"b.txt"),"b\n");run(source,"add","-A");run(source,"commit","-q","-m","b");
 const clone=mkdtempSync(join(tmpdir(),"git-shallow-"));scratch.push(clone);
 run(clone,"clone","-q","--depth","1",`file://${source}`,"work");
 const root=join(clone,"work"),shallow=join(root,".git","shallow"),before=readFileSync(shallow,"utf8");
 expect(run(root,"rev-parse","--is-shallow-repository").trim()).toBe("true");
 expect(await repairGitShallowV1(root,3)).toBe(false);
 expect(readFileSync(shallow,"utf8")).toBe(before);
 writeFileSync(join(source,".git","shallow"),`${"0".repeat(40)}\n`);
 expect(await repairGitShallowV1(source,4)).toBe(false);
 expect(existsSync(join(source,".git","shallow"))).toBe(true);
 expect(existsSync(join(root,".cache","f","lost"))).toBe(false);
});
