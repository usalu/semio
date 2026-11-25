import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {createServer} from "node:net";
import {spawnSync} from "node:child_process";
import {join,resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import committed from "../🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import buildPolicy from "../../../📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {applyFleetAddressEnvironmentV1,peerStorePathV1,clientProtocolV1,factsV1,worktreePathV1,groupV1,mountPartsV1,mountPlanV1,platformV1,readTopologyV1,serverPartsV1,serverPlanV1,windowsSuperviseScriptV1,workerPartsV1,workerServiceV1,type FleetGroupV1,type FleetPlatformV1,type FleetTopologyV1} from "../🟦️.ts";
import {checkReachabilityV1,parseTopologyArgumentsV1,preflightLanguagesV1,preflightTextV1,preflightV1,PREFLIGHT_LIMITS_V1,renderFactsV1,runTopologyCommandAsyncV1,runTopologyCommandV1,type PreflightProbesV1,type ReachabilityPortsV1} from "../⌨️command/🟦️.ts";
import {gitPinsHashV1,type GitGuardsV1} from "../../🌿️git/🟦️.ts";
import {accessDirectoryV1} from "../../../📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.mjs";
import {parseReplicaArgumentsV1} from "../../🔁️replica/⌨️command/🟦️.ts";
import type {StatusReportV1} from "../../🔁️replica/🟦️.ts";
import {runTopologyScriptV1} from "../📜️script.ts";

type Change={name:string;path:(string|number)[];value?:unknown;remove?:boolean};

const hostedScript=(xml:string):string=>Buffer.from(/-EncodedCommand ([A-Za-z0-9+\/=]+)/u.exec(xml)![1]!,"base64").toString("utf16le");
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

test("the committed topology is accepted by Ajv and by the repository validator, g1 is the macOS group server, and every host, network and cache name stays a placeholder",()=>{
 expect(accepts(committed)).toBe(true);
 expect(validateJsonSchemaSubset({$ref:"#/definitions/Topology"},committed,schema)).toEqual([]);
 const read=readTopologyV1();
 expect(read.groups.map(group=>group.id)).toEqual(["g1","g2","g3","g4","g5"]);
 const g1=read.groups[0]!;
 expect(g1.nfs).toEqual({host:"nfs.g1.semio.internal",export:"/Users/Shared/semio-store"});
 expect(g1.server).toEqual({platform:"macos",protocol:"nfs",network:"10.0.1.0/24"});
 expect(g1.cache).toEqual({endpoint:"http://cache.g1.semio.internal:6094",instance:"main"});
 for(const group of read.groups.slice(1)){
  expect(group.nfs).toEqual({host:`nfs.${group.id}.semio.internal`,export:"/srv/semio-store"});
  expect(group.server).toEqual({platform:"linux",protocol:"nfs",network:`10.0.${group.id.slice(1)}.0/24`});
  expect(group.cache).toEqual({endpoint:`http://cache.${group.id}.semio.internal:6094`,instance:"main"});
 }
 for(const group of read.groups){
  expect(group.peers).toEqual(Object.fromEntries(read.groups.filter(other=>other.id!==group.id).map(other=>[other.id,`/mnt/semio-peers/${other.id}`])));
 }
 expect(read.worktree).toEqual({linux:"~/semio",macos:"~/Documents/semio",windows:"C:\\semio"});
 expect(read.storeMount).toEqual({linux:"/mnt/semio-store",macos:"/Volumes/semio-store",windows:"S:\\"});
 expect(read.replica).toEqual({pollMilliseconds:250,scanSeconds:90,debounceMilliseconds:150,retentionDays:14,bootstrapMinutes:60,compaction:{automatic:false,snapshotEvents:100000,safetyHours:3,staleHours:12,catchUpEvents:20000}});
});

test("a group whose cache instance differs from the policy namespace is refused, in both languages, and the committed topology names the policy instance",()=>{
 const policy=buildPolicy.remoteCache.defaults.instance,copy=JSON.parse(JSON.stringify(committed));
 expect(readTopologyV1().groups.map(group=>group.cache.instance)).toEqual(readTopologyV1().groups.map(()=>policy));
 copy.groups[0].cache.instance="semio";
 expect(()=>readTopologyV1(copy)).toThrow(new RegExp(`g1 needs cache.instance to be the policy instance ${policy}.*braucht cache.instance`,"u"));
});

test("address environment variables replace one group's host, network and cache endpoint, the file stays unchanged, and the Python oracle agrees",()=>{
 const environment={SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_NFS_HOST:"203.0.113.10",SEMIO_FLEET_NETWORK:"203.0.113.0/24"};
 const overlaid=applyFleetAddressEnvironmentV1(readTopologyV1(),environment),group=overlaid.groups[0]!;
 expect(group.nfs.host).toBe("203.0.113.10");
 expect(group.server.network).toBe("203.0.113.0/24");
 expect(group.cache.endpoint).toBe("http://203.0.113.10:6094");
 expect(overlaid.groups[1]!.nfs.host).toBe("nfs.g2.semio.internal");
 expect(readTopologyV1().groups[0]!.nfs.host).toBe("nfs.g1.semio.internal");
 expect(()=>applyFleetAddressEnvironmentV1(readTopologyV1(),{SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_NFS_HOST:"203.0.113.10"})).toThrow("set together");
 expect(applyFleetAddressEnvironmentV1(readTopologyV1(),{SEMIO_FLEET_GROUP:"zz",SEMIO_FLEET_NFS_HOST:"203.0.113.10",SEMIO_FLEET_NETWORK:"203.0.113.0/24"}).groups[0]!.nfs.host).toBe("nfs.g1.semio.internal");
 expect(runTopologyCommandV1(["server-plan","--group","g1","--part","exports"],{platform:"darwin",environment})).toContain("-network 203.0.113.0 -mask 255.255.255.0");
 expect(runTopologyCommandV1(["facts","--group","g1","--platform","macos","--shell","sh"],{platform:"darwin",environment:{}})).toContain("FLEET_NFS_HOST='nfs.g1.semio.internal'");
 expect(provisionShell).toContain("SEMIO_FLEET_NFS_HOST");
 expect(provisionShell).toContain("SEMIO_FLEET_GROUP, SEMIO_FLEET_NFS_HOST and SEMIO_FLEET_NETWORK are set together");
 if(!python)return;
 const topologyFile=join(import.meta.dirname,"..",readdirSync(join(import.meta.dirname,"..")).find(name=>name.endsWith(".json"))!);
 const result=Bun.spawnSync([python,"-I",oracle,"address",topologyFile],{env:{...process.env,...environment},stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 expect(JSON.parse(result.stdout.toString())).toEqual({endpoint:"http://203.0.113.10:6094",host:"203.0.113.10",network:"203.0.113.0/24"});
 const partial=Bun.spawnSync([python,"-I",oracle,"address",topologyFile],{env:{...process.env,SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_NFS_HOST:"203.0.113.10",SEMIO_FLEET_NETWORK:""},stderr:"pipe"});
 expect(partial.exitCode).toBe(1);
 expect(partial.stderr.toString()).toContain("set together");
});

test("unresolved peer hosts are left out of the macOS exports before nfsd checkexports, and the Python oracle writes the same exports",()=>{
 const sample=["/Users/Shared/semio-store -mapall=ueli -network 203.0.113.0 -mask 255.255.255.0","/Users/Shared/semio-store -ro -mapall=ueli nfs.g2.semio.internal nfs.g3.semio.internal peer.example","# kept","/Users/Shared/semio-store -ro -mapall=ueli missing.example",""].join("\n");
 const root=mkdtempSync(join(tmpdir(),"semio-exports-")),file=join(root,"exports"),original=join(root,"original"),seen=join(root,"seen");
 writeFileSync(original,sample);writeFileSync(file,sample);
 const filter=provisionShell.slice(provisionShell.indexOf("filter_unresolved_exports() ("),provisionShell.indexOf("\nCLEANUP_MOUNT"));
 const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf '%s\\n' "$*"; }\nfail(){ printf '%s\\n' "$*" >&2; exit 1; }\nhost_resolves(){ printf '%s\\n' "$1" >> "$SEEN"; case ",peer.example," in *,"$1",*) return 0;; *) return 1;; esac; }\n${filter}\nfilter_unresolved_exports "$1"`, "filter", file],{env:{...process.env,SEEN:seen},stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()+run.stdout.toString()).toBe(0);
 const filtered=readFileSync(file,"utf8");
 const oracleRun=Bun.spawnSync([python!,"-I",oracle,"exports",original,"peer.example"],{stdout:"pipe",stderr:"pipe"});
 expect(oracleRun.exitCode,oracleRun.stderr.toString()).toBe(0);
 expect(filtered).toBe(oracleRun.stdout.toString());
 expect(filtered).toBe("/Users/Shared/semio-store -mapall=ueli -network 203.0.113.0 -mask 255.255.255.0\n/Users/Shared/semio-store -ro -mapall=ueli peer.example\n# kept\n");
 expect(readFileSync(seen,"utf8")).toBe("nfs.g2.semio.internal\nnfs.g3.semio.internal\npeer.example\nmissing.example\n");
 expect(run.stdout.toString()).toContain("WARNING: nfs.g2.semio.internal does not resolve");
 expect(run.stdout.toString()).toContain("missing.example has no reachable client, so the export line was left out");
 expect(provisionShell.indexOf("filter_unresolved_exports /etc/exports")).toBeLessThan(provisionShell.indexOf("nfsd checkexports"));
 expect(provisionShell).toContain('"SEMIO_WORKSPACE_ROOT=$WORKTREE"');
 expect(provisionPowerShell).toContain("$env:SEMIO_WORKSPACE_ROOT = $Worktree");
 const runner=provisionShell.slice(provisionShell.indexOf("run_as() {"),provisionShell.indexOf("\nfleet() {"));
 const rooted=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nWORKER_USER=root\nWORKER_HOME=/tmp\nLANGUAGE_CHOICE=en\nWORKTREE=/tmp/fleet-root\n${runner}\nrun_as /usr/bin/printenv SEMIO_WORKSPACE_ROOT`],{stdout:"pipe",stderr:"pipe"});
 expect(rooted.exitCode,rooted.stderr.toString()).toBe(0);
 expect(rooted.stdout.toString().trim()).toBe("/tmp/fleet-root");
 const guardBody=provisionShell.slice(provisionShell.indexOf("skew_message() {"),provisionShell.indexOf("\nfleet() {")),fleetBody=guardBody+provisionShell.slice(provisionShell.indexOf("\nfleet() {"),provisionShell.indexOf("\nreplica() {"));
 const invoked=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nmkdir -p /tmp/fleet-root\nWORKTREE=/tmp/fleet-root\nEXIT_FLEET_CODE_OLD=5\nSKEW_MARK=/tmp/fleet-root/mark\nTOPOLOGY_RELATIVE=topo\nrun_as(){ printf '%s\\n' "$PWD"; printf '%s\\n' "$2"; }\n${fleetBody}\nfleet facts\n`],{stdout:"pipe",stderr:"pipe"});
 expect(invoked.exitCode,invoked.stderr.toString()).toBe(0);
 const [directory,script]=invoked.stdout.toString().trim().split("\n");
 expect(directory).toBe("/tmp/fleet-root");
 expect(script.startsWith("/tmp/fleet-root/topo/")).toBe(true);
 expect(script.endsWith("script.ts")).toBe(true);
 rmSync(root,{recursive:true,force:true});
});

test("a refused launchctl setenv stays a warning and the LaunchDaemon plist carries the fleet environment, and the Python oracle writes the same plist",()=>{
 const sample=["<plist>","<key>EnvironmentVariables</key><dict><key>HOME</key><string>h</string></dict>","<key>EnvironmentVariables</key><dict>untouched</dict>","</plist>",""].join("\n");
 const environmentText=["export SEMIO_FLEET_GROUP=g1","export SEMIO_FLEET_NFS_HOST=203.0.113.10",'export SEMIO_NOTE=a&b<c>d"e',""].join("\n");
 const root=mkdtempSync(join(tmpdir(),"semio-plist-")),plistFile=join(root,"daemon.plist"),environmentFile=join(root,"environment");
 writeFileSync(plistFile,sample);writeFileSync(environmentFile,environmentText);
 const functions=provisionShell.slice(provisionShell.indexOf("xml_escape() {"),provisionShell.indexOf('\nsay "step 5: machine environment (the cache endpoint comes from the fleet facts, never from a profile)"'));
 const rendered=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nWORKTREE=/Users/test/Documents/semio\nLANGUAGE_CHOICE=en\nENVIRONMENT=$(cat "$ENVIRONMENT_FILE")\n${functions}\nplist_with_environment < "$PLIST_FILE"`],{env:{...process.env,ENVIRONMENT_FILE:environmentFile,PLIST_FILE:plistFile},stdout:"pipe",stderr:"pipe"});
 expect(rendered.exitCode,rendered.stderr.toString()).toBe(0);
 const oracleRun=Bun.spawnSync([python!,"-I",oracle,"plist",plistFile,environmentFile,"/Users/test/Documents/semio","en"],{stdout:"pipe",stderr:"pipe"});
 expect(oracleRun.exitCode,oracleRun.stderr.toString()).toBe(0);
 expect(rendered.stdout.toString()).toBe(oracleRun.stdout.toString());
 const plist=rendered.stdout.toString();
 expect(plist).toContain("<key>SEMIO_WORKSPACE_ROOT</key><string>/Users/test/Documents/semio</string><key>SEMIO_LANGUAGE</key><string>en</string><key>SEMIO_FLEET_GROUP</key><string>g1</string>");
 expect(plist).toContain("<key>SEMIO_NOTE</key><string>a&amp;b&lt;c&gt;d&quot;e</string>");
 expect(plist).toContain("<key>EnvironmentVariables</key><dict>untouched</dict>");
 expect(plist.indexOf("SEMIO_WORKSPACE_ROOT")).toBeLessThan(plist.indexOf("<key>HOME</key>"));
 const refusal=provisionShell.slice(provisionShell.indexOf('  while IFS= read -r line; do\n    [ -n "$line" ] || continue\n    assignment='),provisionShell.indexOf('\nfi\n\nsay "step 6:'));
 const refused=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nlaunchctl(){ printf 'SET %s=%s\\n' "$2" "$3"; return 150; }\nwrite_block(){ printf 'BLOCK %s\\n' "$1"; }\nENVIRONMENT=$'export SEMIO_FLEET_GROUP=g1\\nexport SEMIO_FLEET_NFS_HOST=203.0.113.10'\n${refusal}`],{stdout:"pipe",stderr:"pipe"});
 expect(refused.exitCode,refused.stderr.toString()+refused.stdout.toString()).toBe(0);
 const lines=refused.stdout.toString().trim().split("\n");
 expect(lines).toEqual(["SET SEMIO_FLEET_GROUP=g1","SAY WARNING: launchctl setenv SEMIO_FLEET_GROUP was refused: System Integrity Protection does not allow it. The variable is in /etc/zshenv, /etc/profile and the LaunchDaemon environment","SET SEMIO_FLEET_NFS_HOST=203.0.113.10","SAY WARNING: launchctl setenv SEMIO_FLEET_NFS_HOST was refused: System Integrity Protection does not allow it. The variable is in /etc/zshenv, /etc/profile and the LaunchDaemon environment","BLOCK /etc/zshenv","BLOCK /etc/profile"]);
 for(const line of ["install_agent \"$SERVICE_TARGET\"","install_agent \"$COMPACT_TARGET\"","install_agent \"$CACHE_TARGET\"","install_agent \"$RELAY_TARGET\""])expect(provisionShell).toContain(`${line} "$(`);
 expect(provisionShell.split("| plist_with_environment)").length-1).toBe(4);
 rmSync(root,{recursive:true,force:true});
});

