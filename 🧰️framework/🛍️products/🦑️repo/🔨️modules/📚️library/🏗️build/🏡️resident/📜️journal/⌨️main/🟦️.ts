import {runChangeJournalMainV1,type ChangeWatchFactoryV1} from "../🟦️.ts";
import {isMainModuleV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏁️main/🟦️.ts";
import {createWatchSupervisorV1} from "../../🔭️watch/🟨️.mjs";
import {createWatchHubV1} from "../../🔭️watch/🌐️hub/🟨️.mjs";
import {POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {RESIDENT_EXCLUDES_V1,RESIDENT_FENCE_DIRECTORY_V1} from "../../⌨️command/🟨️.mjs";

/** 🏁️ The detached host process of the change journals: every workspace of the user shares the one native watcher process of the hub, the resident exclusions and fence directory apply to all, and the watcher is provisioned by the tool registry only when no binary is named. */
export function runChangeJournalProcessV1(argv:readonly string[],environment:Readonly<Record<string,string|undefined>>=process.env):Promise<number>{
 let hub:ReturnType<typeof createWatchHubV1>|undefined;
 const createSupervisor:ChangeWatchFactoryV1=options=>{
  hub??=createWatchHubV1({binary:options.binary!});
  return createWatchSupervisorV1({...options,spawnProcess:hub.spawnProcess} as never) as never;
 };
 return runChangeJournalMainV1(argv,{
  createSupervisor,watcherPolicy:POLICY_V1 as unknown as Record<string,unknown>,exclude:RESIDENT_EXCLUDES_V1,fence:RESIDENT_FENCE_DIRECTORY_V1,
  locate:async()=>{
   if(environment.SEMIO_REPO_WATCH_BINARY)return environment.SEMIO_REPO_WATCH_BINARY;
   const{ensureResidentWatcherV1}=await import("../../🔭️watch/🏗️provision/🔌️tools/🟨️.mjs");
   return ensureResidentWatcherV1();
  },
  release:async()=>{await hub?.stop();},
 });
}

if(isMainModuleV1(import.meta.url))process.exit(await runChangeJournalProcessV1(process.argv.slice(2)));
