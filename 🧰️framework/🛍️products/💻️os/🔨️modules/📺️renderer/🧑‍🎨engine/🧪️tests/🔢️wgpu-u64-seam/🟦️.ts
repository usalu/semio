import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import laws from "../🧫️fixtures/🔢️wgpu-u64-seam/🔣️.json";

const engineRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const rustText = readFileSync(join(engineRoot, laws.rustSource), "utf8");
const sources: Record<string, string> = { workerSource: readFileSync(join(engineRoot, laws.workerSource), "utf8"), originalSource: readFileSync(join(engineRoot, laws.originalSource), "utf8") };

/** 🦀️ The Rust signature is the ABI authority — the TypeScript handle type is its twin, never its source. */
function rustParameters(rustFn: string): readonly { readonly name: string; readonly type: string }[] {
  const signature = new RegExp(`pub (?:async )?fn ${rustFn}\\s*\\(([^)]*)\\)`, "u").exec(rustText)?.[1];
  expect(signature, `no Rust export ${rustFn}`).toBeTypeOf("string");
  return signature!.split(",").map((part) => part.trim()).filter((part) => part.includes(":")).map((part) => {
    const [name, type] = part.split(":", 2);
    return { name: name!.trim(), type: type!.trim() };
  });
}

/** 🧾️ One declaration line of the hand-written wasm-bindgen carrier type that holds `jsName`. */
function handleDeclaration(exported: (typeof laws.exports)[number]): string {
  const holder = exported.declaration.holder.replace(/[.*+?^${}()|[\]\\]/gu, "\\$&");
  const block = new RegExp(`${holder}[^{]*\\{([^}]*)\\}`, "u").exec(sources[exported.declaration.source]!)?.[1];
  expect(block, `no ${exported.declaration.holder} declaration`).toBeTypeOf("string");
  const line = block!.split("\n").map((row) => row.trim()).find((row) => row.startsWith(`${exported.jsName}(`));
  expect(line, `no ${exported.jsName} member on ${exported.declaration.holder}`).toBeTypeOf("string");
  return line!;
}

describe("wgpu u64 seam", () => {
  it("declares every widened Rust parameter as the JavaScript carrier, not a number", () => {
    for (const exported of laws.exports) {
      const parameters = rustParameters(exported.rustFn);
      const widened = parameters.filter((parameter) => parameter.type === laws.widenedIntegerType).map((parameter) => parameter.name);
      expect(widened).toEqual(exported.widenedParameters);
      const declaration = handleDeclaration(exported);
      const declared = declaration.slice(declaration.indexOf("(") + 1, declaration.lastIndexOf(")")).split(",").map((part) => part.trim()).filter(Boolean);
      expect(declared).toHaveLength(parameters.length);
      for (const [index, parameter] of parameters.entries()) {
        if (parameter.type !== laws.widenedIntegerType) continue;
        const carrier = declared[index]!.split(":", 2)[1]!.trim();
        expect(carrier, `${exported.jsName} parameter ${parameter.name}`).toBe(laws.javascriptCarrier);
      }
    }
  });

  it("lowers every widened argument at the call site", () => {
    for (const exported of laws.exports) {
      const call = new RegExp(`${exported.call.receiver}\\.${exported.jsName}\\(([^;]*?)\\)[;)]`, "u").exec(sources[exported.call.source]!)?.[1];
      expect(call, `no ${exported.jsName} call site`).toBeTypeOf("string");
      expect((call!.match(new RegExp(`${laws.lowering}\\(`, "gu")) ?? []).length).toBe(exported.call.loweredArguments);
    }
  });

  it("a number in a widened slot is a runtime TypeError — the engine itself is the oracle", () => {
    expect(TypeError.name).toBe(laws.numberLoweringFaultType);
    expect(() => BigInt.asUintN(64, laws.firstBatch.generation as unknown as bigint)).toThrowError(TypeError);
    expect(BigInt.asUintN(64, BigInt(laws.firstBatch.generation))).toBe(BigInt(laws.firstBatch.generation));
    expect(BigInt.asUintN(64, BigInt(laws.firstBatch.sequence))).toBe(BigInt(laws.firstBatch.sequence));
  });

  it("faults the whole Worker on the FIRST batch when the lowering is missing", () => {
    const seam = (generation: number | bigint): void => { BigInt.asUintN(64, generation as bigint); };
    const drive = (generation: number | bigint): string | null => { try { seam(generation); return null; } catch (error) { return laws.faultCode; } };
    expect(drive(laws.firstBatch.generation)).toBe(laws.faultCode);
    expect(drive(BigInt(laws.firstBatch.generation))).toBeNull();
  });
});
