// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔗️HubConnection/🏛️workspace/component.tsx
/** @emoji 🏛️ `HubWorkspace` — the one composed surface a shell mounts: `🔐️HubSignIn` above
 * `🏘️SpaceBrowser` above `🤖️AgentDelegations`, all driven by a single `useHubConnection` lane. It
 * lives in its own leaf rather than beside the hook because every pane imports that file's label
 * bundle, and a composition placed there would close an import cycle around
 * `registerUiTranslationBundles`.
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slices AU2 and M6b. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useState, type ReactElement } from "react";
import { Button, useLabel } from "@semio-tech/ui-react";
import { hubUiLabel, useHubConnection, type HubConnectionPortV1 } from "../🟦️.tsx";
import { HubSignInPane } from "../../🔐️HubSignIn/🟦️.tsx";
import { SpaceBrowser } from "../../🏘️SpaceBrowser/🟦️.tsx";
import { HubFirstRun } from "../../🎓️HubFirstRun/🟦️.tsx";
import { AgentDelegations } from "../../🤖️AgentDelegations/🟦️.tsx";
import { hubFirstRunSeenV1, hubFirstRunStateFromV1, markHubFirstRunSeenV1 } from "../../../../../📇️directory/🎓️first-run/🟦️.ts";
import { spaceRowInvitableV1 } from "../../../../../📇️directory/🏘️spaces/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🏛️HubWorkspace
export interface HubWorkspaceProps {
  readonly port: HubConnectionPortV1;
  /** 🌐️ `en` or `de`; the closed sign-in error table refuses anything else rather than defaulting. */
  readonly locale: string;
  readonly activeSpaceId: string | null;
  /** 👥️ Hub user ids the shell's own presence lane currently sees. Joined against the hub's roster
   * inside the hook, so a peer with no hub identity never invents a member row. */
  readonly onlineUserIds: readonly string[];
  readonly onOpenSpace: (spaceId: string) => void;
  readonly onClose: () => void;
}

/** 🏛️ Mounts one hub relationship as a single dialog-shaped surface. Opening a space closes the
 * surface first, so the shell never navigates behind an overlay that still owns focus. */
export function HubWorkspace({ port, locale, activeSpaceId, onlineUserIds, onOpenSpace, onClose }: HubWorkspaceProps): ReactElement {
  const hub = useHubConnection(port, onlineUserIds);
  const { watchSpaceMembers } = hub;
  useEffect(() => watchSpaceMembers(activeSpaceId), [activeSpaceId, watchSpaceMembers]);
  const title = useLabel(hubUiLabel("os.hub.signIn.title"));
  const closeLabel = useLabel(hubUiLabel("os.hub.signIn.cancel"));
  const replayLabel = useLabel(hubUiLabel("os.hub.firstRun.replay"));
  const firstRunState = hubFirstRunStateFromV1(hub.session.phase, hub.rows, activeSpaceId);
  // 🎓️ Auto-start is decided ONCE, when this surface first mounts, and never re-derived: reading the
  // seen flag in a later render would reopen the tour the moment `markHubFirstRunSeenV1` had not yet
  // been observed, and re-deriving `signedIn` would reopen it the instant a session expired. `replay`
  // is the explicit re-open, which ignores the flag by design.
  const [tour, setTour] = useState<"auto" | "replay" | null>(() => (hubFirstRunSeenV1(port.storage) ? null : "auto"));
  const dismissTour = (): void => {
    markHubFirstRunSeenV1(port.storage);
    setTour(null);
  };
  return (
    <section aria-label={title} data-semio-hub-workspace={hub.connection.id} className="flex w-full min-w-0 flex-col">
      {tour === null ? null : <HubFirstRun locale={locale} state={firstRunState} onDismiss={dismissTour} {...(tour === "replay" ? { initialStepIndex: 0 } : {})} />}
      <div className="flex flex-wrap justify-end gap-2 p-2">
        <Button id="os.hub.firstRun.replay" icon="graduation-cap" type="button" variant="ghost" aria-label={replayLabel} onClick={() => setTour("replay")}>
          {replayLabel}
        </Button>
        <Button id="os.hub.signIn.cancel" icon="x" type="button" variant="outline" aria-label={closeLabel} onClick={onClose}>
          {closeLabel}
        </Button>
      </div>
      <HubSignInPane
        book={hub.book}
        session={hub.session}
        locale={locale}
        onSelectConnection={hub.selectConnection}
        onAddHub={hub.addRemoteHub}
        onForgetHub={hub.forgetHub}
        onSignIn={hub.signIn}
        onCancel={hub.cancelSignIn}
        onSignOut={hub.signOut}
      />
      <SpaceBrowser
        rows={hub.rows}
        activeSpaceId={activeSpaceId}
        phase={hub.spacesPhase}
        search={hub.search}
        members={hub.members}
        invite={hub.invite}
        redemption={hub.redemption}
        signedIn={hub.session.phase === "signed-in"}
        onSearch={hub.setSearch}
        onOpenSpace={(spaceId) => {
          onClose();
          onOpenSpace(spaceId);
        }}
        onRefresh={hub.refreshSpaces}
        onCreateSpace={hub.createSpace}
        onArchiveSpace={hub.archiveSpace}
        onCreateInvite={hub.createInvite}
        onCopyInvite={hub.copyInvite}
        onDismissInvite={hub.dismissInvite}
        onRedeemInvite={hub.redeemInvite}
      />
      <AgentDelegations
        spaceId={activeSpaceId}
        rows={hub.delegations}
        phase={hub.delegationPhase}
        error={hub.delegationError}
        credential={hub.agentCredential}
        signedIn={hub.session.phase === "signed-in"}
        canDelegate={hub.rows.some((row) => row.id === activeSpaceId && spaceRowInvitableV1(row))}
        onRefresh={hub.refreshDelegations}
        onCreate={hub.createDelegation}
        onDownloadCredential={hub.downloadAgentCredential}
        onDismissCredential={hub.dismissAgentCredential}
        onRevoke={hub.revokeDelegation}
      />
    </section>
  );
}
// #endregion 🏛️HubWorkspace
