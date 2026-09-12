import { createHash } from "node:crypto";
import { lstatSync } from "node:fs";
import { isAbsolute, join, relative } from "node:path";
import { semanticOwnedInputFileSnapshot } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
export type FreshComponentRequestV1 = Readonly<{
  pluginId: string;
  cargoPackage: string;
  componentPackageId: string;
  outputName: string;
  componentProfile: "wasm-release";
  rootCdylib: boolean;
}>;

export type FreshComponentReceiptV1 = Readonly<{
  pluginId: string;
  packageId: string;
  version: string;
  component: { readonly relativePath: "component.wasm"; readonly byteLength: number; readonly sha256: string; readonly blake3: string };
  descriptor: { readonly relativePath: "descriptor.semio"; readonly byteLength: number; readonly sha256: string };
  coreSha256: string;
  witExports: readonly string[];
}>;

export type FreshBuildControlV1 = Readonly<{
  diagnosticsRoot?: string;
  cancelled(): boolean;
  remainingMs(): number;
  checkpoint(stage: string, completed: number, total: number): void;
}>;

export type FreshComponentLeaseV1 = Readonly<{
  consume<T>(derive: (component: Uint8Array) => Promise<T>): Promise<T>;
}>;

export type FreshComponentProducedV1<T> = Readonly<{
  receipt: FreshComponentReceiptV1;
  derived: T;
}>;

export type FreshSourceEpochLegV1 = Readonly<{ id: string; package: string; args: readonly string[] }>;
export type FreshSourceEpochPlanV1 = Readonly<{
  toolchain: Readonly<{ cargo: string; rustc: string }>;
  environment: readonly (readonly [string, string | null])[];
  legs: readonly FreshSourceEpochLegV1[];
  files: readonly string[];
}>;
export type FreshSourceEpochFileV1 = Readonly<{ path: string; sha256: string; byteLength: number; mode: number }>;
export type FreshSourceEpochRecordV1 = Readonly<{
  schema: "semio.plugin.fresh-source-epoch/v1";
  toolchain: FreshSourceEpochPlanV1["toolchain"];
  environment: FreshSourceEpochPlanV1["environment"];
  legs: readonly FreshSourceEpochLegV1[];
  files: readonly FreshSourceEpochFileV1[];
}>;
export type FreshSourceEpochOwnerV1 = Readonly<{
  digest: string;
  record: FreshSourceEpochRecordV1;
  start(leg: FreshSourceEpochLegV1, environment: FreshSourceEpochPlanV1["environment"]): void;
  complete(id: string, compilerInputs: readonly string[]): void;
  finish(): string;
  abort(): void;
}>;

export const FRESH_SOURCE_EPOCH_LIMITS = Object.freeze({ fileBytes: 4 * 1024 * 1024, totalBytes: 128 * 1024 * 1024, files: 32768, legs: 16 });

export function freshPathIsWithin(root: string, candidate: string): boolean {
  const path = relative(root, candidate);
  return path === "" || (!path.startsWith("..") && !isAbsolute(path));
}

export function freshCheckpoint(control: FreshBuildControlV1, stage: string, completed: number, total: number): void {
  if (control.cancelled()) throw new Error(`fresh component cancelled at ${stage}`);
  if (control.remainingMs() <= 0) throw new Error(`fresh component deadline exceeded at ${stage}`);
  control.checkpoint(stage, completed, total);
}

export function freshSourceOrderedJson(value: unknown): string {
  if (value === null || typeof value === "string" || typeof value === "boolean" || (typeof value === "number" && Number.isFinite(value))) return JSON.stringify(value);
  if (Array.isArray(value)) return "[" + value.map(freshSourceOrderedJson).join(",") + "]";
  if (value && typeof value === "object") return "{" + Object.keys(value).sort().map((key) => JSON.stringify(key) + ":" + freshSourceOrderedJson((value as Record<string, unknown>)[key])).join(",") + "}";
  throw new Error("fresh source epoch contains a non-JSON value");
}

export type FreshRustDepInfoV1 = Readonly<{ targets: readonly string[]; inputs: readonly string[]; environment: readonly (readonly [string, string | null])[] }>;