test("macOS launchd jobs get a log file the service account can write before they load, and the Python oracle names the same logs",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-logs-")),file=join(root,"daemon.plist");
 const plist=["<plist>",`<key>StandardOutPath</key><string>${root}/semio-fleet-cache.log</string>`,`<key>StandardErrorPath</key><string>${root}/semio-fleet-cache.log</string>`,`<key>StandardOutPath</key><string>${root}/semio-fleet-relay.log</string>`,"</plist>",""].join("\n");
 writeFileSync(file,plist);
 const body=provisionShell.slice(provisionShell.indexOf("ensure_daemon_logs() {"),provisionShell.indexOf("\ninstall_daemon() {"));
 const loader=provisionShell.slice(provisionShell.indexOf("load_daemon() {"),provisionShell.indexOf("\ninstall_nfs_server() {"));
 const created=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nmkdir(){ :; }\ninstall(){ printf 'INSTALL %s\\n' "$6"; }\nchown(){ printf 'CHOWN %s\\n' "$*"; }\nchmod(){ :; }\nWORKER_USER=ueli\n${body}\nensure_daemon_logs "$1"`,"logs",file],{stdout:"pipe",stderr:"pipe"});
 expect(created.exitCode,created.stderr.toString()+created.stdout.toString()).toBe(0);
 const oracleRun=Bun.spawnSync([python!,"-I",oracle,"logs",file],{stdout:"pipe",stderr:"pipe"});
 expect(oracleRun.exitCode,oracleRun.stderr.toString()).toBe(0);
 const installed=created.stdout.toString().split("\n").filter(line=>line.startsWith("INSTALL ")).map(line=>line.slice(8));
 expect(installed).toEqual(oracleRun.stdout.toString().trim().split("\n"));
 expect(installed).toEqual([`${root}/semio-fleet-cache.log`,`${root}/semio-fleet-relay.log`]);
 const present=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nlaunchctl(){ if [ "$1" = print ]; then return 0; fi; printf 'KICK %s\\n' "$3"; }\nmkdir(){ :; }\ninstall(){ printf 'INSTALL %s\\n' "$6"; }\nchown(){ :; }\nchmod(){ :; }\nWORKER_USER=ueli\n${body}\n${loader}\nload_daemon "$1"`,"logs",file],{stdout:"pipe",stderr:"pipe"});
 expect(present.exitCode,present.stderr.toString()+present.stdout.toString()).toBe(0);
 expect(present.stdout.toString()).toContain(`SAY service log ${root}/semio-fleet-cache.log`);
 expect(present.stdout.toString()).toContain("KICK system/daemon");
 expect(present.stdout.toString()).toContain("SAY already loaded daemon");
 rmSync(root,{recursive:true,force:true});
});

