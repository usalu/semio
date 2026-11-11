import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {createServer} from "node:net";
import {join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import committed from "../🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {clientProtocolV1,factsV1,groupV1,machineEnvironmentV1,mountPartsV1,mountPlanV1,platformV1,readTopologyV1,serverPartsV1,serverPlanV1,workerPartsV1,workerServiceV1,type FleetGroupV1,type FleetPlatformV1,type FleetTopologyV1} from "../🟦️.ts";
import {checkReachabilityV1,parseTopologyArgumentsV1,preflightLanguagesV1,preflightTextV1,preflightV1,PREFLIGHT_LIMITS_V1,renderEnvironmentV1,renderFactsV1,runTopologyCommandAsyncV1,runTopologyCommandV1,type PreflightProbesV1,type ReachabilityPortsV1} from "../⌨️command/🟦️.ts";
import {parseReplicaArgumentsV1} from "../../🔁️replica/⌨️command/🟦️.ts";
import type {StatusReportV1} from "../../🔁️replica/🟦️.ts";
import {runTopologyScriptV1} from "../📜️script.ts";

type Change={name:string;path:(string|number)[];value?:unknown;remove?:boolean};

const ajv=new Ajv({strict:true,allowUnionTypes:true}),accepts=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Topology"}),python=testPythonV1(),oracle=join(import.meta.dirname,"🐍️.py");
const PLATFORMS:readonly FleetPlatformV1[]=["linux","macos","windows"],REPLICA="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica",SCRIPT="📜️script.ts";
const topology=readTopologyV1(fixture.topology),ids=fixture.topology.groups.map(group=>group.id);
const mutate=(change:Change):unknown=>{
 const copy=structuredClone(fixture.topology) as Record<string,any>;
 let node=copy;
 for(const key of change.path.slice(0,-1))node=node[key];
 const last=change.path.at(-1)!;
 if(change.remove)delete node[last];else node[last]=change.value;
 return copy;
};
const unc=(host:string,path:string)=>`\\\\${host}${path.replaceAll("/","\\")}`;
const variant=(change:(copy:any)=>void):FleetTopologyV1=>{
 const copy=structuredClone(fixture.topology) as any;
 change(copy);
 return readTopologyV1(copy);
};
const smbFleet=variant(copy=>{copy.groups[1].server={platform:"linux",protocol:"nfs",network:"10.2.0.0/16"};copy.groups[1].nfs.export="/srv/store-b";copy.groups[2].server.protocol="smb";});

test("the committed topology is accepted by Ajv and by the repository validator and has five symmetric groups",()=>{
 expect(accepts(committed)).toBe(true);
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Topology"},committed,schema)).toEqual([]);
 const read=readTopologyV1();
 expect(read.groups.map(group=>group.id)).toEqual(["g1","g2","g3","g4","g5"]);
 for(const group of read.groups){
  expect(group.nfs).toEqual({host:`nfs.${group.id}.semio.internal`,export:"/srv/semio-store"});
  expect(group.cache).toEqual({endpoint:`http://cache.${group.id}.semio.internal:6090`,instance:"semio"});
  expect(group.peers).toEqual(Object.fromEntries(read.groups.filter(other=>other.id!==group.id).map(other=>[other.id,`/mnt/semio-peers/${other.id}`])));
 }
 expect(read.worktree).toEqual({linux:"/srv/semio",macos:"/Users/Shared/semio",windows:"C:\\semio"});
 expect(read.storeMount).toEqual({linux:"/mnt/semio-store",macos:"/Volumes/semio-store",windows:"S:\\"});
 expect(read.replica).toEqual({pollMilliseconds:250,scanSeconds:900,debounceMilliseconds:150,retentionDays:14,bootstrapMinutes:60});
});

test("the fixture topology is accepted and every invalid case is refused, and Ajv and the repository validator agree",()=>{
 expect(accepts(fixture.topology)).toBe(true);
 for(const change of fixture.invalid as unknown as Change[]){
  const value=mutate(change);
  let message="accepted";
  try{readTopologyV1(value);}catch(error){message=(error as Error).message;}
  expect(`${change.name}: ${message}`).toContain("Closed fleet topology required");
  expect(validateJsonSchemaSubset({$ref:"#/definitions/Topology"},value,schema).length===0).toBe(Boolean(accepts(value)));
 }
});

test("the environment of every group is its own cache endpoint, read-write access and instance",()=>{
 for(const id of ids){
  const expected=(fixture.environments as Record<string,Record<string,string>>)[id]!;
  expect(machineEnvironmentV1(topology,id)).toEqual(expected);
  expect(renderEnvironmentV1(machineEnvironmentV1(topology,id),"sh")).toBe(Object.entries(expected).map(([name,value])=>`export ${name}=${value}`).join("\n"));
  expect(runTopologyCommandV1(["env","--group",id,"--shell","powershell"],{topology:fixture.topology})).toBe(Object.entries(expected).map(([name,value])=>`$env:${name}="${value}"`).join("\n"));
  expect(runTopologyCommandV1(["env",`--group=${id}`],{platform:"darwin",topology:fixture.topology})).toContain(`export SEMIO_REMOTE_CACHE_ENDPOINT=${expected["SEMIO_REMOTE_CACHE_ENDPOINT"]}\n`);
 }
});

test("an unknown group, platform or shell is refused",()=>{
 expect(()=>groupV1(topology,"zz")).toThrow("Declared fleet group required");
 expect(()=>machineEnvironmentV1(topology,"zz")).toThrow("Declared fleet group required");
 for(const operation of["mount-plan","server-plan","worker-service","facts"])expect(()=>runTopologyCommandV1([operation,"--group","zz"],{platform:"linux",topology:fixture.topology}),operation).toThrow("Declared fleet group required");
 expect(()=>runTopologyCommandV1(["mount-plan","--group","a1","--platform","beos"],{topology:fixture.topology})).toThrow("--platform must be one of");
 expect(()=>runTopologyCommandV1(["env","--group","a1","--shell","fish"],{topology:fixture.topology})).toThrow("--shell must be one of");
 expect(()=>platformV1("freebsd")).toThrow("Supported fleet platform required");
 expect([platformV1("linux"),platformV1("darwin"),platformV1("win32")]).toEqual(["linux","macos","windows"]);
});

test("the worker service keeps the replica daemon alive on every platform",()=>{
 const group=fixture.topology.groups[0]!;
 const linux=workerServiceV1(topology,group.id,"linux"),macos=workerServiceV1(topology,group.id,"macos"),windows=workerServiceV1(topology,group.id,"windows");
 expect(linux).toContain(`WorkingDirectory=/srv/semio/${REPLICA}`);
 expect(linux).toContain(`ExecStart=/usr/local/bin/repo ./${SCRIPT} run`);
 expect(linux).toContain("Restart=always");
 expect(linux).toContain("User=%i");
 expect(macos).toContain(`<key>WorkingDirectory</key><string>/Users/Shared/semio/${REPLICA}</string>`);
 expect(macos).toContain(`<string>./${SCRIPT}</string><string>run</string>`);
 expect(macos).toContain("<key>KeepAlive</key><true/>");
 expect(macos).toContain("__WORKER_USER__");
 expect(windows).toContain(`<WorkingDirectory>C:\\semio\\${REPLICA.replaceAll("/","\\")}</WorkingDirectory>`);
 expect(windows).toContain(`<Arguments>./${SCRIPT} run</Arguments>`);
 expect(windows).toContain("<RestartOnFailure>");
 expect(runTopologyCommandV1(["worker-service","--group",group.id,"--platform","linux","--part","target"],{topology:fixture.topology})).toBe("/etc/systemd/system/semio-fleet-replica@.service");
});

test("the arguments are closed",()=>{
 expect(parseTopologyArgumentsV1(["env","--group","a1","--shell=sh"])).toEqual({operation:"env",values:{"--group":"a1","--shell":"sh"},flags:new Set()});
 expect(parseTopologyArgumentsV1(["validate","--reachable","--timeout","500"])).toEqual({operation:"validate",values:{"--timeout":"500"},flags:new Set(["--reachable"])});
 expect(()=>parseTopologyArgumentsV1(["serve"])).toThrow("Closed fleet topology command required");
 expect(()=>parseTopologyArgumentsV1(["env","--group","a1","--group","b2"])).toThrow("Repeated option");
 expect(()=>parseTopologyArgumentsV1(["env","--platform","linux"])).toThrow("Unknown option");
 expect(()=>parseTopologyArgumentsV1(["env","--reachable"])).toThrow("Unknown option");
 expect(()=>parseTopologyArgumentsV1(["validate","--reachable=yes"])).toThrow("takes no value");
 expect(()=>parseTopologyArgumentsV1(["validate","--reachable","--reachable"])).toThrow("Repeated option");
 expect(()=>parseTopologyArgumentsV1(["env","--group"])).toThrow("Value required");
 expect(()=>parseTopologyArgumentsV1(["env","--group","--shell"])).toThrow("Value required");
 expect(()=>runTopologyCommandV1(["env"],{topology:fixture.topology})).toThrow("Required option missing: --group");
 expect(()=>runTopologyCommandV1(["validate","--reachable"],{topology:fixture.topology})).toThrow("asynchronous");
 expect(runTopologyCommandV1(["validate"],{topology:fixture.topology})).toBe("valid fleet topology: 3 groups (a1, b2, c3)");
});

test("every group names its server and the protocol by which each worker platform mounts its store",()=>{
 const [a1,b2,c3]=topology.groups as [FleetGroupV1,FleetGroupV1,FleetGroupV1];
 expect(PLATFORMS.map(platform=>clientProtocolV1(a1,platform))).toEqual(["nfs4","nfs4","nfs3"]);
 expect(PLATFORMS.map(platform=>clientProtocolV1(b2,platform))).toEqual(["nfs3","nfs3","nfs3"]);
 expect(PLATFORMS.map(platform=>clientProtocolV1(c3,platform))).toEqual(["nfs3","nfs3","nfs3"]);
 expect(clientProtocolV1(smbFleet.groups[2]!,"windows")).toBe("smb");
 for(const platform of["linux","macos"] as const)expect(()=>clientProtocolV1(smbFleet.groups[2]!,platform)).toThrow("Windows workers required");
 expect(()=>factsV1(smbFleet,"c3","linux")).toThrow("Windows workers required");
 expect(()=>mountPlanV1(smbFleet,"c3","macos")).toThrow("Windows workers required");
 expect(factsV1(smbFleet,"c3","windows")).toMatchObject({PROTOCOL:"smb",MOUNT_OPTIONS:"",NFS_SOURCE:unc("nfs.c3.example.test","/store-c"),SERVER_PLATFORM:"windows",SERVER_PROTOCOL:"smb",SERVER_DIRECTORY:"C:\\store-c",SERVER_CACHE_DIRECTORY:"C:\\semio-remote-cache"});
 expect(factsV1(topology,"a1","linux")).toMatchObject({PROTOCOL:"nfs4",SERVER_PLATFORM:"linux",SERVER_NETWORK:"10.1.0.0/24",SERVER_DIRECTORY:"/srv/store-a",SERVER_CACHE_DIRECTORY:"/var/lib/semio-remote-cache"});
 expect(factsV1(topology,"c3","windows")).toMatchObject({PROTOCOL:"nfs3",SERVER_DIRECTORY:"C:\\store-c"});
});

test("the mount plan of every group follows the protocol of its server, for every platform",()=>{
 for(const group of fixture.topology.groups)for(const platform of PLATFORMS){
  const plan=mountPlanV1(topology,group.id,platform),host=group.nfs.host,path=group.nfs.export,fromLinux=group.server.platform==="linux";
  if(platform==="linux"){
   expect(plan).toContain(fromLinux?`${host}:${path} /mnt/semio-store nfs4 vers=4.2,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,_netdev,nofail 0 0`:`${host}:${path} /mnt/semio-store nfs vers=3,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,nolock,_netdev,nofail 0 0`);
   expect(plan).toContain("sudo mount /mnt/semio-store");
  }else if(platform==="macos"){
   expect(plan).toContain(`mount_nfs -o ${fromLinux?"vers=4,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache":"vers=3,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache,nolocks"} ${host}:${path} /Volumes/semio-store`);
   expect(plan).toContain("<key>RunAtLoad</key><true/>");
   expect(plan).toContain("/Library/LaunchDaemons/tech.semio.fleet.store-mount.plist");
  }else{
   expect(plan).toContain("ServicesForNFS-ClientOnly");
   expect(plan).toContain(`mount.exe -o anon,nolock,mtype=hard ${unc(host,path)} S:`);
   expect(plan).toContain("ONLOGON");
  }
  for(const other of fixture.topology.groups.filter(candidate=>candidate.id!==group.id))expect(plan).not.toContain(other.nfs.host);
 }
 expect(mountPartsV1(topology,"a1","linux")["fstab"]).toBe(factsV1(topology,"a1","linux")["FSTAB_LINE"]);
 expect(runTopologyCommandV1(["mount-plan","--group","b2","--platform","linux","--part","commands"],{topology:fixture.topology})).toBe("sudo mkdir -p /mnt/semio-store\nsudo mount /mnt/semio-store");
 expect(()=>runTopologyCommandV1(["mount-plan","--group","b2","--platform","linux","--part","nope"],{topology:fixture.topology})).toThrow("Declared part required");
 const smb=mountPartsV1(smbFleet,"c3","windows");
 expect(smb).toEqual({credential:"cmdkey /add:nfs.c3.example.test /user:nfs\\semio-fleet /pass:<password>",cache:"Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0 -Force",mount:`net use S: ${unc("nfs.c3.example.test","/store-c")} /persistent:yes`,task:`schtasks /Create /TN semio-fleet-store-mount /SC ONLOGON /F /TR "net use S: ${unc("nfs.c3.example.test","/store-c")} /persistent:yes"`});
});

test("a Linux worker plan sets the inotify limits that the tree of about 122k directories needs",()=>{
 for(const group of fixture.topology.groups){
  const parts=mountPartsV1(topology,group.id,"linux"),limits=Object.fromEntries(parts["sysctl"]!.split("\n").map(line=>line.split("=") as [string,string]));
  expect(parts["sysctl-path"]).toBe("/etc/sysctl.d/90-semio-fleet.conf");
  expect(Number(limits["fs.inotify.max_user_watches"])).toBeGreaterThanOrEqual(1048576);
  expect(Number(limits["fs.inotify.max_user_instances"])).toBeGreaterThanOrEqual(512);
  expect(mountPlanV1(topology,group.id,"linux")).toContain("fs.inotify.max_user_watches=1048576");
 }
 expect(Object.keys(mountPartsV1(topology,"a1","macos"))).not.toContain("sysctl");
 expect(runTopologyCommandV1(["mount-plan","--group","a1","--platform","linux","--part","sysctl-path"],{topology:fixture.topology})).toBe("/etc/sysctl.d/90-semio-fleet.conf");
 expect(readTopologyV1().worktree.windows.length).toBeLessThanOrEqual(15);
});

test("the Linux group server exports the store, mounts the peers read-only by their own protocol and runs the cache and the relay forever",()=>{
 const group=fixture.topology.groups[0]!,plan=serverPlanV1(topology,"a1"),parts=serverPartsV1(topology,"a1"),others=fixture.topology.groups.slice(1);
 expect(parts["exports-target"]).toBe("/etc/exports.d/semio-fleet.exports");
 expect(parts["exports"]).toStartWith(`${group.nfs.export} 10.1.0.0/24(rw,sync,no_subtree_check,all_squash,anonuid=1500,anongid=1500)`);
 expect(parts["fstab"]).toContain(`${group.nfs.export} /mnt/semio-store none bind 0 0`);
 for(const other of others){
  expect(parts["exports"]).toContain(` ${other.nfs.host}(ro,sync,no_subtree_check)`);
  expect(parts["fstab"]).toContain(`${other.nfs.host}:${other.nfs.export} ${(group.peers as unknown as Record<string,string>)[other.id]} nfs vers=3,ro,hard,`);
 }
 expect(parts["cache-unit"]).toContain("ExecStart=/usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 6090 --directory /var/lib/semio-remote-cache --max-bytes 214748364800");
 expect(parts["cache-unit"]).toContain("EnvironmentFile=/etc/semio/remote-cache.env");
 expect(parts["cache-unit"]).toContain("Restart=always");
 expect(parts["cache-environment"]).toBe("SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN=<read-token>\nSEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN=<write-token>");
 expect(parts["relay-unit"]).toContain(`WorkingDirectory=/srv/semio/${REPLICA}`);
 expect(parts["relay-unit"]).toContain(`ExecStart=/usr/local/bin/repo ./${SCRIPT} relay`);
 expect(parts["relay-unit"]).toContain("Restart=always");
 expect(parts["relay-peers"]).toBe("b2=/mnt/peers/b2,c3=/mnt/peers/c3");
 expect(plan).toContain(parts["exports"]!);
 expect(serverPartsV1(smbFleet,"a1")["fstab"]).toContain("//nfs.c3.example.test/store-c /mnt/peers/c3 cifs ro,vers=3.1.1,credentials=/etc/semio/smb-peer.credentials,");
 expect(serverPartsV1(variant(copy=>{copy.groups[1].server={platform:"linux",protocol:"nfs",network:"10.2.0.0/16"};copy.groups[1].nfs.export="/srv/store-b";}),"a1")["fstab"]).toContain("nfs.b2.example.test:/srv/store-b /mnt/peers/b2 nfs4 vers=4.2,ro,hard,");
});

test("the macOS group server exports with nfsd, mounts the peers by LaunchDaemon and runs the cache and the relay as LaunchDaemons",()=>{
 const parts=serverPartsV1(topology,"b2");
 expect(Object.keys(parts)).toEqual(["prepare","provision","start","exports-target","exports","nfs-conf-target","nfs-conf","peer-mount-target","peer-mount-unit","relay-peers","cache-environment-target","cache-environment","cache-target","cache-unit","replica-unit","relay-target","relay-unit"]);
 expect(parts["exports-target"]).toBe("/etc/exports");
 expect(parts["exports"]).toBe("/Users/Shared/store-b -mapall=__WORKER_USER__ -network 10.2.0.0 -mask 255.255.0.0\n/Users/Shared/store-b -ro -mapall=__WORKER_USER__ nfs.a1.example.test nfs.c3.example.test");
 expect(parts["nfs-conf"]).toBe("nfs.server.mount.port = 892");
 expect(parts["peer-mount-unit"]).toContain("/sbin/mount_nfs -o vers=4,ro,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache nfs.a1.example.test:/srv/store-a /Volumes/semio-peers/a1");
 expect(parts["peer-mount-unit"]).toContain("/sbin/mount_nfs -o vers=3,ro,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache,nolocks nfs.c3.example.test:/store-c /Volumes/semio-peers/c3");
 expect(parts["cache-unit"]).toContain("set -a; . /etc/semio/remote-cache.env; set +a; exec /usr/local/bin/repo remote-cache serve --host 0.0.0.0 --port 7000 --directory /var/lib/semio-remote-cache --max-bytes 214748364800");
 expect(parts["cache-unit"]).toContain("<key>KeepAlive</key><true/>");
 expect(parts["relay-unit"]).toContain(`<string>./${SCRIPT}</string><string>relay</string><string>--peers</string><string>a1=/Volumes/semio-peers/a1,c3=/Volumes/semio-peers/c3</string>`);
 expect(parts["relay-peers"]).toBe("a1=/Volumes/semio-peers/a1,c3=/Volumes/semio-peers/c3");
 expect(parts["provision"]).toEndWith("b2 b2-server --user <account> --archive /Volumes/semio-peers/a1/v1/bootstrap --server");
 expect(parts["start"]).toContain("sudo nfsd enable");
 for(const key of["cache-unit","relay-unit","replica-unit"])expect(parts[key]).toContain("__WORKER_USER__");
 expect(parts["replica-unit"]).toBe(workerServiceV1(topology,"b2","macos"));
});

test("the Windows group server shares the store with Server for NFS or, for an smb group, as an SMB share, and runs cache and relay as scheduled tasks",()=>{
 const nfs=serverPartsV1(topology,"c3"),smb=serverPartsV1(smbFleet,"c3"),relay=`./${SCRIPT} relay --peers a1=${unc("nfs.a1.example.test","/srv/store-a")},b2=${unc("nfs.b2.example.test","/Users/Shared/store-b")}`;
 expect(Object.keys(nfs)).toEqual(["prepare","provision","start","feature","share","firewall","relay-peers","cache-environment-target","cache-environment","cache-target","cache-unit","replica-unit","relay-target","relay-unit"]);
 expect(nfs["feature"]).toStartWith("Install-WindowsFeature FS-NFS-Service -IncludeManagementTools\n");
 expect(nfs["feature"]).toContain("Install-WindowsFeature NFS-Client");
 expect(smb["feature"]).toStartWith("Set-SmbServerConfiguration -EnableSMB2Protocol $true -Force\n");
 expect(serverPartsV1(variant(copy=>{copy.groups=[{...copy.groups[2],peers:{}}];}),"c3")["feature"]).toBe("Install-WindowsFeature FS-NFS-Service -IncludeManagementTools");
 expect(nfs["share"]).toContain("New-NfsShare -Name store-c -Path C:\\store-c -Authentication sys");
 expect(nfs["share"]).toContain("Grant-NfsSharePermission -Name store-c -ClientName nfs.a1.example.test -ClientType host -Permission readonly");
 expect(nfs["share"]).toContain("Grant-NfsSharePermission -Name store-c -ClientName nfs.b2.example.test -ClientType host -Permission readonly");
 expect(nfs["firewall"]).toContain("-LocalPort 2049,111,1048 -RemoteAddress 10.3.0.0/24");
 expect(nfs["firewall"]).toContain("-LocalPort 443 -RemoteAddress 10.3.0.0/24");
 expect(nfs["relay-unit"]).toContain(`<Arguments>${relay}</Arguments>`);
 expect(nfs["relay-unit"]).toContain("<WorkingDirectory>C:\\semio\\"+REPLICA.replaceAll("/","\\")+"</WorkingDirectory>");
 expect(nfs["cache-unit"]).toContain("<Command>powershell.exe</Command>");
 expect(nfs["cache-unit"]).toContain("remote-cache serve --host 0.0.0.0 --port 443 --directory C:\\semio-remote-cache --max-bytes 214748364800");
 expect(nfs["cache-unit"]).toContain("Get-Content C:\\ProgramData\\semio\\remote-cache.env");
 expect(nfs["cache-unit"]).toContain("&quot;");
 expect(nfs["cache-unit"]).toContain("<RestartOnFailure>");
 expect(nfs["cache-target"]).toBe("semio-fleet-cache");
 expect(nfs["relay-target"]).toBe("semio-fleet-relay");
 expect(nfs["provision"]).toEndWith(`c3 c3-server -Server -ArchivePath ${unc("nfs.a1.example.test","/srv/store-a")}\\v1\\bootstrap`);
 expect(nfs["start"]).toBe("Start-ScheduledTask -TaskName semio-fleet-replica\nStart-ScheduledTask -TaskName semio-fleet-cache\nStart-ScheduledTask -TaskName semio-fleet-relay");
 expect(smb["share"]).toContain("New-SmbShare -Name store-c -Path C:\\store-c -ChangeAccess semio-fleet -ReadAccess semio-fleet-peer");
 expect(smb["share"]).not.toContain("NfsShare");
 expect(smb["firewall"]).toContain("-LocalPort 445 -RemoteAddress 10.3.0.0/24");
 expect(smb["prepare"]).toContain("New-LocalUser -Name semio-fleet ");
 expect(smb["prepare"]).not.toContain("FS-NFS-Service");
 expect(smb["relay-peers"]).toBe(`a1=${unc("nfs.a1.example.test","/srv/store-a")},b2=${unc("nfs.b2.example.test","/srv/store-b")}`);
});

test("the facts quote their values for both shells",()=>{
 const facts=factsV1(topology,"b2","windows");
 expect(facts["NFS_SOURCE"]).toBe(unc("nfs.b2.example.test","/Users/Shared/store-b"));
 expect(facts["DRIVE"]).toBe("S:");
 expect(facts["FSTAB_LINE"]).toBe("");
 expect(renderFactsV1(facts,"powershell")).toContain("$FLEET_WORKTREE='C:\\semio'");
 expect(renderFactsV1({A:"it's"},"sh")).toBe("FLEET_A='it'\\''s'");
 expect(renderFactsV1({A:"it's"},"powershell")).toBe("$FLEET_A='it''s'");
 expect(runTopologyCommandV1(["facts","--group","a1","--platform","macos","--shell","sh"],{topology:fixture.topology})).toContain("FLEET_MOUNT_OPTIONS='vers=4,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache'");
 expect(runTopologyCommandV1(["facts","--group","b2","--platform","macos","--shell","sh"],{topology:fixture.topology})).toContain("FLEET_MOUNT_OPTIONS='vers=3,hard,nobrowse,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache,nolocks'");
});

test("the Linux server plan runs the replica and the relay as the semio account and rebuilds the bootstrap archive through the relay",()=>{
 const parts=serverPartsV1(topology,"a1");
 expect(Object.keys(parts)).toEqual(["prepare","provision","start","exports-target","exports","sysctl-target","sysctl","fstab","relay-peers","cache-environment-target","cache-environment","cache-target","cache-unit","replica-unit","relay-target","relay-unit"]);
 expect(parts["sysctl-target"]).toBe("/etc/sysctl.d/91-semio-fleet-server.conf");
 expect(parts["sysctl"]).toBe("fs.leases-enable=0");
 expect(parts["prepare"]).toContain("--create-home --home-dir /var/lib/semio");
 expect(parts["provision"]).toEndWith("a1 a1-server --user semio --archive /mnt/peers/b2/v1/bootstrap --server");
 expect(parts["start"]).toContain("semio-fleet-replica@semio.service semio-fleet-cache.service semio-fleet-relay.service");
 expect(parts["replica-unit"]).toBe(workerServiceV1(topology,"a1","linux"));
 expect(parts["relay-unit"]).toContain("After=network-online.target remote-fs.target semio-fleet-replica@semio.service");
 expect(parts["relay-unit"]).toContain("ConditionPathExists=/srv/semio/.cache/f/machine.json");
 expect(parts["relay-unit"]).toContain("RequiresMountsFor=/mnt/semio-store\n");
 expect(parts["relay-unit"]).not.toContain("/mnt/peers");
 expect(parts["relay-unit"]).toContain("User=semio");
 expect(topology.replica.bootstrapMinutes).toBe(60);
 const single=serverPartsV1(variant(copy=>{copy.groups=[{...copy.groups[0],peers:{}}];}),"a1");
 expect(single["provision"]).toEndWith("a1 a1-server --user semio --server");
 expect(single["relay-peers"]).toBe("");
});

test("every worker and every group server gets a daily compaction next to its replica daemon",()=>{
 const linux=workerPartsV1(topology,"a1","linux"),macos=workerPartsV1(topology,"b2","macos"),windows=workerPartsV1(topology,"c3","windows");
 expect(linux["compact-target"]).toBe("/etc/systemd/system/semio-fleet-compact@.service");
 expect(linux["timer-target"]).toBe("/etc/systemd/system/semio-fleet-compact@.timer");
 expect(linux["compact-definition"]).toContain(`ExecStart=/usr/local/bin/repo ./${SCRIPT} compact`);
 expect(linux["compact-definition"]).toContain("Type=oneshot");
 expect(linux["compact-definition"]).not.toContain("--collect-blobs");
 expect(macos["compact-definition"]).not.toContain("--collect-blobs");
 expect(windows["compact-definition"]).not.toContain("--collect-blobs");
 expect(workerPartsV1(topology,"a1","linux",true)["compact-definition"]).toContain(`ExecStart=/usr/local/bin/repo ./${SCRIPT} compact --collect-blobs`);
 expect(workerPartsV1(topology,"b2","macos",true)["compact-definition"]).toContain(`<string>./${SCRIPT}</string><string>compact</string><string>--collect-blobs</string>`);
 expect(workerPartsV1(topology,"c3","windows",true)["compact-definition"]).toContain(`<Arguments>./${SCRIPT} compact --collect-blobs</Arguments>`);
 expect(workerPartsV1(topology,"a1","linux",true)["definition"]).toBe(linux["definition"]);
 expect(()=>workerPartsV1(topology,"a1","macos",true)).toThrow("Server platform required");
 expect(()=>workerPartsV1(topology,"b2","linux",true)).toThrow("Server platform required");
 expect(runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--part","compact-definition","--role","server"],{topology:fixture.topology})).toBe(workerPartsV1(topology,"a1","linux",true)["compact-definition"]!);
 expect(runTopologyCommandV1(["worker-service","--group","b2","--platform","macos","--part","compact-definition","--role","server"],{topology:fixture.topology})).toBe(workerPartsV1(topology,"b2","macos",true)["compact-definition"]!);
 expect(runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--role","server"],{topology:fixture.topology})).toBe(linux["definition"]!);
 expect(()=>runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--role","admin"],{topology:fixture.topology})).toThrow("must be one of worker|server");
 expect(linux["timer-definition"]).toContain("OnCalendar=daily");
 expect(macos["compact-target"]).toBe("/Library/LaunchDaemons/tech.semio.fleet.compact.plist");
 expect(macos["compact-definition"]).toContain(`<string>./${SCRIPT}</string><string>compact</string>`);
 expect(macos["compact-definition"]).toContain("<key>StartCalendarInterval</key>");
 expect(macos["compact-definition"]).not.toContain("KeepAlive");
 for(const text of[macos["definition"]!,macos["compact-definition"]!]){expect(text).toContain("__WORKER_USER__");expect(text).toContain("<key>HOME</key><string>__WORKER_HOME__</string>");}
 expect(windows["compact-target"]).toBe("semio-fleet-compact");
 expect(windows["compact-definition"]).toContain(`<Arguments>./${SCRIPT} compact</Arguments>`);
 expect(windows["compact-definition"]).toContain("<CalendarTrigger>");
});

test("the facts name the archive and the topology directory of the canonical worktree",()=>{
 expect(factsV1(topology,"a1","linux")["ARCHIVE_DIRECTORY"]).toBe("/mnt/semio-store/v1/bootstrap");
 expect(factsV1(topology,"a1","windows")["ARCHIVE_DIRECTORY"]).toBe("S:\\v1\\bootstrap");
 expect(factsV1(topology,"a1","macos")["TOPOLOGY_DIRECTORY"]).toBe(`/Users/Shared/semio/${REPLICA.replace("🔁️replica","🗺️topology")}`);
 expect(factsV1(topology,"a1","windows")["TOPOLOGY_DIRECTORY"]).toBe(`C:\\semio\\${REPLICA.replace("🔁️replica","🗺️topology").replaceAll("/","\\")}`);
});

test("the provisioning scripts name existing files and only commands that the replica and topology commands accept",()=>{
 const root=resolve(import.meta.dirname,"../../../../../../.."),directory=join(import.meta.dirname,"..","🥾️provision"),files=readdirSync(directory),shell=readFileSync(join(directory,files.find(name=>name.endsWith(".sh"))!),"utf8"),powershell=readFileSync(join(directory,files.find(name=>name.endsWith(".ps1"))!),"utf8");
 const modules=REPLICA.slice(0,REPLICA.indexOf("🤝️shared-worktree")-1);
 for(const script of[shell,powershell]){
  expect(script).toContain("🔩️native");
  expect(script).not.toContain("--seed");
  expect(script).not.toContain("SeedPath");
  expect(script).toContain("belongs to origin");
  expect(script).toContain("belongs to group");
 }
 for(const relative of["🔩️native/🥾️bootstrap/🐚️.sh","🔩️native/🥾️bootstrap/🔵️.ps1"])expect(existsSync(join(root,modules,relative)),relative).toBe(true);
 expect(shell).toContain(`NATIVE_BOOTSTRAP="$MODULES/🔩️native/🥾️bootstrap/🐚️.sh"`);
 expect(powershell).toContain("$NativeBootstrap = \"$Modules\\🔩️native\\🥾️bootstrap\\🔵️.ps1\"");
 for(const call of[["bootstrap-settle","--store","/x"],["join","--group","g1","--origin","g1-m01","--store","/x"],["seed","--group","g1","--origin","g1-m01","--store","/x"],["run"],["compact"],["compact","--collect-blobs"],["relay","--once","--group","g1","--store","/x","--peers","g2=/p"]])expect<string>(parseReplicaArgumentsV1(call).operation).toBe(call[0]);
 expect(parseReplicaArgumentsV1(["compact","--collect-blobs"]).flags.has("--collect-blobs")).toBe(true);
 expect(()=>parseReplicaArgumentsV1(["run","--collect-blobs"])).toThrow();
 expect(shell).toContain("--part compact-definition --role");
 expect(powershell).toContain("'compact-definition', '--role', $role");
 const group="a1",call=(...call:string[])=>expect(()=>runTopologyCommandV1(call,{topology:fixture.topology}),call.join(" ")).not.toThrow();
 for(const arguments_ of[["facts","--group",group,"--platform","linux","--shell","sh"],["facts","--group",group,"--platform","windows","--shell","powershell"],["env","--group",group,"--shell","sh"],["mount-plan","--group",group,"--platform","macos","--part","plist-path"],["mount-plan","--group",group,"--platform","macos","--part","plist"],["mount-plan","--group",group,"--platform","linux","--part","sysctl-path"],["mount-plan","--group",group,"--platform","linux","--part","sysctl"],
  ...["target","compact-target","compact-definition"].map(part=>["worker-service","--group",group,"--platform","macos","--part",part]),...["target","compact-target","compact-definition"].map(part=>["worker-service","--group",group,"--platform","windows","--part",part]),
  ...["target","compact-target","compact-definition","timer-target","timer-definition"].map(part=>["worker-service","--group",group,"--platform","linux","--part",part]),["worker-service","--group",group,"--platform","linux"],["worker-service","--group",group,"--platform","linux","--part","compact-definition","--role","server"],["worker-service","--group","b2","--platform","macos","--part","compact-definition","--role","server"],["worker-service","--group","c3","--platform","windows","--part","compact-definition","--role","server"]])call(...arguments_);
 const shellParts=[...shell.matchAll(/server_part ([a-z-]+)/gu)].map(match=>match[1]!),windowsParts=[...powershell.matchAll(/Get-ServerPart '([a-z-]+)'/gu)].map(match=>match[1]!);
 expect(shellParts.length).toBeGreaterThan(8);
 for(const name of new Set(shellParts))expect(Object.keys(serverPartsV1(topology,"a1")).concat(Object.keys(serverPartsV1(topology,"b2"))),name).toContain(name);
 for(const name of new Set([...windowsParts,"cache-target","cache-unit","relay-target","relay-unit","feature","share","firewall"]))expect(Object.keys(serverPartsV1(topology,"c3")),name).toContain(name);
 expect(powershell).toContain("LongPathsEnabled");
 expect(powershell).toContain("-Server");
 expect(powershell).toContain("net.exe use");
 expect(shell).toContain('sysctl -p "$SYSCTL_TARGET"');
});

test("every service definition of every server platform is well-formed XML or has balanced sections",()=>{
 if(!python)return;
 const documents=[...["a1","b2","c3"].flatMap(id=>Object.entries(serverPartsV1(topology,id)).filter(([,body])=>body.startsWith("<?xml")).map(([name,body])=>`${id}.${name}`+"\u0000"+body)),...PLATFORMS.flatMap(platform=>Object.entries(workerPartsV1(topology,"a1",platform)).filter(([,body])=>body.startsWith("<?xml")).map(([name,body])=>`w.${platform}.${name}`+"\u0000"+body)),`m.${"plist"}\u0000${mountPartsV1(topology,"a1","macos")["plist"]!}`];
 const check=Bun.spawnSync([python,"-I","-c","import sys,xml.etree.ElementTree as T\nfor item in sys.stdin.read().split('\\x01'):\n    name,body=item.split('\\x00',1)\n    T.fromstring(body.split('\\n',1)[1].encode('utf8') if body.startswith('<?xml version=\"1.0\" encoding=\"UTF-16\"') else body.encode('utf8'))\n    print(name)"],{stdin:new TextEncoder().encode(documents.join("\u0001")),stdout:"pipe",stderr:"pipe"});
 expect(check.stderr.toString()).toBe("");
 expect(check.exitCode).toBe(0);
 expect(check.stdout.toString().trim().split("\n").length).toBe(documents.length);
});

test("server platform rules: smb is Windows only, a Windows server needs a folder and a share name, a macOS server exports below /Users or /Volumes",()=>{
 expect(smbFleet.groups.map(group=>group.server.protocol)).toEqual(["nfs","nfs","smb"]);
 expect(()=>variant(copy=>{copy.groups[1].server.protocol="smb";})).toThrow("shares over smb");
 expect(()=>variant(copy=>{delete copy.groups[2].server.directory;})).toThrow("needs server.directory");
 expect(()=>variant(copy=>{copy.groups[2].nfs.export="/srv/store-c";})).toThrow("single-segment share name");
 expect(()=>variant(copy=>{copy.groups[1].nfs.export="/srv/store-b";})).toThrow("below /Users or /Volumes");
 expect(()=>variant(copy=>{copy.groups[0].server.directory="C:\\x";})).toThrow("only on a windows server");
 expect(()=>variant(copy=>{copy.groups[2].server.protocol="smb";})).toThrow("macos server cannot read the smb store");
});

test("plain validate stays offline and validate --reachable reports every group with progress",async()=>{
 const forbidden:ReachabilityPortsV1={resolve:async()=>{throw Error("network used");},connect:async()=>{throw Error("network used");}};
 const offline=await runTopologyCommandAsyncV1(["validate"],{topology:fixture.topology,signal:new AbortController().signal,progress:()=>{throw Error("progress");},ports:forbidden});
 expect(offline).toEqual({text:"valid fleet topology: 3 groups (a1, b2, c3)",exitCode:0});
 const opened:string[]=[],lines:string[]=[],ports:ReachabilityPortsV1={resolve:async host=>{if(host==="nfs.b2.example.test")throw Error("getaddrinfo ENOTFOUND nfs.b2.example.test");return ["192.0.2.1"];},connect:async(address,port)=>{opened.push(`${address}:${port}`);if(port===6090)throw Error("connect ECONNREFUSED");}};
 const report=await runTopologyCommandAsyncV1(["validate","--reachable","--timeout","1000"],{topology:fixture.topology,signal:new AbortController().signal,progress:line=>lines.push(line),ports});
 expect(report.exitCode).toBe(1);
 expect(report.text.split("\n")).toEqual([
  "valid fleet topology: 3 groups (a1, b2, c3)",
  "a1 store nfs.a1.example.test:2049 ok 192.0.2.1:2049 open",
  "a1 cache cache.a1.example.test:6090 FAILED connect ECONNREFUSED",
  "b2 store nfs.b2.example.test:2049 FAILED getaddrinfo ENOTFOUND nfs.b2.example.test",
  "b2 cache cache.b2.example.test:7000 ok 192.0.2.1:7000 open",
  "c3 store nfs.c3.example.test:2049 ok 192.0.2.1:2049 open",
  "c3 cache cache.c3.example.test:443 ok 192.0.2.1:443 open",
  "unreachable groups: a1, b2"
 ]);
 expect(lines.length).toBe(6);
 expect(opened.sort()).toEqual(["192.0.2.1:2049","192.0.2.1:2049","192.0.2.1:443","192.0.2.1:6090","192.0.2.1:7000"]);
 const all=await runTopologyCommandAsyncV1(["validate","--reachable"],{topology:fixture.topology,signal:new AbortController().signal,progress:()=>{},ports:{resolve:async()=>["192.0.2.9"],connect:async()=>{}}});
 expect(all.exitCode).toBe(0);
 expect(all.text).toEndWith("all 3 groups reachable");
 const smb=await checkReachabilityV1(smbFleet,{timeoutMilliseconds:1000,signal:new AbortController().signal,progress:()=>{},ports:{resolve:async()=>["192.0.2.9"],connect:async()=>{}}});
 expect(smb.checks.find(entry=>entry.group==="c3"&&entry.check==="store")!.port).toBe(445);
 await expect(runTopologyCommandAsyncV1(["validate","--timeout","500"],{topology:fixture.topology,signal:new AbortController().signal,progress:()=>{}})).rejects.toThrow("--timeout needs --reachable");
 for(const bad of["50","70000","x"])await expect(runTopologyCommandAsyncV1(["validate","--reachable","--timeout",bad],{topology:fixture.topology,signal:new AbortController().signal,progress:()=>{},ports:forbidden})).rejects.toThrow("--timeout must be a whole number");
});

test("the reachability check times out a silent host, stops on cancellation and probes real TCP ports",async()=>{
 const silent:Partial<ReachabilityPortsV1>={resolve:async()=>["192.0.2.1"],connect:()=>new Promise<void>(()=>{})};
 const slow=await checkReachabilityV1(topology,{timeoutMilliseconds:100,signal:new AbortController().signal,progress:()=>{},ports:silent});
 expect(slow.ok).toBe(false);
 expect(slow.cancelled).toBe(false);
 expect(slow.checks.every(entry=>entry.detail.includes("100 ms"))).toBe(true);
 const controller=new AbortController(),events:string[]=[],pending=checkReachabilityV1(topology,{timeoutMilliseconds:60000,signal:controller.signal,progress:line=>events.push(line),ports:silent});
 setTimeout(()=>controller.abort(),50);
 const cancelled=await pending;
 expect(cancelled.cancelled).toBe(true);
 expect(cancelled.ok).toBe(false);
 expect(cancelled.text).toEndWith("reachability cancelled");
 expect(events.length).toBe(6);
 expect(cancelled.checks.every(entry=>entry.detail==="cancelled")).toBe(true);
 const early=new AbortController();early.abort();
 expect((await checkReachabilityV1(topology,{timeoutMilliseconds:1000,signal:early.signal,progress:()=>{},ports:silent})).cancelled).toBe(true);
 const listener=createServer(socket=>socket.end());
 await new Promise<void>(done=>listener.listen(0,"127.0.0.1",done));
 const port=(listener.address() as {port:number}).port;
 try{
  const local=variant(copy=>{for(const group of copy.groups){group.nfs.host="localhost";group.cache.endpoint=`http://localhost:${port}`;}});
  const real=await checkReachabilityV1(local,{timeoutMilliseconds:2000,signal:new AbortController().signal,progress:()=>{},ports:{resolve:async()=>["127.0.0.1"]}});
  expect(real.checks.filter(entry=>entry.check==="cache").every(entry=>entry.ok&&entry.detail===`127.0.0.1:${port} open`)).toBe(true);
  expect(real.checks.filter(entry=>entry.check==="store").every(entry=>entry.port===2049)).toBe(true);
 }finally{listener.close();}
});

test.skipIf(!python)("the Python oracle accepts the committed topology and recomputes the same environments",()=>{
 const check=Bun.spawnSync([python!,"-I",oracle,"check"],{stdout:"pipe",stderr:"pipe"});
 expect(check.stdout.toString()+check.stderr.toString()).toBe("");
 expect(check.exitCode).toBe(0);
 const run=(file:string,id:string)=>{
  const result=Bun.spawnSync([python!,"-I",oracle,"env",file,id],{stdout:"pipe"});
  expect(result.exitCode).toBe(0);
  return JSON.parse(result.stdout.toString()) as Record<string,string>;
 };
 for(const id of ids)expect(run(join(import.meta.dirname,"./🧫️fixtures/🔣️.json"),id)).toEqual(machineEnvironmentV1(topology,id));
 for(const group of readTopologyV1().groups)expect(run(join(import.meta.dirname,"..","🔣️.json"),group.id)).toEqual(machineEnvironmentV1(readTopologyV1(),group.id));
});

test("every Linux mount option string revalidates names, because a head replaced by rename stays invisible to stat behind a cached name",()=>{
 for(const group of fixture.topology.groups){
  const fstab=mountPartsV1(topology,group.id,"linux")["fstab"]!;
  expect(fstab).toContain("lookupcache=none");
  expect(fstab).not.toContain("lookupcache=positive");
  if(group.server.platform!=="linux")continue;
  for(const line of serverPartsV1(topology,group.id)["fstab"]!.split("\n").filter(line=>line.includes("/mnt/peers/")&&!line.includes("cifs")))expect(line).toContain("lookupcache=none");
 }
});

test("the Linux cache unit starts without a locale, because the launcher refuses a start that chooses no language",()=>{
 expect(serverPartsV1(topology,"a1")["cache-unit"]).toContain("Environment=SEMIO_LANGUAGE=en");
 const shell=readFileSync(join(import.meta.dirname,"..","🥾️provision",readdirSync(join(import.meta.dirname,"..","🥾️provision")).find(name=>name.endsWith(".sh"))!),"utf8");
 expect(shell).toContain("LANGUAGE_CHOICE=${SEMIO_LANGUAGE:-en}");
 expect(shell).toContain('"SEMIO_LANGUAGE=$LANGUAGE_CHOICE"');
});

test("the topology script runs under the plain launcher: no invocation envelope, exit code 1 and a message on a refusal",async()=>{
 const lines:string[]=[],notes:string[]=[],signal=new AbortController().signal;
 expect(await runTopologyScriptV1(["validate"],"linux",signal,text=>lines.push(text),line=>notes.push(line))).toBe(0);
 expect(lines.join("")).toStartWith("valid fleet topology: 5 groups");
 expect(await runTopologyScriptV1(["facts","--group","g1","--platform","linux","--shell","sh"],"linux",signal,text=>lines.push(text),line=>notes.push(line))).toBe(0);
 expect(lines.join("")).toContain("FLEET_STORE_MOUNT='/mnt/semio-store'");
 expect(await runTopologyScriptV1(["bogus"],"linux",signal,text=>lines.push(text),line=>notes.push(line))).toBe(1);
 expect(notes.join("\n")).toContain("Closed fleet topology command required");
});

test("re-provisioning replaces the fstab line of a changed mount plan instead of refusing it, and a worker without sudo leaves the native build pending instead of failing the fleet setup",()=>{
 const directory=join(import.meta.dirname,"..","🥾️provision"),shell=readFileSync(join(directory,readdirSync(directory).find(name=>name.endsWith(".sh"))!),"utf8");
 expect(shell).toContain("already mounts another source at");
 expect(shell).toContain("the mount plan changed");
 expect(shell).not.toContain("already has a different entry for");
 expect(shell).toContain("has no passwordless sudo");
 expect(shell).toContain('check_toolchain || native_pending "the build tool check before $command" "$TOOLCHAIN_PROBLEM"');
 expect(shell).not.toMatch(/fail "[^"]*passwordless sudo/u);
});

const provisionFolder=join(import.meta.dirname,"..",readdirSync(join(import.meta.dirname,"..")).find(name=>name.endsWith("provision"))!),
 provisionShell=readFileSync(join(provisionFolder,readdirSync(provisionFolder).find(name=>name.endsWith(".sh"))!),"utf8"),
 provisionPowerShell=readFileSync(join(provisionFolder,readdirSync(provisionFolder).find(name=>name.endsWith(".ps1"))!),"utf8");
const optionsOf=(text:string):Map<string,string>=>new Map(text.split(",").map(item=>{const cut=item.indexOf("=");return cut<0?[item,""]:[item.slice(0,cut),item.slice(cut+1)];}));

test("macOS clients revalidate directories on every lookup, measured on macOS 27 (all 100 replaced heads seen after 2 ms, against 28 of 100 with the defaults) because mount_nfs has no lookupcache: acdirmin and acdirmax 0 plus nonegnamecache, file attributes stay at one second",()=>{
 for(const group of fixture.topology.groups){
  const facts=factsV1(topology,group.id,"macos"),options=optionsOf(facts["MOUNT_OPTIONS"]!);
  for(const [key,value] of Object.entries({acdirmin:"0",acdirmax:"0",acregmin:"1",acregmax:"1",nonegnamecache:"",hard:""}))expect(options.get(key),`${group.id} ${key}`).toBe(value);
  for(const forbidden of["noac","actimeo","lookupcache"])expect(options.has(forbidden),forbidden).toBe(false);
  expect(mountPlanV1(topology,group.id,"macos")).toContain(`mount_nfs -o ${facts["MOUNT_OPTIONS"]}`);
 }
 const peers=serverPartsV1(topology,"b2")["peer-mount-unit"]!;
 expect([...peers.matchAll(/mount_nfs -o (\S+)/gu)].map(match=>optionsOf(match[1]!).get("acdirmin")).join()).toBe("0,0");
 expect(peers).not.toContain("actimeo");
});

test("Windows clients get their attribute and name caches switched off for the protocol they use: SMB lifetimes, and registry values of Client for NFS documented by Microsoft for older Windows",()=>{
 const smb=mountPartsV1(smbFleet,"c3","windows"),nfs=mountPartsV1(topology,"a1","windows");
 for(const name of["DirectoryCacheLifetime","FileInfoCacheLifetime","FileNotFoundCacheLifetime"])expect(smb["cache"]).toContain(`-${name} 0`);
 expect(smb["cache"]).toStartWith("Set-SmbClientConfiguration");
 expect(Object.keys(nfs)).toEqual(["feature","cache","mount","task"]);
 expect(nfs["cache"]!.split("\n").length).toBe(5);
 expect(nfs["cache"]).toBe(["New-Item -Path HKLM:\\SOFTWARE\\Microsoft\\ClientForNFS\\CurrentVersion\\Users\\Default\\Cache -Force | Out-Null",...["RemoteWriteCache","FileAttributeCache"].map(name=>`New-ItemProperty -Path HKLM:\\SOFTWARE\\Microsoft\\ClientForNFS\\CurrentVersion\\Users\\Default\\Cache -Name ${name} -Value 0 -PropertyType DWord -Force | Out-Null`),"nfsadmin client stop","nfsadmin client start"].join("\n"));
 const facts=factsV1(topology,"a1","windows");
 expect(facts["SMB_CACHE"]).toBe("DirectoryCacheLifetime=0,FileInfoCacheLifetime=0,FileNotFoundCacheLifetime=0");
 expect(facts["NFS_CACHE"]).toBe("RemoteWriteCache=0,FileAttributeCache=0");
 expect(facts["NFS_CACHE_KEY"]).toBe("HKLM:\\SOFTWARE\\Microsoft\\ClientForNFS\\CurrentVersion\\Users\\Default\\Cache");
 for(const [name,value] of Object.entries({SMB_CACHE:facts["SMB_CACHE"]!,NFS_CACHE:facts["NFS_CACHE"]!}))for(const item of value.split(","))expect(provisionPowerShell,`${name} ${item}`).toContain(item.replace("=0"," = 0"));
 expect(provisionPowerShell).toContain(facts["NFS_CACHE_KEY"]!);
 expect(runTopologyCommandV1(["mount-plan","--group","c3","--platform","windows","--part","cache"],{topology:smbFleet})).toBe(smb["cache"]!);
});

test("both provisioning scripts install the daily compaction on every operating system and the server's passes --collect-blobs",()=>{
 expect(provisionShell).toContain('install_text "$(fleet worker-service --group "$GROUP_ID" --platform linux --part timer-target)"');
 expect(provisionShell).toContain('systemctl enable --now "${STARTED[@]}" "$TIMER"');
 expect(provisionShell).toContain('--platform macos --part compact-definition --role "$ROLE"');
 expect(provisionShell).toContain('for target in "$SERVICE_TARGET" "$COMPACT_TARGET"; do load_daemon "$target"; done');
 expect(provisionShell).toContain('ROLE=worker; [ "$SERVER" = 0 ] || ROLE=server');
 expect(provisionPowerShell).toContain("Register-ScheduledTask -TaskName $compactName -Xml $compactDefinition");
 expect(provisionPowerShell).toContain("$role = if ($Server) { 'server' } else { 'worker' }");
 expect(workerPartsV1(topology,"a1","linux")["timer-definition"]).toMatch(/OnCalendar=daily[\s\S]*Persistent=true/u);
 expect(workerPartsV1(topology,"b2","macos")["compact-definition"]).toContain("<key>StartCalendarInterval</key>");
 expect(workerPartsV1(topology,"c3","windows")["compact-definition"]).toContain("<ScheduleByDay><DaysInterval>1</DaysInterval></ScheduleByDay>");
});

test("a Linux group server switches NFS delegations off before its NFS server starts, and an already mounted peer without the cache options of the plan is reported (both measured with a macOS and a Linux client)",()=>{
 expect(provisionShell).toContain('SERVER_SYSCTL_TARGET=$(server_part sysctl-target)');
 expect(provisionShell.indexOf("SERVER_SYSCTL_TARGET=")).toBeLessThan(provisionShell.indexOf("systemctl enable --now nfs-server"));
 expect(provisionShell).toContain('sysctl -p "$SERVER_SYSCTL_TARGET"');
 expect(provisionShell).toContain("PEER_STRICT=$(server_part fstab | awk '/lookupcache=none/{print $2}')");
 expect(provisionShell).toContain("findmnt -no OPTIONS \"$path\" | grep -q 'lookupcache=none'");
});

const bash=Bun.which("bash");
function shellBlock(from:string,to:string,body:string,tools:readonly string[],extra:string=""):Readonly<{out:string;code:number}>{
 const root=mkdtempSync(join(tmpdir(),"semio-sh-")),bin=join(root,"bin");
 mkdirSync(bin,{recursive:true});
 for(const tool of tools)writeFileSync(join(bin,tool),tool==="timedatectl"?extra:"#!/bin/sh\n:\n",{mode:0o755});
 const block=provisionShell.slice(provisionShell.indexOf(from),provisionShell.indexOf(to));
 const run=Bun.spawnSync([bash!,"-c",`say(){ printf '%s\\n' "$*"; }; fail(){ printf 'FAIL %s\\n' "$*"; exit 3; }; ${block}\n${body}`],{env:{PATH:bin,ROOT:root}});
 rmSync(root,{recursive:true,force:true});
 return {out:run.stdout.toString()+run.stderr.toString(),code:run.exitCode};
}
const TOOLS=["cc","c++","make","pkg-config","curl","git","unzip"];
const prelude='PLATFORM=linux; WORKER_USER=builder; WORKER_HOME=$ROOT/home; WORKTREE=$ROOT/tree; run_as(){ return 1; };';
const toolchain=(tools:readonly string[],body:string="check_toolchain")=>shellBlock("BOOTSTRAP_MARKER=","check_time_sync() {",`${prelude} ${body}`,tools);

test("the shell script repeats a native bootstrap that has no completion marker or no launcher, and skips one that has both",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-marker-")),home=join(root,"home/.local/bin"),tree=join(root,"tree/.cache/f");
 mkdirSync(home,{recursive:true});mkdirSync(tree,{recursive:true});
 const needed=():number=>Bun.spawnSync([bash!,"-c",`WORKTREE=${root}/tree; WORKER_HOME=${root}/home; ${provisionShell.slice(provisionShell.indexOf("BOOTSTRAP_MARKER="),provisionShell.indexOf("toolchain_packages()"))}\nbootstrap_needed`]).exitCode;
 expect(needed()).toBe(0);
 writeFileSync(join(home,"repo"),"#!/bin/sh\n",{mode:0o755});
 expect(needed()).toBe(0);
 writeFileSync(join(tree,"native-bootstrap.done"),"2026-11-10T00:00:00Z\n");
 expect(needed()).toBe(1);
 rmSync(join(home,"repo"));
 expect(needed()).toBe(0);
 rmSync(root,{recursive:true,force:true});
 expect(provisionShell).toContain("if bootstrap_needed; then NEEDS_NATIVE=1; fi");
 expect(provisionShell).toContain('run_as tee "$BOOTSTRAP_MARKER"');
});

test("joining the fleet does not depend on the native build: the standalone launcher, mounts, limits, environment, clock, services and join come first, the native bootstrap is the last step and the exit codes are documented",()=>{
 const order=["bash \"$WORKTREE/$LAUNCHER_INSTALL\" \"$LAUNCHER_BIN\"","FACTS=$(fleet facts",'say "step 4: system limits','say "step 4b: clock"','say "step 5: cache environment"','say "step 6: replica and compact services','replica join --group','systemctl enable --now "${STARTED[@]}" "$TIMER"','for target in "$SERVICE_TARGET" "$COMPACT_TARGET"; do load_daemon','native_bootstrap\nsay "done:'].map(part=>provisionShell.indexOf(part));
 expect(order.every(index=>index>=0)).toBe(true);
 expect(order).toEqual([...order].sort((a,b)=>a-b));
 expect(provisionShell.split('exec bash "$2" setup\' sh "$environment" "$WORKTREE/$NATIVE_BOOTSTRAP"').length-1).toBe(1);
 expect(provisionShell.indexOf('exec bash "$2" setup\' sh "$environment" "$WORKTREE/$NATIVE_BOOTSTRAP"')).toBeGreaterThan(provisionShell.indexOf("native_bootstrap() {"));
 expect(provisionShell).not.toContain("native-bootstrap.done\"; fail");
 expect(provisionShell).toContain("EXIT_NATIVE_PENDING=3");
 expect(provisionShell).toContain("# Exit codes: 0 fleet ready and native build complete; 1 error");
 expect(provisionShell).toContain("3 fleet ready, native build pending");
 expect(provisionPowerShell).toContain("$ExitNativePending = 3");
 expect(provisionPowerShell).toContain("# Exit codes: 0 fleet ready and native build complete; 1 error");
 const powerShellOrder=["$LauncherInstall) -Destination $LauncherBin","Invoke-Fleet @('facts'","Register-ScheduledTask -TaskName $taskName","Invoke-Replica @('join'","Start-ScheduledTask -TaskName $startTask","step 11: native bootstrap"].map(part=>provisionPowerShell.indexOf(part));
 expect(powerShellOrder.every(index=>index>=0)).toBe(true);
 expect(powerShellOrder).toEqual([...powerShellOrder].sort((a,b)=>a-b));
 expect(provisionPowerShell.split("-File $nativeScript setup").length-1).toBe(2);
});

const nativeStep=(options:{repoExit:number;needed?:number;toolchain?:boolean;launcher?:boolean;stale?:boolean;environment?:string;profile?:string}):Readonly<{out:string;code:number;marker:boolean;log:string|undefined;ran:boolean;left:boolean}>=>{
 const root=mkdtempSync(join(tmpdir(),"semio-native-")),home=join(root,"home"),tree=join(root,"tree"),bin=join(root,"bin"),constants=provisionShell.slice(provisionShell.indexOf("MODULES="),provisionShell.indexOf("say() {"));
 const native=/^NATIVE_BOOTSTRAP="\$MODULES\/(.+)"$/mu.exec(constants)![1]!,modules=/^MODULES="(.+)"$/mu.exec(constants)![1]!,script=join(tree,modules,native);
 for(const directory of[join(home,".local/bin"),join(tree,".cache/f"),bin,join(script,"..")])mkdirSync(directory,{recursive:true});
 if(options.launcher??true)writeFileSync(join(home,".local/bin/repo"),"#!/bin/sh\n",{mode:0o755});
 if(options.stale)writeFileSync(join(tree,".cache/f/native-bootstrap.done"),"2026-11-10T00:00:00Z\n");
 writeFileSync(join(bin,"repo"),`#!/bin/sh\necho "[semio] repo $*"\nif [ ${options.repoExit} != 0 ]; then echo "Cannot find module 'typescript'" >&2; fi\nexit ${options.repoExit}\n`,{mode:0o755});
 if(options.profile)writeFileSync(join(home,".profile"),options.profile);
 writeFileSync(script,`#!/usr/bin/env bash\nset -euo pipefail\ntouch "${root}/ran"\necho "[semio] cache ${"${SEMIO_REMOTE_CACHE_ENDPOINT:-}"} ${"${SEMIO_REMOTE_CACHE_ACCESS:-}"} ${"${SEMIO_REMOTE_CACHE_INSTANCE:-}"} token=${"${SEMIO_REMOTE_CACHE_TOKEN:-none}"}"\necho "[semio] workspace:setup..."\nrepo run workspace:setup\n`);
 const toolchain=options.toolchain??true?"check_toolchain(){ return 0; }":'check_toolchain(){ TOOLCHAIN_PROBLEM="missing build tools: cc: install them as root first (apt-get install -y build-essential)"; return 1; }';
 const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf '%s\\n' "$*"; }\n${constants}\nWORKTREE=${tree}; WORKER_HOME=${home}; WORKER_USER=builder; ORIGIN_ID=a1-m01; GROUP_ID=a1; EXIT_NATIVE_PENDING=3; NEEDS_NATIVE=${options.needed??1}; PLATFORM=linux\nrun_as(){ "$@"; }\n${provisionShell.slice(provisionShell.indexOf("BOOTSTRAP_MARKER="),provisionShell.indexOf("check_time_sync() {"))}\n${toolchain}\nnative_bootstrap\necho reached-the-end`],{env:{...process.env,ENVIRONMENT:options.environment??"",PATH:`${bin}:${process.env["PATH"]}`}});
 const log=join(tree,".cache/f/native-bootstrap.log"),result={out:run.stdout.toString()+run.stderr.toString(),code:run.exitCode,marker:existsSync(join(tree,".cache/f/native-bootstrap.done")),log:existsSync(log)?readFileSync(log,"utf8"):undefined,ran:existsSync(join(root,"ran")),left:existsSync(join(tree,".cache/f/native-environment"))};
 rmSync(root,{recursive:true,force:true});
 return result;
};

(bash?test:test.skip)("a failing repo in the native bootstrap leaves the fleet ready: the marker stays absent, the message names the failing command and the log, and the exit code is 3",()=>{
 const failed=nativeStep({repoExit:1});
 expect(failed.code).toBe(3);
 expect(failed.out).not.toContain("reached-the-end");
 expect(failed.marker).toBe(false);
 expect(failed.log).toContain("Cannot find module 'typescript'");
 expect(failed.log).toContain("[semio] repo run workspace:setup");
 expect(failed.out).toContain("NATIVE BUILD PENDING: a1-m01 is a member of group a1 and syncs, but its native build is not ready");
 expect(failed.out).toMatch(/failing command: bash \S+ setup\n/u);
 expect(failed.out).toContain("reason: it exited with code 1");
 expect(failed.out).toMatch(/log: \S+\/\.cache\/f\/native-bootstrap\.log \(last lines below\)/u);
 expect(failed.out).toContain("| Cannot find module 'typescript'");
 expect(failed.out).toMatch(/the completion marker \S+native-bootstrap\.done was not written: run this script again to retry/u);
 expect(failed.out).toContain("exit code 3: fleet ready, native build pending");
 const fixed=nativeStep({repoExit:0});
 expect(fixed.code).toBe(0);
 expect(fixed.marker).toBe(true);
 expect(fixed.out).toContain("recorded the completed bootstrap in");
 expect(fixed.out).toContain("reached-the-end");
 expect(fixed.out).not.toContain("NATIVE BUILD PENDING");
 const nothing=nativeStep({repoExit:1,launcher:false,stale:true});
 expect(nothing.code).toBe(3);
 expect(nothing.marker).toBe(false);
 expect(nothing.out).toContain("reason: it exited with code 1");
 const done=nativeStep({repoExit:1,needed:0});
 expect(done.code).toBe(0);
 expect(done.ran).toBe(false);
 expect(done.out).toContain("native bootstrap complete");
});

(bash?test:test.skip)("the native bootstrap runs with the remote cache environment of the group and the write token of the account, and leaves no environment file behind",()=>{
 const environment="export SEMIO_REMOTE_CACHE_ENDPOINT=http://10.0.0.1:6090\nexport SEMIO_REMOTE_CACHE_ACCESS=read-write\nexport SEMIO_REMOTE_CACHE_INSTANCE=g1\nexport UNRELATED=1",run=nativeStep({repoExit:0,environment,profile:"export SEMIO_REMOTE_CACHE_TOKEN='abc-write'\n"});
 expect(run.code).toBe(0);
 expect(run.log).toContain("[semio] cache http://10.0.0.1:6090 read-write g1 token=abc-write");
 expect(run.left).toBe(false);
 expect(run.out).not.toContain("UNRELATED");
 const anonymous=nativeStep({repoExit:0,environment});
 expect(anonymous.log).toContain("token=none");
 expect(nativeStep({repoExit:0}).log).toContain("[semio] cache    token=none");
 expect(provisionShell.indexOf("native_environment |")).toBeGreaterThan(provisionShell.indexOf("native_bootstrap() {"));
 expect(provisionShell).not.toMatch(/env [^\n]*SEMIO_REMOTE_CACHE_TOKEN=\$/u);
});

(bash?test:test.skip)("a Linux worker gets swap up to the memory of a cold graph build before the native bootstrap, and none when RAM and swap already cover it",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-swap-")),meminfo=(ram:number,swap:number)=>{const path=join(root,`meminfo-${ram}-${swap}`);writeFileSync(path,`MemTotal:       ${ram*1024} kB\nMemFree:        1 kB\nSwapTotal:      ${swap*1024} kB\n`);return path;};
 const shortfall=(path:string):string=>Bun.spawnSync([bash!,"-c",`${provisionShell.slice(provisionShell.indexOf("COLD_GRAPH_MEMORY_MB="),provisionShell.indexOf("ensure_swap() {"))}\nswap_shortfall_mb ${path}`]).stdout.toString().trim();
 try{
  expect(shortfall(meminfo(8192,0))).toBe("8193");
  expect(shortfall(meminfo(8192,2048))).toBe("6145");
  expect(shortfall(meminfo(8192,8192))).toBe("0");
  expect(shortfall(meminfo(16384,0))).toBe("0");
  expect(shortfall(meminfo(65536,0))).toBe("0");
 }finally{rmSync(root,{recursive:true,force:true});}
 expect(provisionShell.indexOf("  ensure_swap\n  install_nfs_client")).toBeGreaterThan(provisionShell.indexOf('say "step 4: system limits'));
 expect(provisionShell.indexOf("  ensure_swap\n  install_nfs_client")).toBeLessThan(provisionShell.indexOf("native_bootstrap\nsay \"done:"));
});

(bash?test:test.skip)("missing build tools make the native build pending before anything is run, without a log and without a marker",()=>{
 const blocked=nativeStep({repoExit:0,toolchain:false});
 expect(blocked.code).toBe(3);
 expect(blocked.ran).toBe(false);
 expect(blocked.marker).toBe(false);
 expect(blocked.log).toBeUndefined();
 expect(blocked.out).toContain("failing command: the build tool check before bash");
 expect(blocked.out).toContain("reason: missing build tools: cc: install them as root first (apt-get install -y build-essential)");
 expect(blocked.out).toContain("exit code 3: fleet ready, native build pending");
});

(bash?test:test.skip)("missing build tools are reported with the exact packages of the package manager and fail the native build only, and macOS names the command line tools",()=>{
 const checked=(tools:readonly string[],body:string="check_toolchain")=>toolchain(tools,`${body} && echo "rc=0" || echo "rc=1 $TOOLCHAIN_PROBLEM"`);
 for(const [manager,line] of Object.entries({"apt-get":"apt-get install -y build-essential pkg-config curl git unzip",dnf:"dnf install -y gcc gcc-c++ make pkgconf-pkg-config curl git unzip",zypper:"zypper install -y gcc gcc-c++ make pkg-config curl git unzip",pacman:"pacman -S --noconfirm base-devel pkgconf curl git unzip"})){
  const refused=checked([...TOOLS.filter(tool=>tool!=="pkg-config"&&tool!=="unzip"),manager]);
  expect(refused.code,manager).toBe(0);
  expect(refused.out).toContain("rc=1 builder has no passwordless sudo");
  expect(refused.out).toContain("these tools are missing: pkg-config unzip");
  expect(refused.out).toContain(`(${line})`);
 }
 expect(checked(TOOLS).out).toContain("rc=0");
 const sudo=checked(TOOLS.filter(tool=>tool!=="make"),'run_as(){ return 0; }; check_toolchain');
 expect(sudo.out).toContain("rc=0");
 expect(sudo.out).toContain("missing build tools: make: the native bootstrap installs them");
 const unknown=checked([]);
 expect(unknown.out).toContain("the C/C++ toolchain (cc, c++, make), pkg-config, curl, git and unzip of your distribution");
 const mac=shellBlock("BOOTSTRAP_MARKER=","check_time_sync() {","PLATFORM=macos; check_toolchain && echo rc=0 || echo \"rc=1 $TOOLCHAIN_PROBLEM\"",["git","curl"]);
 expect(mac.out).toContain("rc=1 missing build tools: cc c++ make xcode-command-line-tools");
 expect(mac.out).toContain("xcode-select --install");
});

(bash?test:test.skip)("the clock check passes on a synchronized Linux machine and recommends chrony on one that is not",()=>{
 const run=(state:string)=>shellBlock("check_time_sync() {","say \"group $GROUP_ID",`PLATFORM=linux; check_time_sync`,["timedatectl","sleep"],`#!/bin/sh\ncase "$1" in show) echo ${state};; esac\n`);
 expect(run("yes").out).toContain("clock synchronized (NTPSynchronized=yes)");
 const bad=run("no").out;
 expect(bad).toContain("WARNING: the clock is not synchronized: install and enable chrony");
 expect(bad).toContain("more than 60 s off");
 expect(shellBlock("check_time_sync() {","say \"group $GROUP_ID","PLATFORM=linux; check_time_sync",[]).out).toContain("WARNING: timedatectl is missing");
 expect(provisionShell.indexOf('say "step 4b: clock"')).toBeGreaterThan(provisionShell.indexOf('say "step 4: system limits'));
});

