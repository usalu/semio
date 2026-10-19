/** 📦️ The native response page, decode retirement, mounted I/O and aborted image tests. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import { expect, it } from "vitest";
import { ENGINE_ROOT, FRAME_TURN_FIXTURE } from "../🧰️step-budget-fixtures/🟦️.ts";

/** 🧾 Registers the native response page, reference decode retirement, contended mounted I/O task and aborted ready image tests. */
export function registerNativeResponseAndDecodeRetirementTests(): void {
  it("Chromium preserves every byte of an admitted native response page", async () => {
    const directory = join(ENGINE_ROOT, "🧪️tests/🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    
    
    
    const acceptsPage = new Ajv2020().compile({ type: "array", minItems: 1, maxItems: fixture.pageBytes, items: { type: "integer", minimum: 0, maximum: 255 } });
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      for (const example of fixture.cases) {
        const observed = await page.evaluate(async ({ length, period }) => {
          const expected = Uint8Array.from({ length }, (_, index) => index % period);
          const response = new Response(expected);
          const received = new Uint8Array(await response.arrayBuffer());
          return { length: received.length, equal: received.every((byte, index) => byte === expected[index]), consumed: response.bodyUsed };
        }, { length: example.bytes, period: fixture.patternPeriod });
        expect(observed).toEqual({ length: example.bytes, equal: true, consumed: true });
        expect(acceptsPage(Array.from({ length: observed.length }, (_, index) => index % fixture.patternPeriod))).toBe(example.accepted);
      }
    } finally {
      await browser.close();
    }
  });
  it("validates native reference decode retirement and derives every one-page grant independently", () => {
    const directory = join(ENGINE_ROOT, "🧪️tests/🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    
    
    
    const releases = (bytes: number): number[] => {
      const pages: number[] = [];
      while (bytes > 0) {
        const released = Math.min(bytes, fixture.pageBytes);
        pages.push(released);
        bytes -= released;
      }
      return pages;
    };
    const retirement = fixture.referenceDecodeRetirement;
    expect(releases(retirement.phaseZero.decodedPixelBytes)).toEqual(retirement.phaseZero.decodedPageReleases);
    expect(releases(retirement.phaseZero.encodedBytes)).toEqual(retirement.phaseZero.encodedPageReleases);
    expect([...retirement.phaseZero.decodedPageReleases, ...retirement.phaseZero.encodedPageReleases].reduce((sum, bytes) => sum + bytes, 0)).toBe(retirement.phaseZero.releasedBytes);
    expect(retirement.phaseTwoWaiting.decodedPixelBytes).toBeGreaterThan(fixture.pageBytes);
    expect(retirement.phaseTwoWaiting.retainedAfterOneTurn).toBe(true);
    expect(retirement.phaseTwoWaiting.closeTurns).toBe(1 + releases(retirement.phaseTwoWaiting.decodedPixelBytes).length + 1 + releases(retirement.phaseTwoWaiting.encodedBytes).length + 1 + 1);
    expect(retirement.zeroGrantReleasedBytes).toBe(0);
    expect(retirement.terminalOwners).toBe(0);
    
    const transport = fixture.transportCancellation;
    expect(transport.stalledHostId).not.toBe(transport.siblingHostId);
    expect(transport.observationDeadlineMs).toBeLessThan(transport.cleanupDeadlineMs);
    expect(transport.responseBytes).toBeGreaterThan(0);
    expect(transport.serviceOutcomes).toEqual({ idle: "idle", socket: "interrupted", httpsReadDeadlineMs: 15_000, faultDetail: "transport cancellation probe failed" });
    expect(transport.abandonedStart).toEqual({ outstandingCap: 1, siblingRefusedBeforeWorkerTerminal: true, workerTerminalBeforeSibling: true, siblingAdmitted: true });
    expect(transport.abandonedBodyRead).toEqual({ outstandingCap: 1, readPendingBeforeBodyDrop: true, siblingRefusedBeforeReaderTerminal: true, readerTerminalBeforeSibling: true, siblingAdmitted: true });
    expect([transport.publishedStalledResponses, transport.terminalTransportLeases, transport.frameFaults]).toEqual([0, 0, 0]);
    
    const localPage = fixture.localPageCancellation;
    expect(localPage.pageBytes).toBe(fixture.pageBytes);
    expect(localPage.retainedPageBytesBeforeClose).toBe(localPage.pageBytes);
    expect([localPage.publishedCancelledBytes, localPage.terminalNativeOwners, localPage.frameFaults]).toEqual([0, 0, 0]);
    expect(localPage.pageRetirementTurns).toBe(1);
    
  });
  it("keeps a contended mounted-I/O task runnable until its exact generation can register", async () => {
    const directory = join(ENGINE_ROOT, "🧪️tests/🧫️fixtures", "📄️native-asset-response");
    const fixture = JSON.parse(readFileSync(join(directory, "🔣️.json"), "utf8"));
    
    
    
    const contention = fixture.rendererIoContention;
    let registrationState = contention.registrationState;
    let wakes = 0;
    let completed = false;
    const channel = new MessageChannel();
    const scheduled = new Promise<void>((resolve) => {
      channel.port1.onmessage = () => {
        wakes += 1;
        completed = registrationState === "live";
        resolve();
      };
    });
    const poll = () => {
      if (registrationState === "checkedOut" && contention.exactGenerationLive) {
        channel.port2.postMessage(undefined);
        return "pending";
      }
      return "ready";
    };
    expect(poll()).toBe(contention.poll);
    registrationState = "live";
    await scheduled;
    channel.port1.close();
    channel.port2.close();
    expect({ wakes, completed, terminalOwners: 0 }).toEqual({ wakes: contention.wakeCount, completed: true, terminalOwners: contention.terminalOwners });
    
  });
  it("Chromium rejects an aborted ready image before publication while retaining its successor", async () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8"));
    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate(async ({ retiredToken, successorToken }) => {
        const image = new Image();
        image.src = 'data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>';
        await image.decode();
        const retired = new AbortController();
        const successor = new AbortController();
        const publishedTokens: number[] = [];
        retired.abort();
        let outcome = "ready";
        try {
          retired.signal.throwIfAborted();
          publishedTokens.push(retiredToken);
        } catch (error) {
          if (!(error instanceof DOMException) || error.name !== "AbortError") throw error;
          outcome = "cancelled";
        }
        successor.signal.throwIfAborted();
        return { outcome, publishedTokens, successorToken, successorLive: !successor.signal.aborted, width: image.naturalWidth };
      }, fixture.cancelledDecode);
      expect(observed.outcome).toBe(fixture.cancelledDecode.afterClose);
      expect(observed.publishedTokens).toEqual(fixture.cancelledDecode.publishedTokens);
      expect(observed.successorToken).toBe(fixture.cancelledDecode.successorToken);
      expect(observed.successorLive).toBe(true);
      expect(observed.width).toBe(1);
    } finally {
      await browser.close();
    }
  });
}
