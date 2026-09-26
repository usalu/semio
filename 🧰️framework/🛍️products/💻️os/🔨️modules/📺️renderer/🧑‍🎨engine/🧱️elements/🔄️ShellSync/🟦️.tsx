// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellSync/component.tsx
/** @emoji 🔗️ `ShellSync` — `SyncAttachCard`, the footer popover for attaching/detaching a document's
 * backbone sync connection (file/folder/remote), rendering the sync-scoped `UtilityTree` above a
 * draft-path input and status badge (`useSyncStatusLabel`), plus `HubConnectionIndicator`, the
 * always-visible shell-chrome badge for the hub link itself.
 *
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice U1 (audit `📓️g5-ux-completeness-audit.md`
 * ranked items 4 and 8): every status word used to be a hard-coded English literal inside
 * `syncStatusLabel`, and the only place a human could learn the hub was unreachable was this popover,
 * per document. Both are fixed here — the words go through `useLabel` (en + de, compile-checked by
 * `UiTranslationSchema`), and the aggregate lives in the footer beside the presence bar.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ReactElement } from "react";
import { Button, Icon, Input, Popover, PopoverAnchor, PopoverContent, useLabel, type IconName } from "@semio-tech/ui-react";
import { type ActionDescriptor, type Conflict, type UtilityNode } from "@semio-tech/framework";
import { type ArtifactSyncStatus, type FrameworkSyncUtilityLeaf, buildFileBackboneUri, buildFolderBackboneUri, buildRemoteBackboneUri } from "@semio-tech/framework-os";
import { type SyncCardKind } from "../🐚️Shell/🟦️.tsx";
import { UtilityTree, groupUtilityNodesByCategory } from "../🎛️UtilityTree/🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🔖️sync-attach-card

type SyncAttachCardProps = {
  readonly activeUri: string | null;
  readonly cardKind: SyncCardKind | null;
  readonly draftPath: string;
  readonly syncUtilities: readonly FrameworkSyncUtilityLeaf[];
  readonly status: ArtifactSyncStatus | null;
  /** ⚖️ Open `Quarantined` conflicts (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-
   * AND-FIRST-CLASS-CONFLICTS` §C6/§C9) — non-empty means a peer batch is being held back by this
   * authority's merge policy rather than applied; see `Shell`'s `selectQuarantinedConflicts`. */
  readonly quarantinedConflicts: readonly Conflict[];
  readonly onAction: (action: ActionDescriptor) => void;
  readonly onDraftPathChange: (value: string) => void;
  readonly onClose: () => void;
  readonly onAttach: (uri: string) => void;
  readonly onDetach: () => void;
  readonly onBrowsePath?: () => void | Promise<void>;
};

/** 🌐️ Every string {@link syncStatusLabelV1} composes, already resolved for the active locale and
 * label tier. Kept as an explicit argument (rather than resolved inside the composer) so the composer
 * stays a pure, React-free function a unit test can drive with either locale's real bundle values. */
export interface SyncStatusTextsV1 {
  readonly live: string;
  readonly connecting: string;
  readonly reconnecting: string;
  readonly offline: string;
  /** 👥️ Already interpolated with the live peer count (`ui.sync.peerOne`/`ui.sync.peerMany`). */
  readonly peers: string;
  readonly saved: string;
  readonly unsaved: string;
  /** 📮️ Already interpolated with `pendingMutations` (`ui.sync.pending`). */
  readonly pending: string;
}

/** 🚦️ Minimal status label for an `ArtifactSyncStatus` — matches this file's small-badge-text style
 * (see the `activeUri` line right below it), not a new component system. Pure: {@link useSyncStatusLabel}
 * is the only place the `ui.sync.*` keys are read. */
export function syncStatusLabelV1(status: ArtifactSyncStatus | null, texts: SyncStatusTextsV1): string | null {
  if (!status) return null;
  const remote = status.remote.kind === "live" ? `${texts.live} · ${texts.peers}` : status.remote.kind === "connecting" ? texts.connecting : status.remote.kind === "backoff" ? texts.reconnecting : texts.offline;
  const persisted = status.persisted ? texts.saved : texts.unsaved;
  return `${remote} · ${persisted}${status.pendingMutations > 0 ? ` · ${texts.pending}` : ""}`;
}

