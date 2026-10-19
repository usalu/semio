/** 🔑️ The hub sign out, agent client configuration and delegation callbacks of the hub connection hook. */
import { useCallback, useMemo } from "react";
import type { DirectoryCommand, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "@semio-tech/framework/directory/schema";
import { runHubSignOutV1 } from "@semio-tech/framework/directory/sign-in";
import { archiveSpaceCommandV1, createInviteCommandV1, createSpaceCommandV1, filterSpaceRowsV1, inviteRedemptionErrorFromStatusV1, parseInviteTokenV1, spaceMemberPresenceV1 } from "@semio-tech/framework/directory/spaces";
import { agentCredentialCommandV1, agentCredentialFileV1, agentDelegationRevokePathV1, agentDelegationRowsV1, createAgentDelegationBodyV1, agentCredentialInstallRequestV1, agentMcpClientConfigJsonV1, agentMcpClientConfigV1, AgentCredentialInstallUnavailableV1, type AgentAudienceV1 } from "@semio-tech/framework/directory/delegations";
import { AgentDelegationRefusalV1, type HubAgentCredentialV1, type HubAgentMcpClientV1 } from "../🔌️connection-contract/🟦️.ts";
import type { useHubCommandCallbacks, useHubConnectionBook } from "../📒️connection-book-and-commands/🟦️.tsx";

/** 🚪 Builds the sign out, agent client configuration and clipboard callbacks. */
export function useHubSignInAndAgentCallbacks({ captureOperationOwner, signOutAbort, authorityAbort, membersAbort, setAuthority, setMemberRows, dispatchSession, portRef, operationOwnerCurrent, setRows, setInvite, setRedemption, setSpacesPhase, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, runCommand, setSearch, connectionRef, invite, redemptionAbort, loadSpaces, loadDelegations, watchedSpaceIdRef, delegationAbort, agentCredential }: Pick<ReturnType<typeof useHubConnectionBook>, "captureOperationOwner" | "signOutAbort" | "authorityAbort" | "membersAbort" | "setAuthority" | "setMemberRows" | "dispatchSession" | "portRef" | "operationOwnerCurrent" | "setRows" | "setInvite" | "setRedemption" | "setSpacesPhase" | "setAgentCredential" | "setDelegationSummaries" | "setDelegationPhase" | "setDelegationError" | "setSearch" | "connectionRef" | "invite" | "redemptionAbort" | "loadSpaces" | "loadDelegations" | "watchedSpaceIdRef" | "delegationAbort" | "agentCredential"> & Pick<ReturnType<typeof useHubCommandCallbacks>, "runCommand">) {
  const signOut = useCallback(() => {
    const owner = captureOperationOwner();
    signOutAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    setAuthority(null);
    setMemberRows([]);
    dispatchSession({ kind: "sign-out" });
    const abort = new AbortController();
    signOutAbort.current = abort;
    void runHubSignOutV1(portRef.current.signIn, owner.origin, abort.signal).then(() => {
      if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
      dispatchSession({ kind: "signed-out" });
      setRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  const createSpace = useCallback((name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) => {
    let command: DirectoryCommand;
    try {
      command = createSpaceCommandV1(name, kind, visibility);
    } catch {
      setSpacesPhase("failed");
      return;
    }
    runCommand(command, () => setSearch(""));
  }, [runCommand]);

  const archiveSpace = useCallback((spaceId: string) => {
    try {
      runCommand(archiveSpaceCommandV1(spaceId), () => undefined);
    } catch {
      setSpacesPhase("failed");
    }
  }, [runCommand]);

  const createInvite = useCallback((spaceId: string, role: DirectorySpaceRole, ttlSecs: number) => {
    let command: DirectoryCommand;
    try {
      command = createInviteCommandV1(spaceId, role, ttlSecs);
    } catch {
      setSpacesPhase("failed");
      return;
    }
    const origin = connectionRef.current.origin;
    runCommand(command, (receipt) => {
      if (receipt.result.kind !== "invite") {
        setSpacesPhase("failed");
        return;
      }
      setInvite({ spaceId, link: `${origin}/#semio-invite=${receipt.result.inviteToken}`, copy: "idle" });
    });
  }, [connectionRef, runCommand]);

  const copyInvite = useCallback(() => {
    const owner = captureOperationOwner();
    const current = invite;
    const write = portRef.current.writeClipboard;
    if (current === null || write === undefined) {
      setInvite((value) => (value === null ? value : { ...value, copy: "failed" }));
      return;
    }
    void write(current.link)
      .then(() => { if (operationOwnerCurrent(owner)) setInvite((value) => (value === null ? value : { ...value, copy: "copied" })); })
      .catch(() => { if (operationOwnerCurrent(owner)) setInvite((value) => (value === null ? value : { ...value, copy: "failed" })); });
  }, [captureOperationOwner, invite, operationOwnerCurrent, portRef]);

  const dismissInvite = useCallback(() => setInvite(null), []);

  const redeemInvite = useCallback((typed: string) => {
    const owner = captureOperationOwner();
    let token: string;
    try {
      token = parseInviteTokenV1(typed);
    } catch {
      setRedemption({ phase: "failed", error: "invalid-invite" });
      return;
    }
    redemptionAbort.current?.abort();
    setRedemption({ phase: "redeeming", error: null });
    const abort = new AbortController();
    redemptionAbort.current = abort;
    void portRef.current
      .redeemInvite(owner.origin, token, abort.signal)
      .then((response) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        if (response.status >= 200 && response.status < 300) {
          setRedemption({ phase: "redeemed", error: null });
          loadSpaces(owner);
          return;
        }
        setRedemption({ phase: "failed", error: inviteRedemptionErrorFromStatusV1(response.status) });
      })
      .catch(() => { if (operationOwnerCurrent(owner)) setRedemption({ phase: "failed", error: abort.signal.aborted ? "cancelled" : "unreachable" }); });
  }, [captureOperationOwner, loadSpaces, operationOwnerCurrent, portRef]);

  const refreshSpaces = useCallback(() => loadSpaces(), [loadSpaces]);

  const refreshDelegations = useCallback(() => loadDelegations(watchedSpaceIdRef.current), [loadDelegations, watchedSpaceIdRef]);

  /** 🤖️ Mints one delegation. The receipt is turned into a credential file HERE, once, and the raw
   * token is never written anywhere else: the pane only ever sees the rendered file. */
  const createDelegation = useCallback((agentLabel: string, audience: AgentAudienceV1, ttlSecs: number) => {
    const owner = captureOperationOwner();
    const spaceId = watchedSpaceIdRef.current;
    if (spaceId === null) {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    let body: string;
    try {
      body = createAgentDelegationBodyV1(spaceId, agentLabel, audience, ttlSecs);
    } catch {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    delegationAbort.current?.abort();
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase("submitting");
    setDelegationError(null);
    void portRef.current
      .createAgentDelegation(owner.origin, body, abort.signal)
      .then((receipt) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setAgentCredential({ receipt, file: agentCredentialFileV1(receipt, owner.origin), command: agentCredentialCommandV1(receipt, owner.origin), save: "idle", mcpClient: { phase: portRef.current.installAgentCredential === undefined ? "unavailable" : "idle", config: null } });
        loadDelegations(spaceId, owner);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, loadDelegations, operationOwnerCurrent, portRef, watchedSpaceIdRef]);

  /** 📄️ Saves the credential file. Like the install below, the outcome belongs to the delegation it was asked for. */
  const downloadAgentCredential = useCallback(() => {
    const current = agentCredential;
    const save = portRef.current.saveFile;
    if (current === null || save === undefined) {
      setAgentCredential((value) => (value === null ? value : { ...value, save: "failed" }));
      return;
    }
    const saved = (outcome: HubAgentCredentialV1["save"]) => setAgentCredential((value) => (value === null || value.receipt.delegationId !== current.receipt.delegationId ? value : { ...value, save: outcome }));
    void save(current.file).then(() => saved("saved"), () => saved("failed"));
  }, [agentCredential, portRef]);

  const dismissAgentCredential = useCallback(() => setAgentCredential(null), []);

  /** 🔌️ Installs the credential for MCP clients. The install is owned by the delegation it writes, not by the connection
   * generation: the file is on disk the moment the host answers, so the pane must say so while that delegation's
   * credential is shown — dropping a late answer left the button disabled at `installing` for good although the file was
   * written (G10 S4 relay, 2 of 3 runs on hub 7800). A credential dismissed, withdrawn or replaced meanwhile ignores it. */
  const installAgentMcpClient = useCallback(() => {
    const current = agentCredential;
    const install = portRef.current.installAgentCredential;
    if (current === null) return;
    const origin = connectionRef.current.origin;
    const mcpClient = (next: HubAgentMcpClientV1) => setAgentCredential((value) => (value === null || value.receipt.delegationId !== current.receipt.delegationId ? value : { ...value, mcpClient: next }));
    if (install === undefined) {
      mcpClient({ phase: "unavailable", config: null });
      return;
    }
    mcpClient({ phase: "installing", config: null });
    void install(agentCredentialInstallRequestV1(current.receipt, origin), new AbortController().signal).then(
      (installed) => mcpClient({ phase: "ready", config: agentMcpClientConfigJsonV1(agentMcpClientConfigV1(current.receipt, origin, installed)) }),
      (error: unknown) => mcpClient({ phase: error instanceof AgentCredentialInstallUnavailableV1 ? "unavailable" : "failed", config: null }),
    );
  }, [agentCredential, connectionRef, portRef]);

  const copyAgentMcpClientConfig = useCallback(() => {
    const config = agentCredential?.mcpClient.config ?? null;
    const write = portRef.current.writeClipboard;
    if (config === null || write === undefined) return;
    const delegationId = agentCredential?.receipt.delegationId;
    void write(config).then(() => setAgentCredential((value) => (value === null || value.receipt.delegationId !== delegationId ? value : { ...value, mcpClient: { phase: "copied", config } })));
  }, [agentCredential, portRef]);
  return { signOut, refreshSpaces, createSpace, archiveSpace, createInvite, copyInvite, dismissInvite, redeemInvite, refreshDelegations, createDelegation, downloadAgentCredential, dismissAgentCredential, installAgentMcpClient, copyAgentMcpClientConfig };
}

/** 🤝 Builds the delegation revocation callback and the delegation rows. */
export function useHubDelegationCallbacks({ captureOperationOwner, watchedSpaceIdRef, setDelegationError, setDelegationPhase, delegationAbort, portRef, operationOwnerCurrent, setAgentCredential, loadDelegations, rows, search, memberRows, onlineUserIds, delegationSummaries }: Pick<ReturnType<typeof useHubConnectionBook>, "captureOperationOwner" | "watchedSpaceIdRef" | "setDelegationError" | "setDelegationPhase" | "delegationAbort" | "portRef" | "operationOwnerCurrent" | "setAgentCredential" | "loadDelegations" | "rows" | "search" | "memberRows" | "delegationSummaries"> & { readonly onlineUserIds: readonly string[] }) {
  const revokeDelegation = useCallback((delegationId: string) => {
    const owner = captureOperationOwner();
    const spaceId = watchedSpaceIdRef.current;
    try {
      agentDelegationRevokePathV1(delegationId);
    } catch {
      setDelegationError("malformed-request");
      setDelegationPhase("failed");
      return;
    }
    delegationAbort.current?.abort();
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase("submitting");
    setDelegationError(null);
    void portRef.current
      .revokeAgentDelegation(owner.origin, delegationId, abort.signal)
      .then(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setAgentCredential((value) => (value === null || value.receipt.delegationId !== delegationId ? value : null));
        void portRef.current.uninstallAgentCredential?.(delegationId).catch(() => undefined);
        loadDelegations(spaceId, owner);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, loadDelegations, operationOwnerCurrent, portRef, watchedSpaceIdRef]);

  const visibleRows = useMemo(() => filterSpaceRowsV1(rows, search), [rows, search]);
  const members = useMemo(() => spaceMemberPresenceV1(memberRows, onlineUserIds), [memberRows, onlineUserIds]);
  const delegations = useMemo(() => agentDelegationRowsV1(delegationSummaries, Date.now()), [delegationSummaries]);
  return { members, visibleRows, delegations, revokeDelegation };
}
