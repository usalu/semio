/** 🧾️ The proctor as a cached build product: `@teaching/proctor:build` leaves the executable at a content-addressed place
 * with a receipt that names its exact path and SHA-256, and every course target resolves that receipt — a missing or
 * stale one is one clear English and German error, never a silent rebuild. The language-agnostic fixture
 * (`🧫️fixtures/🔣️.json`) holds the receipts, the Cargo and dep-info texts and the expected outcomes; the oracles are
 * Ajv for the receipt schema, `@noble/hashes` for the digest, `picomatch` for the declared input globs and, in the
 * opt-in live law, the dep-info files of Cargo's own last build.
 * @see ../../../../🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts — the product under test
 * @see https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages
 * @see https://doc.rust-lang.org/cargo/reference/build-cache.html#dep-info-files */
import Ajv from "ajv";
import picomatch from "picomatch";
import { sha256 } from "@noble/hashes/sha2.js";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import schema from "../../../../🛂️proctor/🏗️bootstrap/🧾️product/🧬️schema/🔣️.json" with { type: "json" };
import inputs from "../../../../🛂️proctor/🏗️bootstrap/🧾️product/📥️inputs/🔣️.json" with { type: "json" };
import messages from "../../../../🛂️proctor/🏗️bootstrap/🧾️product/💬️messages/🔣️.json" with { type: "json" };
import fixtures from "../../../../🛂️proctor/🏗️bootstrap/🧾️product/🧪️tests/🧫️fixtures/🔣️.json" with { type: "json" };
import {
  PROCTOR_PRODUCT_RECEIPT,
  captureProctorProduct,
  checkProctorProduct,
  declaredProctorInputs,
  parseDepInfo,
  proctorDepInfoFiles,
  proctorProductDirectory,
  proctorProductMessage,
  proctorSourceGlobs,
  readProctorProductReceipt,
  selectProctorExecutable,
  uncoveredProctorSources,
  ProctorProductError,
} from "../../../../🛂️proctor/🏗️bootstrap/🧾️product/🟦️.ts";
import { proctorSourceDirectories } from "../../../../🛂️proctor/🏗️bootstrap/🟦️.ts";

const repository = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");
const manifestPath = process.env.PROCTOR_MANIFEST ?? join(repository, "🎓️teaching", "🛂️proctor", "📦️packages", "🦀️rust", "📋️project.json");
const scratch: string[] = [];
const directory = (): string => {
  const path = mkdtempSync(join(tmpdir(), "proctor-product-"));
  scratch.push(path);
  return path;
};
afterEach(() => { for (const path of scratch.splice(0)) rmSync(path, { recursive: true, force: true }); });

const validateReceipt = new Ajv().compile({ ...schema, $ref: "#/definitions/Receipt" });
const validateMessages = new Ajv().compile({ ...schema, $ref: "#/definitions/Messages" });
const validateInputs = new Ajv().compile({ ...schema, $ref: "#/definitions/Inputs" });
const digest = (bytes: string): string => Buffer.from(sha256(Buffer.from(bytes))).toString("hex");
const receiptOf = (id: string): (typeof fixtures.receipts)[number]["receipt"] => fixtures.receipts.find(row => row.id === id)!.receipt;

describe("the receipt", () => {
  it("agrees with the independent schema validator on every fixture", () => {
    for (const row of fixtures.receipts) {
      expect(validateReceipt(row.receipt), row.id).toBe(row.valid);
      if (row.valid) expect(readProctorProductReceipt(row.receipt)).toEqual(row.receipt);
      else expect(() => readProctorProductReceipt(row.receipt), row.id).toThrow();
    }
  });

  it("names a digest that the independent digest reproduces", () => {
    expect(digest(fixtures.bytes)).toBe(fixtures.sha256);
    expect(fixtures.bytes.length).toBe(receiptOf("valid-prototype").bytes);
  });

  it("refuses a receipt whose executable is not its own digest", () => {
    const other = "0".repeat(64);
    expect(() => readProctorProductReceipt({ ...receiptOf("valid-prototype"), executable: `${other}/proctor` })).toThrow();
    expect(() => readProctorProductReceipt({ ...receiptOf("valid-prototype"), executable: `${fixtures.sha256}/proctor.exe` })).toThrow();
  });
});

