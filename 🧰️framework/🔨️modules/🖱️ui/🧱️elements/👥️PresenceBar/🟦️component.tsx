// #region 🧲️Header
// 💻️ framework/ui/elements/👥️PresenceBar/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { TableAvatar } from "../📻️TableAvatar/🟦️component.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️component.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️component.ts";
import { useLabel } from "../🏷️Label/🟦️component.tsx";
// #endregion 🔌️Adapters

// #region 👥️PresenceBar
// Compact horizontal roster of the peers on the same `(space, document, surface)` presence scope
// (contract freeze `26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS` §C0). Consumers
// (lanes 2-C/2-D/3-A) pass the caller-supplied `id` `s-presence-peers`; this element does not talk to
// the hub or the directory read model itself — it only renders whatever `peers` it is given.

/** @emoji 🧮️ Default visible-avatar cap before the "+N" overflow chip takes over — mirrored by the Rust
 * twin's `PRESENCE_BAR_DEFAULT_MAX` (`🧊️component.rs`) so both shells collapse at the same roster size. */
export const PRESENCE_BAR_DEFAULT_MAX = 5;

/** @emoji 🎭️ A peer's editing/viewing stance on the shared `(space, document, surface)` — mirrors the
 * hub's `SpaceRole` vocabulary (contract freeze §C1). */
export type PresenceRole = "author" | "spectator";

/**
 * One peer currently attached to the same `(space, document, surface)` presence scope.
 **/
export interface PresencePeer {
  readonly actor: string;
  readonly userId?: string;
  readonly label: string;
  readonly role?: PresenceRole;
  readonly connectedAtMs?: number;
}

/**
 * Props interface for the PresenceBar component.
 **/
export interface PresenceBarProps {
  readonly peers: readonly PresencePeer[];
  readonly max?: number;
  readonly id?: string;
  readonly className?: string;
}

const PRESENCE_HUE_FNV_OFFSET = 0x811c9dc5;
const PRESENCE_HUE_FNV_PRIME = 0x01000193;

/** @emoji 🎨️ Deterministic FNV-1a hash of the actor id's UTF-8 bytes into an HSL hue (0–359) —
 * byte-for-byte mirrored by the Rust twin's `presence_hue_for_actor` so both shells tint the same
 * peer identically. */
export function presenceHueForActor(actor: string): number {
  const bytes = new TextEncoder().encode(actor);
  let hash = PRESENCE_HUE_FNV_OFFSET;
  for (const byte of bytes) {
    hash ^= byte;
    hash = Math.imul(hash, PRESENCE_HUE_FNV_PRIME);
  }
  return (hash >>> 0) % 360;
}

function presenceRingColor(actor: string): string {
  return `hsl(${presenceHueForActor(actor)}, 65%, 45%)`;
}

/**
 * PresenceBar renders a compact horizontal roster of peers sharing a presence scope: an avatar per
 * peer (reusing {@link TableAvatar}), the peer's name on hover/focus, a deterministic per-actor color,
 * an overflow "+N" chip past `max`, and an empty state.
 **/
export const PresenceBar: React.FC<PresenceBarProps> = ({ peers, max = PRESENCE_BAR_DEFAULT_MAX, id, className }) => {
  const rosterLabel = useLabel("ui.presence.roster");
  const emptyLabel = useLabel("ui.presence.empty");
  const authorRoleLabel = useLabel("ui.presence.role.author");
  const spectatorRoleLabel = useLabel("ui.presence.role.spectator");
  const visible = peers.slice(0, Math.max(max, 0));
  const overflowCount = Math.max(peers.length - visible.length, 0);
  const overflowLabel = useLabel("ui.presence.overflow", { count: overflowCount });

  if (peers.length === 0) {
    return (
      <div id={id} role="list" aria-label={rosterLabel} className={cn("flex items-center gap-1 text-xs text-muted-foreground", className)}>
        {emptyLabel}
      </div>
    );
  }

  return (
    <div id={id} role="list" aria-label={rosterLabel} className={cn("flex items-center -space-x-2", className)}>
      {visible.map((peer) => {
        const roleLabel = peer.role === "author" ? authorRoleLabel : peer.role === "spectator" ? spectatorRoleLabel : undefined;
        const peerTitle = roleLabel ? `${peer.label} (${roleLabel})` : peer.label;
        return (
          <div key={peer.actor} role="listitem" data-row-id={`peer:${peer.actor}`} tabIndex={0} title={peerTitle} aria-label={peerTitle} className="rounded-full">
            <TableAvatar name={peer.label} style={{ borderColor: presenceRingColor(peer.actor), borderWidth: 2 }} />
          </div>
        );
      })}
      {overflowCount > 0 ? (
        <div
          role="listitem"
          data-row-id="peer:overflow"
          tabIndex={0}
          title={overflowLabel}
          aria-label={overflowLabel}
          className={cn(surfaceClass, "flex size-small shrink-0 items-center justify-center rounded-full border text-xs font-medium text-muted-foreground")}
        >
          +{overflowCount}
        </div>
      ) : null}
    </div>
  );
};
PresenceBar.displayName = "PresenceBar";
// #endregion 👥️PresenceBar
