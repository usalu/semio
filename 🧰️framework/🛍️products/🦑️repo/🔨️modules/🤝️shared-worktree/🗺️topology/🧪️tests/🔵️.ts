import {expect,test} from "bun:test";
import {chmodSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,resolve} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {runTopologyCommandV1} from "../⌨️command/🟦️.ts";
import {readTopologyV1,serverPartsV1,workerPartsV1,mountPartsV1} from "../🟦️.ts";

const pwsh=Bun.which("pwsh"),here=import.meta.dirname,topologyDirectory=resolve(here,".."),repository=resolve(here,"../../../../../../.."),modules=resolve(here,"../../..");
const named=(directory:string,suffix:string):string=>join(directory,readdirSync(directory).find(name=>name.endsWith(suffix))!);
const provisionDirectory=named(topologyDirectory,"provision"),script=named(provisionDirectory,".ps1"),harness=named(here,".ps1");
const moduleDirectory=resolve(here,"../.."),native=named(named(named(modules,"native"),"bootstrap"),".ps1"),topologyScript=named(topologyDirectory,"script.ts"),replicaScript=named(named(moduleDirectory,"replica"),"script.ts");
const checkoutFiles=[native,topologyScript,replicaScript].map(file=>relative(repository,file));
const smb=structuredClone(fixture.topology) as any;
smb.groups[1].server={platform:"linux",protocol:"nfs",network:"10.2.0.0/16"};
smb.groups[1].nfs.export="/srv/store-b";
smb.groups[2].server.protocol="smb";
const TAR=`#!/bin/sh\nn=$#\nwhile [ $n -gt 0 ]; do a=$1; shift; case "$a" in [A-Za-z]:*) a="$HARNESS_ROOT/\${a%%:*}/$(printf %s "\${a#?:}" | tr '\\\\' /)";; esac; set -- "$@" "$a"; n=$((n-1)); done\nexec tar "$@"\n`;
function plansOf(topology:unknown,group:string):Record<string,string>{
 const plans:Record<string,string>={},context={platform:"win32",topology},add=(...args:string[]):void=>{try{plans[args.join(" ")]=runTopologyCommandV1(args,context);}catch{}};
 add("facts","--group",group,"--platform","windows","--shell","powershell");
 add("env","--group",group,"--shell","powershell");
 for(const part of[undefined,"target","compact-target"])add("worker-service","--group",group,"--platform","windows",...part?["--part",part]:[]);
 for(const role of["worker","server"])add("worker-service","--group",group,"--platform","windows","--part","compact-definition","--role",role);
 try{for(const part of Object.keys(serverPartsV1(readTopologyV1(topology),group)))add("server-plan","--group",group,"--part",part);}catch{}
 return plans;
}
const hex=(bytes:ArrayBuffer):string=>[...new Uint8Array(bytes)].map(byte=>byte.toString(16).padStart(2,"0")).join("");

type Setup=Readonly<{group:string;origin?:string;flags?:Record<string,unknown>;topology?:unknown;checkout?:boolean;archive?:boolean;peerArchive?:boolean;corrupt?:"sha"|"count";mounted?:boolean;identity?:{originId:string;groupId:string};tokens?:string;running?:string[];users?:string[];registry?:Record<string,number>;keys?:string[];features?:Record<string,boolean>;windowsClient?:boolean;admin?:boolean;exit?:Record<string,number>;strayFile?:boolean;origins?:boolean;incomplete?:boolean;noWinget?:boolean;noTimeService?:boolean;timeStart?:string;timeStatus?:string;smbClient?:Record<string,number>;nativeOutput?:string}>;
type Outcome=Readonly<{error:string|null;calls:string[];tasks:Record<string,{Xml:string|null;User:string;Exe:string|null;Arguments:string|null}>;started:string[];registry:Record<string,number>;machine:Record<string,string>;rules:string[];nfsShares:string[];smbShares:string[];features:Record<string,boolean>;bootstrapped:boolean;launcher:boolean;user:Record<string,string>;exitCode:number;marker:boolean;out:string;root:string}>;

