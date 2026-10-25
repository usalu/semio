import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {HERMETIC_TABLE_V1} from "../../../🏃️execution/🔒️hermetic/🟦️.ts";
import {remoteExecutionBubblewrapArgumentsV1,remoteExecutionDarwinProfileV1,remoteExecutionSandboxOrderV1,selectRemoteExecutionSandboxV1} from "../🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";

test("the strategies of every host come from the hermetic table: the host's own sandbox first, then the private root; remote belongs to the scheduler",()=>{
 expect(remoteExecutionSandboxOrderV1("darwin")).toEqual(["darwin-sandbox","isolated-root"]);
 expect(remoteExecutionSandboxOrderV1("linux")).toEqual(["linux-sandbox","isolated-root"]);
 expect(remoteExecutionSandboxOrderV1("win32")).toEqual(["isolated-root"]);
 for(const [platform,row] of Object.entries(HERMETIC_TABLE_V1.strategies as Record<string,string[]>))expect(remoteExecutionSandboxOrderV1(platform).length).toBe(row.filter(name=>name!=="remote").length);
 expect(()=>remoteExecutionSandboxOrderV1("plan9")).toThrow(/Hermetic strategies required/u);
 expect(selectRemoteExecutionSandboxV1("isolated-root").wrap({argv:["a","b"],env:{X:"1"},cwd:"/r",root:"/r",network:false})).toEqual({argv:["a","b"],env:{X:"1"},cwd:"/r"});
 if(process.platform!=="darwin")expect(()=>selectRemoteExecutionSandboxV1("darwin-sandbox")).toThrow(/not available/u);
});

test("the macOS profile denies network and writes by default, escapes awkward paths, and the Linux command line binds only the input root",()=>{
 const profile=remoteExecutionDarwinProfileV1('/tmp/we"ird \\ 🧬/root',false);
 expect(profile).toContain('(deny network*)');expect(profile).toContain('(deny file-write*)');expect(profile).toContain('(subpath "/tmp/we\\"ird \\\\ 🧬/root")');
 expect(remoteExecutionDarwinProfileV1("/r",true)).not.toContain("(deny network*)");
 const bwrap=remoteExecutionBubblewrapArgumentsV1({argv:["/bin/tool","x"],env:{A:"1",B:"2"},cwd:"/r/work",root:"/r",network:false});
 expect(bwrap.slice(0,5)).toEqual(["bwrap","--unshare-net","--unshare-pid","--unshare-ipc","--die-with-parent"]);
 expect(bwrap.join(" ")).toContain("--ro-bind / / --dev /dev --proc /proc --bind /r /r --chdir /r/work --clearenv --setenv A 1 --setenv B 2 -- /bin/tool x");
 expect(remoteExecutionBubblewrapArgumentsV1({argv:["x"],env:{},cwd:"/r",root:"/r",network:true})).not.toContain("--unshare-net");
});

test.skipIf(process.platform!=="darwin")("the generated macOS profile is accepted by sandbox-exec for paths with emoji, quotes and spaces, and allows exactly the root",()=>{
 const parent=mkdtempSync(join(testScratchRootV1("rexec-inhouse"),"sbx-")),root=join(parent,'we"ird 🧬 root'),outside=join(parent,"outside.txt");
 try{
  mkdirSync(root);writeFileSync(join(parent,"secret.txt"),"s");
  const run=(script:string)=>Bun.spawnSync(["/usr/bin/sandbox-exec","-p",remoteExecutionDarwinProfileV1(realpathSync(root),false),"/bin/sh","-c",script],{cwd:root,stdout:"pipe",stderr:"pipe"});
  expect(run("echo inside > inside.txt").exitCode).toBe(0);
  expect(run(`echo no > ${JSON.stringify(outside)}`).exitCode).not.toBe(0);
  expect(run(`cat ${JSON.stringify(join(parent,"secret.txt"))}`).exitCode).not.toBe(0);
  expect(run("cat inside.txt").stdout.toString()).toBe("inside\n");
 }finally{rmSync(parent,{recursive:true,force:true});}
});