const NOW=1_800_000_000_000,GIB=2**30;
const HEALTHY:StatusReportV1={version:1,originId:"a1-m01",groupId:"a1",storeRoot:"/mnt/semio-store",reachable:true,daemon:{running:true,pid:42},files:270000,queue:{incoming:0,outgoing:0},lost:0,refused:null,origins:[{origin:"a1-server",behindEvents:0,behindBytes:0,lastHlc:null,ageSeconds:1,rejoinRequired:false}]};
const NFS_OPTIONS="rw,vers=4.2,rsize=262144,wsize=262144,namlen=255,hard,proto=tcp,nconnect=4,timeo=600,retrans=2,sec=sys,acregmin=1,acregmax=1,acdirmin=1,acdirmax=1,lookupcache=none,local_lock=none";
const mountinfo=(options:string=NFS_OPTIONS,type:string="nfs4",source:string="nfs.a1.example.test:/srv/store-a",mount:string="/mnt/semio-store"):string=>`36 35 0:41 / ${mount} rw,noatime shared:7 - ${type} ${source} ${options}\n`;
const linuxFiles=(extra:Record<string,string>={},origin:string="a1-m01"):Record<string,string>=>({"/srv/semio/.cache/f/machine.json":JSON.stringify({originId:origin,groupId:"a1",storeRoot:"/mnt/semio-store"}),"/proc/self/mountinfo":mountinfo(),"/proc/sys/fs/inotify/max_user_watches":"1048576\n","/proc/sys/fs/inotify/max_user_instances":"1024\n",...extra});
const linuxCommands=(extra:Record<string,{code:number;stdout:string}>={})=>(command:string,args:readonly string[]):{code:number;stdout:string}=>{
 const key=`${command} ${args[0]??""}`;
 if(Object.hasOwn(extra,key))return extra[key]!;
 if(key==="systemctl list-units")return {code:0,stdout:`${args[1]} loaded active running semio-fleet\n`.replace("@*","@ueli").replace(/^[^ ]*/u,args[1]!.replace("@*","@ueli"))};
 if(key==="systemctl list-timers")return {code:0,stdout:`n/a n/a n/a n/a ${args.at(-1)!.replace("@*","@ueli")} semio-fleet\n`};
 if(command==="timedatectl")return {code:0,stdout:"yes\n"};
 return {code:127,stdout:""};
};
const fake=(files:Record<string,string>,commands:(command:string,args:readonly string[])=>{code:number;stdout:string},extra:Partial<PreflightProbesV1>={}):Partial<PreflightProbesV1>=>({
 read:async path=>files[path],run:async(command,args)=>commands(command,args),
 http:async()=>({status:200,body:'{"status":"ok"}',date:new Date(NOW).toUTCString()}),
 free:async()=>100*GIB,exists:async()=>true,write:async()=>{},now:()=>NOW,status:async()=>HEALTHY,...extra
});
const preflight=(group:string,platform:FleetPlatformV1,probes:Partial<PreflightProbesV1>,extra:Record<string,unknown>={})=>preflightV1(topology,{platform,group,timeoutMilliseconds:1000,languages:["en"],signal:new AbortController().signal,progress:()=>{},probes,...extra});
const statusOf=(report:Awaited<ReturnType<typeof preflight>>):Record<string,string>=>Object.fromEntries(report.checks.map(check=>[check.id,`${check.status}:${check.key}`]));
const LINUX_OK={identity:"ok:identity.ok",mount:"ok:mount.ok",options:"ok:options.ok",write:"ok:write.ok",cache:"ok:cache.ok",clock:"ok:clock.ok",timesync:"ok:timesync.ok",limits:"ok:limits.ok","disk.worktree":"ok:disk.ok","native-build":"ok:native.ok","service.replica":"ok:service.ok","service.compact":"ok:compact.ok",replica:"ok:replica.ok"};