function provision(setup:Setup):Outcome{
 const root=mkdtempSync(join(tmpdir(),"semio-ps-")),layout=join(root,"module",topologyDirectory.split("/").at(-1)!),topologyFile=join(layout,readdirSync(topologyDirectory).find(name=>name.endsWith(".json"))!),copy=join(layout,provisionDirectory.split("/").at(-1)!,script.split("/").at(-1)!);
 mkdirSync(join(copy,".."),{recursive:true});
 cpSync(script,copy);
 writeFileSync(topologyFile,JSON.stringify(setup.topology??fixture.topology));
 for(const directory of["C","store","tmp","Windows/System32","peer/v1/bootstrap","store/v1/bootstrap"])mkdirSync(join(root,directory),{recursive:true});
 writeFileSync(join(root,"Windows/System32/tar.exe"),TAR);
 chmodSync(join(root,"Windows/System32/tar.exe"),0o755);
 const worktree=join(root,"C/semio");
 mkdirSync(worktree,{recursive:true});
 if(setup.checkout)for(const file of checkoutFiles){mkdirSync(join(worktree,file,".."),{recursive:true});writeFileSync(join(worktree,file),"");}
 if(setup.checkout&&!setup.incomplete){mkdirSync(join(worktree,".cache/f"),{recursive:true});writeFileSync(join(worktree,".cache/f/native-bootstrap.done"),"2026-11-10T00:00:00Z\n");}
 if(setup.strayFile)writeFileSync(join(worktree,"stray.txt"),"x");
 if(setup.identity){mkdirSync(join(worktree,".cache/f"),{recursive:true});writeFileSync(join(worktree,".cache/f/machine.json"),JSON.stringify(setup.identity));}
 if(setup.origins)mkdirSync(join(root,"store/v1/origins/someone"),{recursive:true});
 if(setup.tokens!==undefined){mkdirSync(join(root,"C/ProgramData/semio"),{recursive:true});writeFileSync(join(root,"C/ProgramData/semio/remote-cache.env"),setup.tokens);}
 for(const directory of[setup.archive?"store":undefined,setup.peerArchive?"peer":undefined]){
  if(directory===undefined)continue;
  const tar=join(root,directory,"v1/bootstrap/worktree.tar"),made=Bun.spawnSync(["tar","-cf",tar,"-C",repository,...checkoutFiles]),listing=Bun.spawnSync(["tar","-tf",tar]).stdout.toString().trim().split("\n").length;
  expect(made.exitCode).toBe(0);
  writeFileSync(join(root,directory,"v1/bootstrap/manifest.json"),JSON.stringify({files:listing+(setup.corrupt==="count"?1:0),sha256:setup.corrupt==="sha"?"0".repeat(64):hex(new Bun.CryptoHasher("sha256").update(readFileSync(tar)).digest().buffer as ArrayBuffer)}));
 }
 const arguments_:Record<string,unknown>={GroupId:setup.group,OriginId:setup.origin??`${setup.group}-m01`,...setup.flags};
 if(arguments_["ArchivePath"]==="peer")arguments_["ArchivePath"]=join(root,"peer/v1/bootstrap");
 const caseFile=join(root,"case.json");
 writeFileSync(caseFile,JSON.stringify({root,script:copy,arguments:arguments_,plans:plansOf(setup.topology??fixture.topology,setup.group),admin:setup.admin??true,mounted:setup.mounted??false,repo:setup.checkout??false,registry:setup.registry??{},keys:setup.keys??[],features:setup.features??{},users:setup.users??[],running:setup.running??[],windowsClient:setup.windowsClient??false,exit:setup.exit??{},noWinget:setup.noWinget??false,noTimeService:setup.noTimeService??false,timeStart:setup.timeStart,timeStatus:setup.timeStatus,smbClient:setup.smbClient,nativeOutput:setup.nativeOutput}));
 const run=Bun.spawnSync([pwsh!,"-NoProfile","-File",harness,caseFile],{env:{...process.env,PATH:`${process.env["PATH"]}`}}),out=run.stdout.toString(),marker=out.indexOf("@@RESULT@@ ");
 if(marker<0)throw Error(`harness failed\n${out}\n${run.stderr.toString()}`);
 const result=JSON.parse(out.slice(marker+11)),leaves=new Map([[topologyDirectory.split("/").at(-1)!,"topology"],[replicaScript.split("/").at(-2)!,"replica"]]);
 return {...result,marker:existsSync(join(root,"C/semio/.cache/f/native-bootstrap.done")),calls:result.calls.map((call:string)=>call.replace(/^repo \[[^\]]*\/([^\]/]+)\]/u,(_,leaf:string)=>`repo ${leaves.get(leaf)}`)),out:out.slice(0,marker),root};
}
const forget=(outcome:Outcome):void=>rmSync(outcome.root,{recursive:true,force:true});
const run=(name:string,body:()=>void):void=>{(pwsh?test:test.skip)(name,body,120000);};
const user="DOM\\op",drive="S:";

run("the provisioning script and every Windows snippet of the plans parse with the PowerShell parser",()=>{
 const snippets:Record<string,string>={};
 for(const [name,topology] of[["nfs",fixture.topology],["smb",smb]] as const){
  const read=readTopologyV1(topology);
  for(const id of["a1","b2","c3"]){
   for(const [part,text] of Object.entries(mountPartsV1(read,id,"windows")))snippets[`${name}.${id}.mount.${part}`]=text;
  }
  for(const [part,text] of Object.entries(serverPartsV1(read,"c3")).filter(([part])=>["prepare","provision","start","feature","share","firewall"].includes(part)))snippets[`${name}.c3.server.${part}`]=text;
 }
 const root=mkdtempSync(join(tmpdir(),"semio-parse-")),file=join(root,"snippets.json"),parser=join(root,"parse.ps1");
 writeFileSync(file,JSON.stringify(Object.entries(snippets).map(([name,text])=>({name,text})).concat([{name:"script",text:readFileSync(script,"utf8")}])));
 writeFileSync(parser,"$items = Get-Content -Raw -LiteralPath $args[0] | ConvertFrom-Json\nforeach ($item in $items) { $e = $null; $t = $null; [void][Management.Automation.Language.Parser]::ParseInput($item.text, [ref]$t, [ref]$e); foreach ($error in $e) { \"$($item.name): $($error.Message)\" } }\n\"parsed $($items.Count)\"\n");
 const parsed=Bun.spawnSync([pwsh!,"-NoProfile","-File",parser,file]).stdout.toString().trim();
 rmSync(root,{recursive:true,force:true});
 expect(parsed).toMatch(/^parsed \d{2,}$/u);
 expect(Object.keys(snippets).length).toBeGreaterThan(25);
});

