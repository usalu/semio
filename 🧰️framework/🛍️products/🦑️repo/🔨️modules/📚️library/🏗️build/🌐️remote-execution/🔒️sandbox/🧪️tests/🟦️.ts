import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {HERMETIC_TABLE_V1} from "../../../🏃️execution/🔒️hermetic/🟦️.ts";
import {remoteExecutionBubblewrapArgumentsV1,remoteExecutionBubblewrapProbeArgumentsV1,remoteExecutionDarwinProfileV1,remoteExecutionSandboxOrderV1,selectRemoteExecutionSandboxV1} from "../🟦️.ts";
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

test("the macOS profile denies network and writes by default, escapes awkward paths, and the Linux command line binds no host: only system directories, declared tools and the input root",()=>{
 const profile=remoteExecutionDarwinProfileV1('/tmp/we"ird \\ 🧬/root',false);
 expect(profile).toContain('(deny network*)');expect(profile).toContain('(deny file-write*)');expect(profile).toContain('(subpath "/tmp/we\\"ird \\\\ 🧬/root")');
 expect(remoteExecutionDarwinProfileV1("/r",true)).not.toContain("(deny network*)");
 const bwrap=remoteExecutionBubblewrapArgumentsV1({argv:["/bin/tool","x"],env:{A:"1",B:"2"},cwd:"/r/work",root:"/r",network:false});
 expect(bwrap.slice(0,5)).toEqual(["bwrap","--unshare-net","--unshare-pid","--unshare-ipc","--die-with-parent"]);
 expect(bwrap.join(" ")).toContain("--dev /dev --proc /proc --tmpfs /tmp --bind /r /r --chdir /r/work --clearenv --setenv A 1 --setenv B 2 -- /bin/tool x");
 const layout=(path:string)=>path==="/bin"||path==="/lib"?{kind:"symlink" as const,target:"usr"+path}:["/usr","/lib64","/etc/ld.so.cache"].includes(path)?{kind:path==="/etc/ld.so.cache"?"file" as const:"directory" as const}:undefined;
 const confined=remoteExecutionBubblewrapArgumentsV1({argv:["/bin/tool"],env:{},cwd:"/r",root:"/r",network:false,readable:["/home/u/.rustup/toolchains/t"]},layout);
 for(const secret of["/home","/root","/etc/shadow","/var","/mnt","/Users","/srv"])expect(confined.filter(item=>item===secret||item.startsWith(secret+"/")&&!item.startsWith("/home/u/.rustup")),secret).toEqual([]);
 expect(confined.join(" ")).toContain("--ro-bind /usr /usr --symlink usr/bin /bin --symlink usr/lib /lib --ro-bind /lib64 /lib64 --ro-bind /etc/ld.so.cache /etc/ld.so.cache");
 expect(confined.join(" ")).toContain("--ro-bind /home/u/.rustup/toolchains/t /home/u/.rustup/toolchains/t --bind /r /r");
 for(let index=0;index<confined.length;index++)if(confined[index]==="--ro-bind"||confined[index]==="--bind")expect(confined[index+1],"never the whole host").not.toBe("/");
 for(const hidden of["/Users","/private/etc/semio","/private/var/root","/Library/Keychains","/System/Volumes/Data/Users"])expect(profile).toContain(`(subpath "${hidden}")`);
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

test("the Linux availability probe runs the real sandbox command line (namespaces, devices, /proc), so a container that refuses them is not taken for a confining host",()=>{
 const probeArguments=remoteExecutionBubblewrapProbeArgumentsV1("/r"),real=remoteExecutionBubblewrapArgumentsV1({argv:["/bin/true"],env:{},cwd:"/r",root:"/r",network:false});
 expect(probeArguments).toEqual(real.slice(1));
 for(const flag of ["--unshare-net","--unshare-pid","--unshare-ipc","--die-with-parent","--new-session","--dev","--proc"])expect(probeArguments).toContain(flag);
 expect(probeArguments.slice(-2)).toEqual(["--","/bin/true"]);
});

test.skipIf(process.platform!=="darwin")("attack on macOS: the sandbox cannot read the credential directory of the machine or another user's files, but can still run system tools",()=>{
 const parent=mkdtempSync(join(testScratchRootV1("rexec-inhouse"),"sbx2-")),root=join(parent,"root");
 try{
  mkdirSync(root);
  const run=(script:string)=>Bun.spawnSync(["/usr/bin/sandbox-exec","-p",remoteExecutionDarwinProfileV1(realpathSync(root),false),"/bin/sh","-c",script],{cwd:root,stdout:"pipe",stderr:"pipe"});
  expect(run("/usr/bin/true && echo ok").stdout.toString()).toBe("ok\n");
  for(const hidden of["/private/etc/semio","/private/var/root","/Users/Shared","/private/var/db"]){
   const outside=Bun.spawnSync(["/bin/ls",hidden],{stdout:"pipe",stderr:"pipe"});
   if(outside.exitCode===0)expect(run(`/bin/ls ${hidden}`).exitCode,`${hidden} must be unreadable inside`).not.toBe(0);
  }
  if(Bun.spawnSync(["/bin/test","-r","/etc/semio/remote-cache.env"]).exitCode===0)expect(run("/bin/cat /etc/semio/remote-cache.env >/dev/null").exitCode,"the cache token file").not.toBe(0);
 }finally{rmSync(parent,{recursive:true,force:true});}
});