test("a node_modules symlink that leaves the worktree is removed and the Python oracle makes the same decision",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-namespace-")),inside=join(root,"inside"),outside=mkdtempSync(join(tmpdir(),"semio-outside-"));
 mkdirSync(inside,{recursive:true});
 const body=provisionShell.slice(provisionShell.indexOf("release_foreign_namespace() {"),provisionShell.indexOf('\nrelease_foreign_namespace "$WORKTREE/node_modules"'));
 const decide=(link:string)=>{
  const oracleRun=Bun.spawnSync([python!,"-I",oracle,"namespace",root,link],{stdout:"pipe",stderr:"pipe"});
  expect(oracleRun.exitCode,oracleRun.stderr.toString()).toBe(0);
  const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\n${body}\nrelease_foreign_namespace "$1" "$2"\nif [ -e "$1" ]; then printf 'keep\\n'; else printf 'remove\\n'; fi`,"namespace",link,root],{stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()+run.stdout.toString()).toBe(0);
  const decision=run.stdout.toString().trim().split("\n").at(-1);
  expect(decision).toBe(oracleRun.stdout.toString().trim());
  return decision;
 };
 const foreign=join(root,"node_modules");
 symlinkSync(outside, foreign);
 expect(decide(foreign)).toBe("remove");
 expect(existsSync(foreign)).toBe(false);
 symlinkSync(inside, foreign);
 expect(decide(foreign)).toBe("keep");
 rmSync(foreign);
 mkdirSync(foreign);
 expect(decide(foreign)).toBe("keep");
 rmSync(root,{recursive:true,force:true});
 rmSync(outside,{recursive:true,force:true});
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

test("no provisioning path writes a cache pointer: the env operation is gone and neither script sets SEMIO_REMOTE_CACHE_ENDPOINT, _ACCESS or _INSTANCE; the fleet facts are the source of truth",()=>{
 expect(()=>runTopologyCommandV1(["env","--group","a1"],{platform:"darwin",topology:fixture.topology})).toThrow();
 expect(provisionShell).not.toMatch(/fleet env|SEMIO_REMOTE_CACHE_(ENDPOINT|ACCESS|INSTANCE)/u);
 expect(provisionPowerShell).not.toMatch(/'env'|SEMIO_REMOTE_CACHE_(ENDPOINT|ACCESS|INSTANCE)/u);
});

test("an unknown group, platform or shell is refused",()=>{
 expect(()=>groupV1(topology,"zz")).toThrow("Declared fleet group required");
 for(const operation of["mount-plan","server-plan","worker-service","facts"])expect(()=>runTopologyCommandV1([operation,"--group","zz"],{platform:"linux",topology:fixture.topology}),operation).toThrow("Declared fleet group required");
 expect(()=>runTopologyCommandV1(["mount-plan","--group","a1","--platform","beos"],{topology:fixture.topology})).toThrow("--platform must be one of");
 expect(()=>runTopologyCommandV1(["facts","--group","a1","--shell","fish"],{topology:fixture.topology})).toThrow("--shell must be one of");
 expect(()=>platformV1("freebsd")).toThrow("Supported fleet platform required");
 expect([platformV1("linux"),platformV1("darwin"),platformV1("win32")]).toEqual(["linux","macos","windows"]);
});

test("the worker service keeps the replica daemon alive on every platform",()=>{
 const group=fixture.topology.groups[0]!;
 const linux=workerServiceV1(topology,group.id,"linux"),macos=workerServiceV1(topology,group.id,"macos"),windows=workerServiceV1(topology,group.id,"windows");
 expect(linux).not.toContain("WorkingDirectory");
 expect(linux).toContain('ExecStart=/bin/sh "__WORKER_HOME__/.local/share/semio/fleet/entry/service/replica.sh"');
 expect(linux).toContain("AmbientCapabilities=CAP_SYS_NICE");
 expect(linux).not.toContain(SCRIPT);
 expect(linux).toContain("Restart=always");
 expect(linux).toContain("User=%i");
 expect(macos).not.toContain("WorkingDirectory");
 expect(macos).toContain("<key>ProgramArguments</key><array><string>/bin/sh</string><string>__WORKER_HOME__/Library/Application Support/semio/fleet/entry/boot.sh</string><string>run</string></array>");
 expect(macos).not.toContain(SCRIPT);
 expect(macos).toContain("<key>KeepAlive</key><true/>");
 expect(macos).not.toContain("UserName");
 expect(macos).toContain("<key>LimitLoadToSessionType</key><string>Aqua</string>");
 expect(windows).not.toContain("<WorkingDirectory>");
 expect(windows).toContain("<Command>conhost.exe</Command>");
 expect(windows).toContain("<Arguments>--headless powershell.exe -NoProfile -ExecutionPolicy Bypass -EncodedCommand ");expect(hostedScript(windows)).toBe(windowsSuperviseScriptV1("powershell.exe",'-NoProfile -ExecutionPolicy Bypass -File "__WORKER_HOME__\\AppData\\Local\\semio\\fleet\\entry\\boot.ps1" run'));
 expect(windows).not.toContain(SCRIPT);
 expect(windows).toContain("<RestartOnFailure>");
 expect(windows).toContain("<LogonTrigger>");
 expect(windows).toContain("<TimeTrigger><Enabled>true</Enabled><StartBoundary>2026-01-01T00:00:00</StartBoundary><Repetition><Interval>PT1M</Interval><StopAtDurationEnd>false</StopAtDurationEnd></Repetition></TimeTrigger>");
 expect(windows).toContain("<StartWhenAvailable>true</StartWhenAvailable>");
 expect(windows).toContain("<MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>");
 expect(linux).toContain("RestartSec=2");
 expect(linux).toContain("StartLimitIntervalSec=0");
 expect(linux).toContain("KillMode=mixed");
 expect(linux).toContain("TimeoutStopSec=30");
 expect(macos).toContain("<key>ThrottleInterval</key><integer>5</integer>");
 expect(macos).toContain("<key>ExitTimeOut</key><integer>30</integer>");
 expect(windows).toContain("<RestartOnFailure><Interval>PT1M</Interval><Count>255</Count></RestartOnFailure>");
 expect(windows).toContain("<ExecutionTimeLimit>PT0S</ExecutionTimeLimit>");
 expect(macos).not.toContain("SuccessfulExit");
 expect(runTopologyCommandV1(["worker-service","--group",group.id,"--platform","linux","--part","target"],{topology:fixture.topology})).toBe("/etc/systemd/system/semio-fleet-replica@.service");
});

type XmlNode={name:string;children:XmlNode[]};
function parseXml(text:string):XmlNode{
 const root:XmlNode={name:"#document",children:[]},stack=[root];
 for(const match of text.replace(/^<\?xml[^>]*\?>/u,"").matchAll(/<(\/?)([A-Za-z]+)([^>]*?)(\/?)>/gu)){
  if(match[1]==="/"){stack.pop();continue;}
  const node:XmlNode={name:match[2]!,children:[]};
  stack.at(-1)!.children.push(node);
  if(match[4]!=="/")stack.push(node);
 }
 return root.children[0]!;
}
const TRIGGER_ORDER=["Enabled","StartBoundary","EndBoundary","Repetition","ExecutionTimeLimit"],TRIGGER_OWN:Readonly<Record<string,readonly string[]>>={LogonTrigger:["UserId","Delay"],TimeTrigger:["RandomDelay"],CalendarTrigger:["RandomDelay","ScheduleByDay"]};
const TASK_ALL=["RegistrationInfo","Triggers","Settings","Data","Principals","Actions"],SETTINGS_ALL=["AllowStartOnDemand","RestartOnFailure","MultipleInstancesPolicy","DisallowStartIfOnBatteries","StopIfGoingOnBatteries","AllowHardTerminate","StartWhenAvailable","NetworkProfileName","RunOnlyIfNetworkAvailable","WakeToRun","Enabled","Hidden","DeleteExpiredTaskAfter","IdleSettings","NetworkSettings","ExecutionTimeLimit","Priority","RunOnlyIfIdle","UseUnifiedSchedulingEngine","DisallowStartOnRemoteAppSession"],EXEC_ALL=["Command","Arguments","WorkingDirectory"],REPETITION_ALL=["Interval","Duration","StopAtDurationEnd"],PRINCIPAL_ALL=["UserId","LogonType","GroupId","DisplayName","RunLevel","ProcessTokenSidType","RequiredPrivileges"];
function schemaViolations(xml:string):string[]{
 const task=parseXml(xml),bad:string[]=[],names=(node:XmlNode):string[]=>node.children.map(child=>child.name);
 const among=(where:string,node:XmlNode,allowed:readonly string[]):void=>{for(const name of names(node))if(!allowed.includes(name))bad.push(`${where}: ${name} is not allowed`);if(new Set(names(node)).size!==names(node).length)bad.push(`${where}: a child repeats`);};
 if(task.name!=="Task")bad.push("the root is not Task");
 among("Task",task,TASK_ALL);
 for(const section of task.children){
  if(section.name==="Settings"){among("Settings",section,SETTINGS_ALL);for(const child of section.children)if(child.name==="RestartOnFailure")among("RestartOnFailure",child,["Interval","Count"]);}
  if(section.name==="Principals")for(const principal of section.children)among("Principal",principal,PRINCIPAL_ALL);
  if(section.name==="Actions")for(const action of section.children){if(action.name!=="Exec")bad.push(`Actions: ${action.name} is not Exec`);among("Exec",action,EXEC_ALL);}
  if(section.name==="Triggers")for(const trigger of section.children){
   const own=TRIGGER_OWN[trigger.name];
   if(own===undefined){bad.push(`Triggers: ${trigger.name} is not used by the fleet`);continue;}
   const order=[...TRIGGER_ORDER,...own];
   let last=-1;
   for(const child of trigger.children){
    const index=order.indexOf(child.name);
    if(index<0)bad.push(`${trigger.name}: ${child.name} is not allowed`);
    else if(index<=last)bad.push(`${trigger.name}: ${child.name} comes after ${order[last]}, the schema sequence is ${order.join(", ")}`);
    last=Math.max(last,index);
    if(child.name==="Repetition")among("Repetition",child,REPETITION_ALL);
   }
  }
 }
 return bad;
}
const taskDocuments=():[string,string][]=>[...PLATFORMS.flatMap(()=>[] as [string,string][]),...["a1","b2","c3"].flatMap(id=>[...Object.entries(workerPartsV1(topology,id,"windows")),...(()=>{try{return Object.entries(serverPartsV1(topology,id));}catch{return [];}})()].filter(([,body])=>body.startsWith("<Task")||body.startsWith("<?xml version=\"1.0\" encoding=\"UTF-16\"")).map(([name,body])=>[`${id}.${name}`,body] as [string,string]))];

test("every scheduled task of the fleet follows the element order of the published Task Scheduler schema, with its own copy of the rules and with libxml2 against the schema itself",()=>{
 const documents=taskDocuments();
 expect(documents.length).toBeGreaterThanOrEqual(12);
 expect(schemaViolations('<Task><Triggers><TimeTrigger><Repetition><Interval>PT1M</Interval></Repetition><StartBoundary>x</StartBoundary></TimeTrigger></Triggers></Task>')).toEqual(["TimeTrigger: StartBoundary comes after Repetition, the schema sequence is Enabled, StartBoundary, EndBoundary, Repetition, ExecutionTimeLimit, RandomDelay"]);
 const xmllint=Bun.which("xmllint"),xsd=join(import.meta.dirname,"🧫️fixtures","🪟️task-schema.xsd");
 const scratch=mkdtempSync(join(tmpdir(),"semio-task-"));
 try{
  for(const [name,xml] of documents){
   expect(schemaViolations(xml),name).toEqual([]);
   expect(xml,name).toContain('<Task version="1.3" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">');
   expect(Number(xml.match(/<Count>(\d+)<\/Count>/u)?.[1]??1),name).toBeLessThanOrEqual(255);
   if(xmllint!==null){
    const file=join(scratch,"task.xml");
    writeFileSync(file,xml.split("\n").slice(1).join("\n"));
    const result=Bun.spawnSync([xmllint,"--noout","--schema",xsd,file]);
    expect(result.stderr.toString().trim().split("\n").at(-1),`${name}\n${result.stderr.toString()}`).toBe(`${file} validates`);
   }
  }
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

test("a Windows worker mounts its store at every logon and again every minute until the drive exists, in the same session that runs the replica, and survives sleep and wake",()=>{
 for(const [protocol,fleet,group] of [["nfs",topology,"a1"],["smb",smbFleet,"c3"]] as const){
  const parts=workerPartsV1(fleet,group,"windows"),drive=fleet.storeMount.windows.slice(0,2);
  expect(parts["mount-target"]).toBe("semio-fleet-store-mount");
  const definition=parts["mount-definition"]!;
  expect(definition).toContain("<LogonTrigger><Enabled>true</Enabled></LogonTrigger>");
  expect(definition).toContain("<Interval>PT1M</Interval>");
  expect(definition).toContain("<StartWhenAvailable>true</StartWhenAvailable>");
  expect(definition).toContain("<Command>conhost.exe</Command>");
  expect(definition).toContain(`/d /c if not exist ${drive}\\ ${protocol==="smb"?"net.exe use":"mount.exe -o"}`);
  expect(definition).toContain("<LogonType>InteractiveToken</LogonType><RunLevel>LeastPrivilege</RunLevel>");
  expect(parts["definition"]).toContain("<StartWhenAvailable>true</StartWhenAvailable>");
  expect(runTopologyCommandV1(["worker-service","--group",group,"--platform","windows","--part","mount-definition"],{topology:fleet})).toBe(definition);
 }
});

test("the arguments are closed",()=>{
 expect(parseTopologyArgumentsV1(["facts","--group","a1","--shell=sh"])).toEqual({operation:"facts",values:{"--group":"a1","--shell":"sh"},flags:new Set()});
 expect(parseTopologyArgumentsV1(["validate","--reachable","--timeout","500"])).toEqual({operation:"validate",values:{"--timeout":"500"},flags:new Set(["--reachable"])});
 expect(()=>parseTopologyArgumentsV1(["serve"])).toThrow("Closed fleet topology command required");
 expect(()=>parseTopologyArgumentsV1(["facts","--group","a1","--group","b2"])).toThrow("Repeated option");
 expect(()=>parseTopologyArgumentsV1(["server-plan","--platform","linux"])).toThrow("Unknown option");
 expect(()=>parseTopologyArgumentsV1(["facts","--reachable"])).toThrow("Unknown option");
 expect(()=>parseTopologyArgumentsV1(["validate","--reachable=yes"])).toThrow("takes no value");
 expect(()=>parseTopologyArgumentsV1(["validate","--reachable","--reachable"])).toThrow("Repeated option");
 expect(()=>parseTopologyArgumentsV1(["facts","--group"])).toThrow("Value required");
 expect(()=>parseTopologyArgumentsV1(["facts","--group","--shell"])).toThrow("Value required");
 expect(()=>runTopologyCommandV1(["facts"],{topology:fixture.topology})).toThrow("Required option missing: --group");
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
   expect(plan).toContain(`mount.exe -o anon -o nolock -o mtype=hard ${unc(host,path)} S:`);
   expect(plan).toContain("Register-ScheduledTask -TaskName semio-fleet-store-mount");
   expect(plan).toContain("--part mount-definition");
  }
  for(const other of fixture.topology.groups.filter(candidate=>candidate.id!==group.id))expect(plan).not.toContain(other.nfs.host);
 }
 expect(mountPartsV1(topology,"a1","linux")["fstab"]).toBe(factsV1(topology,"a1","linux")["FSTAB_LINE"]);
 expect(runTopologyCommandV1(["mount-plan","--group","b2","--platform","linux","--part","commands"],{topology:fixture.topology})).toBe("sudo mkdir -p /mnt/semio-store\nsudo mount /mnt/semio-store");
 expect(()=>runTopologyCommandV1(["mount-plan","--group","b2","--platform","linux","--part","nope"],{topology:fixture.topology})).toThrow("Declared part required");
 const smb=mountPartsV1(smbFleet,"c3","windows");
 expect(smb).toMatchObject({credential:"cmdkey /add:nfs.c3.example.test /user:nfs\\semio-fleet /pass:<password>",cache:"Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0 -Force",mount:`net use S: ${unc("nfs.c3.example.test","/store-c")} /persistent:yes`});
 expect(Object.keys(smb)).toEqual(["credential","cache","mount","task"]);
 expect(smb["task"]).toContain("worker-service --group c3 --platform windows --part mount-definition");
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
 for(const unit of ["cache-unit","relay-unit","replica-unit"])expect(parts[unit]).toContain("StartLimitIntervalSec=0");
 expect(parts["exports"]).toStartWith(`${group.nfs.export} 10.1.0.0/24(rw,sync,no_subtree_check,all_squash,anonuid=1500,anongid=1500)`);
 expect(parts["fstab"]).toContain(`${group.nfs.export} /mnt/semio-store none bind 0 0`);
 for(const other of others){
  expect(parts["exports"]).toContain(` ${other.nfs.host}(ro,sync,no_subtree_check)`);
  expect(parts["fstab"]).toContain(`${other.nfs.host}:${other.nfs.export} ${(group.peers as unknown as Record<string,string>)[other.id]} nfs vers=3,ro,hard,`);
 }
 expect(parts["cache-launch"]).toContain("export SEMIO_FLEET_NETWORK=10.1.0.0/24");expect(parts["cache-launch"]).toContain("exec /usr/local/bin/repo remote-cache serve --host fleet --port 6090 --directory /var/lib/semio-remote-cache --max-bytes 214748364800");
 expect(parts["cache-launch"]).toContain(". /etc/semio/remote-cache.env");
 expect(parts["cache-unit"]).toContain('ExecStart=/bin/sh "__WORKER_HOME__/.local/share/semio/fleet/entry/service/cache.sh"');
 expect(parts["cache-unit"]).toContain("Restart=always");
 expect(parts["cache-environment"]).toBe("SEMIO_REMOTE_CACHE_SERVER_READ_TOKEN=<read-token>\nSEMIO_REMOTE_CACHE_SERVER_WRITE_TOKEN=<write-token>");
 expect(parts["relay-launch"]).toContain(`cd "__WORKER_HOME__/semio/${REPLICA}" && exec /usr/local/bin/repo ./${SCRIPT} relay`);
 expect(parts["relay-unit"]).toContain('ExecStart=/bin/sh "__WORKER_HOME__/.local/share/semio/fleet/entry/service/relay.sh"');
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
 expect(parts["cache-unit"]).toContain("set -a; . /etc/semio/remote-cache.env; set +a; export SEMIO_FLEET_NETWORK=10.2.0.0/16; exec /usr/local/bin/repo remote-cache serve --host fleet --port 7000 --directory /var/lib/semio-remote-cache --max-bytes 214748364800");
 expect(parts["cache-unit"]).toContain("<key>KeepAlive</key><true/>");
 expect(parts["relay-unit"]).toContain(`<string>./${SCRIPT}</string><string>relay</string><string>--peers</string><string>a1=/Volumes/semio-peers/a1,c3=/Volumes/semio-peers/c3</string>`);
 expect(parts["relay-peers"]).toBe("a1=/Volumes/semio-peers/a1,c3=/Volumes/semio-peers/c3");
 expect(parts["provision"]).toEndWith("b2 b2-server --user <account> --archive /Volumes/semio-peers/a1/v1/bootstrap --server");
 expect(parts["start"]).toContain("sudo nfsd enable");
 for(const key of["cache-unit","relay-unit","replica-unit"]){expect(parts[key]).toContain("<key>HOME</key><string>__WORKER_HOME__</string>");expect(parts[key]).toContain("<key>LimitLoadToSessionType</key><string>Aqua</string>");expect(parts[key]).not.toContain("UserName");}
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
 expect(hostedScript(nfs["relay-unit"]!)).toBe(windowsSuperviseScriptV1("repo",relay));
 expect(nfs["relay-unit"]).toContain("<WorkingDirectory>C:\\semio\\"+REPLICA.replaceAll("/","\\")+"</WorkingDirectory>");
 expect(nfs["cache-unit"]).toContain("<Command>conhost.exe</Command>");
 expect(hostedScript(nfs["cache-unit"]!)).toContain("$env:SEMIO_FLEET_NETWORK='");expect(hostedScript(nfs["cache-unit"]!)).toContain("remote-cache serve --host fleet --port 443 --directory C:\\semio-remote-cache --max-bytes 214748364800");
 expect(hostedScript(nfs["cache-unit"]!)).toContain("Get-Content C:\\ProgramData\\semio\\remote-cache.env");
 expect(hostedScript(nfs["cache-unit"]!)).toContain('"Get-Content');
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
 expect(Object.keys(parts)).toEqual(["prepare","provision","start","exports-target","exports","sysctl-target","sysctl","fstab","relay-peers","cache-environment-target","cache-environment","cache-target","cache-unit","cache-launch-target","cache-launch","replica-unit","relay-launch-target","relay-launch","relay-target","relay-unit"]);
 expect(parts["sysctl-target"]).toBe("/etc/sysctl.d/91-semio-fleet-server.conf");
 expect(parts["sysctl"]).toBe("fs.leases-enable=0");
 expect(parts["prepare"]).toContain("--create-home --home-dir /var/lib/semio");
 expect(parts["provision"]).toEndWith("a1 a1-server --user semio --archive /mnt/peers/b2/v1/bootstrap --server");
 expect(parts["start"]).toContain("semio-fleet-replica@semio.service semio-fleet-cache.service semio-fleet-relay.service");
 expect(parts["replica-unit"]).toBe(workerServiceV1(topology,"a1","linux"));
 expect(parts["relay-unit"]).toContain("After=network-online.target remote-fs.target semio-fleet-replica@semio.service");
 expect(parts["relay-unit"]).toContain("ConditionPathExists=__WORKER_HOME__/semio/.cache/f/machine.json");
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
 expect(linux["compact-launch-definition"]).toContain(`exec /usr/local/bin/repo ./${SCRIPT} compact`);
 expect(linux["compact-definition"]).toContain('ExecStart=/bin/sh "__WORKER_HOME__/.local/share/semio/fleet/entry/service/compact.sh"');
 expect(linux["compact-definition"]).toContain("Type=oneshot");
 expect(linux["compact-launch-definition"]).not.toContain("--collect-blobs");
 expect(macos["compact-definition"]).not.toContain("--collect-blobs");
 expect(windows["compact-definition"]).not.toContain("--collect-blobs");
 expect(workerPartsV1(topology,"a1","linux",true)["compact-launch-definition"]).toContain(`exec /usr/local/bin/repo ./${SCRIPT} compact --collect-blobs`);
 expect(workerPartsV1(topology,"b2","macos",true)["compact-definition"]).toContain(`<string>./${SCRIPT}</string><string>compact</string><string>--collect-blobs</string>`);
 expect(workerPartsV1(topology,"c3","windows",true)["compact-definition"]).toContain(`<Arguments>--headless repo ./${SCRIPT} compact --collect-blobs</Arguments>`);
 expect(workerPartsV1(topology,"a1","linux",true)["definition"]).toBe(linux["definition"]);
 expect(()=>workerPartsV1(topology,"a1","macos",true)).toThrow("Server platform required");
 expect(()=>workerPartsV1(topology,"b2","linux",true)).toThrow("Server platform required");
 expect(runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--part","compact-definition","--role","server"],{topology:fixture.topology})).toBe(workerPartsV1(topology,"a1","linux",true)["compact-definition"]!);
 expect(runTopologyCommandV1(["worker-service","--group","b2","--platform","macos","--part","compact-definition","--role","server"],{topology:fixture.topology})).toBe(workerPartsV1(topology,"b2","macos",true)["compact-definition"]!);
 expect(runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--role","server"],{topology:fixture.topology})).toBe(linux["definition"]!);
 expect(()=>runTopologyCommandV1(["worker-service","--group","a1","--platform","linux","--role","admin"],{topology:fixture.topology})).toThrow("must be one of worker|server");
 expect(linux["timer-definition"]).toContain("OnCalendar=daily");
 expect(macos["compact-target"]).toBe("__WORKER_HOME__/Library/LaunchAgents/tech.semio.fleet.compact.plist");
 expect(macos["compact-definition"]).toContain(`<string>./${SCRIPT}</string><string>compact</string>`);
 expect(macos["compact-definition"]).toContain("<key>StartCalendarInterval</key>");
 expect(macos["compact-definition"]).not.toContain("KeepAlive");
 for(const text of[macos["definition"]!,macos["compact-definition"]!]){expect(text).not.toContain("UserName");expect(text).toContain("<key>HOME</key><string>__WORKER_HOME__</string>");}
 expect(windows["compact-target"]).toBe("semio-fleet-compact");
 expect(windows["compact-definition"]).toContain(`<Arguments>--headless repo ./${SCRIPT} compact</Arguments>`);
 expect(windows["compact-definition"]).toContain("<CalendarTrigger>");
});

test("the worktree template resolves against the account home per platform, keeps spaces and non-ASCII characters, refuses a relative home, and the Python oracle and the facts agree",()=>{
 const homes:readonly (readonly [FleetPlatformV1,string,string])[]=[["linux","/home/ann","/home/ann/semio"],["linux","/home/jürgen müller/","/home/jürgen müller/semio"],["macos","/Users/Zoë Ångström","/Users/Zoë Ångström/Documents/semio"],["macos","/Users/a b","/Users/a b/Documents/semio"],["macos","/Users/日本語","/Users/日本語/Documents/semio"],["windows","C:\\Users\\ann","C:\\semio"],["windows","D:\\Users\\Zoë Ångström","C:\\semio"]];
 const file=join(mkdtempSync(join(tmpdir(),"worktree-")),"topology.json");
 writeFileSync(file,JSON.stringify(fixture.topology));
 for(const [platform,home,expected] of homes){
  expect(worktreePathV1(topology,platform,home)).toBe(expected);
  expect(factsV1(topology,"a1",platform,home)["WORKTREE"]).toBe(expected);
  expect(factsV1(topology,"a1",platform,home)["WORKTREE_TEMPLATE"]).toBe(topology.worktree[platform]);
  if(python){
   const run=Bun.spawnSync([python,"-I",oracle,"worktree",file,platform,home],{stdout:"pipe",stderr:"pipe"});
   expect(run.stdout.toString().trimEnd()).toBe(expected);
  }
 }
 for(const bad of["","relative/home","~","/"]){
  expect(()=>worktreePathV1(topology,"linux",bad)).toThrow("Absolute account home required");
  if(python)expect(Bun.spawnSync([python,"-I",oracle,"worktree",file,"macos",bad],{stdout:"pipe",stderr:"pipe"}).exitCode).toBe(1);
 }
 expect(worktreePathV1(topology,"linux")).toBe("__WORKER_HOME__/semio");
 expect(runTopologyCommandV1(["facts","--group","a1","--platform","macos","--home","/Users/it's me","--shell","sh"],{topology:fixture.topology})).toContain("FLEET_WORKTREE='/Users/it'\\''s me/Documents/semio'");
 expect(runTopologyCommandV1(["facts","--group","a1","--platform","linux","--shell","sh"],{topology:fixture.topology,home:"/home/ann"})).toContain("FLEET_WORKTREE='/home/ann/semio'");
});

test("the provisioning script resolves the worktree template and fills the service placeholders for homes with spaces, quotes, ampersands and non-ASCII characters, as the Python oracle does",()=>{
 const start=provisionShell.indexOf("WORKTREE_TEMPLATE="),block=provisionShell.slice(start,provisionShell.indexOf("esac\n",start)+5),fillStart=provisionShell.indexOf("sed_escape() {"),fillBlock=provisionShell.slice(fillStart,provisionShell.indexOf("\nWORKER_UID=",fillStart));
 const folder=mkdtempSync(join(tmpdir(),"fill-")),file=join(folder,"topology.json"),text=join(folder,"service.txt");
 writeFileSync(file,JSON.stringify(fixture.topology));
 writeFileSync(text,"WorkingDirectory=__WORKER_HOME__/semio/x\n<key>HOME</key><string>__WORKER_HOME__</string>\nUser=__WORKER_USER__\n");
 try{
  for(const [platform,template,home] of [["linux","~/semio","/home/ann"],["macos","~/Documents/semio","/Users/Zoë Ångström"],["macos","~/Documents/semio","/Users/it's & <me>|x\\y/"],["macos","~/Documents/semio","/Users/日本語"]] as const){
   const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nfail(){ printf 'FAIL %s\\n' "$*"; exit 3; }\njson_fact(){ printf '%s' "$TEMPLATE"; }\nTOPOLOGY_FLAT=x; PLATFORM=${platform}\n${block}\nprintf '%s' "$WORKTREE"`],{env:{...process.env,TEMPLATE:template,WORKER_HOME:home},stdout:"pipe",stderr:"pipe"});
   expect(run.stdout.toString()).toBe(worktreePathV1(topology,platform,home));
   if(python)expect(Bun.spawnSync([python,"-I",oracle,"worktree",file,platform,home],{stdout:"pipe"}).stdout.toString().trimEnd()).toBe(run.stdout.toString());
   for(const kind of ["plain","xml"] as const){
    const filled=Bun.spawnSync([bash!,"-c",`set -euo pipefail\n${fillBlock}\nWORKER_USER=builder\n${kind==="xml"?"fill_plist":"fill"} < "$TEXT"`],{env:{...process.env,WORKER_HOME:home,TEXT:text},stdout:"pipe",stderr:"pipe"});
    if(python)expect(filled.stdout.toString()).toBe(Bun.spawnSync([python,"-I",oracle,"fill",text,home,"builder",kind],{stdout:"pipe"}).stdout.toString());
    expect(filled.stdout.toString()).not.toContain("__WORKER_");
    expect(filled.stdout.toString()).toContain(kind==="xml"?home.replace(/\/+$/u,"").replaceAll("&","&amp;").replaceAll("<","&lt;"):home.replace(/\/+$/u,""));
   }
  }
  const bad=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nfail(){ printf 'FAIL %s\\n' "$*"; exit 3; }\njson_fact(){ printf '%s' "$TEMPLATE"; }\nTOPOLOGY_FLAT=x; PLATFORM=linux\n${block}`],{env:{...process.env,TEMPLATE:"~",WORKER_HOME:"/home/ann"},stdout:"pipe",stderr:"pipe"});
  expect(bad.exitCode).toBe(3);expect(bad.stdout.toString()).toContain("no usable worktree template");
 }finally{rmSync(folder,{recursive:true,force:true});}
});

test("an installed replica service names the old worktree, a different path means relocation, and the macOS services are agents of the account, loaded into its gui domain and never into the system domain",()=>{
 const folder=mkdtempSync(join(tmpdir(),"detect-")),agents=join(folder,"Library","LaunchAgents");
 mkdirSync(agents,{recursive:true});
 const modules="🧰️framework/🛍️products/🦑️repo/🔨️modules";
 writeFileSync(join(agents,"tech.semio.fleet.replica.plist"),`<plist><dict><key>WorkingDirectory</key><string>/Users/old &amp; me/semio/${modules}/🤝️shared-worktree/🔁️replica</string></dict></plist>`);
 try{
  const start=provisionShell.indexOf("entry_directory() {"),block=provisionShell.slice(start,provisionShell.indexOf("\nRELOCATE=0",start));
  const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\n${block}\nPLATFORM=macos\ndetect_installed_worktree`],{env:{...process.env,WORKER_HOME:folder},stdout:"pipe",stderr:"pipe"});
  expect(run.stdout.toString().trim()).toBe(`/Users/old & me/semio/${modules}/🤝️shared-worktree/🔁️replica`);
  const loaded=provisionShell.slice(provisionShell.indexOf("agent_domain_present() {"),provisionShell.indexOf("load_daemon() {"));
  const drive=(present:boolean)=>Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nWORKER_UID=501; WORKER_USER=builder\nlaunchctl(){ case "$1" in print) if [ "$2" = gui/501 ]; then return ${present?0:1}; fi; return 1;; *) printf 'LAUNCHCTL %s\\n' "$*";; esac; }\nensure_daemon_logs(){ :; }\n${loaded}\nload_agent /x/tech.semio.fleet.replica.plist\nretire_system_daemon tech.semio.fleet.replica`],{stdout:"pipe",stderr:"pipe"}).stdout.toString();
  expect(drive(true)).toContain("LAUNCHCTL bootstrap gui/501 /x/tech.semio.fleet.replica.plist");
  expect(drive(true)).not.toContain("bootstrap system");
  expect(drive(false)).toContain("WARNING: builder has no graphical login session");
  expect(drive(false)).not.toContain("LAUNCHCTL bootstrap");
 }finally{rmSync(folder,{recursive:true,force:true});}
 expect(provisionShell.indexOf('if [ "$PLAN" = 1 ]; then')).toBeGreaterThan(provisionShell.indexOf('say "step 4b: clock"'));
 expect(provisionShell.indexOf('if [ "$PLAN" = 1 ]; then')).toBeLessThan(provisionShell.indexOf('say "step 5: machine environment (the cache endpoint comes from the fleet facts, never from a profile)"'));
 expect(provisionShell).toContain("replica join --adopt --dry-run");
 expect(provisionShell).toContain("replica relocate --dry-run");
 expect(provisionShell).toContain("--phase finish --wait 120");
 expect(provisionShell).toContain("exit code 4");
});

test("the facts name the archive and the topology directory of the canonical worktree",()=>{
 expect(factsV1(topology,"a1","linux")["ARCHIVE_DIRECTORY"]).toBe("/mnt/semio-store/v1/bootstrap");
 expect(factsV1(topology,"a1","windows")["ARCHIVE_DIRECTORY"]).toBe("S:\\v1\\bootstrap");
 expect(factsV1(topology,"a1","macos","/Users/test")["TOPOLOGY_DIRECTORY"]).toBe(`/Users/test/Documents/semio/${REPLICA.replace("🔁️replica","🗺️topology")}`);
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
 for(const arguments_ of[["facts","--group",group,"--platform","linux","--shell","sh"],["facts","--group",group,"--platform","windows","--shell","powershell"],["mount-plan","--group",group,"--platform","macos","--part","plist-path"],["mount-plan","--group",group,"--platform","macos","--part","plist"],["mount-plan","--group",group,"--platform","linux","--part","sysctl-path"],["mount-plan","--group",group,"--platform","linux","--part","sysctl"],
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

test.skipIf(!python)("the Python oracle accepts the committed topology and the fixture",()=>{
 const check=Bun.spawnSync([python!,"-I",oracle,"check"],{stdout:"pipe",stderr:"pipe"});
 expect(check.stdout.toString()+check.stderr.toString()).toBe("");
 expect(check.exitCode).toBe(0);
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
 expect(serverPartsV1(topology,"a1")["cache-launch"]).toContain("export SEMIO_LANGUAGE=en");
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
 expect(provisionShell).toContain('for target in "$SERVICE_TARGET" "$COMPACT_TARGET"; do load_agent "$target"; done');
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
/** 🐚 Forward-slash form of a host path for splicing into a `bash -c` string, where backslashes would be eaten and mangle the path. */
const posix=(path:string):string=>path.replaceAll("\\","/");
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

test("a plan on an empty worktree extracts nothing, and a tree that the script extracted itself is settled and never adopted, also after an interrupted run",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-extract-")),tree=join(root,"tree"),source=join(root,"source"),archive=join(root,"archive.tar"),launcher="launcher/install.sh",marker=join(tree,".cache/f/bootstrap-extract");
 try{
  mkdirSync(join(source,"launcher"),{recursive:true});writeFileSync(join(source,launcher),"");writeFileSync(join(source,"a.txt"),"alpha\n");
  expect(Bun.spawnSync(["tar","-cf",archive,"-C",source,launcher,"a.txt"]).exitCode).toBe(0);
  const block=provisionShell.slice(provisionShell.indexOf('say "step 1: canonical worktree'),provisionShell.indexOf("\nrelease_foreign_namespace() {"));
  const step=(variables:string=""):Readonly<{out:string;code:number}>=>{
   const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf 'SAY %s\\n' "$*"; }\nfail(){ printf 'FAIL %s\\n' "$*"; exit 3; }\nrun_as(){ "$@"; }\ninstall_nfs_client(){ :; }\nfetch_archive(){ printf 'FETCH\\n'; cp "$ARCHIVE_FILE" "$2"; EXPECTED_FILES=2; }\nWORKTREE=$TREE; LAUNCHER_INSTALL=${launcher}; PLATFORM=linux; UTF8_LOCALE=C; ARCHIVE=/given; SERVER=0; FIRST_MACHINE=0; RELOCATE=0; JOINED=0; PLAN=0; NEEDS_NATIVE=0; ${variables}\n${block}\n[ -z "\${CLEANUP_DIRECTORY:-}" ] || rm -rf "$CLEANUP_DIRECTORY"\nprintf 'ADOPT=%s SETTLE=%s NATIVE=%s\\n' "$ADOPT" "$SETTLE" "$NEEDS_NATIVE"`],{env:{...process.env,TREE:tree,ARCHIVE_FILE:archive},stdout:"pipe",stderr:"pipe"});
   return {out:run.stdout.toString()+run.stderr.toString(),code:run.exitCode};
  };
  const plan=step("PLAN=1");
  expect(plan.code).toBe(0);expect(plan.out).toContain("plan: the worktree is empty: nothing was extracted");expect(plan.out).not.toContain("FETCH");expect(plan.out).not.toContain("ADOPT=");
  expect(readdirSync(tree)).toEqual([]);
  const first=step();
  expect(first.out).toContain("FETCH");expect(first.out).toContain("ADOPT=0 SETTLE=1 NATIVE=1");
  expect(readFileSync(join(tree,"a.txt"),"utf8")).toBe("alpha\n");expect(readFileSync(marker,"utf8").trim()).toBe("complete");
  const again=step();
  expect(again.out).not.toContain("FETCH");expect(again.out).toContain("it is settled, not adopted");expect(again.out).toContain("ADOPT=0 SETTLE=1 NATIVE=0");
  const planned=step("PLAN=1");
  expect(planned.out).not.toContain("FETCH");expect(planned.out).toContain("ADOPT=0 SETTLE=1");
  writeFileSync(marker,"started\n");rmSync(join(tree,"a.txt"));
  const resumed=step();
  expect(resumed.out).toContain("an earlier run stopped while it extracted");expect(resumed.out).toContain("FETCH");expect(resumed.out).toContain("ADOPT=0 SETTLE=1 NATIVE=1");
  expect(readFileSync(join(tree,"a.txt"),"utf8")).toBe("alpha\n");expect(readFileSync(marker,"utf8").trim()).toBe("complete");
  const interruptedPlan=(writeFileSync(marker,"started\n"),step("PLAN=1"));
  expect(interruptedPlan.code).toBe(0);expect(interruptedPlan.out).not.toContain("FETCH");expect(interruptedPlan.out).toContain("plan: an earlier run stopped while it extracted");
  const joined=step("JOINED=1");
  expect(joined.out).not.toContain("FETCH");expect(joined.out).toContain("ADOPT=0 SETTLE=0");
  rmSync(marker);
  const own=step();
  expect(own.out).not.toContain("FETCH");expect(own.out).toContain("it is adopted into the fleet");expect(own.out).toContain("ADOPT=1 SETTLE=0");
 }finally{rmSync(root,{recursive:true,force:true});}
 const settle=provisionShell.indexOf('replica bootstrap-settle --store "$STORE_ROOT"'),joinAt=provisionShell.indexOf('replica join --group "$GROUP_ID" --origin "$ORIGIN_ID" --store "$STORE_ROOT"\n'),cleared=provisionShell.indexOf('run_as rm -f "$EXTRACT_MARKER"');
 expect(settle).toBeGreaterThan(0);expect(joinAt).toBeGreaterThan(settle);expect(cleared).toBeGreaterThan(joinAt);
 expect(provisionShell).toContain("would settle the tree that this script extracted and join");
 const windows=["$extractMarker = Join-Path $Worktree '.cache\\f\\bootstrap-extract'","if ($extract -and $Plan) {","Set-Content -LiteralPath $extractMarker -Value 'started'","& $tar -xf $local -C $Worktree","Set-Content -LiteralPath $extractMarker -Value 'complete'","Say 'step 5: standalone repo launcher","Invoke-Replica @('bootstrap-settle', '--store', $StoreRoot)","Remove-Item -LiteralPath $extractMarker -Force -ErrorAction SilentlyContinue"].map(part=>provisionPowerShell.indexOf(part));
 expect(windows.every(index=>index>=0)).toBe(true);expect(windows).toEqual([...windows].sort((left,right)=>left-right));
 expect(provisionPowerShell).toContain("plan: the worktree is empty: nothing was extracted");expect(provisionPowerShell).toContain("it is settled, not adopted");
});