test("the preflight of a healthy Linux worker passes every check in the fixed order and says what it measured",async()=>{
 const report=await preflight("a1","linux",fake(linuxFiles(),linuxCommands()));
 expect(report.ok).toBe(true);
 expect(report.cancelled).toBe(false);
 expect(statusOf(report)).toEqual(LINUX_OK);
 expect(report.checks.map(check=>check.id)).toEqual(Object.keys(LINUX_OK));
 expect(report.text).toContain("Preflight of a1-m01 (group a1, worker, linux)");
 expect(report.text).toContain("ok   store mount: /mnt/semio-store is nfs4 from nfs.a1.example.test:/srv/store-a");
 expect(report.text).toContain("ok   mount options: the correctness options are in effect: hard,lookupcache=none,acregmin=1,acregmax=1,acdirmin=1,acdirmax=1");
 expect(report.text).toContain("ok   clock skew: this clock is -0.5 s from the clock of a1-server");
 expect(report.text).toContain("All 13 checks passed.");
});

test("the native build check is ok with the completion marker of the provisioning and a warning, never a failure, while the bootstrap is pending",async()=>{
 const marker="/srv/semio/.cache/f/native-bootstrap.done",log="/srv/semio/.cache/f/native-bootstrap.log",asked:string[]=[];
 const pending=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{exists:async path=>{asked.push(path);return path!==marker;}}),{languages:["en","de"]});
 expect(asked).toContain(marker);
 expect(pending.ok).toBe(true);
 expect(statusOf(pending)).toEqual({...LINUX_OK,"native-build":"warn:native.pending"});
 expect(pending.text).toContain(`WARN native build: pending: the native bootstrap has not completed (${marker} is missing), so this machine syncs but cannot build yet; run the provisioning script again to retry it, the log is ${log}`);
 expect(pending.text).toContain("WARN Native Build: ausstehend: Der native Bootstrap ist nicht abgeschlossen");
 expect(pending.text).toContain("13 checks, 1 with warnings");
 const done=await preflight("a1","linux",fake(linuxFiles(),linuxCommands()));
 expect(done.text).toContain(`ok   native build: bootstrap complete (${marker})`);
 const windows=await preflight("c3","windows",fake({"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"c3-m01",groupId:"c3"})},()=>({code:0,stdout:""}),{exists:async path=>!path.endsWith("native-bootstrap.done")}),{worktree:"C:\\semio"});
 expect(statusOf(windows)["native-build"]).toBe("warn:native.pending");
});

test("the Linux mount is judged by its source and by the options in effect: lookupcache=pos, soft and a foreign source fail, an unmounted store fails",async()=>{
 const run=(files:Record<string,string>)=>preflight("a1","linux",fake(files,linuxCommands()));
 const positive=await run(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS.replace("lookupcache=none","lookupcache=pos"))}));
 expect(positive.ok).toBe(false);
 expect(statusOf(positive)["options"]).toBe("fail:options.wrong");
 expect(positive.checks.find(check=>check.id==="options")!.variables["wrong"]).toBe("lookupcache=none (found lookupcache=pos)");
 const soft=await run(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS.replace("hard","soft"))}));
 expect(soft.checks.find(check=>check.id==="options")!.variables["wrong"]).toBe("hard (found soft)");
 const slow=await run(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS.replace("acdirmin=1","acdirmin=30"))}));
 expect(slow.checks.find(check=>check.id==="options")!.variables["wrong"]).toBe("acdirmin=1 (found acdirmin=30)");
 const foreign=await run(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS,"nfs4","nfs.b2.example.test:/srv/store-b")}));
 expect(statusOf(foreign)["mount"]).toBe("fail:mount.source");
 expect(foreign.text).toContain("comes from nfs.b2.example.test:/srv/store-b, expected nfs.a1.example.test:/srv/store-a");
 const absent=await run(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS,"nfs4","x:/y","/mnt/other")}));
 expect(statusOf(absent)["mount"]).toBe("fail:mount.missing");
 expect(statusOf(absent)["options"]).toBe("skip:options.skip");
 const defaults=await run(linuxFiles({"/proc/self/mountinfo":mountinfo("rw,vers=4.2,hard,proto=tcp")}));
 expect(statusOf(defaults)["options"]).toBe("warn:options.unreadable");
});

