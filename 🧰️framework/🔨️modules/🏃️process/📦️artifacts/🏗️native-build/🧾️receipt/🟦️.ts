import { resolve } from "node:path";
import type { FileObservationControlV1, FileObservationProgressV1 } from "../../../../📁️filesystem/🧾️observation/🟦️.ts";

/** 🧾️ Schema id of a completed Cargo invocation provenance receipt document.
 * @see ./🧬️schema/🔣️.json
 * @see ./🧫️fixtures/🔣️.json
 */
export const CARGO_PROVENANCE_RECEIPT_SCHEMA_V1 = "semio.cargo.provenance-receipt/v1";

/** 📏️ Largest receipt document a producer publishes and every reader admits: far below it for any real release graph
 * once paths, input digests and witnesses are interned. */
export const CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1 = 128 * 1024 * 1024;

/** 🧮️ Finite physical observation budget of one completed Cargo invocation: a full release graph digests several
 * hundred MiB of artifacts and tens of thousands of inputs once, then rechecks them before publication. */
export const CARGO_PROVENANCE_PHYSICAL_BUDGET_V1 = Object.freeze({ maxBytes: 16 * 1024 * 1024 * 1024, maxWork: 4194304, chunkBytes: 1024 * 1024, deadlineMs: 600000 });

/** 🔐️ Builds the one finite physical observation control every provenance producer hands to the receipt writer. */
export function cargoProvenancePhysicalControlV1(authority: { readonly cancelled: () => boolean; readonly remainingMs?: () => number; readonly deadlineMs?: number; readonly onProgress: (progress: FileObservationProgressV1) => void }): FileObservationControlV1 {
  const started = performance.now(), deadlineMs = authority.deadlineMs ?? CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.deadlineMs;
  return Object.freeze({
    maxBytes: CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.maxBytes,
    maxWork: CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.maxWork,
    chunkBytes: CARGO_PROVENANCE_PHYSICAL_BUDGET_V1.chunkBytes,
    cancelled: authority.cancelled,
    remainingMs: () => Math.min(authority.remainingMs?.() ?? Infinity, deadlineMs - (performance.now() - started)),
    onProgress: authority.onProgress,
  });
}

/** 📜️ The source files a Cargo dep-info file (`<artifact>.d`) names for its target, with Cargo's
 * `\ ` space escape undone. Only the first `target: deps` rule counts; the empty per-dependency
 * rules Cargo appends after it carry nothing. */
export function cargoDepInfoSourcesV1(text: string): readonly string[] {
  const rule = text.split(/\r?\n/u).find((line) => line.includes(": ")) ?? "";
  return rule
    .slice(rule.indexOf(": ") + 2)
    .split(/(?<!\\) /u)
    .filter((entry) => entry.length > 0)
    .map((entry) => entry.replace(/\\ /gu, " "));
}

/** 🔢️ One consumed-byte checksum rustc itself wrote into a dep-info file, with its path exactly as written. */
export type CargoDepInfoChecksumV1 = Readonly<{ path: string; blake3: string; length: number }>;

/** 🧷️ The consumed-byte checksums of a Cargo dep-info file, in the order rustc wrote them. */
export function cargoDepInfoChecksumsV1(text: string): readonly CargoDepInfoChecksumV1[] {
  return text.split(/\r?\n/u).flatMap((line) => {
    const match = /^# checksum:blake3=([0-9a-f]{64}) file_len:([0-9]+) (.+)$/u.exec(line);
    return match ? [{ path: match[3]!, blake3: match[1]!, length: Number(match[2]) }] : [];
  });
}

/** 📎️ One observed input: absent paths keep a `null` kind and digest instead of a guessed one. */
export type CargoInputObservationV1 = Readonly<{ path: string; kind: "file" | "directory" | null; sha256: string | null }>;

