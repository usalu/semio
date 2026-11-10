/** ⌨ `repo service host …`: start, status, stop and foreground run of the per-host graph service. */
import {POLICY_V1,renderTextV1} from "../📨️protocol/🎚️policy/🟨️.mjs";
import {runHostCommandV1} from "./🌐️host/🟨️.mjs";

/** 🚫 What the watcher and the state never look at: caches, VCS metadata, dependency trees and Bazel's convenience links. */
export const RESIDENT_EXCLUDES_V1=Object.freeze({
 rooted:Object.freeze([".git",".cache", ".r",".🧬semio/🦑️repo/⚡️cache",".🧬semio/🦑️repo/🎫️tickets","bazel-out","bazel-bin","bazel-testlogs","bazel-semio"]),
 names:Object.freeze(["node_modules","target","__pycache__",".venv"]),
});
/** 🚧 Directory the service touches to fence the event stream; never excluded. */
export const RESIDENT_FENCE_DIRECTORY_V1=".🧬semio/🦑️repo/🏡️resident-fence";

const say=(languages,key,fields)=>{const text=renderTextV1(key,fields,languages);return text?text[languages[0]]??Object.values(text)[0]:key;};

/** ⌨ Runs one subcommand and returns the process exit code. */
export async function runResidentCommandV1(options){
 const{root,args,languages,environment=process.env,stdout=process.stdout,stderr=process.stderr}=options;
 if(args[0]==="host")return runHostCommandV1({...options.host,args:args.slice(1),languages,environment,stdout,stderr,cwd:root});
 stderr.write(say(languages,"host-usage",{})+"\n");return 1;
}

export{POLICY_V1};