test("a machine that has not joined, the wrong group, a store that cannot be written and a dead cache fail, and a missing group is refused",async()=>{
 const unjoined=await preflight("a1","linux",fake({...linuxFiles(),"/srv/semio/.cache/f/machine.json":undefined as unknown as string},linuxCommands()));
 expect(statusOf(unjoined)["identity"]).toBe("fail:identity.missing");
 const other=await preflight("b2","linux",fake(linuxFiles(),linuxCommands()));
 expect(statusOf(other)["identity"]).toBe("fail:identity.group");
 await expect(preflightV1(topology,{platform:"linux",timeoutMilliseconds:1000,languages:["en"],signal:new AbortController().signal,progress:()=>{},probes:fake({},linuxCommands())})).rejects.toThrow("No group");
 const unwritable=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{write:async()=>{throw Error("EROFS: read-only file system");}}));
 expect(statusOf(unwritable)["write"]).toBe("fail:write.failed");
 expect(unwritable.text).toContain("cannot write in /mnt/semio-store/v1/tmp: EROFS: read-only file system");
 const dead=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{http:async()=>{throw Error("connect ECONNREFUSED");}}));
 expect(statusOf(dead)["cache"]).toBe("fail:cache.unreachable");
 expect(statusOf(dead)["clock"]).toBe("skip:clock.unreachable");
 const sick=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{http:async()=>({status:503,body:"busy",date:null})}));
 expect(statusOf(sick)["cache"]).toBe("fail:cache.bad");
 expect(statusOf(sick)["clock"]).toBe("skip:clock.nodate");
 expect(dead.ok||sick.ok||unwritable.ok||other.ok||unjoined.ok).toBe(false);
});

