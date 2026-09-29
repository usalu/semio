// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔄️ShellSync/tests/component.tsx
/** 🧪️ `🔄️ShellSync` laws for ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1
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
import { posix } from "node:path";
import { HubConnectionIndicator, SyncAttachCard, hubConnectionSummaryV1, syncAttachTargetV1, syncStatusLabelV1, type HubLinkV1, type HubSessionPresenceV1, type SyncStatusTextsV1 } from "../../🟦️.tsx";
import hubSummary from "../../🧫️fixtures/📶️hub-connection-summary.json";
import attachTargets from "../../🧫️fixtures/🎯️sync-attach-target.json";
import hubProjectionSchema from "../../../../🧬️schema/🔗️hub-projection/🔣️.json";
import hubProjection from "../../../../🧫️fixtures/🔗️hub-projection/🔣️.json";
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
/** 🧪️ Replayed from the language-agnostic fixture `🧫️fixtures/📶️hub-connection-summary.json` (slice U5 added the
 * session link: a verified session that stops answering reads `reconnecting` without a single document). */
describe("hubConnectionSummaryV1", () => {
  it("folds every session, link and document mix into exactly the shared hub projection's states", () => {
    const declared = (hubProjectionSchema as unknown as { readonly definitions: { readonly summary: { readonly properties: { readonly state: { readonly enum: readonly string[] } } } } }).definitions.summary.properties.state.enum;
    const remotes: readonly (readonly ArtifactSyncStatus["remote"][])[] = [[], [{ kind: "live", peerCount: 2 }], [{ kind: "connecting" }], [{ kind: "backoff", retryInMs: 500 }], [{ kind: "detached" }], [{ kind: "connecting" }, { kind: "backoff", retryInMs: 500 }], [{ kind: "detached" }, { kind: "live", peerCount: 1 }]];
    const reached = new Set<string>();
    for (const session of ["none", "signedOut", "signedIn"] as const)
      for (const link of ["verifying", "reachable", "unreachable"] as const)
        for (const mix of remotes) reached.add(hubConnectionSummaryV1(mix.map((remote) => status(remote)), session, link).state);
    expect([...reached].sort()).toEqual([...declared].sort());
  });

  for (const row of hubSummary.cases) {
    it(row.name, () => {
      expect(hubConnectionSummaryV1(row.remotes.map((remote) => status(remote as ArtifactSyncStatus["remote"])), row.session as HubSessionPresenceV1, row.link as HubLinkV1)).toEqual(row.expected);
    });
  }
});
//#endregion 🔖️HubConnectionSummary

//#region 🔖️HubConnectionIndicator
describe("HubConnectionIndicator", () => {
  afterEach(cleanup);

  it("announces itself as a status region with a text label, never colour alone", () => {
    render(<HubConnectionIndicator statuses={[status({ kind: "backoff", retryInMs: 500 })]} session="signedIn" link="reachable" />);
    const badge = screen.getByRole("status");
    expect(badge.getAttribute("data-semio-hub-connection")).toBe("reconnecting");
    expect(badge.getAttribute("aria-live")).toBe("polite");
    expect(badge.getAttribute("aria-label")).toContain("reconnecting");
    expect(badge.textContent).toContain("reconnecting");
  });

  it("carries the attached-document count and the live peer count", () => {
    render(<HubConnectionIndicator statuses={[status({ kind: "live", peerCount: 1 })]} session="signedIn" link="reachable" />);
    const badge = screen.getByRole("status");
    expect(badge.getAttribute("data-hub-connection-documents")).toBe("1");
    expect(badge.textContent).toContain("1 peer");
  });

  it("reads a short shortage of the session link as reconnecting and recovers to online", () => {
    const { rerender } = render(<HubConnectionIndicator statuses={[]} session="signedIn" link="unreachable" />);
    expect(screen.getByRole("status").getAttribute("data-semio-hub-connection")).toBe("reconnecting");
    rerender(<HubConnectionIndicator statuses={[]} session="signedIn" link="reachable" />);
    expect(screen.getByRole("status").getAttribute("data-semio-hub-connection")).toBe("online");
  });

  it("offers the sign-in entry point only when signed out AND an opener exists", () => {
    render(<HubConnectionIndicator statuses={[]} session="signedOut" link="verifying" onSignIn={() => {}} />);
    const expected = hubProjection.footerPresentation;
    const statusRegion = screen.getByRole(expected.status.role, { name: new RegExp(expected.status.labels.en, "i") });
    const signIn = screen.getByRole(expected.action.role, { name: expected.action.labels.en });
    expect(statusRegion.id).not.toBe(expected.action.id);
    expect(signIn.id).toBe(expected.action.id);
    expect(statusRegion.contains(signIn)).toBe(true);
    cleanup();
    render(<HubConnectionIndicator statuses={[]} session="signedOut" link="verifying" />);
    expect(document.querySelector("[data-semio-hub-sign-in]")).toBeNull();
    cleanup();
    render(<HubConnectionIndicator statuses={[]} session="none" link="verifying" onSignIn={() => {}} />);
    expect(document.querySelector("[data-semio-hub-sign-in]")).toBeNull();
    expect(screen.getByRole("status").getAttribute("data-semio-hub-connection")).toBe("local");
  });
});
//#endregion 🔖️HubConnectionIndicator

describe("🎯️ sync attach target", () => {
  for (const row of attachTargets.cases) {
    it(row.name, () => {
      const target = syncAttachTargetV1(row.kind as "file" | "folder" | "remote", row.draft);
      expect(target).toEqual(row.expected);
      if (target?.kind === "file") expect(target.folder).toBe(posix.dirname(target.path));
      if (target?.kind === "remote" && row.draft.trim().split("/").length >= 3) {
        const url = new URL(`http://${row.draft.trim()}`);
        const [spaceId, ...document] = url.pathname.slice(1).split("/");
        expect({ hostPort: target.hostPort, spaceId: target.spaceId, documentId: target.documentId }).toEqual({ hostPort: url.host, spaceId, documentId: document.join("/") });
      }
    });
  }
});

describe("♿️ sync attach card", () => {
  afterEach(() => cleanup());
  for (const cardKind of ["file", "folder", "remote"] as const) {
    it(`names its ${cardKind} path field by the card's own title and attaches the typed target`, () => {
      const attached: unknown[] = [];
      render(<SyncAttachCard activeUri={null} cardKind={cardKind} draftPath="work/doc.json" syncUtilities={[]} status={null} quarantinedConflicts={[]} onAction={() => {}} onDraftPathChange={() => {}} onClose={() => {}} onAttach={(target) => attached.push(target)} onDetach={() => {}} />);
      const title = document.getElementById(`framework.sync.${cardKind}.title`)?.textContent ?? "";
      expect(title.length).toBeGreaterThan(0);
      expect(screen.getByRole("textbox", { name: title })).toBeTruthy();
      const unlabelled = screen.getAllByRole("button").filter((button) => !button.hasAttribute("aria-label"));
      expect(unlabelled).toHaveLength(1);
      unlabelled[0]!.click();
      expect(attached).toEqual([syncAttachTargetV1(cardKind, "work/doc.json")]);
    });
  }
});