/** 🌐️ Resolves every `ui.sync.*` status word for the active locale. Both counted keys are resolved
 * unconditionally with the live counts (hooks may not run behind a branch), and the composer picks
 * which words the current state actually needs. */
export function useSyncStatusTexts(status: ArtifactSyncStatus | null): SyncStatusTextsV1 {
  const peerCount = status?.remote.kind === "live" ? status.remote.peerCount : 0;
  const peerOne = useLabel("ui.sync.peerOne", { count: peerCount });
  const peerMany = useLabel("ui.sync.peerMany", { count: peerCount });
  return {
    live: useLabel("ui.sync.live"),
    connecting: useLabel("ui.sync.connecting"),
    reconnecting: useLabel("ui.sync.reconnecting"),
    offline: useLabel("ui.sync.offline"),
    peers: peerCount === 1 ? peerOne : peerMany,
    saved: useLabel("ui.sync.saved"),
    unsaved: useLabel("ui.sync.unsaved"),
    pending: useLabel("ui.sync.pending", { count: status?.pendingMutations ?? 0 }),
  };
}

/** 🚦️ The localized status line for one document's backbone connection. */
export function useSyncStatusLabel(status: ArtifactSyncStatus | null): string | null {
  return syncStatusLabelV1(status, useSyncStatusTexts(status));
}

export function SyncAttachCard({ activeUri, cardKind, draftPath, syncUtilities, status, quarantinedConflicts, onAction, onDraftPathChange, onClose, onAttach, onDetach, onBrowsePath }: SyncAttachCardProps): ReactElement {
  const open = cardKind != null;
  const attachLabel = useLabel("ui.sync.attach");
  const detachLabel = useLabel("ui.sync.detach");
  const browseLabel = useLabel("ui.sync.browse");
  const quarantinedLabel = useLabel("ui.conflict.quarantined");
  const statusAccessibleLabel = useLabel("ui.sync.statusLabel");
  const statusLine = useSyncStatusLabel(status);
  const backboneTitles: Readonly<Record<SyncCardKind, string>> = {
    file: useLabel("ui.sync.backboneFile"),
    folder: useLabel("ui.sync.backboneFolder"),
    remote: useLabel("ui.sync.backboneRemote"),
  };
  const placeholder = cardKind === "remote" ? "127.0.0.1:8787/studio-1/demo" : cardKind === "folder" ? "/absolute/project/folder" : "/absolute/document.json";

  const attachFromDraft = () => {
    if (!cardKind || !draftPath.trim()) return;
    if (cardKind === "remote") {
      const [hostPort, ...rest] = draftPath.split("/");
      const [spaceId, documentId] = rest.length >= 2 ? [rest[0], rest.slice(1).join("/")] : ["default", rest[0] || "document"];
      onAttach(buildRemoteBackboneUri(hostPort || draftPath, spaceId, documentId));
      return;
    }
    onAttach(cardKind === "folder" ? buildFolderBackboneUri(draftPath) : buildFileBackboneUri(draftPath));
  };

  return (
    <Popover
      open={open}
      onOpenChange={(nextOpen) => {
        if (!nextOpen) onClose();
      }}
    >
      <PopoverAnchor asChild>
        <div>
          <UtilityTree utilities={groupUtilityNodesByCategory(syncUtilities as readonly UtilityNode[], ["sync"])} onAction={onAction} />
        </div>
      </PopoverAnchor>
      {open ? (
        <PopoverContent side="top" align="center" className="w-80 space-y-3 p-3">
          <div className="space-y-1">
            <p className="text-sm font-medium">{cardKind ? backboneTitles[cardKind] : null}</p>
            {activeUri ? <p className="break-all text-xs text-muted-foreground">{activeUri}</p> : null}
            {activeUri && statusLine ? (
              <p role="status" aria-live="polite" aria-label={statusAccessibleLabel} data-semio-sync-status="" className="text-xs text-muted-foreground">
                {statusLine}
              </p>
            ) : null}
            {quarantinedConflicts.length > 0 ? (
              <p className="rounded-sm border border-amber-400 bg-amber-400/10 px-single py-0.5 text-xs text-amber-400" role="status" data-semio-sync-quarantined="">
                {quarantinedLabel} ({quarantinedConflicts.length})
              </p>
            ) : null}
          </div>
          {cardKind === "remote" ? (
            <Input id={`framework.sync.${cardKind}.path`} value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
          ) : (
            <div className="flex items-center gap-2">
              <Input id={`framework.sync.${cardKind}.path`} className="min-w-0 flex-1" value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
              <Button icon="folder-open" type="button" data-semio-sync-browse="" aria-label={browseLabel} title={browseLabel} onClick={() => void onBrowsePath?.()} />
            </div>
          )}
          <div className="flex items-center gap-2">
            <Button icon="link" type="button" onClick={attachFromDraft}>
              {attachLabel}
            </Button>
            {activeUri ? (
              <Button icon="x" type="button" onClick={onDetach}>
                {detachLabel}
              </Button>
            ) : null}
          </div>
        </PopoverContent>
      ) : null}
    </Popover>
  );
}
//#endregion 🔖️sync-attach-card

