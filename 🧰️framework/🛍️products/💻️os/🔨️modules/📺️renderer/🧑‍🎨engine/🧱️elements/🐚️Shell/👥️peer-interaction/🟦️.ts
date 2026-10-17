/** 👥️ Shell peer interaction: the sync card kind, the history entries, the presence peers and the per-domain roster of what peers select and hover. */
import { type PresenceInteraction } from "@semio-tech/framework";

//#region 🔖️types
export type SyncCardKind = "file" | "folder" | "remote";

type UIHistoryEntry = { readonly uri: string };
export type UIHistory = { readonly entries: readonly UIHistoryEntry[]; readonly index: number };

//#region 🕹️PeerInteraction
/** 🕹️ One peer's presence roster entry as the Shell sees it: the existing `clientId`/`name` identity
 * plus the typed `interaction` field from `PresencePeer` (bit 7, wave 2a) — kept optional and defensive
 * since a peer on an older heartbeat, or one whose app declares no `InteractionDefinition`, omits it. */
export type ShellPresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly interaction?: PresenceInteraction;
};

/** 🕹️ Every online peer's selection/hover for one domain, keyed by peer `clientId` — the app-agnostic
 * shape a renderer paints from instead of hand-decoding an app-specific presence JSON per app. */
export type PeerInteractionDomain = {
  readonly selectedByPeer: Readonly<Record<string, readonly string[]>>;
  readonly hoveredByPeer: Readonly<Record<string, readonly string[]>>;
};

/** 🕹️ Every online peer's interaction, regrouped from per-peer `PresenceInteraction.domains` to
 * per-domain-id — the shape {@link derivePeerInteractionByDomain} returns. Keyed by domain id (e.g.
 * "graph", "mesh"), not by app: two apps sharing a domain id share one entry, matching how the
 * framework's own `InteractionState` is keyed. */
export type PeerInteractionRoster = Readonly<Record<string, PeerInteractionDomain>>;

const EMPTY_PEER_INTERACTION_DOMAIN: PeerInteractionDomain = { selectedByPeer: {}, hoveredByPeer: {} };

/**
 * 🕹️ Regroups a peer roster's typed `PresenceInteraction` into a {@link PeerInteractionRoster} — one
 * entry per domain id, each holding every peer's selected/hovered ids for that domain. Replaces the old
 * pattern of hand-decoding an app-specific `presencePeersJson` (only ever wired for a few apps): any
 * renderer — a Tree row, a canvas item, a table cell — can call {@link peerIdsSelecting}/
 * {@link peerIdsHovering} for its own domain+id without knowing which app or plugin the peer runs.
 * Defensive by construction: a peer without `interaction` (older heartbeat, or wave 2a's wire field not
 * landed yet) simply contributes nothing.
 */
export function derivePeerInteractionByDomain(peers: readonly ShellPresencePeer[]): PeerInteractionRoster {
  const roster: Record<string, { selectedByPeer: Record<string, readonly string[]>; hoveredByPeer: Record<string, readonly string[]> }> = {};
  for (const peer of peers) {
    for (const domain of peer.interaction?.domains ?? []) {
      const entry = roster[domain.domain] ?? (roster[domain.domain] = { selectedByPeer: {}, hoveredByPeer: {} });
      if (domain.selected.length > 0) entry.selectedByPeer[peer.clientId] = domain.selected;
      if (domain.hovered.length > 0) entry.hoveredByPeer[peer.clientId] = domain.hovered;
    }
  }
  return roster;
}

/** 🕹️ Peer `clientId`s with `targetId` currently selected in `domain` — empty when the roster carries no
 * peers for that domain (e.g. wave 2a's wire field not landed yet, or nobody else is online). */
export function peerIdsSelecting(roster: PeerInteractionRoster, domain: string, targetId: string): readonly string[] {
  const { selectedByPeer } = roster[domain] ?? EMPTY_PEER_INTERACTION_DOMAIN;
  return Object.entries(selectedByPeer)
    .filter(([, ids]) => ids.includes(targetId))
    .map(([clientId]) => clientId);
}

/** 🕹️ Peer `clientId`s with `targetId` currently hovered in `domain` — see {@link peerIdsSelecting}. */
export function peerIdsHovering(roster: PeerInteractionRoster, domain: string, targetId: string): readonly string[] {
  const { hoveredByPeer } = roster[domain] ?? EMPTY_PEER_INTERACTION_DOMAIN;
  return Object.entries(hoveredByPeer)
    .filter(([, ids]) => ids.includes(targetId))
    .map(([clientId]) => clientId);
}
//#endregion 🕹️PeerInteraction
//#endregion 🔖️types