/** 🔗️ A dep-info file kept as its digest plus the sources and consumed-byte checksums parsed from exactly those bytes. */
export type CargoDepInfoV1 = Readonly<{ path: string; sha256: string; baseDirectory: string | null; sources: readonly string[]; checksums: readonly CargoDepInfoChecksumV1[] }>;

/** 📦️ One observed compiler artifact, with its staged copy when the build published one. */
export type CargoArtifactObservationV1 = Readonly<{ path: string; sha256: string | null; stagedPath?: string; stagedSha256?: string | null }>;

/** 🏗️ One completed compiler unit with its cargo message, dep-info, input digests and artifact digests. */
export type CargoUnitObservationV1 = Readonly<{ message: Readonly<Record<string, unknown>>; observedAtMs: number; depInfo: readonly CargoDepInfoV1[]; inputs: readonly CargoInputObservationV1[]; artifacts: readonly CargoArtifactObservationV1[] }>;

/** 🗂️ One resource witness row: the original producer observation plus the digests observed afterwards. */
export type CargoResourceWitnessV1 = Readonly<{ input: Readonly<Record<string, unknown>> } & Record<string, unknown>>;

/** 🛠️ The resource witnesses of one build script's `OUT_DIR` roster, bound to that roster by digest. */
export type CargoBuildResourcesV1 = Readonly<{ package_id: string; out_dir: string; path: string; sha256: string; observedAtMs: number; resources: readonly CargoResourceWitnessV1[] }>;

/** 🔬️ The resource witnesses of one proc-macro compiler resource capture, bound to it by digest. */
export type CargoCompilerResourcesV1 = Readonly<{
  path: string;
  sha256: string | null;
  observedAtMs: number;
  producer: Readonly<{ manifest: string; source: string }> | null;
  caller: Readonly<{ manifest: string; crate: string; source: string }> | null;
  producerUnit: Readonly<Record<string, unknown>> | null;
  callerUnit: Readonly<Record<string, unknown>> | null;
  resources: readonly CargoResourceWitnessV1[];
}>;

/** 🎚️ One Cargo configuration input digest. */
export type CargoInvocationInputV1 = Readonly<{ path: string; sha256: string | null }>;

/** 📑️ A completed Cargo invocation exactly as readers see it after the interned receipt document is expanded. */
export type CargoProvenanceV1 = Readonly<{
  version: 1;
  manifest: string;
  cwd: string;
  command: string;
  args: readonly string[];
  buildDirectory: string | null;
  builtAtMs: number;
  status: number;
  cancelled: boolean;
  units: readonly CargoUnitObservationV1[];
  buildScripts: readonly Readonly<Record<string, unknown>>[];
  observedAtMs: number;
  invocationInputs: readonly CargoInvocationInputV1[];
  buildResources: readonly CargoBuildResourcesV1[];
  compilerResourceRoot: string | null;
  compilerResources: readonly CargoCompilerResourcesV1[];
}>;

/** 🧭️ The absolute source paths a dep-info record names, resolved against its base directory. */
export function cargoDepInfoResolvedSourcesV1(dep: Pick<CargoDepInfoV1, "baseDirectory" | "sources">): readonly string[] {
  if (dep.baseDirectory === null) throw Error("Cargo dep-info has no base directory");
  const base = dep.baseDirectory;
  return [...new Set(dep.sources.map((source) => resolve(base, source)))].sort();
}

/** 🔏️ The consumed-byte checksums a dep-info record carries, with their paths resolved against its base directory. */
export function cargoDepInfoResolvedChecksumsV1(dep: Pick<CargoDepInfoV1, "baseDirectory" | "checksums">): readonly CargoDepInfoChecksumV1[] {
  if (dep.baseDirectory === null) throw Error("Cargo dep-info has no base directory");
  const base = dep.baseDirectory;
  return dep.checksums.map((row) => ({ path: resolve(base, row.path), blake3: row.blake3, length: row.length }));
}

/** 🚰️ Receives the receipt document as ordered text chunks without ever holding the whole document. */
export interface CargoProvenanceSinkV1 {
  write(chunk: string): Promise<void>;
}