//#region 🔖️hub-connection-indicator
/** 🔐️ Whether this shell holds a hub session. `"none"` means this shell runs without any hub configured
 * (purely local-first), which is NOT the same as being signed out of one — only `"signedOut"` is a state
 * the human can act on, and only it offers the sign-in entry point. */
export type HubSessionPresenceV1 = "signedIn" | "signedOut" | "none";

/** 🔗️ What the shell's own session revalidation last learned about the hub link, independent of any
 * document: still `verifying` a remembered session, `reachable`, or `unreachable` — a short shortage the
 * refresh loop is riding out on a bounded backoff while it keeps the authority it already verified. */
export type HubLinkV1 = "verifying" | "reachable" | "unreachable";

/** 📶️ What the always-visible chrome badge reports about the hub link as a whole. */
export type HubConnectionIndicatorStateV1 = "local" | "signedOut" | "live" | "online" | "connecting" | "reconnecting";

/** 📶️ The aggregate a single badge can honestly show for however many documents are attached. */
export interface HubConnectionSummaryV1 {
  readonly state: HubConnectionIndicatorStateV1;
  /** 👥️ Peers on the busiest live document — `0` for every non-`live` state. A sum across documents
   * would double-count a peer who has two of them open, which no reader could interpret. */
  readonly peerCount: number;
  /** 📄️ How many attached documents this aggregate was folded from; `0` means nothing is attached. */
  readonly documentCount: number;
}

/** 📶️ Folds the shell's own hub link and every attached document's `RemoteState` into ONE aggregate, best
 * state first. No hub configured is `local`; no session is `signedOut`, because no transport state means
 * anything without one. A live document means the hub answers right now, so `live` wins over everything
 * else; an unreachable session link is a shortage in progress (`reconnecting`), which outranks a document
 * still dialling; a verified, reachable link with no live document is `online`. The three link states are exhaustive, so
 * there is no further "offline" state (the hub projection schema's twin, `🧬️schema/🔗️hub-projection`, dropped it too).
 * Pure — the indicator below is the only place this is rendered. */
export function hubConnectionSummaryV1(statuses: readonly ArtifactSyncStatus[], session: HubSessionPresenceV1, link: HubLinkV1): HubConnectionSummaryV1 {
  const documentCount = statuses.length;
  if (session === "none") return { state: "local", peerCount: 0, documentCount };
  if (session === "signedOut") return { state: "signedOut", peerCount: 0, documentCount };
  const peerCount = statuses.reduce((best, status) => (status.remote.kind === "live" ? Math.max(best, status.remote.peerCount) : best), 0);
  if (statuses.some((status) => status.remote.kind === "live")) return { state: "live", peerCount, documentCount };
  if (link === "unreachable") return { state: "reconnecting", peerCount: 0, documentCount };
  if (link === "verifying" || statuses.some((status) => status.remote.kind === "connecting")) return { state: "connecting", peerCount: 0, documentCount };
  if (statuses.some((status) => status.remote.kind === "backoff")) return { state: "reconnecting", peerCount: 0, documentCount };
  return { state: "online", peerCount: 0, documentCount };
}