/** 📃️ Reads the Rust dep-info writer's closed rule and environment grammar without evaluating Make syntax. */
export function parseFreshRustDepInfoV1(bytes: Uint8Array, check: () => void): FreshRustDepInfoV1 {
  check();
  if (!bytes.byteLength || bytes.byteLength > 8 * 1024 * 1024) throw new Error("fresh Rust dep-info byte boundary");
  const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes).replaceAll("\r\n", "\n");
  if (!text.endsWith("\n") || text.includes("\0")) throw new Error("fresh Rust dep-info is truncated or contains NUL");
  const targets: string[] = [], fake = new Set<string>(), environment: (readonly [string, string | null])[] = [], names = new Set<string>();
  let inputs: string[] | undefined, inputSet = new Set<string>(), phase: "outputs" | "inputs" | "environment" = "outputs";
  const path = (value: string): string => {
    if (!value || Buffer.byteLength(value) > 4096 || /[\x00-\x1f\x7f]/u.test(value)) throw new Error("fresh Rust dep-info path boundary");
    return value;
  };
  const paths = (value: string): string[] => {
    const result: string[] = [];
    let token = "";
    for (let index = 0; index < value.length; index++) {
      if ((index & 65535) === 0) check();
      const character = value[index]!;
      if (character === "\\" && value[index + 1] === " ") { token += " "; index++; }
      else if (character === " ") { result.push(path(token)); token = ""; }
      else token += character;
      if (token.length > 4096 || result.length > FRESH_SOURCE_EPOCH_LIMITS.files) throw new Error("fresh Rust dep-info token boundary");
    }
    result.push(path(token));
    if (result.length > FRESH_SOURCE_EPOCH_LIMITS.files || new Set(result).size !== result.length) throw new Error("fresh Rust dep-info input count or duplicate");
    return result;
  };
  const unescapeEnvironment = (value: string): string => {
    if (Buffer.byteLength(value) > 32768) throw new Error("fresh Rust dep-info environment boundary");
    let output = "";
    for (let index = 0; index < value.length; index++) {
      const character = value[index]!;
      if (character !== "\\") { output += character; continue; }
      const escaped = value[++index];
      if (escaped === "n") output += "\n";
      else if (escaped === "r") output += "\r";
      else if (escaped === "\\") output += "\\";
      else throw new Error("fresh Rust dep-info environment escape is unknown");
    }
    if (Buffer.byteLength(output) > 16384) throw new Error("fresh Rust dep-info environment boundary");
    return output;
  };
  const lines = text.split("\n");
  if (lines.length > 4 * FRESH_SOURCE_EPOCH_LIMITS.files + 1024) throw new Error("fresh Rust dep-info line boundary");
  for (const line of lines) {
    check();
    if (!line) continue;
    if (line.startsWith("# env-dep:")) {
      if (!inputs || fake.size !== inputs.length) throw new Error("fresh Rust dep-info environment precedes complete input rules");
      phase = "environment";
      const row = line.slice(10), equal = row.indexOf("="), name = equal < 0 ? row : row.slice(0, equal);
      if (!/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(name) || names.has(name) || environment.length >= 256) throw new Error("fresh Rust dep-info environment name or count");
      names.add(name);
      environment.push(Object.freeze([name, equal < 0 ? null : unescapeEnvironment(row.slice(equal + 1))] as const));
      continue;
    }
    if (line.startsWith("#") || phase === "environment") throw new Error("fresh Rust dep-info contains an unknown or out-of-order record");
    const separator = line.indexOf(": ");
    if (separator >= 0) {
      if (phase !== "outputs" || targets.length >= 128) throw new Error("fresh Rust dep-info output order or count");
      const target = path(line.slice(0, separator)), selected = paths(line.slice(separator + 2));
      if (targets.includes(target) || (inputs && freshSourceOrderedJson(inputs) !== freshSourceOrderedJson(selected))) throw new Error("fresh Rust dep-info output dependency lists disagree");
      targets.push(target);
      inputs = selected;
      inputSet = new Set(selected);
    } else {
      if (!line.endsWith(":") || !inputs) throw new Error("fresh Rust dep-info rule is unknown");
      phase = "inputs";
      const selected = paths(line.slice(0, -1));
      if (selected.length !== 1 || !inputSet.has(selected[0]!) || fake.has(selected[0]!)) throw new Error("fresh Rust dep-info fake input rule disagrees");
      fake.add(selected[0]!);
    }
  }
  if (!targets.length || !inputs?.length || fake.size !== inputs.length) throw new Error("fresh Rust dep-info input closure is incomplete");
  return Object.freeze({ targets: Object.freeze(targets), inputs: Object.freeze(inputs), environment: Object.freeze(environment) });
}

