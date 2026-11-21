import {existsSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";

/** 🧪️ Provides a deterministic native farm look whose completion the parent fixture controls. */
export async function open() { return "look-gate"; }

/** 🔐️ Looks at owners in order, observing the fixture's gate before returning its one terminal reply. */
export async function run(job) {
  const looks=[];
  for (const owner of job.owners) {
    writeFileSync(join(job.directory,owner.name+".started"),"started");
    if (owner.blocked) while (!existsSync(join(job.directory,owner.gate??"release"))) await delay(10);
    looks.push({name:owner.name,native:false,pipeline:false,head:[owner.name],verdict:job.verdict});
    writeFileSync(join(job.directory,owner.name+".done"),"done");
  }
  return {looks,weight:0.01*job.owners.length};
}
