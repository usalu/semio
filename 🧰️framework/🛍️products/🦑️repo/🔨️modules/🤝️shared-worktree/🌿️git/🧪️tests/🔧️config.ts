import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {gitBrokenV1,normaliseGitConfigV1,repairGitConfigV1} from "../🟦️.ts";

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
