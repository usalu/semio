// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🔗️HubConnection/component.tsx
/** 🔗️ `🔗️HubConnection` — the headless hook that owns a human's relationship with one hub:
 * which hub is chosen, signing in against `POST /auth/sessions`, holding the minted capability in
 * memory only, listing/creating spaces and invitations through the directory command path, and
 * redeeming an invitation. Presentation lives in `🔐️HubSignIn` and `🏘️SpaceBrowser`; every network
 * call enters through the injected {@link HubConnectionPortV1}, so a test drives the real contract
 * against a fake transport. Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2; hub-side
 * contract in that ticket's `📓️au1-hub-auth-sessions-and-rate-limit.md` §1. The label bundle
 * (`🌐️ui-labels`), the port and row contract (`🔌️connection-contract`) and the `fetch`-backed port
 * (`🛰️fetch-port`) live in the child folders beside this file.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { type HubConnectionPortV1, type HubConnectionValueV1 } from "./🔌️connection-contract/🟦️.ts";
import { useHubCommandCallbacks, useHubConnectionBook } from "./📒️connection-book-and-commands/🟦️.tsx";
import { useHubDelegationCallbacks, useHubSignInAndAgentCallbacks } from "./🔑️sign-in-agent-and-delegation/🟦️.tsx";
// #endregion 🔌️Adapters
//#region 🔖️Hook

/** 🔗️ Mounts one hub relationship. Every async lane carries its own `AbortController` and
 * connection generation so a switch, cancel press or unmount rejects even abort-insensitive completions.
 * The session capability lives inside the port, never in this hook's state, so React never serializes
 * it into a snapshot. */
export function useHubConnection(port: HubConnectionPortV1, onlineUserIds: readonly string[] = []): HubConnectionValueV1 {
  const { captureOperationOwner, commandAbort, setSpacesPhase, portRef, operationOwnerCurrent, session, setRows, loadSpaces, loadMembers, watchedSpaceIdRef, spacesAbort, connection, rebootstrapped, dispatchSession, readAuthority, operationGeneration, signInAbort, signOutAbort, authorityAbort, membersAbort, redemptionAbort, delegationAbort, book, operationConnectionId, retireOperationsForConnection, setAuthority, setMemberRows, setInvite, setRedemption, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, persist, setSearch, connectionRef, invite, loadDelegations, agentCredential, rows, search, memberRows, delegationSummaries, authority, spacesPhase, redemption, delegationPhase, delegationError, watchSpaceMembers } = useHubConnectionBook({ port });
  const { runCommand, selectConnection, addRemoteHub, forgetHub, signIn, cancelSignIn } = useHubCommandCallbacks({ captureOperationOwner, commandAbort, setSpacesPhase, portRef, operationOwnerCurrent, session, setRows, loadSpaces, loadMembers, watchedSpaceIdRef, spacesAbort, connection, rebootstrapped, dispatchSession, readAuthority, operationGeneration, signInAbort, signOutAbort, authorityAbort, membersAbort, redemptionAbort, delegationAbort, book, operationConnectionId, retireOperationsForConnection, setAuthority, setMemberRows, setInvite, setRedemption, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, persist });
  const { signOut, refreshSpaces, createSpace, archiveSpace, createInvite, copyInvite, dismissInvite, redeemInvite, refreshDelegations, createDelegation, downloadAgentCredential, dismissAgentCredential, installAgentMcpClient, copyAgentMcpClientConfig } = useHubSignInAndAgentCallbacks({ captureOperationOwner, signOutAbort, authorityAbort, membersAbort, setAuthority, setMemberRows, dispatchSession, portRef, operationOwnerCurrent, setRows, setInvite, setRedemption, setSpacesPhase, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, runCommand, setSearch, connectionRef, invite, redemptionAbort, loadSpaces, loadDelegations, watchedSpaceIdRef, delegationAbort, agentCredential });
  const { members, visibleRows, delegations, revokeDelegation } = useHubDelegationCallbacks({ captureOperationOwner, watchedSpaceIdRef, setDelegationError, setDelegationPhase, delegationAbort, portRef, operationOwnerCurrent, setAgentCredential, loadDelegations, rows, search, memberRows, onlineUserIds, delegationSummaries });
  return {
    book,
    connection,
    session,
    authority,
    members,
    rows: visibleRows,
    spacesPhase,
    search,
    invite,
    redemption,
    delegations,
    delegationPhase,
    delegationError,
    agentCredential,
    selectConnection,
    addRemoteHub,
    forgetHub,
    signIn,
    cancelSignIn,
    signOut,
    refreshSpaces,
    watchSpaceMembers,
    setSearch,
    createSpace,
    archiveSpace,
    createInvite,
    copyInvite,
    dismissInvite,
    redeemInvite,
    refreshDelegations,
    createDelegation,
    downloadAgentCredential,
    dismissAgentCredential,
    installAgentMcpClient,
    copyAgentMcpClientConfig,
    revokeDelegation,
  };
}

//#endregion 🔖️Hook