run("the scheduled task XML follows the element order and the limits of the published Task Scheduler schema",()=>{
 const documents=Object.values(snippets()).filter(text=>text.startsWith("<?xml"));
 expect(documents.length).toBeGreaterThan(12);
 const schemaFile=process.env["SEMIO_TASK_SCHEMA"],xmllint=Bun.which("xmllint");
 for(const xml of documents){
  expect(xml).toContain('<Task version="1.3" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">');
  expect(Number(xml.match(/<Count>(\d+)<\/Count>/u)?.[1]??1)).toBeLessThanOrEqual(255);
  if(schemaFile&&xmllint)expect(Bun.spawnSync([xmllint,"--noout","--schema",schemaFile,"-"],{stdin:new TextEncoder().encode(xml.split("\n").slice(1).join("\n"))}).stderr.toString()).toContain("validates");
  for(const trigger of xml.match(/<CalendarTrigger>.*?<\/CalendarTrigger>/gu)??[])expect(trigger.indexOf("<Enabled>")).toBeLessThan(trigger.indexOf("<StartBoundary>"));
 }
});
function snippets():Record<string,string>{
 const all:Record<string,string>={};
 for(const [name,topology] of[["nfs",fixture.topology],["smb",smb]] as const){
  const read=readTopologyV1(topology);
  for(const id of["a1","b2","c3"])for(const [part,text] of Object.entries(workerPartsV1(read,id,"windows")))all[`${name}.${id}.worker.${part}`]=text;
  for(const [part,text] of Object.entries(serverPartsV1(read,"c3")))all[`${name}.c3.server.${part}`]=text;
 }
 return all;
}

const TOKENS=`${serverPartsV1(readTopologyV1(fixture.topology),"c3")["cache-environment"]!.replace(/<[a-z-]+>/gu,"secret")}\r\n`;
const fresh=(outcome:Outcome,...forbidden:RegExp[]):Outcome=>{for(const pattern of forbidden)expect(outcome.calls.filter(call=>pattern.test(call)),String(pattern)).toEqual([]);return outcome;};
const inOrder=(outcome:Outcome,...starts:string[]):void=>{
 let last=-1;
 for(const start of starts){const index=outcome.calls.findIndex((call,position)=>position>last&&call.startsWith(start));expect(index,`${start} after ${last}\n${outcome.calls.join("\n")}`).toBeGreaterThan(last);last=index;}
};
const task=(read:ReturnType<typeof readTopologyV1>,id:string,part:string,server=false):string=>workerPartsV1(read,id,"windows",server)[part]!;
const refused=(setup:Setup,message:string,calls:string[]=[]):Outcome=>{const outcome=provision(setup);expect(outcome.error).toContain(message);expect(outcome.calls).toEqual(calls);return outcome;};
const read=readTopologyV1(fixture.topology),readSmb=readTopologyV1(smb);
const UNC_A="\\\\nfs.a1.example.test\\srv\\store-a";
const NFS_KEY="HKLM:\\SOFTWARE\\Microsoft\\ClientForNFS\\CurrentVersion\\Users\\Default\\Cache",NFS_VALUES=["RemoteWriteCache","FileAttributeCache"],MARKER="C:\\semio\\.cache\\f\\native-bootstrap.done";
const NFS_CACHE=[`New-Item -Path ${NFS_KEY}`,...NFS_VALUES.map(name=>`New-ItemProperty -Force True -Name ${name} -Path ${NFS_KEY} -PropertyType DWord -Value 0`),"nfsadmin client stop","nfsadmin client start"],SMB_CACHE="Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0 -Force True";
const CACHED={registry:{LongPathsEnabled:1,RemoteWriteCache:0,FileAttributeCache:0},keys:[NFS_KEY]};
const WORKER=(group:string,origin:string,cacheLines:string[],mountLines:string[],tail:string[])=>[
 "New-ItemProperty -Force True -Name LongPathsEnabled -Path HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem -PropertyType DWord -Value 1",
 `icacls C:\\semio /grant ${user}:(OI)(CI)M`,...cacheLines,...mountLines,
 `repo topology facts --group ${group} --platform windows --shell powershell`,
 ...tail,
 `repo topology env --group ${group} --shell powershell`,
 `repo topology worker-service --group ${group} --platform windows --part target`,
 `repo topology worker-service --group ${group} --platform windows`,
 `Register-ScheduledTask -Force True -TaskName semio-fleet-replica -User ${user} -Xml`,
 `repo topology worker-service --group ${group} --platform windows --part compact-target`,
 `repo topology worker-service --group ${group} --platform windows --part compact-definition --role worker`,
 `Register-ScheduledTask -Force True -TaskName semio-fleet-compact -User ${user} -Xml`
];

