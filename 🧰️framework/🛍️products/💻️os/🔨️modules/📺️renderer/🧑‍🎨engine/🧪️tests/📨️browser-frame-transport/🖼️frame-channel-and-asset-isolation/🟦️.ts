/** 🖼️ Frame channel and asset isolation: the accepted frame and diagnostic channels, the accessibility mirror refresh, the wake after an asset refusal, the asset isolation and the row-aligned strips. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { browserAssetFailureDisposition } from "@semio-tech/framework-renderer-wgpu/frame-worker/asset-failure";
import { FakeWorker, transport } from "../🧰️transport-test-support/🟦️.ts";
import { readFrameWorkerSource } from "../📖️frame-worker-source/🟦️.ts";

describe("browser frame worker transport", () => {
  it("forwards changed accepted World3d diagnostic receipts onto the page channel", () => {
    const worker = new FakeWorker();
    const diagnostics: string[] = [];
    transport(worker, { diagnostics });
    worker.reply({ kind: "diagnostic", lifecycle: 1, channel: "world3d-accepted-frame", generation: 7, frameSequence: 9, json: '{"surfaces":[]}' });
    expect(diagnostics).toEqual(['world3d-accepted-frame:7:9:{"surfaces":[]}']);
  });

  /** ♿️ LAW: the ARIA mirror is driven by the frame channel, and the pull never runs inside the hook. */
  it("refreshes the accessibility mirror from the frame channel, never from the overrun channel", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = (readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    const mirrorSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts"), "utf8");
    const directiveStart = bootSource.indexOf("onDirectives:");
    const directiveEnd = bootSource.indexOf("onDiagnostic:", directiveStart);
    expect(directiveStart).toBeGreaterThanOrEqual(0);
    expect(directiveEnd).toBeGreaterThan(directiveStart);
    const directiveHook = bootSource.slice(directiveStart, directiveEnd);
    const turnHook = bootSource.slice(bootSource.indexOf("onUiTurn: (outcome)"), bootSource.indexOf("onReady: () =>"));
    expect(directiveHook).toContain("accessibility?.refresh()");
    expect(turnHook).not.toContain("accessibility");
    const refresh = mirrorSource.slice(mirrorSource.indexOf("const refresh = (): void =>"), mirrorSource.indexOf("return { refresh,"));
    expect(refresh).toContain("window.setTimeout");
    expect(refresh).not.toMatch(/\n\s*void pull\(\);\n\s*return;/);
    expect(mirrorSource).toContain("paint(dump.windows ?? [])");
    expect(mirrorSource).toContain('element.dataset.window = surface.windowId');
    expect(mirrorSource).toContain("parent.appendChild(projected.description)");
    expect(mirrorSource).not.toContain("element.appendChild(description)");
  });

  it("wakes the frame owner after a handled asset refusal so the next asset is polled", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFrameWorkerSource(root);
    const pump = workerSource.slice(workerSource.indexOf("async function pumpAsset"), workerSource.indexOf("function requestHostIo"));
    const caught = pump.slice(pump.lastIndexOf("  } catch (error) {"), pump.lastIndexOf("  } finally {"));
    const reject = caught.indexOf('ownedStep("asset-reject"');
    const wake = caught.indexOf('post({ kind: "wake", lifecycle })');
    expect(reject).toBeGreaterThan(-1);
    expect(wake).toBeGreaterThan(reject);
    expect(caught.match(/post\(\{ kind: "wake", lifecycle \}\)/g)).toHaveLength(1);
    expect(caught).toContain('activeRequest && disposition === "missing" && !closing && !closed && !failed');
  });

  it("isolates unavailable browser assets while preserving capacity faults and cancellation", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const fixture = JSON.parse(readFileSync(join(root, "../../🧫️fixtures/📄️native-asset-response/🔣️.json"), "utf8")) as {
      readonly failureIsolation: {
        readonly browser: readonly {
          readonly detail: string;
          readonly errorType: "error" | "typeError" | "abortError";
          readonly referenceImage: boolean;
          readonly disposition: "cancelled" | "missing" | "fault";
        }[];
      };
    };
    
    
    for (const row of fixture.failureIsolation.browser) {
      const error = row.errorType === "typeError" ? new TypeError(row.detail) : row.errorType === "abortError" ? new DOMException(row.detail, "AbortError") : new Error(row.detail);
      expect(browserAssetFailureDisposition(error, row.referenceImage)).toBe(row.disposition);
    }
    const workerSource = readFrameWorkerSource(root);
    const pump = workerSource.slice(workerSource.indexOf("async function pumpAsset"), workerSource.indexOf("function requestHostIo"));
    const caught = pump.slice(pump.lastIndexOf("  } catch (error) {"), pump.lastIndexOf("  } finally {"));
    expect(caught).toContain('activeRequest && disposition === "missing"');
    expect(caught).toContain('ownedStep("asset-reject"');
    expect(caught).toContain('else ownedStep("asset-abort"');
    expect(caught).toContain('disposition === "fault"');
    expect(caught).not.toContain("!activeRequest?.referenceImage");
  });

  it("streams decoded references as cancellable row-aligned strips before the exact seal", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const workerSource = readFrameWorkerSource(root);
    const decoderSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🖼️reference-image-decode/🟦️.ts"), "utf8");
    const rustSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"), "utf8");
    const pump = workerSource.slice(workerSource.indexOf("async function pumpAsset"), workerSource.indexOf("function requestHostIo"));
    const begin = pump.indexOf('ownedStep("reference-image-begin"');
    const stage = pump.indexOf("referenceImageBitmapForStage(");
    const push = pump.indexOf("streamReferenceImageBitmapRows(");
    const seal = pump.indexOf('ownedStep("reference-image-seal"');
    expect(begin).toBeGreaterThan(-1);
    expect(stage).toBeGreaterThan(begin);
    expect(push).toBeGreaterThan(stage);
    expect(seal).toBeGreaterThan(push);
    expect(pump).toContain("stageMode !== 1 && stageMode !== 2");
    expect(workerSource).not.toContain("ReferenceImageDecodeCache");
    expect(pump.indexOf('ownedStep("reference-image-begin"')).toBeLessThan(pump.indexOf('monitoredSuspension("reference-image-decode"'));
    expect(pump).toContain('monitoredSuspension("reference-image-page-decode"');
    expect(pump.match(/streamReferenceImageBitmapRows\(/g)).toHaveLength(1);
    expect(pump).toContain("runtime!.pushReferenceImageRows(offset, pixels)");
    expect(pump).toContain('(operation) => ownedStep("reference-image-row-strip", operation)');
    expect(pump).toContain('post({ kind: "wake", lifecycle })');
    expect(pump).toContain("await macrotask()");
    expect(decoderSource).toContain("new OffscreenCanvas(width, Math.min(height, rowsPerStrip))");
    expect(decoderSource).toContain('context.getImageData(0, 0, width, rows, { colorSpace: "srgb" })');
    expect(decoderSource).toContain("pixels.byteLength > REFERENCE_IMAGE_READBACK_BYTE_CAPACITY");
    expect(decoderSource).toContain("if (stageMode === 2) return undefined");
    expect(decoderSource).not.toContain("getImageData(0, 0, width, height");
    expect(decoderSource).not.toContain("Uint8Array.from(context.getImageData");
    for (const binding of ["beginReferenceImage", "pushReferenceImageRows", "sealReferenceImage"]) expect(rustSource).toContain(`js_name = ${binding}`);
    expect(rustSource).toContain("Result<u8, JsValue>");
    expect(workerSource).not.toContain("stageReferenceImage");
  });
});
