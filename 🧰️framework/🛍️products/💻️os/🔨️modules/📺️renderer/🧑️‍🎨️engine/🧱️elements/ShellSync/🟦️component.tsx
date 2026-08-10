// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/ShellSync/component.tsx
/** @emoji 🔗️ `ShellSync` — `SyncAttachCard`, the footer popover for attaching/detaching a document's
 * backbone sync connection (file/folder/remote), rendering the sync-scoped `UtilityTree` above a
 * draft-path input and status badge (`syncStatusLabel`).
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type ReactElement } from "react";
import { Button, Input, Popover, PopoverAnchor, PopoverContent, useLabel } from "@semio-tech/ui-react";
import { type ActionDescriptor, type UtilityNode } from "@semio-tech/framework";
import { type ArtifactSyncStatus, type FrameworkSyncUtilityLeaf, buildFileBackboneUri, buildFolderBackboneUri, buildRemoteBackboneUri } from "@semio-tech/framework-os";
import { type SyncCardKind } from "../Shell/🟦️component.tsx";
import { UtilityTree, groupUtilityNodesByCategory } from "../UtilityTree/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️sync-attach-card

type SyncAttachCardProps = {
  readonly activeUri: string | null;
  readonly cardKind: SyncCardKind | null;
  readonly draftPath: string;
  readonly syncUtilities: readonly FrameworkSyncUtilityLeaf[];
  readonly status: ArtifactSyncStatus | null;
  readonly onAction: (action: ActionDescriptor) => void;
  readonly onDraftPathChange: (value: string) => void;
  readonly onClose: () => void;
  readonly onAttach: (uri: string) => void;
  readonly onDetach: () => void;
};

/** 🚦️ Minimal status label for an `ArtifactSyncStatus` — matches this file's small-badge-text style
 * (see the `activeUri` line right below it), not a new component system. */
function syncStatusLabel(status: ArtifactSyncStatus | null): string | null {
  if (!status) return null;
  const remote =
    status.remote.kind === "live" ? `live · ${status.remote.peerCount} peer${status.remote.peerCount === 1 ? "" : "s"}` : status.remote.kind === "connecting" ? "connecting…" : status.remote.kind === "backoff" ? "reconnecting…" : "offline";
  const persisted = status.persisted ? "saved" : "unsaved";
  const pending = status.pendingMutations > 0 ? ` · ${status.pendingMutations} pending` : "";
  return `${remote} · ${persisted}${pending}`;
}

export function SyncAttachCard({ activeUri, cardKind, draftPath, syncUtilities, status, onAction, onDraftPathChange, onClose, onAttach, onDetach }: SyncAttachCardProps): ReactElement {
  const open = cardKind != null;
  const attachLabel = useLabel("ui.sync.attach");
  const detachLabel = useLabel("ui.sync.detach");
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
            <p className="text-sm font-medium capitalize">{cardKind} backbone</p>
            {activeUri ? <p className="break-all text-xs text-muted-foreground">{activeUri}</p> : null}
            {activeUri && status ? <p className="text-xs text-muted-foreground">{syncStatusLabel(status)}</p> : null}
          </div>
          <Input value={draftPath} placeholder={placeholder} onChange={(event) => onDraftPathChange(event.target.value)} />
          <div className="flex items-center gap-2">
            <Button type="button" onClick={attachFromDraft}>
              {attachLabel}
            </Button>
            {activeUri ? (
              <Button type="button" onClick={onDetach}>
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