run("a first machine of a Linux-served group sets long paths, mounts over Client for NFS, registers the tasks, seeds and starts the replica",()=>{
 const outcome=provision({group:"a1",flags:{FirstMachine:true},checkout:true});
 expect(outcome.error).toBeNull();
 const mount=`mount.exe -o anon,nolock,mtype=hard ${UNC_A} ${drive}`;
 expect(outcome.calls).toEqual([
  ...WORKER("a1","a1-m01",NFS_CACHE,[
   "Enable-WindowsOptionalFeature -All True -FeatureName ServicesForNFS-ClientOnly -NoRestart True -Online True",
   "Enable-WindowsOptionalFeature -All True -FeatureName ClientForNFS-Infrastructure -NoRestart True -Online True",mount
  ],[`Register-ScheduledTask -Force True -TaskName semio-fleet-store-mount -User ${user} -Action ${mount} -Trigger AtLogOn=True User=${user}`]),
  "repo replica seed --group a1 --origin a1-m01 --store S:\\",
  "Start-ScheduledTask semio-fleet-store-mount","Start-ScheduledTask semio-fleet-replica"
 ]);
 expect(outcome.registry).toEqual({LongPathsEnabled:1,RemoteWriteCache:0,FileAttributeCache:0});
 expect(outcome.features).toEqual({"ServicesForNFS-ClientOnly":true,"ClientForNFS-Infrastructure":true});
 expect({...outcome.machine,Path:undefined}).toEqual({...(fixture.environments as Record<string,Record<string,string>>)["a1"],Path:undefined});
 expect(outcome.tasks["semio-fleet-replica"]).toMatchObject({Xml:task(read,"a1","definition"),User:user});
 expect(outcome.tasks["semio-fleet-compact"]!.Xml).toBe(task(read,"a1","compact-definition"));
 expect(outcome.tasks["semio-fleet-compact"]!.Xml).not.toContain("--collect-blobs");
 expect(outcome.tasks["semio-fleet-store-mount"]).toMatchObject({Exe:"mount.exe",Arguments:`-o anon,nolock,mtype=hard ${UNC_A} ${drive}`});
 expect(outcome.started).toEqual(["semio-fleet-store-mount","semio-fleet-replica"]);
 expect(outcome.out).toContain("done: a1-m01 is a member of group a1");
 forget(outcome);
});

run("a joining worker extracts the verified archive with the system tar, builds the standalone launcher, settles and joins and starts the tasks, and only then bootstraps the tools",()=>{
 const outcome=provision({group:"a1",archive:true});
 expect(outcome.error).toBeNull();
 expect(outcome.exitCode).toBe(0);
 expect(outcome.launcher).toBe(true);
 expect(outcome.bootstrapped).toBe(true);
 expect(outcome.user["Path"]).toBe("C:\\Users\\op\\AppData\\Local\\semio\\launcher\\bin");
 expect(readFileSync(join(outcome.root,"C/semio",checkoutFiles[0]!),"utf8")).toBe(readFileSync(join(repository,checkoutFiles[0]!),"utf8"));
 inOrder(outcome,"mount.exe","powershell.exe -NoProfile -ExecutionPolicy Bypass -File","repo topology facts","Register-ScheduledTask -Force True -TaskName semio-fleet-compact","repo replica bootstrap-settle --store S:\\","repo replica join --group a1 --origin a1-m01 --store S:\\","Start-ScheduledTask semio-fleet-store-mount","Start-ScheduledTask semio-fleet-replica","powershell.exe -NoProfile -ExecutionPolicy Bypass -File","powershell.exe(env) SEMIO_WORKSPACE_ROOT=C:\\semio");
 expect(outcome.calls.filter(call=>call.startsWith("powershell.exe -NoProfile")).map(call=>/launcher/u.test(call)?"launcher":"native")).toEqual(["launcher","native"]);
 expect(outcome.marker).toBe(true);
 expect(outcome.out).toContain("archive verified: sha256");
 fresh(outcome,/^Start-Sleep/u,/ seed /u);
 forget(outcome);
});

run("re-provisioning the same identity skips seed, join and settle, leaves unchanged settings alone and registers the tasks again",()=>{
 for(const flags of[{},{FirstMachine:true}]){
  const outcome=provision({group:"a1",flags,checkout:true,mounted:true,identity:{originId:"a1-m01",groupId:"a1"},...CACHED,features:{"ServicesForNFS-ClientOnly":true,"ClientForNFS-Infrastructure":true}});
  expect(outcome.error).toBeNull();
  fresh(outcome,/^New-ItemProperty/u,/^Enable-WindowsOptionalFeature/u,/^mount\.exe/u,/ (seed|join|bootstrap-settle) /u);
  expect(outcome.calls.filter(call=>call.startsWith("Register-ScheduledTask")).map(call=>call.split(" ")[4])).toEqual(["semio-fleet-store-mount","semio-fleet-replica","semio-fleet-compact"]);
  for(const text of["unchanged LongPathsEnabled","unchanged RemoteWriteCache","unchanged FileAttributeCache","unchanged feature ServicesForNFS-ClientOnly","already mounted S:","already joined as a1-m01 in group a1"])expect(outcome.out).toContain(text);
  expect(outcome.started).toEqual(["semio-fleet-store-mount","semio-fleet-replica"]);
  forget(outcome);
 }
 const running=provision({group:"a1",checkout:true,mounted:true,identity:{originId:"a1-m01",groupId:"a1"},running:["semio-fleet-replica"]});
 expect(running.started).toEqual(["semio-fleet-store-mount"]);
 expect(running.out).toContain("already running semio-fleet-replica");
 forget(running);
});