test("the clock check warns above 2 s and fails from 30 s, in both directions, and reads the Date header of the group server",async()=>{
 const skewed=async(milliseconds:number)=>statusOf(await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{http:async()=>({status:200,body:'{"status":"ok"}',date:new Date(NOW+milliseconds).toUTCString()})})))["clock"];
 expect(PREFLIGHT_LIMITS_V1.skewWarnMilliseconds).toBe(2000);
 expect(PREFLIGHT_LIMITS_V1.skewFailMilliseconds).toBeLessThan(60000);
 expect(await skewed(-1000)).toBe("ok:clock.ok");
 expect(await skewed(-3000)).toBe("warn:clock.warn");
 expect(await skewed(3000)).toBe("warn:clock.warn");
 expect(await skewed(-29000)).toBe("warn:clock.warn");
 expect(await skewed(-31000)).toBe("fail:clock.fail");
 expect(await skewed(120000)).toBe("fail:clock.fail");
 const ahead=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{http:async()=>({status:200,body:'{"status":"ok"}',date:new Date(NOW-10000).toUTCString()})}));
 expect(ahead.text).toContain("this clock is +9.5 s from the clock of a1-server");
 expect(ahead.ok).toBe(true);
});

test("inotify limits, free disk, services, compaction, time synchronisation and the replica status each judge their own thing",async()=>{
 const run=(files:Record<string,string>,commands:Record<string,{code:number;stdout:string}>,extra:Partial<PreflightProbesV1>={})=>preflight("a1","linux",fake(linuxFiles(files),linuxCommands(commands),extra)).then(statusOf);
 expect((await run({"/proc/sys/fs/inotify/max_user_watches":"8192\n"},{}))["limits"]).toBe("fail:limits.low");
 expect((await run({"/proc/sys/fs/inotify/max_user_instances":"128\n"},{}))["limits"]).toBe("fail:limits.low");
 expect((await run({"/proc/sys/fs/inotify/max_user_watches":""},{}))["limits"]).toBe("fail:limits.low");
 expect((await run({},{},{free:async()=>20*GIB}))["disk.worktree"]).toBe("warn:disk.warn");
 expect((await run({},{},{free:async()=>5*GIB}))["disk.worktree"]).toBe("fail:disk.fail");
 expect((await run({},{"systemctl list-units":{code:0,stdout:""}}))["service.replica"]).toBe("fail:service.down");
 expect((await run({},{"systemctl list-timers":{code:0,stdout:""}}))["service.compact"]).toBe("warn:compact.missing");
 expect((await run({},{"timedatectl show":{code:0,stdout:"no\n"}}))["timesync"]).toBe("warn:timesync.no");
 expect((await run({},{"timedatectl show":{code:127,stdout:""}}))["timesync"]).toBe("warn:timesync.unknown");
 expect((await run({},{},{status:async()=>({...HEALTHY,reachable:false})}))["replica"]).toBe("fail:replica.unreachable");
 expect((await run({},{},{status:async()=>({...HEALTHY,daemon:{running:false,pid:null}})}))["replica"]).toBe("fail:replica.stopped");
 expect((await run({},{},{status:async()=>({...HEALTHY,origins:[{...HEALTHY.origins[0],rejoinRequired:true}]})}))["replica"]).toBe("fail:replica.rejoin");
 expect((await run({},{},{status:async()=>({...HEALTHY,origins:[{...HEALTHY.origins[0],behindEvents:PREFLIGHT_LIMITS_V1.lagEvents+1}]})}))["replica"]).toBe("warn:replica.behind");
 expect((await run({},{},{status:async()=>{throw Error("replica status exited with 1");}}))["replica"]).toBe("fail:check.error");
});