/** 🏁️ The scalar identity of the invocation a receipt document opens with. */
export type CargoProvenanceHeaderV1 = Readonly<{ manifest: string; cwd: string; command: string; args: readonly string[]; buildDirectory: string | null; builtAtMs: number; status: number; cancelled: boolean }>;

const PATH_KEYS: ReadonlySet<string> = new Set(["path", "source", "output", "symlinkTarget"]);
const UNITS = 1;
const BUILD_RESOURCES = 2;
const COMPILER_RESOURCES = 3;
const INVOCATION_INPUTS = 4;
const FINISHED = 5;
const refused = (detail: string): never => {
  throw Error("Cargo provenance receipt " + detail);
};

/** 🌊️ Writes one receipt document in bounded memory: every record is serialized and handed to the sink as soon as it is
 * complete, while repeated paths, input digests and resource witnesses are interned once in tables written last.
 * @see ./🧬️schema/🔣️.json
 */
export class CargoProvenanceStreamV1 {
  private readonly strings = new Map<string, number>();
  private readonly table: string[] = [];
  private readonly inputIndex = new Map<string, number>();
  private readonly inputRows: string[] = [];
  private readonly witnessIndex = new Map<string, number>();
  private readonly witnessRows: string[] = [];
  private readonly messageIndex = new Map<string, number>();
  private pending: string[] = [];
  private pendingLength = 0;
  private section = 0;
  private count = 0;

  constructor(
    private readonly sink: CargoProvenanceSinkV1,
    private readonly flushLength: number = 1 << 20,
  ) {}

  /** 🚩️ Opens the document with the invocation identity and the units array. */
  async begin(header: CargoProvenanceHeaderV1): Promise<void> {
    this.advance(0, UNITS);
    await this.emit(
      `{"schema":${JSON.stringify(CARGO_PROVENANCE_RECEIPT_SCHEMA_V1)},"manifest":${JSON.stringify(header.manifest)},"cwd":${JSON.stringify(header.cwd)},"command":${JSON.stringify(header.command)},"args":${JSON.stringify(header.args)},"buildDirectory":${JSON.stringify(header.buildDirectory)},"builtAtMs":${JSON.stringify(header.builtAtMs)},"status":${JSON.stringify(header.status)},"cancelled":${JSON.stringify(header.cancelled)},"units":[`,
    );
    this.count = 0;
  }

  /** 🧱️ Appends one completed compiler unit. */
  async unit(unit: CargoUnitObservationV1): Promise<void> {
    this.expect(UNITS);
    const message = JSON.stringify(unit.message);
    this.messageIndex.set(message, this.messageIndex.has(message) ? -1 : this.count);
    const depInfo = unit.depInfo.map((dep) => ({
      path: this.ref(dep.path),
      sha256: dep.sha256,
      baseDirectory: this.nullableRef(dep.baseDirectory),
      sources: dep.sources.map((source) => this.ref(source)),
      checksums: dep.checksums.map((row) => [this.ref(row.path), row.blake3, row.length]),
    }));
    const artifacts = unit.artifacts.map((artifact) => ({
      path: this.ref(artifact.path),
      sha256: artifact.sha256,
      ...(artifact.stagedPath === undefined ? {} : { stagedPath: this.ref(artifact.stagedPath) }),
      ...(artifact.stagedSha256 === undefined ? {} : { stagedSha256: artifact.stagedSha256 }),
    }));
    await this.element(`{"message":${message},"observedAtMs":${JSON.stringify(unit.observedAtMs)},"depInfo":${JSON.stringify(depInfo)},"inputs":${JSON.stringify(unit.inputs.map((input) => this.input(input)))},"artifacts":${JSON.stringify(artifacts)}}`);
  }