run("re-provisioning a different origin or group stops before anything changes",()=>{
 forget(refused({group:"a1",checkout:true,identity:{originId:"a1-m02",groupId:"a1"}},"belongs to origin a1-m02, not a1-m01: pass that origin"));
 forget(refused({group:"a1",checkout:true,identity:{originId:"a1-m01",groupId:"b2"}},"belongs to group b2, not a1: pass that group"));
 forget(refused({group:"a1",checkout:true,identity:{originId:"",groupId:"a1"}},"has no originId and groupId"));
});

run("a worker of an SMB group mounts with net use, registers a net use logon task and tells how to store the credentials when the mount fails",()=>{
 const outcome=provision({group:"c3",topology:smb,archive:true});
 expect(outcome.error).toBeNull();
 const remote="\\\\nfs.c3.example.test\\store-c";
 expect(outcome.calls).toContain(`net.exe use ${drive} ${remote} /persistent:yes`);
 expect(outcome.tasks["semio-fleet-store-mount"]).toMatchObject({Exe:"net.exe",Arguments:`use ${drive} ${remote} /persistent:yes`});
 fresh(outcome,/^Enable-WindowsOptionalFeature/u,/^mount\.exe/u);
 forget(outcome);
 const failed=provision({group:"c3",topology:smb,archive:true,exit:{"net.exe":2}});
 expect(failed.error).toContain("net use failed with exit code 2");
 expect(failed.error).toContain("cmdkey /add:nfs.c3.example.test /user:<server name>\\semio-fleet");
 fresh(failed,/^Register-ScheduledTask/u,/^repo/u);
 forget(failed);
});

run("the Windows NFS group server applies the share, the per-peer read-only grants, the firewall rules and the tasks in the promised order",()=>{
 const outcome=provision({group:"c3",flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS});
 expect(outcome.error).toBeNull();
 const secret="C:\\ProgramData\\semio\\remote-cache.env",ports="2049,111,1048";
 inOrder(outcome,`icacls C:\\semio /grant ${user}:(OI)(CI)M`,`icacls C:\\store-c /grant ${user}:(OI)(CI)M`,"powershell.exe","repo topology facts --group c3","Register-ScheduledTask -Force True -TaskName semio-fleet-replica","Register-ScheduledTask -Force True -TaskName semio-fleet-compact","repo topology server-plan --group c3 --part cache-environment-target",`icacls ${secret} /inheritance:r /grant:r ${user}:(R)`,`icacls C:\\semio-remote-cache /grant ${user}:(OI)(CI)M`,"repo topology server-plan --group c3 --part feature","Install-WindowsFeature -IncludeManagementTools True -Name FS-NFS-Service","Install-WindowsFeature -Name NFS-Client","New-NfsShare","Grant-NfsSharePermission","Grant-NfsSharePermission","New-NetFirewallRule","Register-ScheduledTask -Force True -TaskName semio-fleet-cache","Register-ScheduledTask -Force True -TaskName semio-fleet-relay","repo replica relay --once","repo replica bootstrap-settle --store C:\\store-c","repo replica join --group c3 --origin c3-m01 --store C:\\store-c","Start-ScheduledTask semio-fleet-replica","Start-ScheduledTask semio-fleet-cache","Start-ScheduledTask semio-fleet-relay");
 expect(outcome.calls).toContain("New-NfsShare -AllowRootAccess True -AnonymousGid 1500 -AnonymousUid 1500 -Authentication sys -EnableAnonymousAccess True -EnableUnmappedAccess True -Name store-c -Path C:\\store-c -Permission readwrite");
 for(const host of["a1","b2"])expect(outcome.calls).toContain(`Grant-NfsSharePermission -AllowRootAccess False -ClientName nfs.${host}.example.test -ClientType host -Name store-c -Permission readonly`);
 expect(outcome.calls).toContain(`New-NetFirewallRule -Action Allow -Direction Inbound -DisplayName semio-fleet-store-workers -LocalPort ${ports} -Protocol TCP -RemoteAddress 10.3.0.0/24`);
 expect(outcome.calls).toContain(`New-NetFirewallRule -Action Allow -Direction Inbound -DisplayName semio-fleet-store-a1 -LocalPort ${ports} -Protocol TCP -RemoteAddress 192.0.2.19`);
 expect(outcome.calls).toContain("New-NetFirewallRule -Action Allow -Direction Inbound -DisplayName semio-fleet-cache -LocalPort 443 -Protocol TCP -RemoteAddress 10.3.0.0/24");
 expect(outcome.calls.find(call=>call.startsWith("repo replica relay"))).toBe(`repo replica relay --once --group c3 --store C:\\store-c --peers a1=${UNC_A},b2=\\\\nfs.b2.example.test\\Users\\Shared\\store-b`);
 expect(outcome.nfsShares).toEqual(["store-c"]);
 expect(outcome.rules.sort()).toEqual(["semio-fleet-cache","semio-fleet-store-a1","semio-fleet-store-b2","semio-fleet-store-workers"]);
 expect(outcome.started).toEqual(["semio-fleet-replica","semio-fleet-cache","semio-fleet-relay"]);
 expect(outcome.tasks["semio-fleet-compact"]!.Xml).toBe(task(read,"c3","compact-definition",true));
 expect(outcome.tasks["semio-fleet-compact"]!.Xml).toContain("--collect-blobs");
 expect(outcome.tasks["semio-fleet-cache"]!.Xml).toBe(serverPartsV1(read,"c3")["cache-unit"]!);
 expect(outcome.tasks["semio-fleet-relay"]!.Xml).toBe(serverPartsV1(read,"c3")["relay-unit"]!);
 fresh(outcome,/^mount\.exe/u,/^net\.exe/u,/^Register-ScheduledTask.*store-mount/u);
 expect(outcome.out).toContain("group server: the store is the local folder C:\\store-c");
 forget(outcome);
});