test("the shell script repeats a native bootstrap that has no completion marker or no launcher, and skips one that has both",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-marker-")),home=join(root,"home/.local/bin"),tree=join(root,"tree/.cache/f");
 mkdirSync(home,{recursive:true});mkdirSync(tree,{recursive:true});
 const needed=():number=>Bun.spawnSync([bash!,"-c",`WORKTREE=${posix(root)}/tree; WORKER_HOME=${posix(root)}/home;${provisionShell.slice(provisionShell.indexOf("BOOTSTRAP_MARKER="),provisionShell.indexOf("toolchain_packages()"))}\nbootstrap_needed`]).exitCode;
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
 const order=["bash \"$WORKTREE/$LAUNCHER_INSTALL\" \"$LAUNCHER_BIN\"","FACTS=$(fleet facts",'say "step 4: system limits','say "step 4b: clock"','say "step 5: machine environment (the cache endpoint comes from the fleet facts, never from a profile)"','say "step 6: replica and compact services','replica join --group','systemctl enable --now "${STARTED[@]}" "$TIMER"','for target in "$SERVICE_TARGET" "$COMPACT_TARGET"; do load_agent','native_bootstrap\nif [ "$ACCESS_PENDING" = 1 ]'].map(part=>provisionShell.indexOf(part));
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
 const powerShellOrder=["$LauncherInstall) -Destination $LauncherBin","Invoke-Fleet @('facts'","Register-ScheduledTask -TaskName $taskName","Invoke-Replica @('join', '--group'","Start-ScheduledTask -TaskName $startTask","step 11: native bootstrap"].map(part=>provisionPowerShell.indexOf(part));
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
 writeFileSync(script,`#!/usr/bin/env bash\nset -euo pipefail\ntouch "${posix(root)}/ran"\necho "[semio] cache ${"${SEMIO_REMOTE_CACHE_ENDPOINT:-}"} ${"${SEMIO_REMOTE_CACHE_ACCESS:-}"} ${"${SEMIO_REMOTE_CACHE_INSTANCE:-}"} token=${"${SEMIO_REMOTE_CACHE_TOKEN:-none}"}"\necho "[semio] workspace:setup..."\nrepo run workspace:setup\n`);
 const toolchain=options.toolchain??true?"check_toolchain(){ return 0; }":'check_toolchain(){ TOOLCHAIN_PROBLEM="missing build tools: cc: install them as root first (apt-get install -y build-essential)"; return 1; }';
 const run=Bun.spawnSync([bash!,"-c",`set -euo pipefail\nsay(){ printf '%s\\n' "$*"; }\n${constants}\nWORKTREE=${posix(tree)}; WORKER_HOME=${posix(home)}; WORKER_USER=builder; ORIGIN_ID=a1-m01; GROUP_ID=a1; EXIT_NATIVE_PENDING=3; NEEDS_NATIVE=${options.needed??1}; PLATFORM=linux\nrun_as(){ "$@"; }\n${provisionShell.slice(provisionShell.indexOf("BOOTSTRAP_MARKER="),provisionShell.indexOf("check_time_sync() {"))}\n${toolchain}\nnative_bootstrap\necho reached-the-end`],{env:{...Object.fromEntries(Object.entries(process.env).filter(([name])=>!name.startsWith("SEMIO_"))),ENVIRONMENT:options.environment??"",PATH:`${bin}:${process.env["PATH"]}`}});
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

(bash?test:test.skip)("the native bootstrap gets the operator's write token of the account (the endpoint comes from the fleet facts) and leaves no environment file behind",()=>{
 const environment="export UNRELATED=1",run=nativeStep({repoExit:0,environment,profile:"export SEMIO_REMOTE_CACHE_TOKEN='abc-write'\n"});
 expect(run.code).toBe(0);
 expect(run.log).toContain("[semio] cache    token=abc-write");
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
 const shortfall=(path:string):string=>Bun.spawnSync([bash!,"-c",`${provisionShell.slice(provisionShell.indexOf("COLD_GRAPH_MEMORY_MB="),provisionShell.indexOf("ensure_swap() {"))}\nswap_shortfall_mb ${posix(path)}`]).stdout.toString().trim();
 try{
  expect(shortfall(meminfo(8192,0))).toBe("8193");
  expect(shortfall(meminfo(8192,2048))).toBe("6145");
  expect(shortfall(meminfo(8192,8192))).toBe("0");
  expect(shortfall(meminfo(16384,0))).toBe("0");
  expect(shortfall(meminfo(65536,0))).toBe("0");
 }finally{rmSync(root,{recursive:true,force:true});}
 expect(provisionShell.indexOf("  ensure_swap\n  install_nfs_client")).toBeGreaterThan(provisionShell.indexOf('say "step 4: system limits'));
 expect(provisionShell.indexOf("  ensure_swap\n  install_nfs_client")).toBeLessThan(provisionShell.indexOf("native_bootstrap\nif [ \"$ACCESS_PENDING\" = 1 ]"));
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
const GUARDS:GitGuardsV1={version:"2.47.3",pins:gitPinsHashV1(),volume:{caseInsensitive:false,normalisationInsensitive:false,symlinks:true,longPaths:null},infoExclude:0,caseIgnore:0,caseMismatch:0,trackedIgnored:0,trackedRefused:0,links:0,overBudget:0,modeMismatch:0,samples:[]};
const HEALTHY:StatusReportV1={version:1,originId:"a1-m01",groupId:"a1",storeRoot:"/mnt/semio-store",reachable:true,daemon:{running:true,pid:42},sync:true,files:270000,queue:{incoming:0,outgoing:0},lost:0,refused:null,origins:[{origin:"a1-server",behindEvents:0,behindBytes:0,lastHlc:null,ageSeconds:1,rejoinRequired:false}]};
const NFS_OPTIONS="rw,vers=4.2,rsize=262144,wsize=262144,namlen=255,hard,proto=tcp,nconnect=4,timeo=600,retrans=2,sec=sys,acregmin=1,acregmax=1,acdirmin=1,acdirmax=1,lookupcache=none,local_lock=none";
const mountinfo=(options:string=NFS_OPTIONS,type:string="nfs4",source:string="nfs.a1.example.test:/srv/store-a",mount:string="/mnt/semio-store"):string=>`36 35 0:41 / ${mount} rw,noatime shared:7 - ${type} ${source} ${options}\n`;
const linuxFiles=(extra:Record<string,string>={},origin:string="a1-m01"):Record<string,string>=>({"/home/test/semio/.cache/f/machine.json":JSON.stringify({originId:origin,groupId:"a1",storeRoot:"/mnt/semio-store"}),"/proc/self/mountinfo":mountinfo(),"/proc/sys/fs/inotify/max_user_watches":"1048576\n","/proc/sys/fs/inotify/max_user_instances":"1024\n",...extra});
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
 free:async()=>100*GIB,exists:async()=>true,resolved:async path=>path,write:async()=>{},now:()=>NOW,status:async()=>HEALTHY,git:async()=>"/usr/bin/git",guards:async()=>GUARDS,scripts:async()=>[],...extra
});
const preflight=(group:string,platform:FleetPlatformV1,probes:Partial<PreflightProbesV1>,extra:Record<string,unknown>={})=>preflightV1(topology,{platform,group,home:platform==="macos"?"/Users/test":"/home/test",uid:501,timeoutMilliseconds:1000,languages:["en"],signal:new AbortController().signal,progress:()=>{},probes,...extra});
const statusOf=(report:Awaited<ReturnType<typeof preflight>>):Record<string,string>=>Object.fromEntries(report.checks.map(check=>[check.id,`${check.status}:${check.key}`]));
const LINUX_OK={identity:"ok:identity.ok",mount:"ok:mount.ok",options:"ok:options.ok",write:"ok:write.ok",cache:"ok:cache.ok",clock:"ok:clock.ok",timesync:"ok:timesync.ok",limits:"ok:limits.ok","disk.worktree":"ok:disk.ok","native-build":"ok:native.ok",git:"ok:git.ok","guards.git":"ok:guards.git.ok","guards.exclude":"ok:guards.exclude.ok","guards.case":"ok:guards.case.ok","guards.tracked":"ok:guards.tracked.ok","guards.paths":"ok:guards.paths.ok","guards.symlinks":"ok:guards.symlinks.ok","guards.scripts":"ok:guards.scripts.ok","service.replica":"ok:service.ok","service.compact":"ok:compact.ok",access:"ok:access.ok",replica:"ok:replica.ok",watcher:"skip:watcher.skip",visibility:"skip:visibility.skip"};

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
 expect(report.text).toContain("All 24 checks passed.");
});