describe("the messages", () => {
  it("are schema-valid, carry English and German with the same placeholders, and say what to run", () => {
    expect(validateMessages(messages)).toBe(true);
    const placeholders = (text: string): string[] => [...text.matchAll(/\{(\w+)\}/gu)].map(row => row[1]!).sort();
    for (const group of [messages.errors, messages.reasons]) {
      for (const [name, row] of Object.entries(group)) {
        expect(placeholders(row.en), name).toEqual(placeholders(row.de));
        expect(row.en).not.toBe(row.de);
      }
    }
  });

  it("render English first and German second, with the build command of the mode and the tool", () => {
    const prototype = proctorProductMessage("missing", { mode: "prototype", path: "/w/receipt.json" }, {});
    const [english, german] = prototype.split("\n");
    expect(english).toContain("has not been built");
    expect(german).toContain("nicht gebaut");
    expect(prototype).toContain("`repo run @teaching/proctor:build`");
    expect(proctorProductMessage("missing", { mode: "release", path: "/w/receipt.json" }, {})).toContain("`repo run @teaching/proctor:build --configuration=release`");
    expect(proctorProductMessage("missing", { mode: "prototype", path: "x" }, { SEMIO_BUILD_TOOL: "bazel" })).toContain("`repo --tool=bazel run @teaching/proctor:build`");
    expect(prototype).not.toMatch(/\{\w+\}/u);
  });
});

describe("Cargo's output", () => {
  const { cargo } = fixtures;
  it("names the proctor executable and nothing else", () => {
    expect(selectProctorExecutable(cargo.stdout.join("\n"))).toBe(cargo.executable);
    expect(() => selectProctorExecutable(cargo.withoutExecutable.join("\n"))).toThrow(/executable/u);
    expect(() => selectProctorExecutable(cargo.twoExecutables.join("\n"))).toThrow(/executable/u);
  });

  it("points at the dep-info of every local unit and of no registry crate", () => {
    expect(proctorDepInfoFiles(cargo.stdout.join("\n"))).toEqual(cargo.depInfoFiles);
  });

  it("parses dep-info like a Makefile rule reader", () => {
    for (const row of fixtures.depInfo) expect(parseDepInfo(row.text), row.id).toEqual(row.paths);
  });
});

describe("the declared inputs", () => {
  const { globs } = fixtures;
  it("list data files exactly, in the schema's closed form, and every one exists", () => {
    expect(validateInputs(inputs)).toBe(true);
    expect([...inputs.files].sort()).toEqual(inputs.files);
    for (const file of inputs.files) expect(existsSync(join(repository, file)), file).toBe(true);
  });

  it("are one pair of globs per owner root and exclude build output and installed packages", () => {
    expect(proctorSourceGlobs(globs.roots, globs.data)).toEqual(globs.inputs);
  });

  it("match, by the independent glob library, what the owner roots hold and nothing beside them", () => {
    const include = globs.inputs.filter(row => !row.startsWith("!")).map(row => row.replace("{workspaceRoot}/", ""));
    const exclude = globs.inputs.filter(row => row.startsWith("!")).map(row => row.slice(1).replace("{workspaceRoot}/", ""));
    const declared = (file: string): boolean => include.some(pattern => picomatch(pattern, { dot: true })(file)) && !exclude.some(pattern => picomatch(pattern, { dot: true })(file));
    for (const file of globs.covered) expect(declared(file), file).toBe(true);
    for (const file of globs.uncovered) expect(declared(file), file).toBe(false);
  });

  it("are checked by the build with the same verdicts", () => {
    expect(uncoveredProctorSources(globs.covered, globs.inputs)).toEqual([]);
    expect(uncoveredProctorSources([...globs.covered, ...globs.uncovered], globs.inputs)).toEqual(globs.uncovered);
  });
});