test("a group server holds its store on a local disk, is the clock reference, and also has its cache and relay and its store volume checked",async()=>{
 const local=mountinfo("rw","ext4","/dev/nvme0n1p2");
 const report=await preflight("a1","linux",fake(linuxFiles({"/proc/self/mountinfo":local},"a1-server"),linuxCommands(),{free:async path=>path==="/mnt/semio-store"?400*GIB:100*GIB}));
 expect(report.ok).toBe(true);
 expect(statusOf(report)).toEqual({...LINUX_OK,mount:"ok:mount.local",options:"skip:options.skip",clock:"skip:clock.reference","disk.store":"ok:disk.ok","service.cache":"ok:service.ok","service.relay":"ok:service.ok"});
 const remote=await preflight("a1","linux",fake(linuxFiles({},"a1-server"),linuxCommands()));
 expect(statusOf(remote)["mount"]).toBe("fail:mount.remote");
 const small=await preflight("a1","linux",fake(linuxFiles({"/proc/self/mountinfo":local},"a1-server"),linuxCommands(),{free:async path=>path==="/mnt/semio-store"?200*GIB:100*GIB}));
 expect(statusOf(small)["disk.store"]).toBe("warn:disk.warn");
 const down=await preflight("a1","linux",fake(linuxFiles({"/proc/self/mountinfo":local},"a1-server"),linuxCommands({"systemctl list-units":{code:0,stdout:""}})));
 expect(down.checks.filter(check=>check.status==="fail").map(check=>check.id)).toEqual(["service.replica","service.cache","service.relay"]);
});

