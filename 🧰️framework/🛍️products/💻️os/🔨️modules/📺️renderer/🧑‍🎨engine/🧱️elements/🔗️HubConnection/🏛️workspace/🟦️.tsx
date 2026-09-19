// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔗️HubConnection/🏛️workspace/component.tsx
/** @emoji 🏛️ `HubWorkspace` — the one composed surface a shell mounts: `🔐️HubSignIn` above
 * `🏘️SpaceBrowser`, both driven by a single `useHubConnection` lane. It lives in its own leaf rather
 * than beside the hook because both panes import that file's label bundle, and a composition placed
 * there would close an import cycle around `registerUiTranslationBundles`.
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, type ReactElement } from "react";
import { Button, useLabel } from "@semio-tech/ui-react";
import { hubUiLabel, useHubConnection, type HubConnectionPortV1 } from "../🟦️.tsx";
import { HubSignInPane } from "../../🔐️HubSignIn/🟦️.tsx";
import { SpaceBrowser } from "../../🏘️SpaceBrowser/🟦️.tsx";
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
  /** 🔐️ Reports the session phase up to the shell's persistent hub badge. The workspace unmounts
   * when the overlay closes while the port keeps the capability, so the shell — not this element —
   * is where "am I signed in" has to live. */
  readonly onSessionChange?: (presence: "signedIn" | "signedOut") => void;
  readonly onClose: () => void;
}

/** 🏛️ Mounts one hub relationship as a single dialog-shaped surface. Opening a space closes the
 * surface first, so the shell never navigates behind an overlay that still owns focus. */
export function HubWorkspace({ port, locale, activeSpaceId, onlineUserIds, onSessionChange, onOpenSpace, onClose }: HubWorkspaceProps): ReactElement {
  const hub = useHubConnection(port, onlineUserIds);
  const { watchSpaceMembers } = hub;
  const sessionPhase = hub.session.phase;
  useEffect(() => watchSpaceMembers(activeSpaceId), [activeSpaceId, watchSpaceMembers]);
  useEffect(() => onSessionChange?.(sessionPhase === "signed-in" ? "signedIn" : "signedOut"), [onSessionChange, sessionPhase]);
  const title = useLabel(hubUiLabel("os.hub.signIn.title"));
  const closeLabel = useLabel(hubUiLabel("os.hub.signIn.cancel"));
  return (
    <section aria-label={title} data-semio-hub-workspace={hub.connection.id} className="flex w-full min-w-0 flex-col">
      <div className="flex justify-end p-2">
        <Button icon="x" type="button" variant="outline" aria-label={closeLabel} onClick={onClose}>
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
    </section>
  );
}
// #endregion 🏛️HubWorkspace