  /** 🔧️ Closes the units and records the verbatim build-script messages. */
  async buildScripts(rows: readonly Readonly<Record<string, unknown>>[]): Promise<void> {
    this.advance(UNITS, BUILD_RESOURCES);
    await this.emit(`],"buildScripts":${JSON.stringify(rows)},"buildResources":[`);
    this.count = 0;
  }

  /** ⚙️ Appends the resource witnesses of one build script. */
  async buildResource(row: CargoBuildResourcesV1): Promise<void> {
    this.expect(BUILD_RESOURCES);
    await this.element(JSON.stringify({ package_id: row.package_id, out_dir: row.out_dir, path: row.path, sha256: row.sha256, observedAtMs: row.observedAtMs, resources: row.resources.map((witness) => this.witness(witness)) }));
  }

  /** 🔭️ Closes the build resources and opens the compiler resource captures under their explicit root. */
  async compilerResources(root: string | null): Promise<void> {
    this.advance(BUILD_RESOURCES, COMPILER_RESOURCES);
    await this.emit(`],"compilerResourceRoot":${JSON.stringify(root)},"compilerResources":[`);
    this.count = 0;
  }

  /** 🧫️ Appends one proc-macro compiler resource capture. */
  async compilerResource(row: CargoCompilerResourcesV1): Promise<void> {
    this.expect(COMPILER_RESOURCES);
    await this.element(
      JSON.stringify({
        path: this.ref(row.path),
        sha256: row.sha256,
        observedAtMs: row.observedAtMs,
        producer: row.producer === null ? null : { manifest: this.ref(row.producer.manifest), source: this.ref(row.producer.source) },
        caller: row.caller === null ? null : { manifest: this.ref(row.caller.manifest), crate: row.caller.crate, source: this.ref(row.caller.source) },
        producerUnit: this.unitOf(row.producerUnit),
        callerUnit: this.unitOf(row.callerUnit),
        resources: row.resources.map((witness) => this.witness(witness)),
      }),
    );
  }

  /** 📋️ Closes the compiler resources and records the Cargo configuration input digests. */
  async invocationInputs(rows: readonly CargoInvocationInputV1[]): Promise<void> {
    this.advance(COMPILER_RESOURCES, INVOCATION_INPUTS);
    await this.emit(`],"invocationInputs":${JSON.stringify(rows.map((row) => [this.ref(row.path), row.sha256]))}`);
  }

  /** 🔚️ Writes the observation time and the interned tables, then closes the document. */
  async finish(observedAtMs: number): Promise<void> {
    this.advance(INVOCATION_INPUTS, FINISHED);
    await this.emit(`,"observedAtMs":${JSON.stringify(observedAtMs)},"strings":[`);
    for (const [index, value] of this.table.entries()) await this.emit((index === 0 ? "" : ",") + JSON.stringify(value));
    await this.emit(`],"inputs":[`);
    for (const [index, value] of this.inputRows.entries()) await this.emit((index === 0 ? "" : ",") + value);
    await this.emit(`],"witnesses":[`);
    for (const [index, value] of this.witnessRows.entries()) await this.emit((index === 0 ? "" : ",") + value);
    await this.emit("]}\n");
    await this.flush();
  }

  private advance(from: number, to: number): void {
    if (this.section !== from) refused("stream is out of order");
    this.section = to;
  }

  private expect(section: number): void {
    if (this.section !== section) refused("stream is out of order");
  }

  private async element(text: string): Promise<void> {
    await this.emit((this.count === 0 ? "" : ",") + text);
    this.count++;
  }

  private async emit(text: string): Promise<void> {
    this.pending.push(text);
    this.pendingLength += text.length;
    if (this.pendingLength >= this.flushLength) await this.flush();
  }

  private async flush(): Promise<void> {
    if (!this.pending.length) return;
    const chunk = this.pending.join("");
    this.pending = [];
    this.pendingLength = 0;
    await this.sink.write(chunk);
  }

