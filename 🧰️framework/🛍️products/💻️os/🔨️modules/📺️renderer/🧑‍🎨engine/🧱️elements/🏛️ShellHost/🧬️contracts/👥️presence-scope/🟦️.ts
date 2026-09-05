import type { BackboneWorkerResponse, DocumentScope } from "@semio-tech/framework-os";
import type { ArtifactPresencePeer } from "@semio-tech/framework-replication";
import type { PresencePeer } from "@semio-tech/ui-react";

function presenceRole(role: ArtifactPresencePeer["role"]): PresencePeer["role"] {
  if (role === "author" || role === "owner" || role === "member") return "author";
  if (role === "spectator" || role === "viewer") return "spectator";
  return undefined;
}

/** 👥️ Projects only a worker-verified, exact-scope surface roster into host chrome. */
export function scopedPresencePeersV1(
  message: Extract<BackboneWorkerResponse, { readonly kind: "event" }>,
  expectedScope: DocumentScope,
): readonly PresencePeer[] {
  if (message.event.kind !== "presence" || message.scope?.spaceId !== expectedScope.spaceId || message.scope.documentId !== expectedScope.documentId || message.documentId !== expectedScope.documentId || message.verifiedSurfaceId === undefined) return [];
  return message.event.peers
    .filter((peer) => peer.surface === message.verifiedSurfaceId)
    .map((peer) => ({
      actor: peer.actor,
      ...(peer.userId === undefined ? {} : { userId: peer.userId }),
      label: peer.label ?? peer.actor,
      ...(presenceRole(peer.role) === undefined ? {} : { role: presenceRole(peer.role) }),
      connectedAtMs: peer.connectedAtMs,
      ...(peer.color === undefined ? {} : { color: peer.color }),
    }));
}
