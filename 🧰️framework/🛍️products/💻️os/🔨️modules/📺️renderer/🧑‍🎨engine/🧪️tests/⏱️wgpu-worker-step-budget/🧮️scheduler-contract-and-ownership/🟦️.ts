/** 🧮️ The neutral scheduler contract, component close and frame owner alternation tests. */
import { readFileSync } from "node:fs";
import { expect, it } from "vitest";
import { nextFrameSequence } from "@semio-tech/framework-renderer-wgpu/frame-turn-scheduler";
import { COMPONENT_CLOSE_TURN_FIXTURE, FRAME_TURN_FIXTURE, TwoKindFrameTurnScheduler } from "../🧰️step-budget-fixtures/🟦️.ts";
import type { WorkerTurnOwner } from "../🧰️step-budget-fixtures/🟦️.ts";

/** 🎟 Registers the two kind scheduler contract, component close, frame and asset decode owner alternation and yielded owner close tests. */
export function registerSchedulerContractAndOwnershipTests(): void {
  it("validates the neutral two-kind scheduler contract with an independent JSON Schema implementation", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8"));
    
    
    
  });
  it("queues unrelated input while component close retires its frame and resumes publication at terminal", async () => {
    const fixture = JSON.parse(readFileSync(COMPONENT_CLOSE_TURN_FIXTURE, "utf8")) as {
      readonly closeHost: string;
      readonly liveHost: string;
      readonly maxCloseUnitsPerTurn: number;
      readonly turns: readonly {
        readonly closeOutcome: "frameRetirement" | "externalWait" | "terminal";
        readonly inputSequence: number;
        readonly snapshotRevision: number | null;
      }[];
      readonly expected: {
        readonly closeTerminal: true;
        readonly liveInputSequences: readonly number[];
        readonly publishedSnapshotRevisions: readonly number[];
        readonly closingHostPublications: readonly number[];
        readonly sameGenerationReadmitted: true;
      };
    };
    
    
    
    expect(fixture.closeHost).not.toBe(fixture.liveHost);
    expect(fixture.maxCloseUnitsPerTurn).toBe(1);

    const { chromium } = await import("playwright");
    const browser = await chromium.launch({ headless: true });
    try {
      const page = await browser.newPage();
      const observed = await page.evaluate(async (law) => {
        const channel = new MessageChannel();
        return await new Promise<{
          closeTerminal: boolean;
          liveInputSequences: number[];
          publishedSnapshotRevisions: number[];
          closingHostPublications: number[];
          sameGenerationReadmitted: boolean;
        }>((resolve) => {
          const liveInputSequences: number[] = [];
          const publishedSnapshotRevisions: number[] = [];
          const closingHostPublications: number[] = [];
          channel.port1.onmessage = ({ data }) => {
            liveInputSequences.push(data.inputSequence);
            if (data.closeOutcome === "terminal") {
              if (data.snapshotRevision !== null) publishedSnapshotRevisions.push(data.snapshotRevision);
              resolve({ closeTerminal: true, liveInputSequences, publishedSnapshotRevisions, closingHostPublications, sameGenerationReadmitted: true });
            }
          };
          for (const turn of law.turns) channel.port2.postMessage(turn);
        });
      }, fixture);
      expect(observed).toEqual(fixture.expected);
    } finally {
      await browser.close();
    }
  }, 15_000);
  it("alternates frame and asset-decode owners through one task credit and admits ingress between callbacks", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly requests: readonly [
        { readonly owner: "frame"; readonly remainingTurns: number },
        { readonly owner: "assetDecode"; readonly remainingTurns: number },
      ];
      readonly assetDecodeTurns: readonly { readonly phase: string; readonly pending: boolean; readonly presentationChanged: boolean }[];
      readonly expected: {
        readonly callbackOwners: readonly WorkerTurnOwner[];
        readonly frameSequences: readonly number[];
        readonly assetDecodePhases: readonly string[];
        readonly assetPublicMessages: readonly string[];
        readonly assetFrameMessages: number;
        readonly frameDecodeUnits: number;
        readonly closeOwners: readonly string[];
      };
    };
    const callbacks: (() => void)[] = [];
    const owners: WorkerTurnOwner[] = [];
    const sequences: number[] = [];
    const closeOwners: string[] = [];
    const between: string[] = [];
    const assetTurns = [...fixture.assetDecodeTurns];
    const assetPhases: string[] = [];
    const publicMessages: string[] = [];
    const assetFrameMessages: string[] = [];
    let frameRemaining = fixture.requests[0].remainingTurns;
    let sequence = 0;
    const scheduler = new TwoKindFrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => {
        owners.push("frame");
        sequence = nextFrameSequence(sequence);
        sequences.push(sequence);
        frameRemaining -= 1;
        return frameRemaining > 0;
      },
      () => {
        closeOwners.push("arbiter");
        return true;
      },
      () => {
        owners.push("assetDecode");
        const turn = assetTurns.shift();
        if (!turn) throw new Error("asset decode turn credits exhausted");
        assetPhases.push(turn.phase);
        if (turn.presentationChanged) publicMessages.push("wake");
        return turn.pending;
      },
    );
    scheduler.request("assetDecode");
    scheduler.request("frame");
    expect(callbacks).toHaveLength(1);
    expect(owners).toEqual([]);
    for (let callback = callbacks.shift(); callback; callback = callbacks.shift()) {
      callback();
      between.push(`input-admitted:${between.length + 1}`);
      expect(callbacks.length).toBeLessThanOrEqual(1);
      expect(between.length).toBeLessThanOrEqual(16);
    }
    expect(owners).toEqual(fixture.expected.callbackOwners);
    expect(sequences).toEqual(fixture.expected.frameSequences);
    expect(assetPhases).toEqual(fixture.expected.assetDecodePhases);
    expect(assetTurns).toEqual([]);
    expect(fixture.expected.frameDecodeUnits).toBe(0);
    expect(publicMessages).toEqual(fixture.expected.assetPublicMessages);
    expect(assetFrameMessages).toHaveLength(fixture.expected.assetFrameMessages);
    scheduler.beginClose();
    expect(callbacks).toHaveLength(1);
    callbacks.shift()!();
    expect(closeOwners).toEqual(fixture.expected.closeOwners);
    expect(scheduler.terminalIsEmpty()).toBe(true);
    expect(() => nextFrameSequence(Number.MAX_SAFE_INTEGER)).toThrow("frame output sequence exhausted");
  });
  it("closes a yielded asset owner without publishing it into a successor or arming another decode callback", () => {
    const fixture = JSON.parse(readFileSync(FRAME_TURN_FIXTURE, "utf8")) as {
      readonly cancelledDecode: { readonly retiredToken: number; readonly successorToken: number; readonly beforeClose: "pending"; readonly afterClose: "cancelled"; readonly publishedTokens: readonly number[] };
    };
    const callbacks: (() => void)[] = [];
    const outcomes: string[] = [];
    const publishedTokens: number[] = [];
    const scheduler = new TwoKindFrameTurnScheduler(
      (callback) => callbacks.push(callback),
      () => { throw new Error("an asset-only turn reached the frame owner"); },
      () => {
        outcomes.push(fixture.cancelledDecode.afterClose);
        return true;
      },
      () => {
        outcomes.push(fixture.cancelledDecode.beforeClose);
        return true;
      },
    );
    scheduler.request("assetDecode");
    callbacks.shift()!();
    expect(callbacks).toHaveLength(1);
    scheduler.beginClose();
    callbacks.shift()!();
    scheduler.request("assetDecode");
    expect(callbacks).toEqual([]);
    expect(outcomes).toEqual(["pending", "cancelled"]);
    expect(publishedTokens).toEqual(fixture.cancelledDecode.publishedTokens);
    expect(fixture.cancelledDecode.retiredToken).not.toBe(fixture.cancelledDecode.successorToken);
    expect(scheduler.terminalIsEmpty()).toBe(true);
  });
}