test("a macOS worker is judged through mount and nfsstat, launchd and systemsetup; options that nfsstat does not show are a warning, not a failure",async()=>{
 const flags="/Volumes/semio-store from nfs.b2.example.test:/Users/Shared/store-b\n -- Current mount parameters:\n    NFS parameters: vers=3,tcp,hard,nolocks,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache\n";
 const commands=(listing:string,nfsstat:string,network:string="Network Time: On\n")=>(command:string,args:readonly string[]):{code:number;stdout:string}=>command==="/sbin/mount"?{code:0,stdout:listing}:command==="/usr/bin/nfsstat"?{code:nfsstat===""?1:0,stdout:nfsstat}:command==="launchctl"?{code:0,stdout:args[1]==="system/tech.semio.fleet.replica"?"state = running\n":"state = waiting\n"}:command==="/usr/sbin/systemsetup"?{code:0,stdout:network}:{code:127,stdout:""};
 const listing="nfs.b2.example.test:/Users/Shared/store-b on /Volumes/semio-store (nfs, nodev, nosuid, mounted by op)\n";
 const files={"/Users/Shared/semio/.cache/f/machine.json":JSON.stringify({originId:"b2-m01",groupId:"b2",storeRoot:"/Volumes/semio-store"})};
 const good=await preflight("b2","macos",fake(files,commands(listing,flags)));
 expect(good.ok).toBe(true);
 expect(statusOf(good)).toMatchObject({mount:"ok:mount.ok",options:"ok:options.ok",timesync:"ok:timesync.ok","service.replica":"ok:service.ok","service.compact":"ok:compact.ok"});
 expect(Object.keys(statusOf(good))).not.toContain("limits");
 const hidden=await preflight("b2","macos",fake(files,commands(listing,"")));
 expect(statusOf(hidden)["options"]).toBe("warn:options.unreadable");
 expect(hidden.ok).toBe(true);
 const wrong=await preflight("b2","macos",fake(files,commands(listing,flags.replace("acdirmin=0","acdirmin=30"))));
 expect(statusOf(wrong)["options"]).toBe("fail:options.wrong");
 const off=await preflight("b2","macos",fake(files,commands(listing,flags,"Network Time: Off\n")));
 expect(statusOf(off)["timesync"]).toBe("warn:timesync.no");
 const stopped=await preflight("b2","macos",fake(files,(command,args)=>command==="launchctl"?{code:113,stdout:""}:commands(listing,flags)(command,args)));
 expect(statusOf(stopped)["service.replica"]).toBe("fail:service.down");
});

