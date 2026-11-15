import {mkdirSync,statSync,utimesSync,writeFileSync,closeSync,openSync} from "node:fs";
import {homedir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";

/** 🩹️ The label of the replica agent of the fleet (a LaunchAgent of the user). */
export const REPLICA_AGENT_LABEL_V1="tech.semio.fleet.replica";

/** 🔌️ What the check asks of the system: run a program and answer its status and output. */
export type AgentRunV1=(program:string,args:readonly string[])=>Readonly<{status:number;stdout:string}>;

/** 🩹️ Heals a replica agent that launchd has loaded but does not run (`pended nondemand spawn = inefficient|speculative`, a penalised agent that nobody kickstarted): every `repo` command passes here first, on macOS only, and at most once a minute looks at the agent; a loaded agent that is not running gets a demand start (`launchctl kickstart`, allowed in the own user domain) and one bilingual line. A machine without the agent, an agent that runs and an agent that is not loaded are left alone. Answers what it did. */
export function healReplicaAgentV1(options:Readonly<{platform?:string;home?:string;uid?:number;run?:AgentRunV1;exists?:(path:string)=>boolean;now?:()=>number;stamp?:string;minimumMilliseconds?:number;notice?:(line:string)=>void}>={}):"skipped"|"running"|"unloaded"|"kickstarted"|"failed"{
 const platform=options.platform??process.platform,home=options.home??homedir(),uid=options.uid??process.getuid?.();
 if(platform!=="darwin"||uid===undefined)return"skipped";
 const exists=options.exists??(path=>{try{statSync(path);return true;}catch{return false;}});
 if(!exists(join(home,"Library","LaunchAgents",REPLICA_AGENT_LABEL_V1+".plist")))return"skipped";
 const now=options.now??Date.now,stamp=options.stamp??join(home,"Library","Application Support","semio","fleet","agent-check");
 try{
  const age=now()-statSync(stamp).mtimeMs;
  if(age>=0&&age<(options.minimumMilliseconds??60_000))return"skipped";
 }catch{}
 try{mkdirSync(dirname(stamp),{recursive:true,mode:0o700});const descriptor=openSync(stamp,"a");closeSync(descriptor);utimesSync(stamp,new Date(now()),new Date(now()));}catch{writeFileSync(stamp,"",{flag:"a"});}
 const run=options.run??((program,args)=>{const result=spawnSync(program,[...args],{encoding:"utf8",timeout:3000});return{status:result.status??1,stdout:result.stdout??""};});
 const target=`gui/${uid}/${REPLICA_AGENT_LABEL_V1}`,state=run("launchctl",["print",target]);
 if(state.status!==0)return"unloaded";
 if(/\bstate = running\b/u.test(state.stdout))return"running";
 const started=run("launchctl",["kickstart",target]).status===0;
 (options.notice??(line=>{process.stderr.write(line+"\n");}))(started
  ?"The replica agent was loaded but not running (launchd had deferred it); started it now / Der Replica-Agent war geladen, lief aber nicht (launchd hatte ihn zurückgestellt); er wurde jetzt gestartet"
  :"The replica agent is not running and launchd refused to start it; run: launchctl kickstart -k "+target+" / Der Replica-Agent läuft nicht und launchd lehnte den Start ab; ausführen: launchctl kickstart -k "+target);
 return started?"kickstarted":"failed";
}