  private ref(value: string): number {
    if (typeof value !== "string") return refused("path must be a string");
    let index = this.strings.get(value);
    if (index === undefined) {
      index = this.table.length;
      const own = Buffer.from(value, "utf8").toString("utf8");
      this.table.push(own);
      this.strings.set(own, index);
    }
    return index;
  }

  private nullableRef(value: string | null | undefined): number | null {
    return value === null || value === undefined ? null : this.ref(value);
  }

  private input(row: CargoInputObservationV1): number {
    const text = JSON.stringify([this.ref(row.path), row.kind, row.sha256]);
    const known = this.inputIndex.get(row.path);
    if (known !== undefined) {
      if (this.inputRows[known] !== text) refused("observed one input twice with different digests");
      return known;
    }
    this.inputIndex.set(row.path, this.inputRows.length);
    this.inputRows.push(text);
    return this.inputRows.length - 1;
  }

  private witness(row: CargoResourceWitnessV1): number {
    const text = JSON.stringify(this.intern(row, undefined));
    let index = this.witnessIndex.get(text);
    if (index === undefined) {
      index = this.witnessRows.length;
      this.witnessRows.push(text);
      this.witnessIndex.set(text, index);
    }
    return index;
  }

  private intern(value: unknown, key: string | undefined): unknown {
    if (typeof value === "string") return key !== undefined && PATH_KEYS.has(key) ? this.ref(value) : value;
    if (value !== null && key !== undefined && PATH_KEYS.has(key)) return refused(`witness field ${key} must be a string or null`);
    if (Array.isArray(value)) return value.map((item) => this.intern(item, undefined));
    if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).filter(([, item]) => item !== undefined).map(([name, item]) => [name, this.intern(item, name)]));
    return value;
  }

  private unitOf(message: Readonly<Record<string, unknown>> | null): number | null {
    if (message === null) return null;
    const index = this.messageIndex.get(JSON.stringify(message));
    if (index === undefined || index < 0) return refused("compiler resource names no single recorded unit");
    return index;
  }
}

/** 🖨️ Serializes a complete invocation record to its receipt document text. */
export async function encodeCargoProvenanceV1(observation: CargoProvenanceV1): Promise<string> {
  const chunks: string[] = [];
  const stream = new CargoProvenanceStreamV1({ write: async (chunk) => void chunks.push(chunk) });
  await stream.begin(observation);
  for (const unit of observation.units) await stream.unit(unit);
  await stream.buildScripts(observation.buildScripts);
  for (const row of observation.buildResources) await stream.buildResource(row);
  await stream.compilerResources(observation.compilerResourceRoot);
  for (const row of observation.compilerResources) await stream.compilerResource(row);
  await stream.invocationInputs(observation.invocationInputs);
  await stream.finish(observation.observedAtMs);
  return chunks.join("");
}

type Json = Record<string, unknown>;
const record = (value: unknown, name: string): Json => {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return refused(`${name} must be an object`);
  return value as Json;
};
const closed = (value: unknown, name: string, keys: readonly string[]): Json => {
  const row = record(value, name);
  for (const key of Object.keys(row)) if (!keys.includes(key)) return refused(`${name} carries the unknown field ${key}`);
  return row;
};
const list = (value: unknown, name: string): unknown[] => {
  if (!Array.isArray(value)) return refused(`${name} must be an array`);
  return value;
};
const text = (value: unknown, name: string): string => {
  if (typeof value !== "string") return refused(`${name} must be a string`);
  return value;
};
const finite = (value: unknown, name: string): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return refused(`${name} must be a finite number`);
  return value;
};
const digest = (value: unknown, name: string): string => {
  if (typeof value !== "string" || !/^[0-9a-f]{64}$/u.test(value)) return refused(`${name} must be a sha256 digest`);
  return value;
};
const nullableDigest = (value: unknown, name: string): string | null => (value === null ? null : digest(value, name));

