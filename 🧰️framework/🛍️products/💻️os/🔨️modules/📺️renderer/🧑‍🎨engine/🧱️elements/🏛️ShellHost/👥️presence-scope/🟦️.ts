import type { BackboneWorkerResponse, DocumentScope } from "@semio-tech/framework-os";
import type { ArtifactPresenceHistoryEdit, ArtifactPresencePeer } from "@semio-tech/framework-replication";
import type { PresencePeer } from "@semio-tech/ui-react";

function presenceRole(role: ArtifactPresencePeer["role"]): PresencePeer["role"] {
  if (role === "author" || role === "owner" || role === "member") return "author";
  if (role === "spectator" || role === "viewer") return "spectator";
  return undefined;
}

/** 👥️ One roster row as host chrome holds it: the chip's own fields plus the peer's open history edit, which the shell
 * labels from its own history rows (no locale text travels on the wire). */
export type ScopedPresencePeerV1 = PresencePeer & { readonly historyEdit?: ArtifactPresenceHistoryEdit };

/** 👥️ Projects a worker-verified roster into host chrome: the message must name the session's exact scope and the surface
 * the session was admitted on (`ScopedPresenceRejectionV1`), and then every peer of the document is projected whatever
 * surface it runs — the roster is document-wide (contract §C7.0), so an editor sees its spectators and a viewer sees the
 * authors it watches.
 * @see ../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION/📋️contract-freeze.md */
export function scopedPresencePeersV1(
  message: Extract<BackboneWorkerResponse, { readonly kind: "event" }>,
  expectedScope: DocumentScope,
  expectedSurfaceId: string | null,
): readonly ScopedPresencePeerV1[] {
  if (message.event.kind !== "presence" || message.scope?.spaceId !== expectedScope.spaceId || message.scope.documentId !== expectedScope.documentId || message.documentId !== expectedScope.documentId || message.verifiedSurfaceId === undefined || message.verifiedSurfaceId !== expectedSurfaceId) return [];
  return message.event.peers
    .map((peer) => ({
      actor: peer.actor,
      ...(peer.userId === undefined ? {} : { userId: peer.userId }),
      label: peer.label ?? peer.actor,
      ...(presenceRole(peer.role) === undefined ? {} : { role: presenceRole(peer.role) }),
      connectedAtMs: peer.connectedAtMs,
      ...(peer.color === undefined ? {} : { color: peer.color }),
      // 🤖️ `principalKind` is admitted by the Hub from the session it authenticated, exactly like
      // `color` and `surface`; an absent value is the pre-agent wire shape and means a person.
      ...(peer.principalKind === "agent" ? { isAgent: true } : {}),
      ...(peer.historyEdit === undefined ? {} : { historyEdit: peer.historyEdit }),
    }));
}
