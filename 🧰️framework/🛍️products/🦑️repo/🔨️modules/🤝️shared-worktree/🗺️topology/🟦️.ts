import {validateJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import topologyJson from "./🔣️.json" with {type:"json"};
import buildPolicy from "../../📚️library/🏗️build/🔣️policy.json" with {type:"json"};

/** 🖥️ The operating systems of the fleet; the canonical worktree and store mount are fixed per one of them. */
export type FleetPlatformV1="linux"|"macos"|"windows";

type ByPlatform=Readonly<Record<FleetPlatformV1,string>>;

/** 🖥️ The group server: its operating system, the protocol of its share, the network of its workers and, on Windows, the local folder of the share, shaped by `🧬️schema/🔣️.json`. */
export type FleetServerV1=Readonly<{platform:FleetPlatformV1;protocol:"nfs"|"smb";network:string;directory?:string}>;

/** 👥️ One group of machines with its NFS store, its cache server and the read-only mounts of the other groups, shaped by `🧬️schema/🔣️.json`. */
export type FleetGroupV1=Readonly<{id:string;nfs:Readonly<{host:string;export:string}>;server:FleetServerV1;cache:Readonly<{endpoint:string;instance:string}>;peers:Readonly<Record<string,string>>}>;

/** 🗺️ The committed fleet topology, shaped by `🧬️schema/🔣️.json`. */
export type FleetTopologyV1=Readonly<{version:1;worktree:ByPlatform;storeMount:ByPlatform;groups:readonly FleetGroupV1[];replica:Readonly<{pollMilliseconds:number;scanSeconds:number;debounceMilliseconds:number;retentionDays:number;bootstrapMinutes:number}>}>;

const MODULE="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree",
 REPLICA=`${MODULE}/🔁️replica`,
 PROVISION="🥾️provision/🐚️.sh",
 REPO="/usr/local/bin/repo",
 STORE_USER="semio",
 STORE_HOME="/var/lib/semio",
 STORE_ID=1500,
 CACHE_BYTES=214748364800,
 CACHE_DIRECTORY="/var/lib/semio-remote-cache",
 LINUX_OPTIONS="vers=4.2,hard,noatime,actimeo=1,lookupcache=none,nconnect=4",
 PEER_OPTIONS="vers=4.2,ro,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,_netdev,nofail",
 MACOS_ATTRIBUTES="acregmin=1,acregmax=1,acdirmin=0,acdirmax=0,nonegnamecache",
 MACOS_OPTIONS=`vers=4,hard,nobrowse,${MACOS_ATTRIBUTES}`,
 WINDOWS_OPTIONS="anon,nolock,mtype=hard",
 MACOS_MOUNT_LABEL="tech.semio.fleet.store-mount",
 MACOS_MOUNT_PLIST=`/Library/LaunchDaemons/${MACOS_MOUNT_LABEL}.plist`,
 MACOS_REPLICA_LABEL="tech.semio.fleet.replica",
 MACOS_REPLICA_PLIST=`/Library/LaunchDaemons/${MACOS_REPLICA_LABEL}.plist`,
 MACOS_COMPACT_LABEL="tech.semio.fleet.compact",
 MACOS_COMPACT_PLIST=`/Library/LaunchDaemons/${MACOS_COMPACT_LABEL}.plist`,
 LINUX_REPLICA_UNIT="/etc/systemd/system/semio-fleet-replica@.service",
 LINUX_COMPACT_UNIT="/etc/systemd/system/semio-fleet-compact@.service",
 LINUX_COMPACT_TIMER="/etc/systemd/system/semio-fleet-compact@.timer",
 WINDOWS_REPLICA_TASK="semio-fleet-replica",
 WINDOWS_COMPACT_TASK="semio-fleet-compact",
 WINDOWS_MOUNT_TASK="semio-fleet-store-mount";

/** 👤️ The placeholder the macOS replica LaunchDaemon carries for the account that owns the worktree; the provisioning script replaces it. */
export const FLEET_WORKER_USER_PLACEHOLDER_V1="__WORKER_USER__";

/** 🏠️ The placeholder the macOS LaunchDaemons carry for the home directory of that account (the `repo` launcher keeps its runtimes there, and the daemon must not depend on launchd setting `HOME`); the provisioning script replaces it. */
export const FLEET_WORKER_HOME_PLACEHOLDER_V1="__WORKER_HOME__";

/** 📖️ Reads the committed topology (or the given value) and refuses any drift from its schema, duplicate group ids, peer sets that are not exactly the other groups and server settings that cannot work (smb off Windows, a Windows server without a folder or share name, a macOS server that exports outside /Users and /Volumes, a macOS server next to an smb group). */
export function readTopologyV1(value:unknown=topologyJson):FleetTopologyV1{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Topology"},value,schema);
 if(errors.length)throw Error(`Closed fleet topology required: ${errors[0]}`);
 const topology=value as FleetTopologyV1,ids=topology.groups.map(group=>group.id);
 if(new Set(ids).size!==ids.length)throw Error("Closed fleet topology required: group ids must be unique");
 for(const group of topology.groups){
  const expected=ids.filter(id=>id!==group.id).sort().join(),actual=Object.keys(group.peers).sort().join();
  if(expected!==actual)throw Error(`Closed fleet topology required: ${group.id} must mount exactly the peers ${expected}`);
  const {server}=group,refuse=(reason:string):never=>{throw Error(`Closed fleet topology required: ${group.id} ${reason}`);};
  if(server.protocol==="smb"&&server.platform!=="windows")refuse("shares over smb, which only a windows server does");
  if(server.platform==="windows"){
   if(server.directory===undefined)refuse("needs server.directory, the local folder of the share, on a windows server");
   if(!/^\/[A-Za-z0-9._-]+$/u.test(group.nfs.export))refuse("needs a single-segment share name such as /semio-store as nfs.export on a windows server");
  }else if(server.directory!==undefined)refuse("takes server.directory only on a windows server");
  if(server.platform==="macos"&&!/^\/(Users|Volumes)\//u.test(group.nfs.export))refuse("needs an nfs.export below /Users or /Volumes on a macos server");
 }
 if(topology.groups.some(group=>group.server.protocol==="smb")&&topology.groups.some(group=>group.server.platform==="macos"))throw Error("Closed fleet topology required: a macos server cannot read the smb store of another group");
 return Object.freeze(topology);
}

const FLEET_GROUP_ENVIRONMENT_V1="SEMIO_FLEET_GROUP",FLEET_HOST_ENVIRONMENT_V1="SEMIO_FLEET_NFS_HOST",FLEET_NETWORK_ENVIRONMENT_V1="SEMIO_FLEET_NETWORK",FLEET_CACHE_ENVIRONMENT_V1="SEMIO_FLEET_CACHE_ENDPOINT";
const environmentText=(environment:Readonly<Record<string,string|undefined>>,name:string):string|undefined=>{const value=environment[name];return value===undefined||value===""?undefined:value;};
const endpointWithHost=(endpoint:string,host:string):string=>{const url=new URL(endpoint);return `${url.protocol}//${host}${url.port===""?"":`:${url.port}`}`;};

/** 🌐️ Replaces one group's host, worker network and cache endpoint from `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK` and optional `SEMIO_FLEET_CACHE_ENDPOINT`. The three required names are set together. Absent names leave the topology as read. A group id that is not in this topology leaves it unchanged. The values are checked with `readTopologyV1`. */
export function applyFleetAddressEnvironmentV1(topology:FleetTopologyV1,environment:Readonly<Record<string,string|undefined>>):FleetTopologyV1{
 const groupId=environmentText(environment,FLEET_GROUP_ENVIRONMENT_V1),host=environmentText(environment,FLEET_HOST_ENVIRONMENT_V1),network=environmentText(environment,FLEET_NETWORK_ENVIRONMENT_V1),endpoint=environmentText(environment,FLEET_CACHE_ENVIRONMENT_V1);
 if(groupId===undefined&&host===undefined&&network===undefined&&endpoint===undefined)return topology;
 if(groupId===undefined||host===undefined||network===undefined)throw Error("SEMIO_FLEET_GROUP, SEMIO_FLEET_NFS_HOST and SEMIO_FLEET_NETWORK are set together");
 if(!topology.groups.some(group=>group.id===groupId))return topology;
 return readTopologyV1({...topology,groups:topology.groups.map(group=>group.id===groupId?{...group,nfs:{...group.nfs,host},server:{...group.server,network},cache:{...group.cache,endpoint:endpoint??endpointWithHost(group.cache.endpoint,host)}}:group)});
}

/** 🔎️ Resolves one group by id and refuses an unknown one. */
export function groupV1(topology:FleetTopologyV1,groupId:string):FleetGroupV1{
 const group=topology.groups.find(candidate=>candidate.id===groupId);
 if(!group)throw Error(`Declared fleet group required: ${groupId} (known: ${topology.groups.map(candidate=>candidate.id).join(", ")})`);
 return group;
}

/** 🧭️ Maps a Node platform name (default: this process) to the fleet platform and refuses every other system. */
export function platformV1(nodePlatform:string=process.platform):FleetPlatformV1{
 const found=new Map<string,FleetPlatformV1>([["linux","linux"],["darwin","macos"],["win32","windows"]]).get(nodePlatform);
 if(!found)throw Error(`Supported fleet platform required: ${nodePlatform}`);
 return found;
}

/** 🌍️ Renders the environment of a machine of one group: its group's cache endpoint with read-write access and the cache instance, under the variable names of the build policy. */
export function machineEnvironmentV1(topology:FleetTopologyV1,groupId:string):Readonly<Record<string,string>>{
 const group=groupV1(topology,groupId),names=buildPolicy.remoteCache.environment;
 return Object.freeze({[names.endpoint]:group.cache.endpoint,[names.access]:"read-write",[names.instance]:group.cache.instance});
}

const SCRIPT="./📜️script.ts",
 TOPOLOGY_FOLDER="🗺️topology",
 PROVISION_POWERSHELL=PROVISION.replace(/[^/]+$/u,"🔵️.ps1"),
 LINUX_V3_OPTIONS="vers=3,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,nolock",
 PEER_V3_OPTIONS="vers=3,ro,hard,noatime,actimeo=1,lookupcache=none,nconnect=4,nolock,_netdev,nofail",
 PEER_SMB_OPTIONS="ro,vers=3.1.1,credentials=/etc/semio/smb-peer.credentials,actimeo=1,_netdev,nofail",
 MACOS_V3_OPTIONS=`vers=3,hard,nobrowse,${MACOS_ATTRIBUTES},nolocks`,
 WINDOWS_SMB_CACHE:Readonly<Record<string,number>>={DirectoryCacheLifetime:0,FileInfoCacheLifetime:0,FileNotFoundCacheLifetime:0},
 WINDOWS_NFS_KEY="HKLM:\\SOFTWARE\\Microsoft\\ClientForNFS\\CurrentVersion\\Users\\Default\\Cache",
 WINDOWS_NFS_CACHE:Readonly<Record<string,number>>={RemoteWriteCache:0,FileAttributeCache:0}, MACOS_PEERS="/Volumes/semio-peers",
 MACOS_CACHE_LABEL="tech.semio.fleet.cache",
 MACOS_RELAY_LABEL="tech.semio.fleet.relay",
 MACOS_PEERS_LABEL="tech.semio.fleet.peer-mounts",
 WINDOWS_CACHE_TASK="semio-fleet-cache",
 WINDOWS_RELAY_TASK="semio-fleet-relay",
 WINDOWS_CACHE_DIRECTORY="C:\\semio-remote-cache",
 WINDOWS_TOKENS="C:\\ProgramData\\semio\\remote-cache.env",
 TOKENS="/etc/semio/remote-cache.env",
 SMB_USER="semio-fleet",
 SMB_PEER_USER="semio-fleet-peer",
 SYSCTL_PATH="/etc/sysctl.d/90-semio-fleet.conf",
 SERVER_SYSCTL_PATH="/etc/sysctl.d/91-semio-fleet-server.conf",
 SERVER_SYSCTL="fs.leases-enable=0",
 SYSCTL=["fs.inotify.max_user_watches=1048576","fs.inotify.max_user_instances=1024"].join("\n"),
 PLIST_HEAD=['<?xml version="1.0" encoding="UTF-8"?>','<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">','<plist version="1.0">','<dict>'];

type FleetPeerV1=Readonly<{group:FleetGroupV1;path:string}>;

const xml=(text:string):string=>text.replaceAll("&","&amp;").replaceAll("<","&lt;").replaceAll(">","&gt;").replaceAll('"',"&quot;");
const maskOf=(network:string):string=>{
 const bits=Number(network.split("/")[1]),value=bits===0?0:(0xFFFFFFFF<<(32-bits))>>>0;
 return [24,16,8,0].map(shift=>(value>>>shift)&255).join(".");
};
const addressOf=(network:string):string=>network.split("/")[0]!;

/** 🔀️ Names the protocol by which a worker of one platform mounts the store of a group: `nfs4` (Linux and macOS workers of a Linux server), `nfs3` (every worker of a macOS or Windows NFS server, and Windows Client for NFS, which speaks NFSv3 only) or `smb` (Windows workers of a Windows server that has no Server for NFS). A linux or macOS worker of an SMB group has no protocol and is refused. */
export type FleetClientProtocolV1="nfs4"|"nfs3"|"smb";

/** 🔀️ Resolves the protocol by which a worker of the given platform mounts the store of one group, and refuses the workers that an SMB group cannot serve. */
export function clientProtocolV1(group:FleetGroupV1,platform:FleetPlatformV1):FleetClientProtocolV1{
 if(group.server.protocol==="smb"){
  if(platform!=="windows")throw Error(`Windows workers required: ${group.id} shares its store over SMB, which ${platform} workers do not mount`);
  return "smb";
 }
 return group.server.platform==="linux"&&platform!=="windows"?"nfs4":"nfs3";
}

const optionsOf=(group:FleetGroupV1,platform:FleetPlatformV1):string=>{
 const protocol=clientProtocolV1(group,platform);
 if(protocol==="smb")return "";
 return {linux:protocol==="nfs4"?LINUX_OPTIONS:LINUX_V3_OPTIONS,macos:protocol==="nfs4"?MACOS_OPTIONS:MACOS_V3_OPTIONS,windows:WINDOWS_OPTIONS}[platform];
};
const source=(platform:FleetPlatformV1,nfs:FleetGroupV1["nfs"]):string=>platform==="windows"?`\\\\${nfs.host}${nfs.export.replaceAll("/","\\")}`:`${nfs.host}:${nfs.export}`;
const separator=(platform:FleetPlatformV1):string=>platform==="windows"?"\\":"/";
const replicaDirectory=(topology:FleetTopologyV1,platform:FleetPlatformV1):string=>`${topology.worktree[platform]}${separator(platform)}${platform==="windows"?REPLICA.replaceAll("/","\\"):REPLICA}`;
const topologyDirectory=(topology:FleetTopologyV1,platform:FleetPlatformV1):string=>replicaDirectory(topology,platform).replace(/[\\/][^\\/]*$/u,`${separator(platform)}${TOPOLOGY_FOLDER}`);
const archiveDirectory=(topology:FleetTopologyV1,platform:FleetPlatformV1):string=>platform==="windows"?`${topology.storeMount.windows}v1\\bootstrap`:`${topology.storeMount[platform]}/v1/bootstrap`;
const sections=(parts:Readonly<Record<string,string>>):string=>Object.entries(parts).map(([name,body])=>`# ${name}\n${body}`).join("\n\n");
const peersOf=(topology:FleetTopologyV1,group:FleetGroupV1):readonly FleetPeerV1[]=>Object.keys(group.peers).sort().map(id=>{
 const peer=groupV1(topology,id);
 return {group:peer,path:group.server.platform==="linux"?group.peers[id]!:group.server.platform==="macos"?`${MACOS_PEERS}/${id}`:source("windows",peer.nfs)};
});
const peerList=(peers:readonly FleetPeerV1[]):string=>peers.map(peer=>`${peer.group.id}=${peer.path}`).join(",");
const relayArguments=(peers:readonly FleetPeerV1[]):string=>peers.length===0?"":` --peers ${peerList(peers)}`;
const archiveOf=(peers:readonly FleetPeerV1[],platform:FleetPlatformV1):string=>peers.length===0?"":`${peers[0]!.path}${separator(platform)}v1${separator(platform)}bootstrap`;
const listOf=(values:Readonly<Record<string,number>>):string=>Object.entries(values).map(([name,value])=>`${name}=${value}`).join(",");
const smbCacheCommand=`Set-SmbClientConfiguration ${Object.entries(WINDOWS_SMB_CACHE).map(([name,value])=>`-${name} ${value}`).join(" ")} -Force`;
const nfsCacheCommands=[`New-Item -Path ${WINDOWS_NFS_KEY} -Force | Out-Null`,...Object.entries(WINDOWS_NFS_CACHE).map(([name,value])=>`New-ItemProperty -Path ${WINDOWS_NFS_KEY} -Name ${name} -Value ${value} -PropertyType DWord -Force | Out-Null`),"nfsadmin client stop","nfsadmin client start"].join("\n");
const portOf=(group:FleetGroupV1):string=>new URL(group.cache.endpoint).port||(group.cache.endpoint.startsWith("https")?"443":"80");

/** 🧱️ The named pieces of the worker mount plan: Linux `fstab`, `commands`, `sysctl-path` and `sysctl` (the inotify limits of the ~122k directories); macOS `command`, `plist-path` and `plist`; Windows over NFS `feature`, `cache`, `mount` and `task`, over SMB `credential`, `cache`, `mount` and `task` (`cache` sets the attribute and name cache of the client, see `mountPlanV1`). The mount options follow the protocol that `clientProtocolV1` names. */
export function mountPartsV1(topology:FleetTopologyV1,groupId:string,platform:FleetPlatformV1):Readonly<Record<string,string>>{
 const group=groupV1(topology,groupId),{nfs}=group,mount=topology.storeMount[platform],remote=source(platform,nfs),protocol=clientProtocolV1(group,platform),options=optionsOf(group,platform);
 if(platform==="linux")return Object.freeze({
  fstab:`${remote} ${mount} ${protocol==="nfs4"?"nfs4":"nfs"} ${options},_netdev,nofail 0 0`,
  commands:`sudo mkdir -p ${mount}\nsudo mount ${mount}`,
  "sysctl-path":SYSCTL_PATH,
  sysctl:SYSCTL
 });
 if(platform==="macos"){
  const command=`/bin/mkdir -p ${mount}; /sbin/mount_nfs -o ${options} ${remote} ${mount}`;
  return Object.freeze({
   command:`sudo ${command}`,
   "plist-path":MACOS_MOUNT_PLIST,
   plist:[
    ...PLIST_HEAD,
    `<key>Label</key><string>${MACOS_MOUNT_LABEL}</string>`,
    `<key>ProgramArguments</key><array><string>/bin/sh</string><string>-c</string><string>/sbin/mount | /usr/bin/grep -q ' on ${mount} ' || { ${command}; }</string></array>`,
    '<key>RunAtLoad</key><true/>','<key>StartInterval</key><integer>60</integer>',
    '</dict>','</plist>'
   ].join("\n")
  });
 }
 const drive=mount.slice(0,2);
 if(protocol==="smb")return Object.freeze({
  credential:`cmdkey /add:${nfs.host} /user:${nfs.host.split(".")[0]}\\${SMB_USER} /pass:<password>`,
  cache:smbCacheCommand,
  mount:`net use ${drive} ${remote} /persistent:yes`,
  task:`schtasks /Create /TN ${WINDOWS_MOUNT_TASK} /SC ONLOGON /F /TR "net use ${drive} ${remote} /persistent:yes"`
 });
 return Object.freeze({
  feature:"Enable-WindowsOptionalFeature -Online -FeatureName ServicesForNFS-ClientOnly,ClientForNFS-Infrastructure -All -NoRestart",
  cache:nfsCacheCommands,
  mount:`mount.exe -o ${options} ${remote} ${drive}`,
  task:`schtasks /Create /TN ${WINDOWS_MOUNT_TASK} /SC ONLOGON /F /TR "mount.exe -o ${options} ${remote} ${drive}"`
 });
}

/** 🔌️ Renders the commands and configuration that mount the store of one group on a worker (Linux fstab line plus the sysctl.d file with the inotify limits, macOS `mount_nfs` plus a boot-time LaunchDaemon, Windows "Client for NFS" feature plus `mount` to the drive letter, or `net use` for an SMB group). `actimeo=1` caps the attribute cache at one second: followers poll `head.json` every 250 ms and must see a fresh size and mtime, while the default of 3 to 60 seconds would delay every remote save by that long; `hard` keeps writers waiting instead of failing when the server blinks, `lookupcache=none` (Linux) revalidates every name, which matters more than `actimeo`: `head.json` is replaced by a rename, a cached name keeps pointing at the old file for up to `acdirmin` (measured on real NFS 4.2: a `stat` of the new head came after 505 ms median and 960 ms p95 with `lookupcache=positive`, and only 46 of 100 versions were seen at all with the mount defaults, against 10 ms with `lookupcache=none`; an `open` and read is fresh in every mode), and it also shows new segments and blobs at once; it costs about four `LOOKUP` calls per `stat` of a head (32 RPC/s for two origins polled every 250 ms), and `nconnect=4` spreads the many small requests over four TCP connections. The same correctness is set on the macOS client, measured on macOS 27 against a Linux server: it has no `lookupcache`, but `acdirmin=0,acdirmax=0` makes every lookup ask the directory again (a `stat` of a replaced head came after 2 ms median and all 100 versions were seen, against 672 ms and 28 of 100 with the mount defaults and 446 ms with `actimeo=1`), `acregmin=1,acregmax=1` keeps the one-second file attributes (`noac` is as fresh and costs 79 instead of 65 NFS calls per second for four heads polled four times a second) and `nonegnamecache` made no difference to the head; on the other clients it is by reasoning, not by measurement: the Windows SMB client caches directory listings (10 s), file information (10 s) and failed lookups (5 s) by default, so `Set-SmbClientConfiguration -DirectoryCacheLifetime 0 -FileInfoCacheLifetime 0 -FileNotFoundCacheLifetime 0` is the `cache` part of an SMB group; the Windows Client for NFS has no mount option for it, the `cache` part creates the key `HKLM\SOFTWARE\Microsoft\ClientForNFS\CurrentVersion\Users\Default\Cache`, sets the DWORD values `RemoteWriteCache` and `FileAttributeCache` to 0 (documented by Microsoft for older Windows in "Steps to disable Client for NFS cache", https://learn.microsoft.com/en-us/archive/blogs/technet/sfu/steps-to-disable-client-for-nfs-cache; not yet confirmed on Windows 11 or Server 2022) and restarts the Client for NFS service with `nfsadmin client stop` and `nfsadmin client start`, because a mount made before the change keeps its old caches. The store uses only ASCII names, so the character set limits of the Windows NFS client never matter; on Windows call `mount.exe` from PowerShell, where `mount` is an alias of `New-PSDrive`. The inotify limits matter because the tree has about 122,000 directories and the replica daemon needs one watch per directory; the Linux default of 8192 watches would silently drop events. */
export function mountPlanV1(topology:FleetTopologyV1,groupId:string,platform:FleetPlatformV1):string{
 return sections(mountPartsV1(topology,groupId,platform));
}

const environmentOf=`<key>EnvironmentVariables</key><dict><key>HOME</key><string>${FLEET_WORKER_HOME_PLACEHOLDER_V1}</string><key>PATH</key><string>/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin</string></dict>`;
const daemonOf=(label:string,directory:string,program:readonly string[],schedule:readonly string[],log:string):string=>[
 ...PLIST_HEAD,
 `<key>Label</key><string>${label}</string>`,
 `<key>UserName</key><string>${FLEET_WORKER_USER_PLACEHOLDER_V1}</string>`,
 `<key>WorkingDirectory</key><string>${directory}</string>`,
 `<key>ProgramArguments</key><array>${program.map(item=>`<string>${xml(item)}</string>`).join("")}</array>`,
 environmentOf,...schedule,
 `<key>StandardOutPath</key><string>${log}</string>`,`<key>StandardErrorPath</key><string>${log}</string>`,
 "</dict>","</plist>"
].join("\n");
const KEEP_ALIVE=["<key>RunAtLoad</key><true/>","<key>KeepAlive</key><true/>","<key>ThrottleInterval</key><integer>2</integer>"];
const taskOf=(description:string,trigger:string,settings:string,directory:string,executable:string,args:string):string=>[
 '<?xml version="1.0" encoding="UTF-16"?>',
 '<Task version="1.3" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">',
 `<RegistrationInfo><Description>${xml(description)}</Description></RegistrationInfo>`,
 `<Triggers>${trigger}</Triggers>`,
 '<Principals><Principal id="Author"><LogonType>InteractiveToken</LogonType><RunLevel>LeastPrivilege</RunLevel></Principal></Principals>',
 `<Settings><MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy><DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries><StopIfGoingOnBatteries>false</StopIfGoingOnBatteries><StartWhenAvailable>true</StartWhenAvailable>${settings}</Settings>`,
 '<Actions Context="Author"><Exec>',`<Command>${executable}</Command>`,`<Arguments>${xml(args)}</Arguments>`,`<WorkingDirectory>${directory}</WorkingDirectory>`,"</Exec></Actions>",
 "</Task>"
].join("\n");
const LOGON="<LogonTrigger><Enabled>true</Enabled></LogonTrigger>",
 FOREVER="<RestartOnFailure><Interval>PT1M</Interval><Count>255</Count></RestartOnFailure><ExecutionTimeLimit>PT0S</ExecutionTimeLimit>";

/** 🧰️ The named pieces of the worker services: `target` and `definition` of the replica daemon (`run`), `compact-target` and `compact-definition` of the daily `compact`, and on Linux `timer-target` and `timer-definition` of the timer that starts it. With `server` (the group server, on the platform that the topology names for it) the daily compaction passes `--collect-blobs`: only group servers delete unreferenced blobs, because a worker's view of the other origins may lag. */
export function workerPartsV1(topology:FleetTopologyV1,groupId:string,platform:FleetPlatformV1,server:boolean=false):Readonly<Record<string,string>>{
 const group=groupV1(topology,groupId),directory=replicaDirectory(topology,platform),collect=server?["--collect-blobs"]:[];
 if(server&&group.server.platform!==platform)throw Error(`Server platform required: ${group.id} runs its server on ${group.server.platform}, not ${platform}`);
 if(platform==="linux")return Object.freeze({target:LINUX_REPLICA_UNIT,definition:[
  "[Unit]",`Description=semio fleet replica daemon (${group.id}) for %i`,"After=network-online.target remote-fs.target",`RequiresMountsFor=${topology.storeMount.linux}`,"",
  "[Service]","User=%i",`WorkingDirectory=${directory}`,`ExecStart=${REPO} ${SCRIPT} run`,
  "Restart=always","RestartSec=2","LimitNOFILE=65536","",
  "[Install]","WantedBy=multi-user.target"
 ].join("\n"),"compact-target":LINUX_COMPACT_UNIT,"compact-definition":[
  "[Unit]",`Description=semio fleet journal compaction (${group.id}) for %i`,"After=network-online.target remote-fs.target",`RequiresMountsFor=${topology.storeMount.linux}`,"",
  "[Service]","Type=oneshot","User=%i",`WorkingDirectory=${directory}`,`ExecStart=${REPO} ${SCRIPT} compact${collect.map(flag=>` ${flag}`).join("")}`
 ].join("\n"),"timer-target":LINUX_COMPACT_TIMER,"timer-definition":[
  "[Unit]",`Description=daily semio fleet journal compaction (${group.id}) for %i`,"",
  "[Timer]","OnCalendar=daily","RandomizedDelaySec=3h","Persistent=true","",
  "[Install]","WantedBy=timers.target"
 ].join("\n")});
 if(platform==="macos")return Object.freeze({target:MACOS_REPLICA_PLIST,definition:daemonOf(MACOS_REPLICA_LABEL,directory,[REPO,SCRIPT,"run"],KEEP_ALIVE,"/var/log/semio-fleet-replica.log"),
  "compact-target":MACOS_COMPACT_PLIST,"compact-definition":daemonOf(MACOS_COMPACT_LABEL,directory,[REPO,SCRIPT,"compact",...collect],["<key>StartCalendarInterval</key><dict><key>Hour</key><integer>3</integer><key>Minute</key><integer>30</integer></dict>"],"/var/log/semio-fleet-compact.log")});
 return Object.freeze({target:WINDOWS_REPLICA_TASK,definition:taskOf(`semio fleet replica daemon (${group.id})`,LOGON,FOREVER,directory,"repo",`${SCRIPT} run`),
  "compact-target":WINDOWS_COMPACT_TASK,"compact-definition":taskOf(`semio fleet journal compaction (${group.id})`,"<CalendarTrigger><Enabled>true</Enabled><StartBoundary>2026-01-01T03:30:00</StartBoundary><ScheduleByDay><DaysInterval>1</DaysInterval></ScheduleByDay></CalendarTrigger>","<ExecutionTimeLimit>PT6H</ExecutionTimeLimit>",directory,"repo",`${SCRIPT} compact${collect.map(flag=>` ${flag}`).join("")}`)});
}

const linuxServerV1=(topology:FleetTopologyV1,group:FleetGroupV1):Record<string,string>=>{
 const store=group.nfs.export,mount=topology.storeMount.linux,peers=peersOf(topology,group),names=buildPolicy.remoteCache.server.tokenEnvironment,port=portOf(group),
  directory=`${topology.worktree.linux}/${REPLICA}`,replica=`semio-fleet-replica@${STORE_USER}.service`,archive=peers.length>0?` --archive ${archiveOf(peers,"linux")}`:"";
 const peerLine=(peer:FleetPeerV1):string=>{
  const {nfs,server}=peer.group;
  if(server.protocol==="smb")return `//${nfs.host}${nfs.export} ${peer.path} cifs ${PEER_SMB_OPTIONS} 0 0`;
  return server.platform==="linux"?`${nfs.host}:${nfs.export} ${peer.path} nfs4 ${PEER_OPTIONS} 0 0`:`${nfs.host}:${nfs.export} ${peer.path} nfs ${PEER_V3_OPTIONS} 0 0`;
 };
 return {
  prepare:[
   `sudo useradd --system --uid ${STORE_ID} --user-group --create-home --home-dir ${STORE_HOME} --shell /bin/bash ${STORE_USER}`,
   `sudo install -d -o ${STORE_USER} -g ${STORE_USER} ${store} ${CACHE_DIRECTORY}`,
   `sudo install -d -m 0750 -o root -g ${STORE_USER} /etc/semio`,
   `sudo mkdir -p ${[mount,...peers.map(peer=>peer.path)].join(" ")}`
  ].join("\n"),
  provision:`sudo bash ${MODULE}/${TOPOLOGY_FOLDER}/${PROVISION} ${group.id} ${group.id}-server --user ${STORE_USER}${archive} --server`,
  start:[
   "sudo systemctl daemon-reload",
   `sudo systemctl enable --now ${replica} semio-fleet-cache.service semio-fleet-relay.service`
  ].join("\n"),
  "exports-target":"/etc/exports.d/semio-fleet.exports",
  exports:`${store} ${group.server.network}(rw,sync,no_subtree_check,all_squash,anonuid=${STORE_ID},anongid=${STORE_ID})${peers.map(peer=>` ${peer.group.nfs.host}(ro,sync,no_subtree_check)`).join("")}`,
  "sysctl-target":SERVER_SYSCTL_PATH,
  sysctl:SERVER_SYSCTL,
  fstab:[`${store} ${mount} none bind 0 0`,...peers.map(peerLine)].join("\n"),
  "relay-peers":peerList(peers),
  "cache-environment-target":TOKENS,
  "cache-environment":`${names.read}=<read-token>\n${names.write}=<write-token>`,
  "cache-target":"/etc/systemd/system/semio-fleet-cache.service",
  "cache-unit":[
   "[Unit]",`Description=semio fleet remote cache (${group.id})`,"After=network-online.target","Wants=network-online.target","",
   "[Service]",`User=${STORE_USER}`,`Group=${STORE_USER}`,`WorkingDirectory=${topology.worktree.linux}`,`EnvironmentFile=${TOKENS}`,"Environment=SEMIO_LANGUAGE=en",
   `ExecStart=${REPO} remote-cache serve --host 0.0.0.0 --port ${port} --directory ${CACHE_DIRECTORY} --max-bytes ${CACHE_BYTES}`,
   "Restart=always","RestartSec=2","LimitNOFILE=65536","",
   "[Install]","WantedBy=multi-user.target"
  ].join("\n"),
  "replica-unit":workerPartsV1(topology,group.id,"linux")["definition"]!,
  "relay-target":"/etc/systemd/system/semio-fleet-relay.service",
  "relay-unit":[
   "[Unit]",`Description=semio fleet relay (${group.id})`,`After=network-online.target remote-fs.target ${replica}`,"Wants=network-online.target",`RequiresMountsFor=${mount}`,`ConditionPathExists=${topology.worktree.linux}/.cache/f/machine.json`,"",
   "[Service]",`User=${STORE_USER}`,`Group=${STORE_USER}`,`WorkingDirectory=${directory}`,
   `ExecStart=${REPO} ${SCRIPT} relay`,
   "Restart=always","RestartSec=2","LimitNOFILE=65536","",
   "[Install]","WantedBy=multi-user.target"
  ].join("\n")
 };
};

const macosServerV1=(topology:FleetTopologyV1,group:FleetGroupV1):Record<string,string>=>{
 const store=group.nfs.export,peers=peersOf(topology,group),names=buildPolicy.remoteCache.server.tokenEnvironment,port=portOf(group),user=FLEET_WORKER_USER_PLACEHOLDER_V1,
  worktree=topology.worktree.macos,directory=replicaDirectory(topology,"macos"),archive=peers.length>0?` --archive ${archiveOf(peers,"macos")}`:"",
  mountOptions=(peer:FleetPeerV1):string=>peer.group.server.platform==="linux"?`vers=4,ro,hard,nobrowse,${MACOS_ATTRIBUTES}`:`vers=3,ro,hard,nobrowse,${MACOS_ATTRIBUTES},nolocks`,
  mounts=peers.length===0?"/usr/bin/true":peers.map(peer=>`/sbin/mount | /usr/bin/grep -q ' on ${peer.path} ' || { /bin/mkdir -p ${peer.path}; /sbin/mount_nfs -o ${mountOptions(peer)} ${source("macos",peer.group.nfs)} ${peer.path}; }`).join("; ");
 return {
  prepare:[
   `sudo install -d -o ${user} ${store} ${CACHE_DIRECTORY}`,
   `sudo install -d -m 0750 -o ${user} /etc/semio`,
   `sudo mkdir -p ${MACOS_PEERS}`
  ].join("\n"),
  provision:`sudo bash ${MODULE}/${TOPOLOGY_FOLDER}/${PROVISION} ${group.id} ${group.id}-server --user <account>${archive} --server`,
  start:[
   "sudo nfsd enable","sudo nfsd update",
   ...[MACOS_PEERS_LABEL,MACOS_CACHE_LABEL,MACOS_RELAY_LABEL].map(label=>`sudo launchctl bootstrap system /Library/LaunchDaemons/${label}.plist`),
   `sudo launchctl bootstrap system ${MACOS_REPLICA_PLIST}`
  ].join("\n"),
  "exports-target":"/etc/exports",
  exports:[
   `${store} -mapall=${user} -network ${addressOf(group.server.network)} -mask ${maskOf(group.server.network)}`,
   ...peers.length>0?[`${store} -ro -mapall=${user} ${peers.map(peer=>peer.group.nfs.host).join(" ")}`]:[]
  ].join("\n"),
  "nfs-conf-target":"/etc/nfs.conf",
  "nfs-conf":"nfs.server.mount.port = 892",
  "peer-mount-target":`/Library/LaunchDaemons/${MACOS_PEERS_LABEL}.plist`,
  "peer-mount-unit":[...PLIST_HEAD,`<key>Label</key><string>${MACOS_PEERS_LABEL}</string>`,`<key>ProgramArguments</key><array><string>/bin/sh</string><string>-c</string><string>${xml(mounts)}</string></array>`,"<key>RunAtLoad</key><true/>","<key>StartInterval</key><integer>60</integer>","</dict>","</plist>"].join("\n"),
  "relay-peers":peerList(peers),
  "cache-environment-target":TOKENS,
  "cache-environment":`${names.read}=<read-token>\n${names.write}=<write-token>`,
  "cache-target":`/Library/LaunchDaemons/${MACOS_CACHE_LABEL}.plist`,
  "cache-unit":daemonOf(MACOS_CACHE_LABEL,worktree,["/bin/sh","-c",`set -a; . ${TOKENS}; set +a; exec ${REPO} remote-cache serve --host 0.0.0.0 --port ${port} --directory ${CACHE_DIRECTORY} --max-bytes ${CACHE_BYTES}`],KEEP_ALIVE,"/var/log/semio-fleet-cache.log"),
  "replica-unit":workerPartsV1(topology,group.id,"macos")["definition"]!,
  "relay-target":`/Library/LaunchDaemons/${MACOS_RELAY_LABEL}.plist`,
  "relay-unit":daemonOf(MACOS_RELAY_LABEL,directory,[REPO,SCRIPT,"relay",...peers.length>0?["--peers",peerList(peers)]:[]],KEEP_ALIVE,"/var/log/semio-fleet-relay.log")
 };
};

const NFS_CLIENT="if(Get-Command Install-WindowsFeature -ErrorAction SilentlyContinue){Install-WindowsFeature NFS-Client | Out-Null}else{Enable-WindowsOptionalFeature -Online -FeatureName ServicesForNFS-ClientOnly,ClientForNFS-Infrastructure -All -NoRestart | Out-Null}";

const windowsServerV1=(topology:FleetTopologyV1,group:FleetGroupV1):Record<string,string>=>{
 const {nfs,server}=group,share=nfs.export.slice(1),directory=server.directory!,peers=peersOf(topology,group),names=buildPolicy.remoteCache.server.tokenEnvironment,port=portOf(group),
  worktree=topology.worktree.windows,workdir=replicaDirectory(topology,"windows"),archive=peers.length>0?` -ArchivePath ${archiveOf(peers,"windows")}`:"",
  smb=server.protocol==="smb",ports=smb?"445":"2049,111,1048",
  rule=(name:string,address:string,list:string)=>`Get-NetFirewallRule -DisplayName ${name} -ErrorAction SilentlyContinue | Remove-NetFirewallRule; New-NetFirewallRule -DisplayName ${name} -Direction Inbound -Action Allow -Protocol TCP -LocalPort ${list} -RemoteAddress ${address} | Out-Null`;
 return {
  prepare:[
   `New-Item -ItemType Directory -Force -Path ${directory},${WINDOWS_CACHE_DIRECTORY},${WINDOWS_TOKENS.replace(/\\[^\\]*$/u,"")} | Out-Null`,
   ...smb?[SMB_USER,SMB_PEER_USER].map(name=>`New-LocalUser -Name ${name} -Password (Read-Host -AsSecureString 'password for ${name}') -PasswordNeverExpires`):["Install-WindowsFeature FS-NFS-Service -IncludeManagementTools"]
  ].join("\n"),
  provision:`powershell -NoProfile -ExecutionPolicy Bypass -File "${worktree}\\${MODULE.replaceAll("/","\\")}\\${TOPOLOGY_FOLDER}\\${PROVISION_POWERSHELL.replaceAll("/","\\")}" ${group.id} ${group.id}-server -Server${archive}`,
  start:[WINDOWS_REPLICA_TASK,WINDOWS_CACHE_TASK,WINDOWS_RELAY_TASK].map(task=>`Start-ScheduledTask -TaskName ${task}`).join("\n"),
  feature:[smb?"Set-SmbServerConfiguration -EnableSMB2Protocol $true -Force":"Install-WindowsFeature FS-NFS-Service -IncludeManagementTools",...peers.some(peer=>peer.group.server.protocol==="nfs")?[NFS_CLIENT]:[]].join("\n"),
  share:smb?[
   `if(-not (Get-SmbShare -Name ${share} -ErrorAction SilentlyContinue)){New-SmbShare -Name ${share} -Path ${directory} -ChangeAccess ${SMB_USER} -ReadAccess ${SMB_PEER_USER} | Out-Null}`,
   `& icacls ${directory} /grant '${SMB_USER}:(OI)(CI)M' '${SMB_PEER_USER}:(OI)(CI)RX' | Out-Null`
  ].join("\n"):[
   `if(-not (Get-NfsShare -Name ${share} -ErrorAction SilentlyContinue)){New-NfsShare -Name ${share} -Path ${directory} -Authentication sys -EnableAnonymousAccess $true -EnableUnmappedAccess $true -AnonymousUid ${STORE_ID} -AnonymousGid ${STORE_ID} -Permission readwrite -AllowRootAccess $true | Out-Null}`,
   ...peers.map(peer=>`Grant-NfsSharePermission -Name ${share} -ClientName ${peer.group.nfs.host} -ClientType host -Permission readonly -AllowRootAccess $false | Out-Null`)
  ].join("\n"),
  firewall:[
   rule("semio-fleet-store-workers",server.network,ports),
   ...peers.map(peer=>`$address=(Resolve-DnsName -Name ${peer.group.nfs.host} -Type A).IPAddress; ${rule(`semio-fleet-store-${peer.group.id}`,"$address",ports)}`),
   rule("semio-fleet-cache",server.network,port)
  ].join("\n"),
  "relay-peers":peerList(peers),
  "cache-environment-target":WINDOWS_TOKENS,
  "cache-environment":`${names.read}=<read-token>\n${names.write}=<write-token>`,
  "cache-target":WINDOWS_CACHE_TASK,
  "cache-unit":taskOf(`semio fleet remote cache (${group.id})`,LOGON,FOREVER,worktree,"powershell.exe",`-NoProfile -ExecutionPolicy Bypass -Command "Get-Content ${WINDOWS_TOKENS} | ForEach-Object { $n,$v = $_ -split '=',2; Set-Item -Path ('Env:'+$n) -Value $v }; & repo remote-cache serve --host 0.0.0.0 --port ${port} --directory ${WINDOWS_CACHE_DIRECTORY} --max-bytes ${CACHE_BYTES}"`),
  "replica-unit":workerPartsV1(topology,group.id,"windows")["definition"]!,
  "relay-target":WINDOWS_RELAY_TASK,
  "relay-unit":taskOf(`semio fleet relay (${group.id})`,LOGON,FOREVER,workdir,"repo",`${SCRIPT} relay${relayArguments(peers)}`)
 };
};

/** 🏭️ The named pieces of the group server plan, which follow the operating system that `server.platform` of the group names. Every platform has `prepare` (account or folders), `provision` (the provisioning of the server as a replica of its own group: it joins and starts the services first and runs the native bootstrap last, and its exit code 3 means that the server is ready and only the native build is pending, so run it again to retry), `start`, `relay-peers` (the `--peers` value of the relay: `g2=<path>,...`), `cache-environment-target`, `cache-environment`, `cache-target`, `cache-unit`, `replica-unit`, `relay-target` and `relay-unit`. Linux adds `exports-target`, `exports`, `sysctl-target` and `sysctl` (`fs.leases-enable=0`: no NFSv4 delegations, because a macOS client stalls for 4 s on every head that another client holds a delegation on, measured) and `fstab` (bind mount of the export at the store mount, read-only mounts of the peers: `nfs4` of a Linux peer, `nfs` v3 of other peers, `cifs` of an SMB peer); macOS adds `exports-target`, `exports`, `nfs-conf-target`, `nfs-conf`, `peer-mount-target` and `peer-mount-unit` (LaunchDaemon that mounts the peers at /Volumes/semio-peers); Windows adds `feature`, `share` (Server for NFS `New-NfsShare`, or `New-SmbShare` when the group is `smb`) and `firewall`; its relay reads the peers by UNC path. */
export function serverPartsV1(topology:FleetTopologyV1,groupId:string):Readonly<Record<string,string>>{
 const group=groupV1(topology,groupId),{platform}=group.server;
 return Object.freeze(platform==="linux"?linuxServerV1(topology,group):platform==="macos"?macosServerV1(topology,group):windowsServerV1(topology,group));
}

/** 🗄️ Renders the setup of one group server on its own platform. The server is a replica itself: its home group is the group and its origin is `<group>-server` (written to `.cache/f/machine.json` by the `join` that `provision` runs, which owns the worktree and the store), so its worktree stays current and feeds the bootstrap archive. Linux exports the store with `/etc/exports.d` (read-write for the group network, read-only for the relays of the other groups) and bind-mounts it at the store mount; macOS exports it with `/etc/exports` and `nfsd` and mounts the peers with a LaunchDaemon; Windows shares it with Server for NFS (`New-NfsShare`) or, for a group with `protocol: smb`, as an SMB share, and runs cache and relay as scheduled tasks. On macOS and Windows the server's own store root is the local folder itself (`--store`), recorded in `machine.json`. The relay also rebuilds the bootstrap archive every `replica.bootstrapMinutes`. The server's daily compaction is rendered with `worker-service --role server` (`compact --collect-blobs`); `provision --server` installs that one, so the server is the only machine that deletes unreferenced blobs. Run the parts in the order `prepare`, `provision`, `start`; `provision` writes the exports, mounts, tasks and units of the other parts itself. */
export function serverPlanV1(topology:FleetTopologyV1,groupId:string):string{
 return sections(serverPartsV1(topology,groupId));
}

/** 🔁️ Renders the service definition that keeps the replica daemon (`run`) alive: a systemd template unit (enable it as `semio-fleet-replica@<account>`) on Linux, a LaunchDaemon plist with `KeepAlive` on macOS and a Scheduled Task XML with restart on failure on Windows. */
export function workerServiceV1(topology:FleetTopologyV1,groupId:string,platform:FleetPlatformV1):string{
 return workerPartsV1(topology,groupId,platform)["definition"]!;
}

/** 📋️ The values a provisioning script needs for one group on one platform: paths, the protocol, mount source and options, the fstab line, the server (platform, protocol, network, local store directory) and the cache settings. A platform that cannot mount the store of the group (Linux or macOS on an SMB group) is refused. */
export function factsV1(topology:FleetTopologyV1,groupId:string,platform:FleetPlatformV1):Readonly<Record<string,string>>{
 const group=groupV1(topology,groupId),mount=topology.storeMount[platform],protocol=clientProtocolV1(group,platform),{server}=group;
 return Object.freeze({
  GROUP:group.id,PLATFORM:platform,WORKTREE:topology.worktree[platform],STORE_MOUNT:mount,PROTOCOL:protocol,
  NFS_HOST:group.nfs.host,NFS_EXPORT:group.nfs.export,NFS_SOURCE:source(platform,group.nfs),MOUNT_OPTIONS:optionsOf(group,platform),
  FSTAB_LINE:platform==="linux"?mountPartsV1(topology,groupId,platform)["fstab"]!:"",DRIVE:platform==="windows"?mount.slice(0,2):"",
  SERVER_PLATFORM:server.platform,SERVER_PROTOCOL:server.protocol,SERVER_NETWORK:server.network,SERVER_DIRECTORY:server.directory??group.nfs.export,SERVER_CACHE_DIRECTORY:server.platform==="windows"?WINDOWS_CACHE_DIRECTORY:CACHE_DIRECTORY,
  SMB_CACHE:listOf(WINDOWS_SMB_CACHE),NFS_CACHE:listOf(WINDOWS_NFS_CACHE),NFS_CACHE_KEY:WINDOWS_NFS_KEY,
  CACHE_ENDPOINT:group.cache.endpoint,CACHE_INSTANCE:group.cache.instance,REPLICA_DIRECTORY:replicaDirectory(topology,platform),TOPOLOGY_DIRECTORY:topologyDirectory(topology,platform),ARCHIVE_DIRECTORY:archiveDirectory(topology,platform)
 });
}
