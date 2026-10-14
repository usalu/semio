import { acquireCompilerCapacityLeaseV1, type CompilerAdmissionControl } from "../🎟️capacity/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import { join, resolve, relative, isAbsolute } from "node:path";
import { readBuildMode, type BuildMode } from "../🎚️mode/🟦️.ts";
import { acquireResourceLease, checkResourceLeaseControl, resourceLeasePathV1, requireMaterializedPathV1 } from "../../🔒️leases/🟦️.ts";

/** 🗃️ Owns one compiler's mutable intermediates independently of its staged deliverable. */
export type CompilerStateOwner = Readonly<{ root: string; owner: string; compiler: string; configuration: string; mode: BuildMode; platform?: string; architecture?: string }>;

/** 🔑️ Partitions state by compiler, configuration, platform, architecture, owner and intent. */
export function compilerStateDirectory(options: CompilerStateOwner): string {
  const failures = validateJsonSchemaSubset(schema, Object.fromEntries(Object.entries(options).filter(([key, value]) => !["control", "environment", "capacity", "requestedParallelism", "leaseDirectory", "files"].includes(key) && value !== undefined)));
  if (failures.length) throw Error("Invalid compiler state owner: " + failures.join("; "));
  const mode = readBuildMode(options.mode), platform = options.platform ?? process.platform, architecture = options.architecture ?? process.arch;
  const identity = createHash("sha256").update(JSON.stringify([options.owner, options.compiler, options.configuration, platform, architecture])).digest("hex").slice(0, 24);
  const directory = join(resolve(options.root), "compiler", `${platform}-${architecture}`, mode, identity);
  return requireMaterializedPathV1(directory);
}

/** 🧮️ Bounds mutable compiler lock metadata independently of input and toolchain history. */
export function compilerStateResourceV1(directory:string):string{return "compiler-state:v1:"+(parseInt(createHash("sha256").update(requireMaterializedPathV1(resolve(directory))).digest("hex").slice(0,4),16)%schema.definitions.Protocol.const.writerShards);}

/** 🔒️ Serializes access to owned mutable compiler state while retaining warm intermediates. */
export async function withCompilerState<T>(options: CompilerStateOwner & { readonly control: CompilerAdmissionControl; readonly leaseDirectory: string; readonly files: readonly string[]; readonly environment?: Readonly<Record<string, string | undefined>>; readonly capacity?: boolean; readonly requestedParallelism?: 1 }, operation: (directory: string, environment: Readonly<Record<string, string | undefined>>) => Promise<T>): Promise<T> {
  checkResourceLeaseControl(options.control);
  if (validateJsonSchemaSubset(schema.definitions.Materialization, { leaseDirectory: options.leaseDirectory, files: options.files }).length) throw Error("Complete compiler materialized paths required");
  const directory = compilerStateDirectory(options), environment = options.environment ?? process.env;
  for (const file of options.files) {
    const path = requireMaterializedPathV1(join(directory, file)), child = relative(directory, path);
    if (isAbsolute(file) || child === ".." || child.startsWith("../") || child.startsWith("..\\") || file.includes("\0")) throw Error("Compiler materialized path must remain within its state owner");
  }
  const resource=compilerStateResourceV1(directory);resourceLeasePathV1(options.leaseDirectory,resource);
  let capacity: Awaited<ReturnType<typeof acquireCompilerCapacityLeaseV1>> | undefined, lease: Awaited<ReturnType<typeof acquireResourceLease>> | undefined;
  try {
    checkResourceLeaseControl(options.control);
    if (options.capacity !== false) capacity = await acquireCompilerCapacityLeaseV1({ directory: options.leaseDirectory, control: options.control, environment, requestedParallelism: options.requestedParallelism });
    lease = await acquireResourceLease({ directory: options.leaseDirectory, resource, mode: "exclusive", signal: options.control.signal, control: options.control });
    checkResourceLeaseControl(options.control);
    await mkdir(directory, { recursive: true });
    checkResourceLeaseControl(options.control);
    const result = await operation(directory, capacity ? { ...environment, SEMIO_COMPILER_CAPACITY_HELD: "true", SEMIO_COMPILER_PARALLELISM: String(capacity.parallelism), GOMAXPROCS: String(capacity.parallelism) } : environment);
    checkResourceLeaseControl(options.control);
    return result;
  } finally { lease?.release(); capacity?.release(); }
}
