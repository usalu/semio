// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔄️ShellSync/tests/component.tsx
/** @emoji 🧪️ `🔄️ShellSync` laws for ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1
 * (audit `📓️g5-ux-completeness-audit.md` ranked items 4 and 8): the sync status line is composed from
 * localized words rather than English literals, and the always-visible hub badge folds every attached
 * document's `RemoteState` into one honest aggregate that is readable without colour.
 *
 * Registered in `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`'s `engineTestSuites` — a co-located suite no
 * runner includes is a gate that reads green while measuring nothing.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { type ArtifactSyncStatus } from "@semio-tech/framework-os";
import { HubConnectionIndicator, hubConnectionSummaryV1, syncStatusLabelV1, type SyncStatusTextsV1 } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️Fixtures
/** 🇩🇪️ The real German bundle values (`ui.sync.*` in `🖱️ui/🎯️targets/⚛️react/🟦️.tsx`), already
 * interpolated — proving the composer carries translated words rather than assembling English. */
const DE: SyncStatusTextsV1 = {
  live: "verbunden",
  connecting: "verbinde…",
  reconnecting: "verbinde erneut…",
  offline: "offline",
  peers: "3 Mitwirkende",
  saved: "gespeichert",
  unsaved: "ungespeichert",
  pending: "2 ausstehend",
};

function status(remote: ArtifactSyncStatus["remote"], persisted = true, pendingMutations = 0): ArtifactSyncStatus {
  return { persisted, pendingMutations, remote };
}
//#endregion 🔖️Fixtures

//#region 🔖️SyncStatusLabel
describe("syncStatusLabelV1", () => {
  it("returns null without a status rather than an empty badge", () => {
    expect(syncStatusLabelV1(null, DE)).toBeNull();
  });

  it("composes the live line from the supplied locale's words, never an English literal", () => {
    expect(syncStatusLabelV1(status({ kind: "live", peerCount: 3 }), DE)).toBe("verbunden · 3 Mitwirkende · gespeichert");
  });

  it("maps every RemoteState variant onto its own localized word", () => {
    expect(syncStatusLabelV1(status({ kind: "connecting" }), DE)).toBe("verbinde… · gespeichert");
    expect(syncStatusLabelV1(status({ kind: "backoff", retryInMs: 500 }), DE)).toBe("verbinde erneut… · gespeichert");
    expect(syncStatusLabelV1(status({ kind: "detached" }), DE)).toBe("offline · gespeichert");
  });

  it("appends the pending segment only when mutations are actually outstanding", () => {
    expect(syncStatusLabelV1(status({ kind: "detached" }, false, 2), DE)).toBe("offline · ungespeichert · 2 ausstehend");
    expect(syncStatusLabelV1(status({ kind: "detached" }, false, 0), DE)).toBe("offline · ungespeichert");
  });
});
//#endregion 🔖️SyncStatusLabel

//#region 🔖️HubConnectionSummary
describe("hubConnectionSummaryV1", () => {
  it("is offline with nothing attached — not a false 'live'", () => {
    expect(hubConnectionSummaryV1([], "none")).toEqual({ state: "offline", peerCount: 0, documentCount: 0 });
  });

  it("reports signedOut ahead of any transport state, because no transport state means anything without a session", () => {
    expect(hubConnectionSummaryV1([status({ kind: "live", peerCount: 4 })], "signedOut")).toEqual({ state: "signedOut", peerCount: 0, documentCount: 1 });
  });

  it("lets one live document carry the aggregate, and reports the busiest document's peers (never a double-counting sum)", () => {
    const summary = hubConnectionSummaryV1([status({ kind: "detached" }), status({ kind: "live", peerCount: 2 }), status({ kind: "live", peerCount: 5 })], "signedIn");
    expect(summary).toEqual({ state: "live", peerCount: 5, documentCount: 3 });
  });

  it("prefers a document still dialling over one already in backoff", () => {
    expect(hubConnectionSummaryV1([status({ kind: "backoff", retryInMs: 800 }), status({ kind: "connecting" })], "none").state).toBe("connecting");
    expect(hubConnectionSummaryV1([status({ kind: "backoff", retryInMs: 800 }), status({ kind: "detached" })], "none").state).toBe("reconnecting");
  });
});
//#endregion 🔖️HubConnectionSummary

//#region 🔖️HubConnectionIndicator
describe("HubConnectionIndicator", () => {
  afterEach(cleanup);

  it("announces itself as a status region with a text label, never colour alone", () => {
    render(<HubConnectionIndicator statuses={[status({ kind: "backoff", retryInMs: 500 })]} session="none" />);
    const badge = screen.getByRole("status");
    expect(badge.getAttribute("data-semio-hub-connection")).toBe("reconnecting");
    expect(badge.getAttribute("aria-live")).toBe("polite");
    expect(badge.getAttribute("aria-label")).toContain("reconnecting");
    expect(badge.textContent).toContain("reconnecting");
  });

  it("carries the attached-document count and the live peer count", () => {
    render(<HubConnectionIndicator statuses={[status({ kind: "live", peerCount: 1 })]} session="signedIn" />);
    const badge = screen.getByRole("status");
    expect(badge.getAttribute("data-hub-connection-documents")).toBe("1");
    expect(badge.textContent).toContain("1 peer");
  });

  it("offers the sign-in entry point only when signed out AND an opener exists", () => {
    render(<HubConnectionIndicator statuses={[]} session="signedOut" onSignIn={() => {}} />);
    expect(document.querySelector("[data-semio-hub-sign-in]")).not.toBeNull();
    cleanup();
    render(<HubConnectionIndicator statuses={[]} session="signedOut" />);
    expect(document.querySelector("[data-semio-hub-sign-in]")).toBeNull();
    cleanup();
    render(<HubConnectionIndicator statuses={[]} session="none" onSignIn={() => {}} />);
    expect(document.querySelector("[data-semio-hub-sign-in]")).toBeNull();
  });
});
//#endregion 🔖️HubConnectionIndicator