/** 📖️ Expands a receipt document into the invocation record readers verify, sharing every interned string and witness. */
export function decodeCargoProvenanceV1(source: string): CargoProvenanceV1 {
  let document: Json;
  try {
    document = closed(JSON.parse(source), "document", ["schema", "manifest", "cwd", "command", "args", "buildDirectory", "builtAtMs", "status", "cancelled", "units", "buildScripts", "buildResources", "compilerResourceRoot", "compilerResources", "invocationInputs", "observedAtMs", "strings", "inputs", "witnesses"]);
  } catch (error) {
    if (error instanceof SyntaxError) return refused("is not valid JSON");
    throw error;
  }
  if (document.schema !== CARGO_PROVENANCE_RECEIPT_SCHEMA_V1) return refused("has an unknown schema");
  const strings = list(document.strings, "strings").map((value) => text(value, "string"));
  const str = (value: unknown, name: string): string => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value >= strings.length) return refused(`${name} names no interned string`);
    return strings[value]!;
  };
  const nullableStr = (value: unknown, name: string): string | null => (value === null ? null : str(value, name));
  const inputRows = list(document.inputs, "inputs").map((value) => {
    const row = list(value, "input");
    if (row.length !== 3 || (row[1] !== null && row[1] !== "file" && row[1] !== "directory")) return refused("input row lost its path, kind or digest");
    return Object.freeze({ path: str(row[0], "input path"), kind: row[1] as "file" | "directory" | null, sha256: nullableDigest(row[2], "input digest") });
  });
  const witnessRows = list(document.witnesses, "witnesses");
  const witnesses = new Map<number, CargoResourceWitnessV1>();
  const expand = (value: unknown, key: string | undefined): unknown => {
    if (key !== undefined && PATH_KEYS.has(key)) return value === null ? null : str(value, `witness ${key}`);
    if (Array.isArray(value)) return value.map((item) => expand(item, undefined));
    if (value !== null && typeof value === "object") return Object.fromEntries(Object.entries(value).map(([name, item]) => [name, expand(item, name)]));
    return value;
  };
  const witness = (value: unknown): CargoResourceWitnessV1 => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value >= witnessRows.length) return refused("resource names no interned witness");
    let row = witnesses.get(value);
    if (row === undefined) {
      row = record(expand(record(witnessRows[value], "witness"), undefined), "witness") as CargoResourceWitnessV1;
      record(row.input, "witness input");
      witnesses.set(value, row);
    }
    return row;
  };
  const input = (value: unknown): CargoInputObservationV1 => {
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value >= inputRows.length) return refused("unit names no interned input");
    return inputRows[value]!;
  };
  const units: CargoUnitObservationV1[] = list(document.units, "units").map((value) => {
    const unit = closed(value, "unit", ["message", "observedAtMs", "depInfo", "inputs", "artifacts"]);
    return {
      message: record(unit.message, "unit message"),
      depInfo: list(unit.depInfo, "dep-info").map((entry) => {
        const dep = closed(entry, "dep-info", ["path", "sha256", "baseDirectory", "sources", "checksums"]);
        return {
          path: str(dep.path, "dep-info path"),
          sha256: digest(dep.sha256, "dep-info digest"),
          baseDirectory: nullableStr(dep.baseDirectory, "dep-info base directory"),
          sources: list(dep.sources, "dep-info sources").map((source) => str(source, "dep-info source")),
          checksums: list(dep.checksums, "dep-info checksums").map((row) => {
            const checksum = list(row, "dep-info checksum");
            if (checksum.length !== 3 || typeof checksum[2] !== "number" || !Number.isSafeInteger(checksum[2]) || checksum[2] < 0) return refused("dep-info checksum lost its path, digest or length");
            return { path: str(checksum[0], "dep-info checksum path"), blake3: digest(checksum[1], "dep-info checksum digest"), length: checksum[2] };
          }),
        };
      }),
      inputs: list(unit.inputs, "unit inputs").map(input),
      observedAtMs: finite(unit.observedAtMs, "unit observation time"),
      artifacts: list(unit.artifacts, "unit artifacts").map((entry) => {
        const artifact = closed(entry, "artifact", ["path", "sha256", "stagedPath", "stagedSha256"]);
        return {
          path: str(artifact.path, "artifact path"),
          sha256: nullableDigest(artifact.sha256, "artifact digest"),
          ...(artifact.stagedPath === undefined ? {} : { stagedPath: str(artifact.stagedPath, "staged artifact path") }),
          ...(artifact.stagedSha256 === undefined ? {} : { stagedSha256: nullableDigest(artifact.stagedSha256, "staged artifact digest") }),
        };
      }),
    };
  });
  const unitMessage = (value: unknown): Readonly<Record<string, unknown>> | null => {
    if (value === null) return null;
    if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0 || value >= units.length) return refused("compiler resource names no recorded unit");
    return units[value]!.message;
  };
  const buildResources: CargoBuildResourcesV1[] = list(document.buildResources, "build resources").map((value) => {
    const row = closed(value, "build resource", ["package_id", "out_dir", "path", "sha256", "observedAtMs", "resources"]);
    return { package_id: text(row.package_id, "build resource package"), out_dir: text(row.out_dir, "build resource directory"), path: text(row.path, "build resource path"), sha256: digest(row.sha256, "build resource digest"), observedAtMs: finite(row.observedAtMs, "build resource observation time"), resources: list(row.resources, "build resource witnesses").map(witness) };
  });
  const compilerResources: CargoCompilerResourcesV1[] = list(document.compilerResources, "compiler resources").map((value) => {
    const row = closed(value, "compiler resource", ["path", "sha256", "observedAtMs", "producer", "caller", "producerUnit", "callerUnit", "resources"]);
    const producer = row.producer === null ? null : closed(row.producer, "compiler resource producer", ["manifest", "source"]);
    const caller = row.caller === null ? null : closed(row.caller, "compiler resource caller", ["manifest", "crate", "source"]);
    return {
      path: str(row.path, "compiler resource path"),
      sha256: nullableDigest(row.sha256, "compiler resource digest"),
      observedAtMs: finite(row.observedAtMs, "compiler resource observation time"),
      producer: producer === null ? null : { manifest: str(producer.manifest, "producer manifest"), source: str(producer.source, "producer source") },
      caller: caller === null ? null : { manifest: str(caller.manifest, "caller manifest"), crate: text(caller.crate, "caller crate"), source: str(caller.source, "caller source") },
      producerUnit: unitMessage(row.producerUnit),
      callerUnit: unitMessage(row.callerUnit),
      resources: list(row.resources, "compiler resource witnesses").map(witness),
    };
  });
  const root = document.compilerResourceRoot === null ? null : text(document.compilerResourceRoot, "compiler resource root");
  return {
    version: 1,
    manifest: text(document.manifest, "manifest"),
    cwd: text(document.cwd, "cwd"),
    command: text(document.command, "command"),
    args: list(document.args, "args").map((value) => text(value, "argument")),
    buildDirectory: document.buildDirectory === null ? null : text(document.buildDirectory, "build directory"),
    builtAtMs: finite(document.builtAtMs, "build time"),
    status: finite(document.status, "status"),
    cancelled: typeof document.cancelled === "boolean" ? document.cancelled : refused("cancelled must be a boolean"),
    units,
    buildScripts: list(document.buildScripts, "build scripts").map((value) => record(value, "build script")),
    observedAtMs: finite(document.observedAtMs, "observation time"),
    invocationInputs: list(document.invocationInputs, "invocation inputs").map((value) => {
      const row = list(value, "invocation input");
      if (row.length !== 2) return refused("invocation input lost its path or digest");
      return { path: str(row[0], "invocation input path"), sha256: nullableDigest(row[1], "invocation input digest") };
    }),
    buildResources,
    compilerResourceRoot: root,
    compilerResources,
  };
}