test("a Windows worker is judged through net use, the registry, schtasks and w32tm: long paths, the NFS client values and the SMB cache lifetimes",async()=>{
 const registry=(key:string,name:string,value:number):string=>`\n${key}\n    ${name}    REG_DWORD    0x${value.toString(16)}\n`;
 const commands=(overrides:Record<string,{code:number;stdout:string}>={},nfs:number=0,longPaths:number=1)=>(command:string,args:readonly string[]):{code:number;stdout:string}=>{
  const key=`${command} ${args[0]??""}`;
  if(Object.hasOwn(overrides,key))return overrides[key]!;
  if(command==="net.exe")return {code:0,stdout:"New connections will be remembered.\n\nStatus       Local     Remote                    Network\n-------------------------------------------------------------------------------\nOK           S:        \\\\nfs.a1.example.test\\srv\\store-a   NFS Network\nThe command completed successfully.\n"};
  if(command==="reg.exe")return {code:0,stdout:args[1]!.includes("FileSystem")?registry("FileSystem","LongPathsEnabled",longPaths):registry("Default",args[3]!,nfs)};
  if(command==="schtasks.exe")return {code:0,stdout:args[2]==="semio-fleet-replica"?"Status:    Running\n":"Status:    Ready\n"};
  if(command==="w32tm.exe")return {code:0,stdout:"Leap Indicator: 0\nSource: time.windows.com\n"};
  return {code:127,stdout:""};
 };
 const files={"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"a1-m01",groupId:"a1",storeRoot:"S:\\"})},windows=(extra:Record<string,{code:number;stdout:string}>={},nfs?:number,longPaths?:number)=>preflight("a1","windows",fake(files,commands(extra,nfs,longPaths),{status:async()=>HEALTHY}));
 const good=await windows();
 expect(good.ok).toBe(true);
 expect(statusOf(good)).toMatchObject({mount:"ok:mount.ok",options:"skip:options.skip",longpaths:"ok:longpaths.ok",clientcache:"ok:clientcache.nfs",timesync:"ok:timesync.ok","service.replica":"ok:service.ok","service.compact":"ok:compact.ok"});
 expect(good.text).toContain("not verified on a Windows machine");
 expect(statusOf(await windows({},1))["clientcache"]).toBe("fail:clientcache.wrong");
 expect(statusOf(await windows({},0,0))["longpaths"]).toBe("fail:longpaths.off");
 expect(statusOf(await windows({"w32tm.exe /query":{code:0,stdout:"Source: Local CMOS Clock\n"}}))["timesync"]).toBe("warn:timesync.no");
 expect(statusOf(await windows({"w32tm.exe /query":{code:1,stdout:"The service has not been started."}}))["timesync"]).toBe("warn:timesync.no");
 expect(statusOf(await windows({"net.exe use":{code:0,stdout:"There are no entries in the list.\n"}}))["mount"]).toBe("fail:mount.missing");
 const smb=readTopologyV1(variantSmb());
 const smbFiles={"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"c3-m01",groupId:"c3",storeRoot:"S:\\"})};
 const smbRun=(json:string)=>preflightV1(smb,{platform:"windows",group:"c3",timeoutMilliseconds:1000,languages:["en"],signal:new AbortController().signal,progress:()=>{},probes:fake(smbFiles,(command,args)=>command==="net.exe"?{code:0,stdout:"OK           S:        \\\\nfs.c3.example.test\\store-c   Microsoft Windows Network\n"}:command==="powershell.exe"?{code:0,stdout:json}:commands()(command,args),{status:async()=>({...HEALTHY,groupId:"c3"})})});
 expect(statusOf(await smbRun('{"DirectoryCacheLifetime":0,"FileInfoCacheLifetime":0,"FileNotFoundCacheLifetime":0}'))).toMatchObject({mount:"ok:mount.ok",clientcache:"ok:clientcache.smb"});
 const stale=await smbRun('{"DirectoryCacheLifetime":10,"FileInfoCacheLifetime":0,"FileNotFoundCacheLifetime":5}');
 expect(statusOf(stale)["clientcache"]).toBe("fail:clientcache.wrong");
 expect(stale.checks.find(check=>check.id==="clientcache")!.variables["wrong"]).toBe("DirectoryCacheLifetime=10, FileNotFoundCacheLifetime=5");
});
function variantSmb():unknown{const copy=structuredClone(fixture.topology) as any;copy.groups[1].server={platform:"linux",protocol:"nfs",network:"10.2.0.0/16"};copy.groups[1].nfs.export="/srv/store-b";copy.groups[2].server.protocol="smb";return copy;}

test("every finished check is reported as progress in every chosen language, a silent probe times out, and cancellation stops the rest",async()=>{
 const lines:string[]=[],both=await preflightV1(topology,{platform:"linux",group:"a1",timeoutMilliseconds:1000,languages:["en","de"],signal:new AbortController().signal,progress:line=>lines.push(line),probes:fake(linuxFiles(),linuxCommands())});
 expect(lines.length).toBe(both.checks.length*2);
 expect(lines.filter(line=>/^\[ ?\d+\/13\] ok   Speicher-Einbindung: /u.test(line)).length).toBe(1);
 expect(lines.filter(line=>/^\[ ?\d+\/13\] ok   store mount: /u.test(line)).length).toBe(1);
 expect(both.text).toContain("Preflight von a1-m01 (Gruppe a1, Worker, linux)");
 expect(both.text).toContain("Alle 13 Prüfungen bestanden.");
 expect(both.text.split("\n\n").length).toBe(2);
 const silent=await preflightV1(topology,{platform:"linux",group:"a1",timeoutMilliseconds:100,languages:["en"],signal:new AbortController().signal,progress:()=>{},probes:fake(linuxFiles(),linuxCommands(),{write:()=>new Promise(()=>{})})});
 expect(statusOf(silent)["write"]).toBe("fail:check.error");
 expect(silent.text).toContain("write timed out after 100 ms");
 const controller=new AbortController(),seen:string[]=[],cancelled=preflightV1(topology,{platform:"linux",group:"a1",timeoutMilliseconds:5000,languages:["en"],signal:controller.signal,progress:line=>{seen.push(line);if(seen.length===3)controller.abort();},probes:fake(linuxFiles(),linuxCommands(),{http:()=>new Promise(()=>{}),write:()=>new Promise(()=>{})})});
 const report=await cancelled;
 expect(report.cancelled).toBe(true);
 expect(report.ok).toBe(false);
 expect(report.checks.length).toBeLessThan(13);
 expect(report.checks.length).toBeGreaterThanOrEqual(3);
 expect(report.text).toContain("Preflight cancelled after");
 expect(seen.length).toBe(report.checks.length);
});

test("the preflight command takes its closed options, picks its languages, needs the asynchronous runner and exits 1 on a failure",async()=>{
 expect(parseTopologyArgumentsV1(["preflight","--group","a1","--timeout","2000","--lang","de","--worktree","/x","--store","/y"]).operation).toBe("preflight");
 expect(()=>parseTopologyArgumentsV1(["preflight","--reachable"])).toThrow("Unknown option for preflight");
 expect(()=>runTopologyCommandV1(["preflight","--group","a1"],{topology:fixture.topology})).toThrow("asynchronous");
 const context=(probes:Partial<PreflightProbesV1>,environment:Record<string,string>={})=>({platform:"linux",topology:fixture.topology,signal:new AbortController().signal,progress:()=>{},environment,probes});
 const good=await runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","de"],context(fake(linuxFiles(),linuxCommands())));
 expect(good.exitCode).toBe(0);
 expect(good.text).toContain("Alle 13 Prüfungen bestanden.");
 expect(good.text).not.toContain("All 13 checks passed.");
 const english=await runTopologyCommandAsyncV1(["preflight","--lang","en"],context(fake(linuxFiles(),linuxCommands())));
 expect(english.text).toContain("All 13 checks passed.");
 const bad=await runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","en"],context(fake(linuxFiles({"/proc/self/mountinfo":""}),linuxCommands())));
 expect(bad.exitCode).toBe(1);
 expect(bad.text).toContain("fix the failures before the machine joins the fleet");
 await expect(runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","fr"],context(fake(linuxFiles(),linuxCommands())))).rejects.toThrow("Unknown language: fr");
 await expect(runTopologyCommandAsyncV1(["preflight","--group","a1","--timeout","5"],context(fake(linuxFiles(),linuxCommands())))).rejects.toThrow("--timeout must be a whole number");
 expect(preflightLanguagesV1(undefined,{}).length).toBe(2);
 expect(preflightLanguagesV1("en",{})).toEqual(["en"]);
});

test("the preflight catalogue has the same variables in English and German for every message and a name for every check",()=>{
 const folder=join(import.meta.dirname,"..",readdirSync(join(import.meta.dirname,"..")).find(name=>name.endsWith("texts"))!),catalogue=JSON.parse(readFileSync(join(folder,readdirSync(folder)[0]!),"utf8")) as {languages:string[];messages:Record<string,Record<string,string>>};
 expect(catalogue.languages).toEqual(["en","de"]);
 const variables=(text:string):string=>[...text.matchAll(/\{([a-z0-9]+)\}/gu)].map(match=>match[1]).sort().join();
 for(const [key,message] of Object.entries(catalogue.messages)){
  expect(Object.keys(message).sort(),key).toEqual(["de","en"]);
  expect(message["en"]!.length&&message["de"]!.length,key).toBeGreaterThan(0);
  expect(variables(message["en"]!),key).toBe(variables(message["de"]!));
  expect(()=>preflightTextV1("de",key,Object.fromEntries(variables(message["de"]!).split(",").filter(Boolean).map(name=>[name,"x"]))),key).not.toThrow();
 }
 expect(()=>preflightTextV1("en","preflight.nope")).toThrow("Declared preflight message required");
 expect(()=>preflightTextV1("en","preflight.mount.ok",{})).toThrow("variable missing");
 for(const id of Object.keys(LINUX_OK).concat(["disk.store","service.cache","service.relay","clientcache","longpaths"]))expect(catalogue.messages[`preflight.name.${id}`],id).toBeDefined();
});