test("the native build check is ok with the completion marker of the provisioning and a warning, never a failure, while the bootstrap is pending",async()=>{
 const marker="/home/test/semio/.cache/f/native-bootstrap.done",log="/home/test/semio/.cache/f/native-bootstrap.log",asked:string[]=[];
 const pending=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{exists:async path=>{asked.push(path);return path!==marker;}}),{languages:["en","de"]});
 expect(asked).toContain(marker);
 expect(pending.ok).toBe(true);
 expect(statusOf(pending)).toEqual({...LINUX_OK,"native-build":"warn:native.pending"});
 expect(pending.text).toContain(`WARN native build: pending: the native bootstrap has not completed (${marker} is missing), so this machine syncs but cannot build yet; run the provisioning script again to retry it, the log is ${log}`);
 expect(pending.text).toContain("WARN Native Build: ausstehend: Der native Bootstrap ist nicht abgeschlossen");
 expect(pending.text).toContain("24 checks, 1 with warnings");
 const done=await preflight("a1","linux",fake(linuxFiles(),linuxCommands()));
 expect(done.text).toContain(`ok   native build: bootstrap complete (${marker})`);
 const windows=await preflight("c3","windows",fake({"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"c3-m01",groupId:"c3"})},()=>({code:0,stdout:""}),{exists:async path=>!path.endsWith("native-bootstrap.done")}),{worktree:"C:\\semio"});
 expect(statusOf(windows)["native-build"]).toBe("warn:native.pending");
});

test("a missing git is a warning that says stage and commit state is not shared on this machine, in both languages",async()=>{
 const report=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{git:async()=>null,guards:async()=>null,scripts:async()=>null}),{languages:["en","de"]});
 expect(report.ok).toBe(true);
 expect(statusOf(report)).toEqual({...LINUX_OK,git:"warn:git.missing",...Object.fromEntries(["guards.git","guards.exclude","guards.case","guards.tracked","guards.paths","guards.symlinks","guards.scripts"].map(id=>[id,"skip:guards.skip"]))});
 expect(report.text).toContain("WARN git: no git executable found on PATH or in the usual install locations (on Windows Git for Windows): stage and commit state is NOT shared on this machine, files still sync");
 expect(report.text).toContain("WARN git: keine ausführbare git-Datei");
});