run("the Windows SMB group server needs its two local accounts, shares with Change and Read access and never calls the NFS cmdlets",()=>{
 const outcome=provision({group:"c3",topology:smb,flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS,users:["semio-fleet","semio-fleet-peer"]});
 expect(outcome.error).toBeNull();
 inOrder(outcome,"Set-SmbServerConfiguration -EnableSMB2Protocol True -Force True","Install-WindowsFeature -Name NFS-Client","New-SmbShare -ChangeAccess semio-fleet -Name store-c -Path C:\\store-c -ReadAccess semio-fleet-peer","icacls C:\\store-c /grant semio-fleet:(OI)(CI)M semio-fleet-peer:(OI)(CI)RX","New-NetFirewallRule");
 expect(outcome.calls).toContain("New-NetFirewallRule -Action Allow -Direction Inbound -DisplayName semio-fleet-store-workers -LocalPort 445 -Protocol TCP -RemoteAddress 10.3.0.0/24");
 fresh(outcome,/Nfs/u,/FS-NFS-Service/u);
 expect(outcome.smbShares).toEqual(["store-c"]);
 forget(outcome);
 const client=provision({group:"c3",topology:smb,flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS,users:["semio-fleet","semio-fleet-peer"],windowsClient:true});
 expect(client.error).toBeNull();
 expect(client.calls).toContain("Enable-WindowsOptionalFeature -All True -FeatureName ServicesForNFS-ClientOnly,ClientForNFS-Infrastructure -NoRestart True -Online True");
 fresh(client,/^Install-WindowsFeature/u);
 forget(client);
 const missing=provision({group:"c3",topology:smb,flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS,users:["semio-fleet"]});
 expect(missing.error).toContain("The local account semio-fleet-peer does not exist");
 fresh(missing,/^New-SmbShare/u);
 forget(missing);
});

run("the server refuses a platform that differs from the topology, a missing archive, a missing or placeholder tokens file and Windows 10/11 for NFS",()=>{
 forget(refused({group:"a1",flags:{Server:true},checkout:true},"puts the server of a1 on linux, this machine is windows"));
 forget(refused({group:"b2",flags:{Server:true},checkout:true},"puts the server of b2 on macos"));
 const server={group:"c3",flags:{Server:true,ArchivePath:"peer"},peerArchive:true};
 const noArchive=provision({group:"c3",flags:{Server:true}});
 expect(noArchive.error).toContain("A group server takes the archive of a peer group: pass -ArchivePath");
 fresh(noArchive,/^New-NfsShare/u);
 forget(noArchive);
 const noTokens=provision(server);
 expect(noTokens.error).toContain("C:\\ProgramData\\semio\\remote-cache.env does not exist");
 expect(noTokens.nfsShares).toEqual([]);
 forget(noTokens);
 const placeholder=provision({...server,tokens:"SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN=<read-token>\nSEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN=x\n"});
 expect(placeholder.error).toContain("has no real value for SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN");
 forget(placeholder);
 const client=provision({...server,tokens:TOKENS,windowsClient:true});
 expect(client.error).toContain("Server for NFS needs Windows Server");
 expect(client.nfsShares).toEqual([]);
 forget(client);
});

