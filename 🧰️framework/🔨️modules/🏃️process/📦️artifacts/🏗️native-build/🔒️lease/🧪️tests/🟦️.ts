import assert from "node:assert/strict";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import corpus from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import casesSchema from "../🧬️schema/🧪️cases.json" with {type:"json"};
import { cargoBuildLeaseIdentityV1 } from "../🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../🏗️build/🎟️capacity/🟦️.ts";

/** 🚦️ Verifies portable profile identities and actual cross-process ownership, cancellation and crash recovery. */
export async function proveCargoBuildLeasesV1(artifactRoot: string): Promise<void> {
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "c-"));
  const leaseRoot=join(process.cwd(),".cache/b/l/m/data");mkdirSync(leaseRoot,{recursive:true});const leases=mkdtempSync(join(leaseRoot,"c-"));
  const implementation = fileURLToPath(new URL("../🟦️.ts", import.meta.url));
  const worker = join(root, "📜️script.ts");
  const oracle = new Ajv({ strict: true }).addSchema(schema);
  
  const validate = oracle.getSchema(schema.$id)!;
  const validateCases=oracle.compile(casesSchema);assert(validateCases(corpus),JSON.stringify(validateCases.errors));
  const children: { process: Bun.Subprocess<"pipe", "pipe", "pipe">; output: string; error: string; done: Promise<number> }[] = [];
  writeFileSync(worker, `import { acquireCargoBuildLeaseV1,acquireCargoProfileMaintenanceLeaseV1,acquireCargoStoreLeaseV1 } from ${JSON.stringify(implementation)};
const [directory, buildDirectory, profile, access, targetDirectory] = process.argv.slice(2), controller = new AbortController();
let finish; const finished = new Promise(resolve => { finish = resolve; });
process.stdin.on("data", data => { if (String(data).trim() === "cancel") controller.abort(); finish(); });
const timeout = setTimeout(() => controller.abort(), 15000);
try {
 const options={directory,buildDirectory,targetDirectory,args:["--profile",profile],signal:controller.signal,onWait:()=>console.log("waiting")};
 const lease=await (access==="store-maintain"?acquireCargoStoreLeaseV1({...options,mode:"exclusive"}):(access==="maintain"?acquireCargoProfileMaintenanceLeaseV1:acquireCargoBuildLeaseV1)(options));
 try { console.log("held"); await finished; } finally { lease.release(); console.log("released"); }
} catch (error) { if (controller.signal.aborted) console.log("cancelled"); else throw error; }
finally { clearTimeout(timeout); process.stdin.destroy(); }
`);
  const until = async (child: typeof children[number], event: string): Promise<void> => { const deadline = Date.now() + corpus.maximumWaitMilliseconds; while (!child.output.split("\n").includes(event)) { assert.equal(child.process.exitCode, null, child.error || child.output); assert.ok(Date.now() < deadline, event + ": " + child.output); await Bun.sleep(20); } };
  const start = (profile: string, runtime: string,access:string,build="build",target=build) => {
    const identity=cargoBuildLeaseIdentityV1(join(root,build),["--profile",profile]),resource=JSON.stringify([schema.definitions.Protocol.const.namespace,identity.buildDirectory,identity.profile]),database=join(leases,createHash("sha256").update(resource).digest("hex")+".sqlite");
    const argv=access==="oracle"?[runtime,"--input-type=module","-e","import {DatabaseSync} from \"node:sqlite\";const d=new DatabaseSync(process.argv[1]);try{let denied=false;try{d.exec(\"BEGIN EXCLUSIVE\")}catch(e){if(!/locked|busy/i.test(e.message))throw e;denied=true}if(!denied)throw Error(\"Independent SQLite writer acquired active profile\");console.log(\"held\")}finally{d.close()}",database]:[runtime,worker,leases,join(root,build),profile,access,join(root,target)];
    const child = { process: Bun.spawn(argv, { stdin: "pipe", stdout: "pipe", stderr: "pipe" }), output: "", error: "", done: undefined as unknown as Promise<number> };
    const drain = async (stream: ReadableStream<Uint8Array>, key: "output" | "error") => { for await (const bytes of stream) child[key] += new TextDecoder().decode(bytes); };
    child.done = Promise.all([drain(child.process.stdout, "output"), drain(child.process.stderr, "error"), child.process.exited]).then(rows => rows[2]);
    children.push(child); return child;
  };
  const release = async (child: typeof children[number]) => { child.process.stdin.write("release\n"); child.process.stdin.end(); assert.equal(await child.done, 0, child.error); };
  try {
    for (const row of corpus.identities) {
      const identity = cargoBuildLeaseIdentityV1(join(root, "build"), row.args);
      assert.equal(identity.profile, row.profile);
      assert(validate(identity), JSON.stringify(validate.errors));
      assert.deepEqual(JSON.parse(JSON.stringify(identity)), identity);
    }
    for (const args of corpus.hostileArgs) assert.throws(() => cargoBuildLeaseIdentityV1(join(root, "build"), args));
    const {proveCargoTargetStoreBindingV1}=await import("../🎯️target/🧪️tests/🟦️.ts");
    await proveCargoTargetStoreBindingV1(root,leases);
    for (const row of corpus.cases) {
      const first = start(row.first, process.execPath,row.firstAccess); await until(first, "held");
      const second = start(row.second, "node",row.secondAccess);
      if (row.action === "parallel") { await until(second, "held");const oracle=start(row.first,"node","oracle");assert.equal(await oracle.done,0,oracle.error);assert.equal(oracle.output.trim(),"held");await release(second); await release(first); }
      else {
        await until(second, "waiting"); assert.equal(second.output.includes("held"), false);
        if (row.action === "cancel-second") { second.process.stdin.write("cancel\n"); second.process.stdin.end(); assert.equal(await second.done, 0, second.error); assert.equal(second.output.includes("held"), false); await release(first); }
        else { if (row.action === "kill-first") { first.process.kill("SIGKILL"); await first.done; } else await release(first); await until(second, "held"); await release(second); }
      }
    }
    for(const row of corpus.targets){
      const first=start("prototype",process.execPath,"compile",row.firstBuild,row.firstTarget);await until(first,"held");
      const second=start("prototype","node",row.secondAccess,row.secondBuild,row.secondTarget);
      if(row.action==="parallel"){await until(second,"held");const oracle=start("prototype","node","oracle",row.firstTarget,row.firstTarget);assert.equal(await oracle.done,0,oracle.error);await release(second);await release(first);}
      else{await until(second,"waiting");assert.equal(second.output.includes("held"),false);if(row.action==="cancel-second"){second.process.stdin.write("cancel\n");second.process.stdin.end();assert.equal(await second.done,0,second.error);assert.equal(second.output.includes("held"),false);await release(first);}else{await release(first);await until(second,"held");await release(second);}}
    }
    for(const row of corpus.fairness){
      const first=start(row.profile,process.execPath,"compile");await until(first,"held");
      const writer=start(row.profile,"node",row.writerAccess);await until(writer,"waiting");
      const late=start(row.profile,process.execPath,"compile");await until(late,"waiting");assert.equal(late.output.includes("held"),false);
      await release(first);await until(writer,"held");assert.equal(late.output.includes("held"),false);
      await release(writer);await until(late,"held");await release(late);
    }
    const { testResourceLeases } = await import("../../../../🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts");
    await receiveScriptProcessInvocation(process.env, original => testResourceLeases(leases, scriptCompilerAdmissionControlV1(original, "test:resource-leases", 0), original));
  } finally { for (const child of children) if (child.process.exitCode === null) child.process.kill("SIGKILL"); await Promise.all(children.map(child => child.done)); rmSync(root, { recursive: true, force: true });rmSync(leases,{recursive:true,force:true}); }
}