test("a tracked shell script with a carriage return is a warning that names the scripts, in both languages",async()=>{
 const report=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),{scripts:async()=>["a/run.sh","b/build.sh"]}),{languages:["en","de"]});
 expect(statusOf(report)["guards.scripts"]).toBe("warn:guards.scripts.crlf");
 expect(report.ok).toBe(true);
 expect(report.text).toContain("2 tracked shell scripts contain a carriage return (CRLF)");
 expect(report.text).toContain("a/run.sh, b/build.sh");
 expect(report.text).toContain("2 versionierte Shell-Skripte enthalten ein Wagenrücklaufzeichen (CRLF)");
});

test("the guards fail on what makes this machine see another view and warn on what a human git interface would show differently, in both languages",async()=>{
 const with_=(change:Partial<GitGuardsV1>,platform:FleetPlatformV1="linux",files:Record<string,string>=linuxFiles())=>preflight("a1",platform,fake(files,linuxCommands(),{guards:async()=>({...GUARDS,...change})}),{languages:["en","de"]});
 const exclude=await with_({infoExclude:2});
 expect(exclude.ok).toBe(false);
 expect(statusOf(exclude)["guards.exclude"]).toBe("fail:guards.exclude.fail");
 expect(exclude.text).toContain("2 rules in /home/test/semio/.git/info/exclude");
 expect(exclude.text).toContain("2 Regeln in /home/test/semio/.git/info/exclude");
 const old=await with_({version:"2.30.2"});
 expect(statusOf(old)["guards.git"]).toBe("fail:guards.git.old");
 const mismatch=await with_({caseMismatch:3,samples:["case-mismatch Foo.txt","case-mismatch Dir/in.txt"]});
 expect(mismatch.ok).toBe(true);
 expect(statusOf(mismatch)["guards.case"]).toBe("warn:guards.case.mismatch");
 expect(mismatch.text).toContain("3 tracked paths are spelled differently on this disk (case or NFC/NFD): Foo.txt, Dir/in.txt");
 const ignore=await with_({caseIgnore:1,samples:["case-ignore backup"]});
 expect(statusOf(ignore)["guards.case"]).toBe("warn:guards.case.ignore");
 expect(ignore.text).toContain("hidden by a case-insensitive .gitignore match");
 expect(ignore.text).toContain("Gross-/Kleinschreibung ignorierenden");
 const refused=await with_({trackedRefused:2,samples:["refused CON.txt","unsafe-link abs"]});
 expect(refused.ok).toBe(false);
 expect(statusOf(refused)["guards.tracked"]).toBe("fail:guards.tracked.refused");
 expect(refused.text).toContain("CON.txt, abs");
 expect(statusOf(await with_({trackedIgnored:1}))["guards.tracked"]).toBe("warn:guards.tracked.ignored");
 expect(statusOf(await with_({modeMismatch:1,samples:["mode run.sh"]}))["guards.tracked"]).toBe("warn:guards.tracked.mode");
 expect(statusOf(await with_({overBudget:1,samples:["over-budget deep/path"]}))["guards.paths"]).toBe("warn:guards.paths.over");
 const windows=(change:Partial<GitGuardsV1>)=>preflight("a1","windows",fake({"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"a1-m01",groupId:"a1",storeRoot:"S:\\"})},()=>({code:127,stdout:""}),{guards:async()=>({...GUARDS,...change}),status:async()=>HEALTHY}),{worktree:"C:\\semio"});
 expect(statusOf(await windows({overBudget:2,volume:{...GUARDS.volume,longPaths:false}}))["guards.paths"]).toBe("fail:guards.paths.over");
 expect(statusOf(await windows({overBudget:2,volume:{...GUARDS.volume,longPaths:true}}))["guards.paths"]).toBe("warn:guards.paths.over");
 expect(statusOf(await windows({volume:{...GUARDS.volume,symlinks:false}}))["guards.symlinks"]).toBe("warn:guards.symlinks.warn");
 expect(statusOf(await windows({links:3,volume:{...GUARDS.volume,symlinks:false}}))["guards.symlinks"]).toBe("fail:guards.symlinks.fail");
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
 const linked=await preflight("a1","linux",fake(linuxFiles({"/proc/self/mountinfo":mountinfo(NFS_OPTIONS,"nfs4","nfs.a1.example.test:/srv/store-a","/var/mnt/semio-store")}),linuxCommands(),{resolved:async path=>path==="/mnt/semio-store"?"/var/mnt/semio-store":path}));
 expect(statusOf(linked)["mount"]).toBe(statusOf(await run(linuxFiles()))["mount"]);
 expect(statusOf(absent)["options"]).toBe("skip:options.skip");
 const defaults=await run(linuxFiles({"/proc/self/mountinfo":mountinfo("rw,vers=4.2,hard,proto=tcp")}));
 expect(statusOf(defaults)["options"]).toBe("warn:options.unreadable");
});

test("a machine that has not joined, the wrong group, a store that cannot be written and a dead cache fail, and a missing group is refused",async()=>{
 const unjoined=await preflight("a1","linux",fake({...linuxFiles(),"/home/test/semio/.cache/f/machine.json":undefined as unknown as string},linuxCommands()));
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

test("the access check is ok without a record and fails with the exact consent instruction on macOS, in both languages, and with the permission instruction elsewhere",async()=>{
 const record=(origin:string,state:string)=>JSON.stringify({version:1,originId:origin,state,wallMs:1,worktree:"/Users/test/Documents/semio",platform:"darwin",error:"EPERM"});
 const macFiles=(state?:string):Record<string,string>=>({"/Users/test/Documents/semio/.cache/f/machine.json":JSON.stringify({originId:"b2-m01",groupId:"b2",storeRoot:"/Volumes/semio-store"}),...state===undefined?{}:{"/Volumes/semio-store/v1/origins/b2-m01/access.json":record("b2-m01",state)}});
 const flags="/Volumes/semio-store from nfs.b2.example.test:/Users/Shared/store-b\n -- Current mount parameters:\n    NFS parameters: vers=3,tcp,hard,nolocks,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache\n";
 const listing="nfs.b2.example.test:/Users/Shared/store-b on /Volumes/semio-store (nfs, nodev, nosuid, mounted by op)\n";
 const commands=(command:string,args:readonly string[]):{code:number;stdout:string}=>command==="/sbin/mount"?{code:0,stdout:listing}:command==="/usr/bin/nfsstat"?{code:0,stdout:flags}:command==="launchctl"?{code:0,stdout:"state = running\n"}:command==="/usr/sbin/systemsetup"?{code:0,stdout:"Network Time: On\n"}:{code:127,stdout:""};
 const run=async(state:string|undefined,language:"en"|"de"="en")=>{const report=await preflightV1(topology,{platform:"macos",group:"b2",home:"/Users/test",uid:501,timeoutMilliseconds:1000,languages:[language],signal:new AbortController().signal,progress:()=>{},probes:fake(macFiles(state),commands)});return report;};
 expect(statusOf(await run(undefined))["access"]).toBe("ok:access.ok");
 const denied=await run("denied");
 expect(statusOf(denied)["access"]).toBe("fail:access.denied.macos");expect(denied.ok).toBe(false);
 expect(denied.text).toContain("worktree-access-denied: the replica cannot read /Users/test/Documents/semio");expect(denied.text).toContain("Privacy & Security > Files & Folders");
 expect((await run("denied","de")).text).toContain("Datenschutz & Sicherheit > Dateien & Ordner");
 expect(statusOf(await run("waiting-for-consent"))["access"]).toBe("fail:access.waiting.macos");
 const linux=await preflight("a1","linux",fake(linuxFiles({"/mnt/semio-store/v1/origins/a1-m01/access.json":record("a1-m01","denied")}),linuxCommands()));
 expect(statusOf(linux)["access"]).toBe("fail:access.denied.other");expect(linux.text).toContain("read and write permission");
 expect(statusOf(await run("broken"))["access"]).toBe("ok:access.ok");
});

test("the access check names the role that lacks the consent, in both languages, once per machine identity, and a role that started fine clears it",async()=>{
 const home="/Users/test",worktree="/Users/test/Documents/semio",directory=accessDirectoryV1(worktree,{platform:"darwin",home,environment:{}}),identity=`${home}/.local/share/semio/launcher/bin/repo`;
 const receipt=(role:string,state:string,at=NOW-1000)=>JSON.stringify({version:1,role,pid:4242,state,root:worktree,folder:"Documents",identity,at});
 const macFiles=(extra:Record<string,string>):Record<string,string>=>({[`${worktree}/.cache/f/machine.json`]:JSON.stringify({originId:"b2-m01",groupId:"b2",storeRoot:"/Volumes/semio-store"}),...extra});
 const flags="/Volumes/semio-store from nfs.b2.example.test:/Users/Shared/store-b\n -- Current mount parameters:\n    NFS parameters: vers=3,tcp,hard,nolocks,acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache\n";
 const listing="nfs.b2.example.test:/Users/Shared/store-b on /Volumes/semio-store (nfs, nodev, nosuid, mounted by op)\n";
 const commands=(command:string):{code:number;stdout:string}=>command==="/sbin/mount"?{code:0,stdout:listing}:command==="/usr/bin/nfsstat"?{code:0,stdout:flags}:command==="launchctl"?{code:0,stdout:"state = running\n"}:command==="/usr/sbin/systemsetup"?{code:0,stdout:"Network Time: On\n"}:{code:127,stdout:""};
 const run=async(files:Record<string,string>,language:"en"|"de"="en")=>preflightV1(topology,{platform:"macos",group:"b2",home,uid:501,timeoutMilliseconds:1000,languages:[language],signal:new AbortController().signal,progress:()=>{},probes:fake(macFiles(files),commands)});
 expect(statusOf(await run({}))["access"]).toBe("ok:access.ok");
 expect(statusOf(await run({[`${directory}/service.json`]:receipt("service","ok"),[`${directory}/client.json`]:receipt("client","unprotected")}))["access"]).toBe("ok:access.ok");
 const denied=await run({[`${directory}/service.json`]:receipt("service","denied"),[`${directory}/client.json`]:receipt("client","ok")});
 expect(statusOf(denied)["access"]).toBe("fail:access.role.denied");expect(denied.ok).toBe(false);
 expect(denied.text).toContain("these macOS roles cannot read /Users/test/Documents/semio: service");expect(denied.text).not.toContain("client, service");expect(denied.text).toContain(identity);expect(denied.text).toContain("Files & Folders");expect(denied.text).toContain("Documents Folder");
 const german=await run({[`${directory}/service.json`]:receipt("service","denied"),[`${directory}/client.json`]:receipt("client","denied")},"de");
 expect(german.text).toContain("client, service");expect(german.text).toContain("Dateien & Ordner");expect(german.text).toContain("Datenschutz & Sicherheit");
 expect(statusOf(await run({[`${directory}/client.json`]:receipt("client","waiting")}))["access"]).toBe("fail:access.role.waiting");
 const launcher=await run({[`${directory}/launcher.json`]:receipt("launcher","waiting")});
 expect(statusOf(launcher)["access"]).toBe("fail:access.role.waiting");expect(launcher.text).toContain("launcher");expect(launcher.text).toContain("repo would like to access files in your Documents folder");
 expect(statusOf(await run({[`${directory}/service.json`]:receipt("service","denied",NOW-7*3600_000)}))["access"]).toBe("ok:access.ok");
 expect(statusOf(await run({[`${directory}/service.json`]:"{broken"}))["access"]).toBe("ok:access.ok");
 expect(statusOf(await run({[`${directory}/service.json`]:receipt("client","denied")}))["access"]).toBe("ok:access.ok");
 expect(statusOf(await run({"/Volumes/semio-store/v1/origins/b2-m01/access.json":JSON.stringify({version:1,originId:"b2-m01",state:"denied",wallMs:1,worktree,platform:"darwin",error:"EPERM"}),[`${directory}/service.json`]:receipt("service","denied")}))["access"]).toBe("fail:access.denied.macos");
});

test("a native bootstrap that ends with the consent code 4 ends the provision with exit code 4 and the click to make, not with the generic pending code 3",()=>{
 const body=/native_consent\(\) \{[\s\S]*?\n\}\n/u.exec(provisionShell)?.[0];
 expect(body).toBeDefined();
 expect(provisionShell).toContain("EXIT_CONSENT=4");
 expect(provisionShell).toMatch(/\[ "\$status" != "\$EXIT_CONSENT" \] \|\| native_consent\n\s*\[ "\$status" = 0 \] \|\| native_pending/u);
 const script=`say() { printf '%s\\n' "$*"; }; ORIGIN_ID=b2-m01; GROUP_ID=b2; WORKTREE=/Users/t/Documents/semio; WORKER_USER=t; WORKER_HOME=/Users/t; BOOTSTRAP_LOG=/nonexistent; EXIT_CONSENT=4; ${body}\nnative_consent`;
 const result=spawnSync("bash",["-c",script],{encoding:"utf8"});
 expect(result.status).toBe(4);
 for(const needle of ["Allow","would like to access files in your Documents folder","Privacy & Security > Files & Folders","/Users/t/.local/share/semio/launcher/bin/repo","once per machine","exit code 4"])expect(result.stdout,needle).toContain(needle);
 expect(spawnSync("bash",["-n","-c",provisionShell]).status).toBe(0);
});

test("on macOS /usr/local/bin/repo links to the launcher itself, the one identity of the file access consent, not to the user command that the native build replaces",()=>{
 expect(provisionShell).toMatch(/if \[ "\$PLATFORM" = macos \] && \[ -x "\$LAUNCHER_BIN\/repo" \]; then\n\s*ln -sfn "\$LAUNCHER_BIN\/repo" \/usr\/local\/bin\/repo/u);
 expect(provisionShell).toMatch(/else\n\s*ln -sfn "\$WORKER_HOME\/\.local\/bin\/repo" \/usr\/local\/bin\/repo/u);
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
 const commands=(listing:string,nfsstat:string,network:string="Network Time: On\n")=>(command:string,args:readonly string[]):{code:number;stdout:string}=>command==="/sbin/mount"?{code:0,stdout:listing}:command==="/usr/bin/nfsstat"?{code:nfsstat===""?1:0,stdout:nfsstat}:command==="launchctl"?{code:0,stdout:args[1]==="gui/501/tech.semio.fleet.replica"?"state = running\n":"state = waiting\n"}:command==="/usr/sbin/systemsetup"?{code:0,stdout:network}:{code:127,stdout:""};
 const listing="nfs.b2.example.test:/Users/Shared/store-b on /Volumes/semio-store (nfs, nodev, nosuid, mounted by op)\n";
 const files={"/Users/test/Documents/semio/.cache/f/machine.json":JSON.stringify({originId:"b2-m01",groupId:"b2",storeRoot:"/Volumes/semio-store"})};
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
  if(command==="reg.exe")return {code:0,stdout:args[1]!.includes("W32Time")?`\n${args[1]}\n    Type    REG_SZ    ${overrides["w32time-type"]?.stdout??"NT5DS"}\n`:args[1]!.includes("FileSystem")?registry("FileSystem","LongPathsEnabled",longPaths):registry("Default",args[3]!,nfs)};
  if(command==="powershell.exe")return {code:0,stdout:args[3]!.includes("semio-fleet-replica")?"Running\r\n":"Ready\r\n"};
  if(command==="sc.exe")return {code:0,stdout:"SERVICE_NAME: w32time\n        TYPE               : 20  WIN32_SHARE_PROCESS\n        STATE              : 4  RUNNING\n"};

  return {code:127,stdout:""};
 };
 const files={"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"a1-m01",groupId:"a1",storeRoot:"S:\\"})},windows=(extra:Record<string,{code:number;stdout:string}>={},nfs?:number,longPaths?:number)=>preflight("a1","windows",fake(files,commands(extra,nfs,longPaths),{status:async()=>HEALTHY}));
 const good=await windows();
 expect(good.ok).toBe(true);
 expect(statusOf(good)).toMatchObject({mount:"ok:mount.ok",options:"skip:options.skip",longpaths:"ok:longpaths.ok",clientcache:"ok:clientcache.nfs",timesync:"ok:timesync.ok","service.replica":"ok:service.ok","service.compact":"ok:compact.ok"});
 expect(good.text).toContain("not verified on a Windows machine");
 expect(statusOf(await windows({},1))["clientcache"]).toBe("fail:clientcache.wrong");
 expect(statusOf(await windows({},0,0))["longpaths"]).toBe("fail:longpaths.off");
 expect(statusOf(await windows({"w32time-type":{code:0,stdout:"NoSync"}}))["timesync"]).toBe("warn:timesync.no");
 expect(statusOf(await windows({"sc.exe query":{code:0,stdout:"        STATE              : 1  STOPPED\n"}}))["timesync"]).toBe("warn:timesync.no");
 expect(statusOf(await windows({"sc.exe query":{code:1,stdout:"The specified service does not exist."}}))["timesync"]).toBe("warn:timesync.no");
 expect(statusOf(await windows({"powershell.exe -NoProfile":{code:0,stdout:"Ready\r\n"}}))["service.replica"]).toBe("fail:service.down");
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
 const lines:string[]=[],both=await preflightV1(topology,{platform:"linux",group:"a1",home:"/home/test",timeoutMilliseconds:1000,languages:["en","de"],signal:new AbortController().signal,progress:line=>lines.push(line),probes:fake(linuxFiles(),linuxCommands())});
 expect(lines.length).toBe(both.checks.length*2);
 expect(lines.filter(line=>/^\[ ?\d+\/24\] ok   Speicher-Einbindung: /u.test(line)).length).toBe(1);
 expect(lines.filter(line=>/^\[ ?\d+\/24\] ok   store mount: /u.test(line)).length).toBe(1);
 expect(both.text).toContain("Preflight von a1-m01 (Gruppe a1, Worker, linux)");
 expect(both.text).toContain("Alle 24 Prüfungen bestanden.");
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
 const context=(probes:Partial<PreflightProbesV1>,environment:Record<string,string>={})=>({platform:"linux",home:"/home/test",topology:fixture.topology,signal:new AbortController().signal,progress:()=>{},environment,probes});
 const good=await runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","de"],context(fake(linuxFiles(),linuxCommands())));
 expect(good.exitCode).toBe(0);
 expect(good.text).toContain("Alle 24 Prüfungen bestanden.");
 expect(good.text).not.toContain("All 24 checks passed.");
 const english=await runTopologyCommandAsyncV1(["preflight","--lang","en"],context(fake(linuxFiles(),linuxCommands())));
 expect(english.text).toContain("All 24 checks passed.");
 const bad=await runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","en"],context(fake(linuxFiles({"/proc/self/mountinfo":""}),linuxCommands())));
 expect(bad.exitCode).toBe(1);
 expect(bad.text).toContain("fix the failures before the machine joins the fleet");
 await expect(runTopologyCommandAsyncV1(["preflight","--group","a1","--lang","fr"],context(fake(linuxFiles(),linuxCommands())))).rejects.toThrow("Unknown language: fr");
 await expect(runTopologyCommandAsyncV1(["preflight","--group","a1","--timeout","5"],context(fake(linuxFiles(),linuxCommands())))).rejects.toThrow("--timeout must be a whole number");
 expect(preflightLanguagesV1(undefined,{}).length).toBe(2);
 expect(preflightLanguagesV1("en",{})).toEqual(["en"]);
});

test("the preflight reads the watcher mode and the event visibility from the running replica, and on Windows whether the replica starts without a logon",async()=>{
 const running=(change:object):Partial<PreflightProbesV1>=>({status:async()=>({...HEALTHY,...change}) as unknown as StatusReportV1});
 const health=(failures:readonly object[])=>({version:1,wallMs:1,store:"ok",storeSinceMs:0,code:"x",pending:null,failures});
 const latency=(apply:number,p95:number|null,p50:number|null=40)=>({publishMs:{p50:null,p95:null},applyMs:{p50,p95},samples:{publish:0,apply},tiers:{}});
 const run=async(change:object)=>statusOf(await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running(change))));
 expect(await run({health:health([])})).toMatchObject({watcher:"ok:watcher.ok",visibility:"skip:visibility.skip"});
 expect(await run({health:health([{kind:"watch",count:1,lastMs:1}])})).toMatchObject({watcher:"warn:watcher.degraded"});
 expect(await run({health:null})).toMatchObject({watcher:"skip:watcher.skip"});
 expect(await run({daemon:{running:false,pid:null},health:null})).toMatchObject({watcher:"skip:watcher.skip"});
 expect(await run({latency:latency(3,90)})).toMatchObject({visibility:"skip:visibility.skip"});
 expect(await run({latency:latency(50,null)})).toMatchObject({visibility:"skip:visibility.skip"});
 expect(await run({latency:latency(50,900)})).toMatchObject({visibility:"ok:visibility.ok"});
 const slow=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running({latency:latency(50,PREFLIGHT_LIMITS_V1.visibilityWarnMilliseconds+1,2000)})));
 expect(statusOf(slow)["visibility"]).toBe("warn:visibility.slow");
 expect(slow.text).toContain("reach this linux machine slowly");
 const german=await preflight("a1","linux",fake(linuxFiles(),linuxCommands(),running({health:health([{kind:"watch",count:1,lastMs:1}])})),{languages:["de"]});
 expect(german.text).toContain("Dateiwächter wurde unter linux beendet");
 const windowsRun=(registry:string)=>preflight("a1","windows",fake({"C:\\semio\\.cache\\f\\machine.json":JSON.stringify({originId:"a1-m01",groupId:"a1",storeRoot:"S:\\"})},(command,args)=>command==="reg.exe"&&args.join(" ").includes("AutoAdminLogon")?{code:registry===""?1:0,stdout:registry}:{code:127,stdout:""},{status:async()=>HEALTHY}),{worktree:"C:\\semio"});
 expect(statusOf(await windowsRun("    AutoAdminLogon    REG_SZ    1\r\n"))["logon"]).toBe("ok:logon.auto");
 expect(statusOf(await windowsRun("    AutoAdminLogon    REG_SZ    0\r\n"))["logon"]).toBe("warn:logon.manual");
 expect(statusOf(await windowsRun(""))["logon"]).toBe("warn:logon.manual");
 expect(Object.keys(statusOf(await preflight("a1","linux",fake(linuxFiles(),linuxCommands())))).includes("logon")).toBe(false);
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
 for(const id of Object.keys(LINUX_OK).concat(["disk.store","service.cache","service.relay","clientcache","longpaths","logon"]))expect(catalogue.messages[`preflight.name.${id}`],id).toBeDefined();
});

test("the group server exports the store to exactly the named machines, never to the whole network, when SEMIO_FLEET_MACHINES is set",()=>{
 const environment={SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_NFS_HOST:"nfs.example.test",SEMIO_FLEET_NETWORK:"203.0.113.0/24",SEMIO_FLEET_MACHINES:"203.0.113.5,203.0.113.6"};
 const mac=runTopologyCommandV1(["server-plan","--group","g1","--part","exports"],{platform:"darwin",environment});
 expect(mac).toContain("-mapall=");expect(mac).toContain("203.0.113.5 203.0.113.6");expect(mac).not.toContain("-network 203.0.113.0");
 expect(()=>runTopologyCommandV1(["server-plan","--group","g1","--part","exports"],{platform:"darwin",environment:{...environment,SEMIO_FLEET_MACHINES:"203.0.113.5,not-an-address"}})).toThrow();
});

test("every group declares the ports of its services and the network policy, links name only mounted peers, and the peer store is found on each platform",()=>{
 const topology=readTopologyV1(),ajv=new Ajv({strict:true,allowUnionTypes:true}),check=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Topology"});
 expect(check(committed),JSON.stringify(check.errors)).toBe(true);
 for(const group of topology.groups){expect(group.ports,group.id).toEqual({cache:6094,scheduler:8980,worker:8981});expect(group.access).toEqual({enforceNetwork:false,transport:{crossGroup:"sealed",inGroup:"trusted-lan"}});}
 for(const group of topology.groups)expect(new URL(group.cache.endpoint).port,group.id).toBe(String(group.ports!.cache));
 expect(()=>{const copy=structuredClone(committed) as any;copy.groups[0].cache.endpoint="http://cache.g1.semio.internal:6090";readTopologyV1(copy);}).toThrow("needs the port of cache.endpoint to be ports.cache 6094");
 const g1=groupV1(topology,"g1"),g2=groupV1(topology,"g2");
 expect(peerStorePathV1(topology,g1,"g2"),"a macOS group server mounts the peers below /Volumes").toBe("/Volumes/semio-peers/g2");
 expect(peerStorePathV1(topology,g2,"g1")).toBe("/mnt/semio-peers/g1");
 const withLinks=(links:unknown)=>({...committed,groups:committed.groups.map(group=>group.id==="g1"?{...group,links}:group)});
 const key="A".repeat(60);
 expect(()=>readTopologyV1(withLinks({g2:{transport:"direct",endpoints:["https://cache.g2.example.org:7000"],serverKey:key},g3:{transport:"relayed"}}))).not.toThrow();
 expect(()=>readTopologyV1(withLinks({g9:{transport:"direct"}}))).toThrow("does not mount");
 expect(()=>readTopologyV1(withLinks({g2:{transport:"relayed",endpoints:["https://cache.g2.example.org:7000"]}}))).toThrow("relayed link");
 expect(()=>readTopologyV1(withLinks({g2:{transport:"pigeon"}}))).toThrow();
 expect(()=>readTopologyV1({...committed,groups:committed.groups.map(group=>group.id==="g1"?{...group,ports:{cache:0,scheduler:1,worker:2}}:group)})).toThrow();
 expect(()=>readTopologyV1({...committed,groups:committed.groups.map(group=>group.id==="g1"?{...group,server:{...group.server,advertise:["cache.g1.example.org"]}}:group)})).not.toThrow();
 const withAccess=(access:unknown)=>({...committed,groups:committed.groups.map(group=>group.id==="g1"?{...group,access}:group)});
 expect(()=>readTopologyV1(withAccess({enforceNetwork:false}))).not.toThrow();
 expect(()=>readTopologyV1(withAccess({enforceNetwork:false,transport:{inGroup:"sealed"}}))).not.toThrow();
 expect(()=>readTopologyV1(withAccess({enforceNetwork:false,transport:{crossGroup:"plain"}})),"a request of another group is never plain").toThrow();
 expect(()=>readTopologyV1(withAccess({enforceNetwork:false,transport:{inGroup:"clear"}}))).toThrow();
 const python=testPythonV1();
 if(python!==undefined){const run=spawnSync(python,["-I",join(import.meta.dirname,"🐍️.py"),"check"],{encoding:"utf8"});expect(run.status,run.stdout+run.stderr).toBe(0);}
});

test("every service definition declares its scheduling class: ProcessType on macOS (plutil reads it back), Nice and scheduling policy on Linux, Priority in the Windows task",()=>{
 const group=fixture.topology.groups[0]!.id,expected:Record<string,string>={interactive:"Interactive",standard:"Standard",background:"Background"};
 const mac=workerPartsV1(topology,group,"macos"),linux=workerPartsV1(topology,group,"linux"),windows=workerPartsV1(topology,group,"windows");
 expect(mac["definition"]).toContain("<key>ProcessType</key><string>Interactive</string>");
 expect(mac["definition"]).not.toContain("LowPriorityIO");
 expect(mac["compact-definition"]).toContain("<key>ProcessType</key><string>Background</string>");
 expect(mac["compact-definition"]).toContain("<key>LowPriorityIO</key><true/>");
 expect(mac["compact-definition"]).toContain("<key>Nice</key><integer>10</integer>");
 expect(linux["launch-definition"]).toContain("renice -n -5 -p $$");
 expect(linux["launch-definition"]).toContain("ionice -c 2 -n 2 -p $$");
 expect(linux["compact-launch-definition"]).toContain("renice -n 10 -p $$");
 expect(linux["compact-launch-definition"]).toContain("chrt --idle -p 0 $$");
 expect(linux["compact-launch-definition"]).toContain("ionice -c 3 -p $$");
 for(const unit of [linux["definition"]!,linux["compact-definition"]!])for(const key of["Nice=","CPUSchedulingPolicy=","IOSchedulingClass="])expect(unit).not.toContain(key);
 expect(windows["definition"]).toContain("<Priority>3</Priority>");
 expect(windows["compact-definition"]).toContain("<Priority>8</Priority>");
 expect(windows["mount-definition"]).not.toContain("<Priority>");
 for(const id of ids)for(const platform of PLATFORMS){
  let parts:Readonly<Record<string,string>>;
  try{parts=serverPartsV1(topology,id);}catch{continue;}
  if(platform==="linux"&&parts["cache-unit"]?.includes("[Service]")){expect(parts["cache-launch"]).toContain("renice -n -5");expect(parts["relay-launch"]).toContain("renice -n 0");expect(parts["cache-unit"]).not.toContain("Nice=");}
  if(parts["cache-unit"]?.includes("<plist")){expect(parts["cache-unit"]).toContain("<key>ProcessType</key><string>Interactive</string>");expect(parts["relay-unit"]).toContain("<key>ProcessType</key><string>Standard</string>");}
  if(parts["cache-unit"]?.startsWith("<?xml version=\"1.0\" encoding=\"UTF-16\"")){expect(parts["cache-unit"]).toContain("<Priority>3</Priority>");expect(parts["relay-unit"]).toContain("<Priority>5</Priority>");}
 }
 if(process.platform==="darwin"){
  for(const [name,body] of [["definition",mac["definition"]!],["compact-definition",mac["compact-definition"]!]] as const){
   const file=join(mkdtempSync(join(tmpdir(),"semio-e27-")),"agent.plist");writeFileSync(file,body.replaceAll("__WORKER_HOME__","/Users/test").replaceAll("__WORKER_USER__","test"));
   expect(spawnSync("plutil",["-lint",file]).status).toBe(0);
   const read=spawnSync("plutil",["-extract","ProcessType","raw",file],{encoding:"utf8"});
   expect(read.stdout.trim()).toBe(expected[name==="definition"?"interactive":"background"]!);
  }
 }
});

test("a crashed Windows service restarts although the headless console host hides its exit status: the hosted script reruns every non-zero exit, and the mount and compaction tasks stay plain",()=>{
 const windows=workerPartsV1(topology,"c3","windows"),server=serverPartsV1(topology,"c3");
 for(const xml of [windows["definition"]!,server["cache-unit"]!,server["relay-unit"]!]){
  const script=hostedScript(xml);
  expect(script).toStartWith("do{& ");expect(script).toContain(" --% ");expect(script).toContain("$c=$LASTEXITCODE;if($c -eq 0){exit 0};Start-Sleep -Seconds 5}while($true)");
  expect(xml).toContain("<RestartOnFailure>");
  const python=testPythonV1();
  if(python){
   const run=spawnSync(python,["-I","-c","import base64,re,sys;print(base64.b64decode(re.search(r'-EncodedCommand (\\S+?)<',sys.stdin.read()).group(1)).decode('utf-16-le'),end='')"],{input:xml,encoding:"utf8",windowsHide:true});
   expect(run.status,run.stderr).toBe(0);expect(run.stdout).toBe(script);
  }
 }
 for(const xml of [windows["mount-definition"]!,windows["compact-definition"]!])expect(xml).not.toContain("-EncodedCommand");
 expect(windows["mount-definition"]).toContain("<Arguments>--headless cmd.exe ");
});