describe("the resolution of a built product", () => {
  const product = (row: (typeof fixtures.resolution)[number]): { root: string; host: { platform: NodeJS.Platform; architecture: string } } => {
    const root = directory();
    const folder = proctorProductDirectory(root, row.mode as "prototype" | "release");
    mkdirSync(folder, { recursive: true });
    const receipt = "receipt" in row && row.receipt ? receiptOf(row.receipt) : undefined;
    if ("raw" in row && typeof row.raw === "string") writeFileSync(join(folder, PROCTOR_PRODUCT_RECEIPT), row.raw);
    else if (receipt) writeFileSync(join(folder, PROCTOR_PRODUCT_RECEIPT), JSON.stringify(receipt));
    for (const [file, bytes] of Object.entries(row.files)) {
      mkdirSync(dirname(join(folder, file)), { recursive: true });
      writeFileSync(join(folder, file), bytes);
    }
    const host = "host" in row && row.host ? (row.host as { platform: NodeJS.Platform; architecture: string }) : { platform: (receipt?.platform ?? process.platform) as NodeJS.Platform, architecture: receipt?.architecture ?? process.arch };
    return { root, host };
  };

  for (const row of fixtures.resolution) {
    it(`${row.expect === null ? "accepts" : "refuses"} ${row.id}`, async () => {
      const { root, host } = product(row);
      const mode = row.mode as "prototype" | "release";
      if (row.expect === null) {
        const resolved = await checkProctorProduct(root, mode, host);
        expect(resolved.executable).toBe(join(proctorProductDirectory(root, mode), fixtures.sha256, "proctor"));
        expect(resolved.receipt.sha256).toBe(digest(readFileSync(resolved.executable, "utf8")));
        return;
      }
      const failure = await checkProctorProduct(root, mode, host).then(() => undefined, (error: unknown) => error);
      expect(failure).toBeInstanceOf(ProctorProductError);
      const error = failure as ProctorProductError;
      expect(error.code).toBe(row.expect === "missing" ? "missing" : "stale");
      expect(error.reason).toBe(row.expect === "missing" ? undefined : row.expect);
      const [english, german] = error.message.split("\n");
      expect(english).toMatch(/proctor/iu);
      expect(german).toMatch(/proctor/iu);
      expect(error.message).toContain(`repo${process.env.SEMIO_BUILD_TOOL === "bazel" ? " --tool=bazel" : ""} run @teaching/proctor:build`);
      expect(readdirSync(root).filter(name => name !== "🎓️teaching")).toEqual([]);
    });
  }
});

describe("the capture of a cargo build", () => {
  const { cargo } = fixtures;
  const layout = (): { root: string; target: string; stdout: string } => {
    const root = directory();
    const target = join(root, ".target");
    const write = (file: string, text: string): void => { mkdirSync(dirname(file), { recursive: true }); writeFileSync(file, text); };
    const source = (relative: string): string => { write(join(root, relative), "// source\n"); return join(root, relative); };
    write(join(target, "debug", "proctor"), fixtures.bytes);
    write(join(target, "debug", "deps", "dep-aa.d"), `x: ${source("🧰️framework/dep/lib.rs")} ${source("🧰️framework/dep/Cargo.toml")}\n`);
    write(join(target, "debug", "build", "dep-bb", "build_script_build-bb.d"), `x: ${source("🧰️framework/dep/build.rs")}\n`);
    write(join(target, "debug", "deps", "macro-dd.d"), `x: ${source("🧰️framework/macro/lib.rs")}\n`);
    write(join(target, "debug", "deps", "proctor-cc.d"), `x: ${source("🎓️teaching/proctor/main.rs")} ${source("🧰️framework/dep/lib.rs")} ${join(dirname(root), "registry", "serde.rs")}\n`);
    const stdout = cargo.stdout.join("\n").replaceAll("/t/", `${target}/`).replaceAll("file:///w/", `file://${root}/`);
    return { root, target, stdout };
  };

  it("copies the executable beside a receipt that names its path and digest, and lists the sources Cargo read", async () => {
    const { root, stdout } = layout();
    const receipt = await captureProctorProduct(root, "prototype", stdout, { host: { platform: "darwin", architecture: "arm64" } });
    const folder = proctorProductDirectory(root, "prototype");
    expect(receipt).toEqual(JSON.parse(readFileSync(join(folder, PROCTOR_PRODUCT_RECEIPT), "utf8")));
    expect(validateReceipt(receipt)).toBe(true);
    expect(receipt.sha256).toBe(digest(fixtures.bytes));
    expect(receipt.executable).toBe(`${digest(fixtures.bytes)}/proctor`);
    expect(receipt.bytes).toBe(fixtures.bytes.length);
    expect(readFileSync(join(folder, receipt.executable), "utf8")).toBe(fixtures.bytes);
    expect(receipt.sources).toEqual(["🎓️teaching/proctor/main.rs", "🧰️framework/dep/Cargo.toml", "🧰️framework/dep/build.rs", "🧰️framework/dep/lib.rs", "🧰️framework/macro/lib.rs"]);
    const resolved = await checkProctorProduct(root, "prototype", { platform: "darwin", architecture: "arm64" });
    expect(resolved.receipt).toEqual(receipt);
  });

  it("keeps modes apart and replaces older generations only when asked to prune", async () => {
    const { root, target, stdout } = layout();
    const host = { platform: "linux" as const, architecture: "x64" };
    const first = await captureProctorProduct(root, "prototype", stdout, { host });
    writeFileSync(join(target, "debug", "proctor"), `${fixtures.bytes}!`);
    const second = await captureProctorProduct(root, "prototype", stdout, { host });
    const folder = proctorProductDirectory(root, "prototype");
    expect(second.sha256).not.toBe(first.sha256);
    expect(readdirSync(folder).sort()).toEqual([PROCTOR_PRODUCT_RECEIPT, first.sha256, second.sha256].sort());
    await captureProctorProduct(root, "prototype", stdout, { host, prune: true });
    expect(readdirSync(folder).sort()).toEqual([PROCTOR_PRODUCT_RECEIPT, second.sha256].sort());
    expect(existsSync(proctorProductDirectory(root, "release"))).toBe(false);
    await expect(checkProctorProduct(root, "release", host)).rejects.toMatchObject({ code: "missing" });
  });

  it("is idempotent for identical bytes", async () => {
    const { root, stdout } = layout();
    const host = { platform: "linux" as const, architecture: "x64" };
    const first = await captureProctorProduct(root, "prototype", stdout, { host });
    const folder = proctorProductDirectory(root, "prototype");
    const before = readFileSync(join(folder, PROCTOR_PRODUCT_RECEIPT), "utf8");
    expect(await captureProctorProduct(root, "prototype", stdout, { host, prune: true })).toEqual(first);
    expect(readFileSync(join(folder, PROCTOR_PRODUCT_RECEIPT), "utf8")).toBe(before);
  });

  it("fails closed when Cargo read a source that the declared inputs do not name", async () => {
    const { root, stdout } = layout();
    const declaredGlobs = proctorSourceGlobs(["🎓️teaching/proctor", "🧰️framework/macro"]);
    const failure = await captureProctorProduct(root, "prototype", stdout, { host: { platform: "linux", architecture: "x64" }, inputs: declaredGlobs }).then(() => undefined, (error: unknown) => error);
    expect(failure).toBeInstanceOf(ProctorProductError);
    expect((failure as ProctorProductError).code).toBe("uncovered");
    const [english, german] = (failure as ProctorProductError).message.split("\n");
    expect(english).toContain("do not declare");
    expect(german).toContain("nicht deklarieren");
    expect(existsSync(join(proctorProductDirectory(root, "prototype"), PROCTOR_PRODUCT_RECEIPT))).toBe(false);
  });
});

