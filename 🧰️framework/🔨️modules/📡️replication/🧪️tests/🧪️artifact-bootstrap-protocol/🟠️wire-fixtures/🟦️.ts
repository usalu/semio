/** 🟠️ The wire fixtures phase of registerTests1. */
import type { buildArtifactBootstrapAssemblerState } from "../🔶️artifact-bootstrap-assembler/🟦️.ts";
import type { TestSource } from "../🟢️shared/🟦️.ts";

/** 🟡 Registers the wire fixtures tests of registerTests1. */
export function registerWireFixturesTests({ describe, it, source, decodeClientFrame, expect, encodeClientFrame, decodeServerFrame, encodeServerFrame, decodePresencePeer, encodePresencePeer }: Pick<ReturnType<typeof buildArtifactBootstrapAssemblerState>, "describe" | "it" | "decodeClientFrame" | "expect" | "encodeClientFrame" | "decodeServerFrame" | "encodeServerFrame" | "decodePresencePeer" | "encodePresencePeer"> & { readonly source: TestSource }) {
  describe("wire fixtures", () => {
      it("decodes the Rust-generated binary wire fixtures byte-identically", async () => {
        const { readFileSync } = await import("node:fs");
        const { fileURLToPath } = await import("node:url");
        const { dirname, join } = await import("node:path");
        const fixturesDir = join(dirname(fileURLToPath(source.url)), "🧪️tests/🧫️fixtures/📡️wire");
  
        function loadClient(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeClientFrame(bytes);
          expect(encodeClientFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function loadServer(name: string) {
          const bytes = new Uint8Array(readFileSync(join(fixturesDir, name)));
          const decoded = decodeServerFrame(bytes);
          expect(encodeServerFrame(decoded.frame, decoded.lane)).toEqual(bytes);
          return decoded;
        }
        function assertOpBinaryPayload(payload: readonly number[]) {
          expect(payload.length).toBeGreaterThan(0);
          expect(payload[0]).toBe(1);
        }
  
        const legacyHello = new Uint8Array(readFileSync(join(fixturesDir, "🚫️legacy-client-hello-rejected", "💾️.bin")));
        expect(() => decodeClientFrame(legacyHello)).toThrow(/unknown tag 0/u);
  
        const commands = loadClient("🕹️client-commands/💾️.bin");
        if (typeof commands.frame === "string" || !("Commands" in commands.frame)) throw new Error("expected a Commands frame");
        expect(commands.frame.Commands.envelopes).toHaveLength(1);
        assertOpBinaryPayload(commands.frame.Commands.envelopes[0]?.diff.payload ?? []);
  
        const frontierAdvertise = loadClient("🚩️client-frontier-advertise/💾️.bin");
        if (typeof frontierAdvertise.frame === "string" || !("FrontierAdvertise" in frontierAdvertise.frame)) throw new Error("expected a FrontierAdvertise frame");
  
        const previewPublish = loadClient("📣️client-preview-publish/💾️.bin");
        if (typeof previewPublish.frame === "string" || !("PreviewPublish" in previewPublish.frame)) throw new Error("expected a PreviewPublish frame");
        expect(previewPublish.frame.PreviewPublish.key).toBe("cursor");
  
        const presence = loadClient("🙋️client-presence/💾️.bin");
        if (typeof presence.frame === "string" || !("Presence" in presence.frame)) throw new Error("expected a Presence frame");
        const peer = decodePresencePeer(new Uint8Array(presence.frame.Presence.peer), [0]);
        expect(peer.actor).toBe("actor-1");
        expect(peer.label).toBe("Ada");
        expect(peer.userId).toBe("user-9");
        expect(peer.role).toBe("owner");
        expect(peer.connectedAtMs).toBe(1_700_000_000_000);
        expect(peer.color).toBe(5);
        expect(peer.surface).toBe("s.space.home@1/*#editor");
        expect(peer.views).toHaveLength(2);
        expect(peer.views[0]).toEqual({ windowId: "w1", space: "world", kind: { kind: "orbit", position: [1, 2, 3], target: [0, 0, 0], up: [0, 1, 0], fov: 45 }, size: [1024, 768], pointer: [0.5, 0.5, 0.5], rayOrigin: [1, 2, 3] });
        expect(peer.views[1]).toEqual({ windowId: "w2", space: "canvas", kind: { kind: "canvas", x: 12.5, y: -4, zoom: 1 }, size: [800, 600], pointer: undefined, rayOrigin: undefined });
        expect(peer.ui).toEqual({ hoveredPath: "row[2]#t1", focusedPath: undefined, pressedPath: undefined });
        expect(peer.interaction?.app_id).toBe("space");
        expect(peer.interaction?.domains).toEqual([
          { domain: "outline", granularity: "task", selected: ["t1", "t2"], hovered: [] },
          { domain: "board", granularity: "card", selected: [], hovered: ["c1"] },
          { domain: "canvas", granularity: "node", selected: ["n9"], hovered: ["n9", "n10"] },
        ]);
        expect(encodePresencePeer(peer)).toEqual(Array.from(new Uint8Array(presence.frame.Presence.peer)));
  
        const creditGrant = loadClient("🎟️client-credit-grant/💾️.bin");
        if (typeof creditGrant.frame === "string" || !("CreditGrant" in creditGrant.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrant.frame.CreditGrant.n).toBe(16);
  
        const bye = loadClient("👋️client-bye/💾️.bin");
        expect(bye.frame).toBe("Bye");
  
        const welcomeTail = loadServer("🔗️server-welcome-tail/💾️.bin");
        if (typeof welcomeTail.frame === "string" || !("Welcome" in welcomeTail.frame)) throw new Error("expected a Welcome frame");
        expect(welcomeTail.frame.Welcome.resume_token).toBe("resume-1");
        expect(welcomeTail.frame.Welcome.bootstrap).toBe("Tail");
  
        const welcomeSnapshot = loadServer("📸️server-welcome-snapshot-inline/💾️.bin");
        if (typeof welcomeSnapshot.frame === "string" || !("Welcome" in welcomeSnapshot.frame)) throw new Error("expected a Welcome frame");
        if (welcomeSnapshot.frame.Welcome.bootstrap === "None" || welcomeSnapshot.frame.Welcome.bootstrap === "Tail" || !("Snapshot" in welcomeSnapshot.frame.Welcome.bootstrap)) throw new Error("expected a Snapshot bootstrap");
        expect(welcomeSnapshot.frame.Welcome.bootstrap.Snapshot.inline).toEqual([9, 9, 9]);
  
        const snapshotChunk = loadServer("🧩️server-snapshot-chunk/💾️.bin");
        if (typeof snapshotChunk.frame === "string" || !("SnapshotChunk" in snapshotChunk.frame)) throw new Error("expected a SnapshotChunk frame");
        expect(snapshotChunk.frame.SnapshotChunk.bytes).toEqual([1, 2, 3, 4]);
  
        const snapshotDone = loadServer("🏁️server-snapshot-done/💾️.bin");
        if (typeof snapshotDone.frame === "string" || !("SnapshotDone" in snapshotDone.frame)) throw new Error("expected a SnapshotDone frame");
        expect(snapshotDone.frame.SnapshotDone.seq_count).toBe(4);
  
        const serverCommands = loadServer("🎮️server-commands/💾️.bin");
        if (typeof serverCommands.frame === "string" || !("Commands" in serverCommands.frame)) throw new Error("expected a Commands frame");
        expect(serverCommands.frame.Commands.envelopes).toHaveLength(1);
  
        const ackAccepted = loadServer("✅️server-ack-accepted/💾️.bin");
        if (typeof ackAccepted.frame === "string" || !("Ack" in ackAccepted.frame)) throw new Error("expected an Ack frame");
        expect(ackAccepted.frame.Ack.batch_id).toBe(1);
        expect(ackAccepted.frame.Ack.stages).toHaveLength(3);
  
        const ackTransformed = loadServer("🔀️server-ack-transformed/💾️.bin");
        if (typeof ackTransformed.frame === "string" || !("Ack" in ackTransformed.frame)) throw new Error("expected an Ack frame");
        expect(ackTransformed.frame.Ack.batch_id).toBe(2);
  
        const ackRejected = loadServer("⛔️server-ack-rejected/💾️.bin");
        if (typeof ackRejected.frame === "string" || !("Ack" in ackRejected.frame)) throw new Error("expected an Ack frame");
        expect(ackRejected.frame.Ack.batch_id).toBe(3);
        const rejectedStage = ackRejected.frame.Ack.stages.find((stage) => typeof stage !== "string" && "Applied" in stage);
        if (typeof rejectedStage === "string" || rejectedStage === undefined || !("Applied" in rejectedStage) || typeof rejectedStage.Applied.outcome === "string" || !("Rejected" in rejectedStage.Applied.outcome)) throw new Error("expected a rejected apply outcome");
        expect(rejectedStage.Applied.outcome.Rejected.messages).toEqual([1, 2, 3]);
  
        const preview = loadServer("👁️server-preview/💾️.bin");
        if (typeof preview.frame === "string" || !("Preview" in preview.frame)) throw new Error("expected a Preview frame");
        expect(preview.frame.Preview.key).toBe("cursor");
  
        const serverPresence = loadServer("👥️server-presence/💾️.bin");
        if (typeof serverPresence.frame === "string" || !("Presence" in serverPresence.frame)) throw new Error("expected a Presence frame");
        expect(serverPresence.frame.Presence.peers).toHaveLength(2);
        expect(JSON.parse(new TextDecoder().decode(new Uint8Array(serverPresence.frame.Presence.peers[0]!)))).toEqual({ id: "a" });
        expect(decodePresencePeer(new Uint8Array(serverPresence.frame.Presence.peers[1]!), [0])).toEqual(peer);
  
        const creditGrantServer = loadServer("🎫️server-credit-grant/💾️.bin");
        if (typeof creditGrantServer.frame === "string" || !("CreditGrant" in creditGrantServer.frame)) throw new Error("expected a CreditGrant frame");
        expect(creditGrantServer.frame.CreditGrant.n).toBe(32);
  
        const error = loadServer("🚨️server-error/💾️.bin");
        if (typeof error.frame === "string" || !("Error" in error.frame)) throw new Error("expected an Error frame");
        expect(error.frame.Error.code).toBe("rejected");
  
        const session = loadServer("🪪️server-session/💾️.bin");
        if (typeof session.frame === "string" || !("Session" in session.frame)) throw new Error("expected a Session frame");
        expect(session.frame.Session.actor).toBe("actor-1");
        expect(session.frame.Session.color).toBe(5);
      });
  });
}