run("the checks before the first change: elevation, the account, the group, the worktree and the first-machine rules",()=>{
 forget(refused({group:"a1",admin:false},"Run this script from an elevated PowerShell"));
 forget(refused({group:"a1",flags:{User:"DOM\\other"}},"-User must be the account of this elevated shell (DOM\\op)"));
 forget(refused({group:"zz"},"The topology has no worktree for windows or no group zz"));
 const stray=provision({group:"a1",strayFile:true,mounted:true,registry:{LongPathsEnabled:1},features:{"ServicesForNFS-ClientOnly":true,"ClientForNFS-Infrastructure":true}});
 expect(stray.error).toContain("neither empty nor a checkout");
 fresh(stray,/^repo/u,/^Register/u);
 forget(stray);
 expect(provision({group:"a1",flags:{FirstMachine:true}}).error).toContain("-FirstMachine needs a complete checkout");
 expect(provision({group:"a1",flags:{FirstMachine:true},checkout:true,origins:true}).error).toContain("The store already has origins");
 expect(provision({group:"a1"}).error).toContain("No bootstrap archive in S:\\v1\\bootstrap");
});

run("a bootstrap archive that never matches its manifest is retried five times and then refused; a wrong entry count is refused",()=>{
 const sha=provision({group:"a1",archive:true,corrupt:"sha"});
 expect(sha.error).toContain("never matched its manifest.json");
 expect(sha.calls.filter(call=>call==="Start-Sleep 20").length).toBe(5);
 fresh(sha,/^powershell\.exe/u);
 forget(sha);
 const count=provision({group:"a1",archive:true,corrupt:"count"});
 expect(count.error).toMatch(/The archive lists \d+ entries, manifest\.json says \d+/u);
 fresh(count,/^powershell\.exe/u);
 forget(count);
});

run("a failing icacls, mount.exe or repo command stops the script with the exit code",()=>{
 forget(refused({group:"a1",exit:{icacls:5}},"icacls failed for C:\\semio",["New-ItemProperty -Force True -Name LongPathsEnabled -Path HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem -PropertyType DWord -Value 1","icacls C:\\semio /grant DOM\\op:(OI)(CI)M"]));
 const mount=provision({group:"a1",archive:true,exit:{"mount.exe":1}});
 expect(mount.error).toContain("mount.exe failed with exit code 1");
 fresh(mount,/^Register-ScheduledTask/u);
 forget(mount);
 const seed=provision({group:"a1",flags:{FirstMachine:true},checkout:true,exit:{"repo:seed":3}});
 expect(seed.error).toContain("seed --group a1 --origin a1-m01 --store S:\\ failed with exit code 3");
 fresh(seed,/^Start-ScheduledTask/u);
 forget(seed);
});

run("the store client caches are set before the mount: SMB lifetimes for an SMB group, the NFS client values for an NFS group, and an unchanged setting is left alone",()=>{
 const smbOutcome=provision({group:"c3",topology:smb,archive:true});
 expect(smbOutcome.error).toBeNull();
 inOrder(smbOutcome,SMB_CACHE,`net.exe use ${drive}`);
 fresh(smbOutcome,/^(New-ItemProperty -Force True -Name (RemoteWriteCache|FileAttributeCache)|nfsadmin)/u);
 expect(smbOutcome.out).toContain("set the SMB client cache lifetimes");
 forget(smbOutcome);
 const same=provision({group:"c3",topology:smb,archive:true,smbClient:{DirectoryCacheLifetime:0,FileInfoCacheLifetime:0,FileNotFoundCacheLifetime:0}});
 fresh(same,/^Set-SmbClientConfiguration/u);
 expect(same.out).toContain("unchanged SMB client cache lifetimes");
 forget(same);
 const nfsOutcome=provision({group:"a1",archive:true});
 inOrder(nfsOutcome,...NFS_CACHE,"mount.exe");
 fresh(nfsOutcome,/^Set-SmbClientConfiguration/u);
 forget(nfsOutcome);
 expect(mountPartsV1(read,"a1","windows")["cache"]).toBe([`New-Item -Path ${NFS_KEY} -Force | Out-Null`,...NFS_VALUES.map(name=>`New-ItemProperty -Path ${NFS_KEY} -Name ${name} -Value 0 -PropertyType DWord -Force | Out-Null`),"nfsadmin client stop","nfsadmin client start"].join("\n"));
 expect(mountPartsV1(readSmb,"c3","windows")["cache"]).toBe(SMB_CACHE.replace(/ True$/u,""));
});

run("a Windows server sets the client caches that its peers need, SMB and NFS together when both kinds are among them",()=>{
 const both=structuredClone(smb);
 both.groups[0].server={platform:"windows",protocol:"nfs",network:"10.1.0.0/24",directory:"C:\\store-a"};
 both.groups[0].nfs.export="/store-a";
 const outcome=provision({group:"a1",topology:both,flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS});
 expect(outcome.error).toBeNull();
 expect(outcome.calls).toContain(SMB_CACHE);
 for(const call of NFS_CACHE)expect(outcome.calls).toContain(call);
 forget(outcome);
 const nfsOnly=provision({group:"c3",flags:{Server:true,ArchivePath:"peer"},peerArchive:true,tokens:TOKENS});
 expect(nfsOnly.error).toBeNull();
 fresh(nfsOnly,/^Set-SmbClientConfiguration/u);
 expect(nfsOnly.calls).toContain(NFS_CACHE[0]!);
 forget(nfsOnly);
});