describe("the manifest of @teaching/proctor", () => {
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { namedInputs: Record<string, unknown[]>; targets: Record<string, { dependsOn?: unknown[]; inputs?: unknown[]; outputs?: string[]; cache?: boolean }> };
  const declared = declaredProctorInputs(manifest);

  it.skipIf(declared === undefined)("declares exactly the Cargo closure of the proctor as the inputs of its build", async () => {
    const roots = (await proctorSourceDirectories(repository)).map(root => root.slice(repository.length + 1).split("\\").join("/"));
    expect(declared).toEqual(proctorSourceGlobs(roots, inputs.files));
    expect(roots.some(root => root.includes("⏳️async"))).toBe(true);
    expect(manifest.namedInputs.cargoSources).toContain("proctorRust");
    expect(manifest.namedInputs.cargoSources).not.toContain("^cargo");
  });

  it.skipIf(declared === undefined)("covers every source Cargo's own dep-info of the last build lists", () => {
    const folder = join(repository, "🎓️teaching", "🛂️proctor", "📦️packages", "🦀️rust", "dist", "build");
    const receipts = ["prototype", "release"].map(mode => join(folder, mode, PROCTOR_PRODUCT_RECEIPT)).filter(path => existsSync(path));
    for (const path of receipts) {
      const receipt = readProctorProductReceipt(JSON.parse(readFileSync(path, "utf8")));
      expect(receipt.sources.length).toBeGreaterThan(0);
      expect(uncoveredProctorSources(receipt.sources, declared!)).toEqual([]);
    }
  });

  it.skipIf(declared === undefined)("makes the build depend on nothing it does not declare, cached with the product directories as outputs", () => {
    const build = manifest.targets.build!;
    expect(build.cache).toBe(true);
    expect(build.inputs).toContain("cargoSources");
    expect(build.outputs).toEqual(["{projectRoot}/dist/build/prototype", "{projectRoot}/dist/build/release"]);
    for (const name of ["check", "dev", "rebuild", "health", "backup", "restore", "erase", "prune"]) expect(manifest.targets[name]!.dependsOn, name).toContain("build");
    expect(manifest.targets.check!.inputs).toContainEqual({ dependentTasksOutputFiles: "**/receipt.json", transitive: false });
  });
});
