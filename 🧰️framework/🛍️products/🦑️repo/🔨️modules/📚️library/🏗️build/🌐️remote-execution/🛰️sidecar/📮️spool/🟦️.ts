import {existsSync} from "node:fs";
import {readFleetSettingsV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📡️client/🟦️.ts";
import {FLEET_POLICY_V1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {forgetSpoolV1,readSpoolRegistryV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📮️registry/🟦️.ts";
import {startUploaderV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";
import {spoolLengthV1} from "../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📏️spool-size/🟦️.ts";
import {sleep,type RemoteExecutionSidecarContextV1} from "../🧰️shared/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;

/** 📮️ What one sweep found: the registered roots, the units waiting in their spools, the upload workers it started, the roots it could not drain (no upload right, or queued under another cache identity) and the registrations it forgot. */
export type SpoolSweepV1=Readonly<{roots:number;waiting:number;started:number;skipped:number;forgotten:number}>;

/** 🧹️ One look at every registered spool of the machine: a root that is gone, or whose spool has been empty for `uploader.registryRetentionMilliseconds`, is forgotten; a root whose spool holds jobs gets an upload worker when none lives (the worker is a detached process of its own, so a restart of this sidecar never interrupts an upload; the build that queued the jobs needs to be neither alive nor ever run again). The cache identity decides whether the sidecar may drain a root: the registration holds no secret, so a spool queued under a token is drained only when this process names the same token. */
export function sweepSpoolsV1(environment:Environment):SpoolSweepV1{
 const policy=FLEET_POLICY_V1.uploader;
 let roots=0,waiting=0,started=0,skipped=0,forgotten=0;
 for(const {registration,ageMs,file} of readSpoolRegistryV1(environment)){
  roots++;
  if(!existsSync(registration.root)){forgetSpoolV1(file);forgotten++;continue;}
  const depth=spoolLengthV1(registration.root);
  if(depth===0){if(ageMs>policy.registryRetentionMilliseconds){forgetSpoolV1(file);forgotten++;}continue;}
  waiting+=depth;
  const token=environment.SEMIO_REMOTE_CACHE_TOKEN,effective={...registration.environment,...token===undefined||token===""?{}:{SEMIO_REMOTE_CACHE_TOKEN:token}},settings=readFleetSettingsV1(registration.root,effective);
  if(settings===undefined||!settings.upload||settings.identity!==registration.identity){skipped++;continue;}
  if(startUploaderV1(registration.root,effective))started++;
 }
 return Object.freeze({roots,waiting,started,skipped,forgotten});
}

/** 📮️ Runs the spool role of this machine until the signal aborts: every `uploader.sweepMilliseconds` it sweeps the registered spools ({@link sweepSpoolsV1}), so every unit a build compiled reaches the fleet cache although the build ended, the upload worker died or the machine restarted while the cache was down. Started by the build agent as a child with a closure of its own. */
export async function runSpool(context:RemoteExecutionSidecarContextV1):Promise<void>{
 const policy=FLEET_POLICY_V1.uploader;
 while(!context.signal.aborted){
  try{
   const sweep=sweepSpoolsV1(context.environment);
   if(sweep.started>0||sweep.forgotten>0)context.write(`unit spool: ${sweep.waiting} unit(s) wait in ${sweep.roots} cache root(s), ${sweep.started} upload worker(s) started, ${sweep.forgotten} registration(s) forgotten / Einheiten-Warteschlange: ${sweep.waiting} Einheit(en) warten in ${sweep.roots} Cache-Wurzel(n), ${sweep.started} Upload-Worker gestartet, ${sweep.forgotten} Registrierung(en) vergessen`);
  }catch(error){context.write(`unit spool sweep failed: ${String((error as Error)?.message??error)} / Sweep der Einheiten-Warteschlange fehlgeschlagen: ${String((error as Error)?.message??error)}`);}
  await sleep(policy.sweepMilliseconds,context.signal);
 }
}