run("the native bootstrap runs last and is repeated until it succeeded once: a failure leaves the fleet setup standing, writes no marker, names the command and the log and exits with 3",()=>{
 const base={group:"a1",checkout:true,mounted:true,identity:{originId:"a1-m01",groupId:"a1"},...CACHED,features:{"ServicesForNFS-ClientOnly":true,"ClientForNFS-Infrastructure":true}};
 const complete=provision(base);
 fresh(complete,/^powershell\.exe/u);
 expect(complete.out.replaceAll("/","\\")).toContain(`native bootstrap complete (${MARKER})`);
 expect(complete.marker).toBe(true);
 forget(complete);
 const redone=provision({...base,incomplete:true});
 expect(redone.error).toBeNull();
 expect(redone.calls.filter(call=>call.startsWith("powershell.exe -NoProfile")).length).toBe(1);
 expect(redone.out.replaceAll("/","\\")).toContain(`recorded the completed bootstrap in ${MARKER}`);
 expect(redone.marker).toBe(true);
 forget(redone);
 const failed=provision({...base,incomplete:true,exit:{"powershell.exe:native":1},nativeOutput:"[semio] workspace:setup...\nCannot find module 'typescript'"});
 expect(failed.error).toBeNull();
 expect(failed.exitCode).toBe(3);
 expect(failed.marker).toBe(false);
 expect(failed.bootstrapped).toBe(false);
 inOrder(failed,"Register-ScheduledTask -Force True -TaskName semio-fleet-replica","Start-ScheduledTask semio-fleet-replica","powershell.exe -NoProfile -ExecutionPolicy Bypass -File");
 const report=failed.out.replaceAll("/","\\");
 expect(report).toContain("fleet ready: a1-m01 is a member of group a1 and syncs");
 expect(report).toContain("NATIVE BUILD PENDING: a1-m01 is a member of group a1 and syncs, but its native build is not ready");
 expect(report).toMatch(/failing command: powershell\.exe -NoProfile -ExecutionPolicy Bypass -File \S+ setup/u);
 expect(report).toContain("reason: it exited with code 1");
 expect(report).toContain("C:\\semio\\.cache\\f\\native-bootstrap.log");
 expect(report).toContain("Cannot find module 'typescript'");
 expect(report).toContain(`the completion marker ${MARKER} was not written`);
 expect(report).toContain("exit code 3: fleet ready, native build pending");
 expect(report).not.toContain("its native build is complete");
 expect(readFileSync(join(failed.root,"C/semio/.cache/f/native-bootstrap.log"),"utf8")).toContain("Cannot find module 'typescript'");
 forget(failed);
});

run("a launcher that cannot be built stops the script before any topology command: the machine cannot join without it",()=>{
 const outcome=provision({group:"a1",archive:true,exit:{"powershell.exe:launcher":1}});
 expect(outcome.error).toContain("The repo launcher could not be built (exit code 1)");
 expect(outcome.exitCode).toBe(1);
 fresh(outcome,/^repo/u,/^Register-ScheduledTask/u,/^powershell\.exe\(env\)/u);
 forget(outcome);
});

run("missing tools make the native build pending, never the fleet setup fail: no winget leaves a machine that needs the bootstrap joined and syncing, and does not matter to one that finished it",()=>{
 const blocked=provision({group:"a1",archive:true,noWinget:true});
 expect(blocked.error).toBeNull();
 expect(blocked.exitCode).toBe(3);
 expect(blocked.launcher).toBe(true);
 expect(blocked.marker).toBe(false);
 expect(blocked.out).toContain("winget is missing");
 expect(blocked.out).toContain("NATIVE BUILD PENDING");
 inOrder(blocked,"repo replica join","Start-ScheduledTask semio-fleet-replica");
 fresh(blocked,/^powershell\.exe\(env\)/u);
 forget(blocked);
 const done=provision({group:"a1",checkout:true,noWinget:true});
 expect(done.error).toBeNull();
 forget(done);
 const tools=provision({group:"a1",archive:true});
 expect(tools.out).toContain("the Visual Studio C++ Build Tools are missing");
 expect(tools.out).toContain("winget install --id Microsoft.VisualStudio.2022.BuildTools");
 expect(tools.out).toContain("Microsoft.VisualStudio.Workload.VCTools");
 forget(tools);
});

run("the Windows Time service is started and set to automatic when it is not, and its absence is only a warning",()=>{
 const idle=provision({group:"a1",checkout:true,mounted:true,timeStart:"Manual",timeStatus:"Stopped"});
 expect(idle.error).toBeNull();
 inOrder(idle,"Set-Service -Name w32time -StartupType Automatic","Start-Service -Name w32time");
 expect(idle.out).toContain("started w32time");
 forget(idle);
 const running=provision({group:"a1",checkout:true,mounted:true});
 fresh(running,/^Set-Service/u,/^Start-Service/u);
 expect(running.out).toContain("w32time is running");
 expect(running.out).toContain("time source: time.windows.com");
 forget(running);
 const missing=provision({group:"a1",checkout:true,mounted:true,noTimeService:true});
 expect(missing.error).toBeNull();
 expect(missing.out).toContain("WARNING: the Windows Time service (w32time) is missing");
 forget(missing);
});
