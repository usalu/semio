/** ✂️ The generated shard worker is an ES module that imports the WASI monotonic clock from its vendored preview2 shim; a `node:vm` classic script cannot import, so the suites that run it in a context bind a fixed clock instead of the import.
 * @see ../../🌐️browser-bundle/🏗️materialization/🟦️.ts — `shardWorkerSource` */
export function classicShardWorkerSource(source: string): string {
  const clockImport = /^import \{ monotonicClock \} from "[^"]*";$/mu;
  if (!clockImport.test(source)) throw new Error("shard-worker.clock-import-missing");
  return source.replace(clockImport, "const monotonicClock = { now: () => 5_000_000n };");
}

/** ✂️ The original operation source a generated worker's turn drive reads from its actor's api; a fake api that answers `poll` supplies this beside it.
 * @see ../../🌐️browser-bundle/🏗️materialization/🟦️.ts — `shardWorkerSource`'s `turn` case */
export const vmOriginalSource = Object.freeze({ deadlineUs: 10_000_000n, operationClockUs: (): bigint => 5_000_000n, fuelRemaining: 1 });

/** 🏭️ The jco instantiation factory a generated bridge instantiates once per activation, around a test guest body that declares the actor world's exports (`reactor`, `jobs`, `checkpoint`, `describe`, `codec`; `export` keywords are dropped): the body runs per instantiation, sees the activation's host as `host`, and an export it does not declare answers as an empty interface.
 * @see ../../🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts */
export function guestFactorySource(body: string): string {
  const members = ["reactor", "jobs", "checkpoint", "describe", "codec"];
  return `export async function instantiate(getCoreModule, imports) {\nconst host = imports["./🟨️.js"];\n${body.replace(/\bexport (const|let|function|async function) /gu, "$1 ")}\nreturn { ${members.map((name) => `${name}: typeof ${name} === "undefined" ? {} : ${name}`).join(", ")} };\n}\nexport const wasiImports = Object.freeze({});\n`;
}

/** 🪢️ A host shim module in the generated shape (`createActorHost(actorId, activationGeneration)` answering one host per activation) whose host is the given object literal source.
 * @see ../../🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts */
export function hostFactorySource(members: string): string {
  return `export function createActorHost(actorId, activationGeneration) {\nreturn { ${members} };\n}\n`;
}

/** 🧊️ The core module loader of a test guest that asks for none: a request is a defect of the case. */
export const noCoreModule = async (url: string): Promise<never> => { throw new Error(`fixture guest has no core module: ${url}`); };
