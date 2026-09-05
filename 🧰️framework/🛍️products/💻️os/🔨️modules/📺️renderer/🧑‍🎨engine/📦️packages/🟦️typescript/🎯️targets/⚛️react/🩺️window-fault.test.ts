/** 🩺 Window-fault classification conformance against the SAME language-neutral vector fixture the
 * Rust plugin runtime decodes (`🔌️plugin/🩺️runtime-fault-vectors.json`), with strict Ajv as the
 * independent oracle for the fixture's own shape. */

import Ajv from "ajv";
import { describe, expect, it } from "vitest";
import { classifyWindowFault, WINDOW_FAULT_ATTRIBUTE, windowFaultFromError } from "../../../../🧱️elements/🏛️ShellHost/🩺️fault/🟦️.ts";
import faultVectorSchema from "../../../../../../🔌️plugin/🧯️runtime-fault-vectors.schema.json";
import faultVectors from "../../../../../../🔌️plugin/🩺️runtime-fault-vectors.json";
import shellSource from "../../../../🧱️elements/🏛️ShellHost/🟦️.tsx?raw";
import uiBundleSource from "../../../../../../../../../🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx?raw";

type FaultVector = { readonly variant: string; readonly code: string; readonly class: string; readonly detail: string };
type ClassificationCase = { readonly id: string; readonly code?: string; readonly supervisor?: string; readonly class: string };

const message = (scope: string, vector: FaultVector, instance: number, elapsed: string): string =>
  faultVectors.messageTemplate
    .replace("{scope}", scope)
    .replace("{instance}", String(instance))
    .replace("{detail}", vector.detail)
    .replace("{variant}", vector.variant)
    .replace("{elapsed}", elapsed)
    .replace("{ceilingUs}", String(faultVectors.ceilingUs));

describe("window fault discriminators", () => {
  it("accepts the shared fixture under a strict independent schema oracle and rejects adversarial shapes", () => {
    const ajv = new Ajv({ strict: true, allErrors: true });
    const validate = ajv.compile(faultVectorSchema);
    expect(validate(faultVectors), JSON.stringify(validate.errors)).toBe(true);
    const hostile = [
      { ...faultVectors, extra: true },
      { ...faultVectors, ceilingUs: 16000 },
      { ...faultVectors, vectors: faultVectors.vectors.slice(1) },
      { ...faultVectors, vectors: faultVectors.vectors.map((vector) => ({ ...vector, code: vector.code.replace("plugin.internal.", "plugin.") })) },
      { ...faultVectors, vectors: faultVectors.vectors.map((vector) => ({ ...vector, class: "install-failed" })) },
    ];
    for (const candidate of hostile) expect(validate(candidate), JSON.stringify(candidate).slice(0, 80)).toBe(false);
  });

  it("classifies every declared runtime fault code exactly as the fixture says, for both cleanup scopes", () => {
    expect(faultVectors.vectors.length).toBe(19);
    for (const vector of faultVectors.vectors as readonly FaultVector[]) {
      expect(classifyWindowFault(vector.code), vector.variant).toBe(vector.class);
      for (const scope of faultVectors.scopes) {
        const wire = { origin: "plugin", code: vector.code, severity: "error", message: message(scope, vector, 7, "12345us") };
        const fault = windowFaultFromError(wire);
        expect(fault.class, `${scope}:${vector.variant}`).toBe(vector.class);
        expect(fault.code).toBe(vector.code);
        expect(fault.origin).toBe("plugin");
        expect(fault.message).toContain(`instance 7`);
        expect(fault.message).toContain(`[${vector.variant}]`);
        expect(fault.message).toContain(`(elapsed 12345us, ceiling ${faultVectors.ceilingUs}us)`);
        expect(windowFaultFromError({ ...wire, message: message(scope, vector, 7, "unmeasured") }).class).toBe(vector.class);
      }
    }
  });

  it("resolves the three named discriminators the empty-window taxonomy asked for", () => {
    expect(classifyWindowFault("plugin.internal.abi-mismatch")).toBe("abi-mismatch");
    expect(classifyWindowFault("plugin.internal.interactive-ceiling")).toBe("interactive-ceiling");
    expect(classifyWindowFault("plugin.internal.clock")).toBe("clock");
    expect(classifyWindowFault("plugin.internal.clock-regression")).toBe("clock");
    expect(classifyWindowFault("plugin.internal.clock-cooperative")).toBe("clock");
  });

  it("applies the fixture's supervisor and non-plugin classification cases", () => {
    for (const row of faultVectors.classifications as readonly ClassificationCase[]) {
      expect(classifyWindowFault(row.code, row.supervisor), row.id).toBe(row.class);
    }
  });

  it("recovers the code from a nested fault, an Error, and an unstructured rejection", () => {
    expect(windowFaultFromError({ fault: { origin: "plugin", code: "plugin.internal.abi-mismatch", message: "trapped" }, message: "trapped" }).class).toBe("abi-mismatch");
    expect(windowFaultFromError(new Error("plain failure"))).toEqual({ class: "unknown", code: undefined, origin: undefined, message: "plain failure" });
    expect(windowFaultFromError("boom").message).toBe("boom");
    expect(windowFaultFromError({ code: "plugin.internal.zero-progress", message: "stalled" }, "crashed").class).toBe("install-failed");
  });

  it("keeps the DOM contract a catalog smoke reads wired into the live shell", () => {
    expect(WINDOW_FAULT_ATTRIBUTE).toBe("data-semio-window-fault");
    const shell = shellSource;
    expect(shell).toContain(`${WINDOW_FAULT_ATTRIBUTE}={fault.class}`);
    expect(shell).toContain('role="status"');
    expect(shell).toContain("windowFaultFromError(renderError, pluginSupervisorByIdRef.current[current.pluginId])");
    expect(shell).toContain("windowFaultFromError(renderError, pluginSupervisorByIdRef.current[activeSpawned.pluginId])");
    expect(shell).toContain("windowFaultFromError(commandError, pluginSupervisorByIdRef.current[session.pluginId])");
    for (const key of ["ui.windowFault.abiMismatch", "ui.windowFault.interactiveCeiling", "ui.windowFault.clock", "ui.windowFault.pluginInternal", "ui.windowFault.installFailed", "ui.windowFault.unknown"]) expect(shell).toContain(key);
  });

  it("carries a distinct English and German label for every class, with no default language", () => {
    const bundles = uiBundleSource;
    const windowFaultBlocks = [...bundles.matchAll(/windowFault: \{([\s\S]*?)\n        \},/g)].map((match) => match[1]!);
    expect(windowFaultBlocks.length).toBe(2);
    const [de, en] = windowFaultBlocks;
    for (const key of ["title", "abiMismatch", "interactiveCeiling", "clock", "pluginInternal", "installFailed", "unknown"]) {
      expect(de, `de:${key}`).toContain(`${key}: {`);
      expect(en, `en:${key}`).toContain(`${key}: {`);
    }
    expect(de).not.toEqual(en);
  });
});