function freshSourceCoordinate(path: string): string {
  if (typeof path !== "string" || Buffer.byteLength(path) > 4096 || /[\\\\\x00-\x1f\x7f:*?"<>|]/u.test(path) || Buffer.from(path).toString("utf8") !== path || path.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("fresh source epoch coordinate is unsafe");
  if (path.split("/").some((part) => [".git", "node_modules", "target", "🗑️generated"].includes(part))) throw new Error("fresh source epoch refuses compiler, dependency, or ticket output inputs");
  return path;
}

function freshSourceEpochEnvironment(value: FreshSourceEpochPlanV1["environment"]): FreshSourceEpochPlanV1["environment"] {
  if (!Array.isArray(value) || value.length > 256) throw new Error("fresh source epoch environment exceeds its boundary");
  const names = new Set<string>();
  return Object.freeze(value.map((row) => {
    if (!Array.isArray(row) || row.length !== 2 || !/^[A-Za-z_][A-Za-z0-9_]{0,255}$/u.test(row[0]) || names.has(row[0]) || (row[1] !== null && (typeof row[1] !== "string" || Buffer.byteLength(row[1]) > 16384 || row[1].includes("\0")))) throw new Error("fresh source epoch environment is not a closed unique projection");
    names.add(row[0]);
    return Object.freeze([row[0], row[1]] as const);
  }).sort((left, right) => Buffer.from(left[0]).compare(Buffer.from(right[0]))));
}

function freshSourceEpochLeg(leg: FreshSourceEpochLegV1): FreshSourceEpochLegV1 {
  if (!leg || Object.keys(leg).sort().join(",") !== "args,id,package" || !/^[a-z][a-z0-9-]*$/u.test(leg.id) || !/^[a-z][a-z0-9-]*$/u.test(leg.package) || !Array.isArray(leg.args) || !leg.args.length || leg.args.length > 128 || leg.args.some((arg) => typeof arg !== "string" || Buffer.byteLength(arg) > 4096 || arg.includes("\0"))) throw new Error("fresh source epoch leg is not a bounded exact command");
  return Object.freeze({ id: leg.id, package: leg.package, args: Object.freeze([...leg.args]) });
}

/** 🧾️ Encodes a bounded source epoch independently of object insertion order. */
export function freshSourceEpochBytesV1(record: FreshSourceEpochRecordV1): Uint8Array {
  const pieces: Buffer[] = [];
  const field = (value: string): void => {
    const bytes = Buffer.from(value), length = Buffer.alloc(4);
    length.writeUInt32BE(bytes.byteLength);
    pieces.push(length, bytes);
  };
  field(record.schema);
  field(freshSourceOrderedJson(record.toolchain));
  field(freshSourceOrderedJson(record.environment));
  field(freshSourceOrderedJson(record.legs));
  field(freshSourceOrderedJson(record.files));
  return Buffer.concat(pieces);
}

/** 🧊️ Retains one no-follow source epoch across exact ordered compiler legs; it does not freeze collaborators' files. */
export function captureFreshSourceEpochV1(repoRoot: string, plan: FreshSourceEpochPlanV1, control: FreshBuildControlV1): FreshSourceEpochOwnerV1 {
  const check = (stage: string, index = 0, total = plan.files.length): void => freshCheckpoint(control, `source-epoch-${stage}`, index, total);
  check("capture");
  if (!plan.toolchain || Object.keys(plan.toolchain).sort().join(",") !== "cargo,rustc" || Object.values(plan.toolchain).some((value) => typeof value !== "string" || !value.length || Buffer.byteLength(value) > 8192 || value.includes("\0"))) throw new Error("fresh source epoch toolchain identity is invalid");
  if (!Array.isArray(plan.legs) || !plan.legs.length || plan.legs.length > FRESH_SOURCE_EPOCH_LIMITS.legs || !Array.isArray(plan.files) || !plan.files.length || plan.files.length > FRESH_SOURCE_EPOCH_LIMITS.files) throw new Error("fresh source epoch count exceeds its boundary");
  const environment = freshSourceEpochEnvironment(plan.environment), legs = plan.legs.map(freshSourceEpochLeg);
  if (new Set(legs.map((leg) => leg.id)).size !== legs.length) throw new Error("fresh source epoch repeats a compiler leg");
  const paths = plan.files.map(freshSourceCoordinate).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
  if (new Set(paths).size !== paths.length) throw new Error("fresh source epoch repeats an input");
  const physical = new Map<string, string>();
  let totalBytes = 0;
  const identity = (path: string): string => {
    const node = lstatSync(join(repoRoot, path));
    if (!node.isFile() || node.isSymbolicLink() || node.size > FRESH_SOURCE_EPOCH_LIMITS.fileBytes) throw new Error("fresh source epoch input is non-regular or oversized: " + path);
    return [node.dev, node.ino, node.mode, node.size, node.mtimeMs, node.ctimeMs].join(":");
  };
  const read = (path: string): FreshSourceEpochFileV1 => {
    const before = identity(path), input = semanticOwnedInputFileSnapshot(repoRoot, path, { maximumBytes: FRESH_SOURCE_EPOCH_LIMITS.fileBytes, checkpoint: () => check("read") });
    if (!input) throw new Error("fresh source epoch input is missing: " + path);
    try {
      if (input.size > FRESH_SOURCE_EPOCH_LIMITS.fileBytes || before !== identity(path)) throw new Error("fresh source epoch input changed during capture: " + path);
      if (physical.has(path) && physical.get(path) !== before) throw new Error("fresh source epoch input identity changed: " + path);
      physical.set(path, before);
      return Object.freeze({ path, sha256: input.contentHash, byteLength: input.size, mode: input.mode });
    } finally { input.bytes.fill(0); }
  };
  const files = paths.map((path, index) => {
    check("capture", index);
    const input = read(path);
    totalBytes += input.byteLength;
    if (totalBytes > FRESH_SOURCE_EPOCH_LIMITS.totalBytes) throw new Error("fresh source epoch aggregate exceeds its byte boundary");
    return input;
  });
  const record: FreshSourceEpochRecordV1 = Object.freeze({ schema: "semio.plugin.fresh-source-epoch/v1", toolchain: Object.freeze({ ...plan.toolchain }), environment, legs: Object.freeze(legs), files: Object.freeze(files) });
  const encoded = freshSourceEpochBytesV1(record), digest = createHash("sha256").update(encoded).digest("hex");
  encoded.fill(0);
  const known = new Set(paths);
  let next = 0, phase: "ready" | "running" | "closed" = "ready";
  const verify = (): void => {
    for (const [index, expected] of files.entries()) {
      check("verify", index);
      if (freshSourceOrderedJson(read(expected.path)) !== freshSourceOrderedJson(expected)) throw new Error("fresh source epoch input bytes changed: " + expected.path);
    }
    check("verified", files.length);
  };
  const transition = <T>(operation: () => T): T => {
    try {
      if (phase === "closed") throw new Error("fresh source epoch owner is closed");
      check("transition");
      return operation();
    } catch (error) { phase = "closed"; throw error; }
  };
  return Object.freeze({
    digest, record,
    start(leg, actualEnvironment) {
      transition(() => {
        if (phase !== "ready" || next >= legs.length || freshSourceOrderedJson(freshSourceEpochLeg(leg)) !== freshSourceOrderedJson(legs[next])) throw new Error("fresh source epoch compiler leg is out of order or changed");
        if (freshSourceOrderedJson(freshSourceEpochEnvironment(actualEnvironment)) !== freshSourceOrderedJson(environment)) throw new Error("fresh source epoch compiler environment changed");
        verify();
        phase = "running";
      });
    },
    complete(id, compilerInputs) {
      transition(() => {
        if (phase !== "running" || legs[next]?.id !== id) throw new Error("fresh source epoch completion has no exact running leg");
        if (!Array.isArray(compilerInputs) || !compilerInputs.length || compilerInputs.length > FRESH_SOURCE_EPOCH_LIMITS.files || new Set(compilerInputs).size !== compilerInputs.length) throw new Error("fresh source epoch compiler input report is empty, duplicated, or oversized");
        for (const [index, input] of compilerInputs.entries()) {
          check("compiler-input", index, compilerInputs.length);
          if (!known.has(freshSourceCoordinate(input))) throw new Error("fresh source epoch has an unknown compiler input: " + input);
        }
        verify();
        next++;
        phase = "ready";
      });
    },
    finish() {
      return transition(() => {
        if (phase !== "ready" || next !== legs.length) throw new Error("fresh source epoch cannot finish incomplete compiler legs");
        verify();
        phase = "closed";
        return digest;
      });
    },
    abort() { phase = "closed"; },
  });
}
