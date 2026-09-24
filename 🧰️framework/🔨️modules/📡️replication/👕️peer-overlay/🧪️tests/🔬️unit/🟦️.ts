/** @emoji 🧪️ Peer-overlay derivation against the language-agnostic `👕️peer-overlay-v1` fixture. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  canvasPeerViewportRect,
  canvasPointToScreen,
  orbitFrustumCorners,
  orbitFrustumSegments,
  orbitPointToScreen,
  peerOverlayPath,
  peersForWindow,
  type PresencePeerInput,
} from "../../🟦️.ts";

const fixturePath = join(dirname(fileURLToPath(import.meta.url)), "../../../🧫️fixtures/👕️peer-overlay-v1/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as { readonly cases: readonly any[] };

describe("👕️peer-overlay-v1", () => {
  for (const row of fixture.cases) {
    it(row.id, () => {
      const spec = peersForWindow(row.roster as PresencePeerInput[], row.windowId, row.space, row.mySurface, row.myActor, row.localColor);
      expect(spec.artifactPeers.map((peer) => peer.actor)).toEqual(row.expected.artifactPeerActors);
      if (row.expected.cursorScreen) {
        const peer = spec.artifactPeers[0]!;
        if (peer.view.kind === "canvas") {
          const screen = canvasPointToScreen(row.localView, row.localSize, [peer.pointer![0], peer.pointer![1]]);
          expect(screen[0]).toBeCloseTo(row.expected.cursorScreen[0], 5);
          expect(screen[1]).toBeCloseTo(row.expected.cursorScreen[1], 5);
          const rect = canvasPeerViewportRect(peer.view as { x: number; y: number; zoom: number }, peer.size, row.localView, row.localSize);
          expect([...rect].map((n) => Math.round(n * 1000) / 1000)).toEqual(row.expected.viewportRect);
          expect(peerOverlayPath(row.scenePath, "Cursor", 0, peer.actor)).toBe(row.expected.paths.cursor);
          expect(peerOverlayPath(row.scenePath, "Camera", 0, peer.actor)).toBe(row.expected.paths.viewport);
        } else if (peer.view.kind === "orbit") {
          expect(peer.rayOrigin).toEqual(row.expected.rayOrigin);
          const screen = orbitPointToScreen(row.localView, row.localSize, peer.pointer!);
          expect(screen).not.toBeNull();
          expect(screen![0]).toBeCloseTo(row.expected.cursorScreen[0], 5);
          expect(screen![1]).toBeCloseTo(row.expected.cursorScreen[1], 5);
          expect(peerOverlayPath(row.scenePath, "Cursor", 0, peer.actor)).toBe(row.expected.paths.cursor);
        }
      }
      if (row.expected.markKeys) {
        const keys = Object.entries(spec.marks).flatMap(([domain, byId]) => Object.keys(byId).map((id) => `${domain}:${id}`)).sort();
        expect(keys).toEqual([...row.expected.markKeys].sort());
      }
      if (row.expected.activeTools) {
        expect(spec.artifactPeers.map((peer) => peer.activeTool)).toEqual(row.expected.activeTools);
      }
      if (row.expected.frustumCornerCount) {
        const peer = spec.artifactPeers[0]!;
        expect(peer.view.kind).toBe("orbit");
        const view = peer.view as { position: [number, number, number]; target: [number, number, number]; up: [number, number, number]; fov: number };
        const corners = orbitFrustumCorners(view.position, view.target, view.up, view.fov, peer.size[0] / peer.size[1], 4);
        expect(corners).toHaveLength(row.expected.frustumCornerCount);
        expect(orbitFrustumSegments(corners)).toHaveLength(row.expected.frustumSegmentCount);
      }
      if (row.expected.paths?.markSelection) {
        expect(peerOverlayPath(row.scenePath, "Marks", 0, "layer:s1")).toBe(row.expected.paths.markSelection);
        expect(peerOverlayPath(row.scenePath, "Marks", 1, "layer:s2")).toBe(row.expected.paths.markHover);
      }
    });
  }
});