/** 🎨️ One icon per state — the badge carries icon AND text, never colour alone, so it survives a
 * monochrome theme and a colour-vision deficiency exactly as it survives a screen reader. */
const HUB_CONNECTION_ICON: Readonly<Record<HubConnectionIndicatorStateV1, IconName>> = {
  local: "link-2-off",
  signedOut: "user",
  live: "cloud",
  online: "cloud",
  connecting: "loader-2",
  reconnecting: "rotate-ccw",
};

const HUB_CONNECTION_TONE: Readonly<Record<HubConnectionIndicatorStateV1, string>> = {
  local: "text-muted-foreground",
  signedOut: "text-muted-foreground",
  live: "text-emphasized",
  online: "text-emphasized",
  connecting: "text-muted-foreground",
  reconnecting: "text-amber-400",
};

export interface HubConnectionIndicatorProps {
  readonly statuses: readonly ArtifactSyncStatus[];
  readonly session: HubSessionPresenceV1;
  readonly link: HubLinkV1;
  /** 🔐️ Entry point into the sign-in surface, offered only while `session` is `"signedOut"`. Absent
   * means no such surface is mounted yet, and the badge then reports the state without offering an
   * action it cannot perform. */
  readonly onSignIn?: () => void;
}

/** @emoji 📶️ The persistent hub-connection badge. Lives in the shell footer beside the presence bar,
 * so "the hub is unreachable" is readable without opening any one document's sync popover — audit
 * `📓️g5-ux-completeness-audit.md` §5's last gap. `role="status"` + `aria-live="polite"` announce a
 * transition once; the accessible name is the localized state text, never a colour. It only renders:
 * a shortage never blocks the shell, it just reads `reconnecting` until the link answers again. */
export function HubConnectionIndicator({ statuses, session, link, onSignIn }: HubConnectionIndicatorProps): ReactElement {
  const summary = hubConnectionSummaryV1(statuses, session, link);
  const hubLabel = useLabel("ui.sync.hubLabel");
  const signInLabel = useLabel("ui.sync.hubSignIn");
  const peerOne = useLabel("ui.sync.peerOne", { count: summary.peerCount });
  const peerMany = useLabel("ui.sync.peerMany", { count: summary.peerCount });
  const stateText: Readonly<Record<HubConnectionIndicatorStateV1, string>> = {
    local: useLabel("ui.sync.localOnly"),
    signedOut: useLabel("ui.sync.signedOut"),
    live: useLabel("ui.sync.live"),
    online: useLabel("ui.sync.online"),
    connecting: useLabel("ui.sync.connecting"),
    reconnecting: useLabel("ui.sync.reconnecting"),
  };
  const text = summary.state === "live" ? `${stateText.live} · ${summary.peerCount === 1 ? peerOne : peerMany}` : stateText[summary.state];
  return (
    <div
      role="status"
      aria-live="polite"
      aria-label={`${hubLabel}: ${text}`}
      data-semio-hub-connection={summary.state}
      data-hub-connection-documents={summary.documentCount}
      className={`flex items-center gap-single px-single text-2xs ${HUB_CONNECTION_TONE[summary.state]}`}
    >
      <Icon icon={HUB_CONNECTION_ICON[summary.state]} size="small" />
      <span>{text}</span>
      {summary.state === "signedOut" && onSignIn ? <Button id="framework.hub.signIn" type="button" variant="ghost" icon="user" data-semio-hub-sign-in="" text={signInLabel} onClick={onSignIn} /> : null}
    </div>
  );
}
//#endregion 🔖️hub-connection-indicator
