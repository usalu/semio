// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🤖️AgentDelegations/component.tsx
/** 🤖️ `🤖️AgentDelegations` — the human-facing half of M6's delegated agent credential: give an
 * AI agent its own scoped, revocable access to the space that is open, download its credential file
 * exactly once, see what is outstanding and when each delegation was last used, and withdraw one
 * behind a confirmation. Purely presentational: every mutation leaves as an intent callback and
 * `useHubConnection` turns it into one of the hub's `/auth/agent-delegations` routes.
 *
 * The one-time disclosure is the whole point of the pane's shape. A delegation token is readable
 * exactly once, at creation, and no route can ever read it back; so the credential block is rendered
 * only while the receipt is in memory, and dismissing it destroys the only copy. Ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M6b, spec `📓️m6-agent-principal-in-hub-space.md`.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useId, useState, type FormEvent, type ReactElement } from "../⚛️react-port/🟦️.ts";
import { Button, Input, SharedBorderRow, SharedBorderRowCell, cn, useLabel } from "@semio-tech/ui-react";
import {
  AGENT_DELEGATION_TTL_CHOICES_SECS_V1,
  agentDelegationRevocableV1,
  isAgentAudienceV1,
  type AgentAudienceV1,
  type AgentDelegationErrorCodeV1,
  type AgentDelegationPhaseV1,
  type AgentDelegationRowV1
} from "@semio-tech/framework/directory/delegations";
import { HubFieldRow, HubRowItem, HubRowList, HubSelectField } from "../🔗️HubConnection/🪢️hub-rows/🟦️.tsx";
import { hubUiLabel } from "../🔗️HubConnection/🌐️ui-labels/🟦️.ts";
import { type HubAgentCredentialV1, type HubAgentMcpClientV1 } from "../🔗️HubConnection/🔌️connection-contract/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Types
export interface AgentDelegationsProps {
  /** 🏠️ The space whose agents these are, or `null` when no space is open. */
  readonly spaceId: string | null;
  readonly rows: readonly AgentDelegationRowV1[];
  readonly phase: AgentDelegationPhaseV1;
  readonly error: AgentDelegationErrorCodeV1 | null;
  readonly credential: HubAgentCredentialV1 | null;
  readonly signedIn: boolean;
  /** ✍️ Whether the caller is an author of this space. The hub refuses a spectator's delegation with
   * `403`; showing the form to one anyway would be a lie the hub has to correct. */
  readonly canDelegate: boolean;
  readonly onRefresh: () => void;
  readonly onCreate: (agentLabel: string, audience: AgentAudienceV1, ttlSecs: number) => void;
  readonly onDownloadCredential: () => void;
  readonly onDismissCredential: () => void;
  /** 🔌️ Installs the credential for MCP clients and yields the client configuration. */
  readonly onInstallMcpClient: () => void;
  readonly onCopyMcpClientConfig: () => void;
  readonly onRevoke: (delegationId: string) => void;
}
//#endregion 🔖️Types

//#region 🔖️Time
/** 🕰️ A locale-independent timestamp (`YYYY-MM-DD HH:MM` in UTC). Deliberately not `toLocaleString`:
 * two devices rendering the same delegation must agree byte for byte, and a delegation's deadline is
 * an authority fact rather than a local one. */
export function agentDelegationInstantV1(milliseconds: number): string {
  if (!Number.isSafeInteger(milliseconds)) return "—";
  const iso = new Date(milliseconds).toISOString();
  return `${iso.slice(0, 10)} ${iso.slice(11, 16)} UTC`;
}
//#endregion 🔖️Time

//#region 🔖️Create
function CreateDelegationForm({ disabled, onCreate }: { readonly disabled: boolean; readonly onCreate: AgentDelegationsProps["onCreate"] }): ReactElement {
  const id = useId();
  const [name, setName] = useState("");
  const [audience, setAudience] = useState<AgentAudienceV1>("edit");
  const [ttlSecs, setTtlSecs] = useState<number>(AGENT_DELEGATION_TTL_CHOICES_SECS_V1[1]);
  const createTitleLabel = useLabel(hubUiLabel("os.hub.agent.createTitle"));
  const nameLabel = useLabel(hubUiLabel("os.hub.agent.name"));
  const audienceLabel = useLabel(hubUiLabel("os.hub.agent.audience"));
  const readLabel = useLabel(hubUiLabel("os.hub.agent.audienceRead"));
  const editLabel = useLabel(hubUiLabel("os.hub.agent.audienceEdit"));
  const expiryLabel = useLabel(hubUiLabel("os.hub.agent.expiry"));
  const dayLabel = useLabel(hubUiLabel("os.hub.agent.expiryDay"));
  const weekLabel = useLabel(hubUiLabel("os.hub.agent.expiryWeek"));
  const monthLabel = useLabel(hubUiLabel("os.hub.agent.expiryMonth"));
  const createLabel = useLabel(hubUiLabel("os.hub.agent.create"));
  const valid = name.trim().length > 0 && !/\p{Cc}/u.test(name);

  const submit = (event: FormEvent): void => {
    event.preventDefault();
    if (!disabled && valid) {
      onCreate(name, audience, ttlSecs);
      setName("");
    }
  };

  return (
    <SharedBorderRow as="form" orientation="vertical" aria-label={createTitleLabel} onSubmit={submit}>
      <SharedBorderRowCell><h3 className="text-sm font-medium">{createTitleLabel}</h3></SharedBorderRowCell>
        <HubFieldRow htmlFor={`${id}-name`} label={nameLabel}>
          <Input id={`${id}-name`} data-element-alias="os.hub.agent.name" name="agentName" required value={name} disabled={disabled} onChange={(event) => setName(event.target.value)} />
        </HubFieldRow>
        <HubSelectField id={`${id}-audience`} alias="os.hub.agent.audience" label={audienceLabel} value={audience} disabled={disabled} options={[{ value: "read", label: readLabel }, { value: "edit", label: editLabel }]} onValueChange={(next) => setAudience(isAgentAudienceV1(next) ? next : "read")} />
        <HubSelectField id={`${id}-ttl`} alias="os.hub.agent.expiry" label={expiryLabel} value={String(ttlSecs)} disabled={disabled} options={[{ value: String(AGENT_DELEGATION_TTL_CHOICES_SECS_V1[0]), label: dayLabel }, { value: String(AGENT_DELEGATION_TTL_CHOICES_SECS_V1[1]), label: weekLabel }, { value: String(AGENT_DELEGATION_TTL_CHOICES_SECS_V1[2]), label: monthLabel }]} onValueChange={(next) => setTtlSecs(Number.parseInt(next, 10))} />
        <Button id="os.hub.agent.create" icon="cpu" type="submit" variant="outline" aria-label={createLabel} disabled={disabled || !valid}>{createLabel}</Button>
    </SharedBorderRow>
  );
}
//#endregion 🔖️Create

//#region 🔖️Credential
/** 🎁️ The one-time disclosure. A live region, because it appears without the human moving focus, and
 * the download control comes first: everything else on the block is explanation. */
function CredentialBlock({ credential, onDownload, onDismiss, onInstallMcpClient, onCopyMcpClientConfig }: {
  readonly credential: HubAgentCredentialV1;
  readonly onDownload: AgentDelegationsProps["onDownloadCredential"];
  readonly onDismiss: AgentDelegationsProps["onDismissCredential"];
  readonly onInstallMcpClient: AgentDelegationsProps["onInstallMcpClient"];
  readonly onCopyMcpClientConfig: AgentDelegationsProps["onCopyMcpClientConfig"];
}): ReactElement {
  const readyLabel = useLabel(hubUiLabel("os.hub.agent.ready"));
  const downloadLabel = useLabel(hubUiLabel("os.hub.agent.download"));
  const downloadedLabel = useLabel(hubUiLabel("os.hub.agent.downloaded"));
  const downloadFailedLabel = useLabel(hubUiLabel("os.hub.agent.downloadFailed"));
  const dismissLabel = useLabel(hubUiLabel("os.hub.agent.dismiss"));
  const permissionLabel = useLabel(hubUiLabel("os.hub.agent.permission"));
  const commandLabel = useLabel(hubUiLabel("os.hub.agent.command"));
  const principalLabel = useLabel(hubUiLabel("os.hub.agent.principal"), { principal: credential.receipt.agentPrincipalId });
  return (
    <SharedBorderRow orientation="vertical" role="status" aria-live="polite" data-semio-hub-agent-credential={credential.receipt.delegationId}>
      <SharedBorderRowCell><p className="text-xs text-muted-foreground">{readyLabel}</p></SharedBorderRowCell>
      <SharedBorderRowCell><p data-semio-hub-agent-principal={credential.receipt.agentPrincipalId} className="break-all text-xs">{principalLabel}</p></SharedBorderRowCell>
      <SharedBorderRow>
        <Button id="os.hub.agent.download" icon="download" type="button" variant="outline" aria-label={downloadLabel} className="flex-1 min-w-0" onClick={onDownload}>{downloadLabel}</Button>
        <Button icon="x" type="button" variant="outline" aria-label={dismissLabel} className="flex-1 min-w-0" onClick={onDismiss}>{dismissLabel}</Button>
      </SharedBorderRow>
      <SharedBorderRowCell><p className="text-xs text-muted-foreground">{permissionLabel}</p></SharedBorderRowCell>
      <SharedBorderRowCell><p className="text-xs text-muted-foreground">{commandLabel}</p></SharedBorderRowCell>
      <SharedBorderRowCell><code data-semio-hub-agent-command className="block min-w-0 break-all bg-muted text-xs">{credential.command}</code></SharedBorderRowCell>
      <McpClientBlock mcpClient={credential.mcpClient} onInstall={onInstallMcpClient} onCopy={onCopyMcpClientConfig} />
      {credential.save === "saved" ? <SharedBorderRowCell><p className="text-xs">{downloadedLabel}</p></SharedBorderRowCell> : null}
      {credential.save === "failed" ? (
        <>
          <SharedBorderRowCell><p role="alert" className="text-xs text-destructive">{downloadFailedLabel}</p></SharedBorderRowCell>
          <SharedBorderRowCell><code data-semio-hub-agent-credential-file={credential.file.fileName} className="block max-h-48 min-w-0 overflow-auto break-all bg-muted text-xs">{credential.file.contents}</code></SharedBorderRowCell>
        </>
      ) : null}
    </SharedBorderRow>
  );
}

/** 🔌️ Turns the delegation into a working MCP client: one control installs the credential owner-only
 * where `semio-os-mcp` reads it, then the exact configuration entry is shown with a copy control.
 * The configuration names the credential file, never the secret, so copying it is safe. A host that
 * cannot install files says so and points back at the download and the command above. */
function McpClientBlock({ mcpClient, onInstall, onCopy }: {
  readonly mcpClient: HubAgentMcpClientV1;
  readonly onInstall: AgentDelegationsProps["onInstallMcpClient"];
  readonly onCopy: AgentDelegationsProps["onCopyMcpClientConfig"];
}): ReactElement {
  const id = useId();
  const titleLabel = useLabel(hubUiLabel("os.hub.agent.mcpTitle"));
  const installLabel = useLabel(hubUiLabel("os.hub.agent.mcpInstall"));
  const installingLabel = useLabel(hubUiLabel("os.hub.agent.mcpInstalling"));
  const readyLabel = useLabel(hubUiLabel("os.hub.agent.mcpReady"));
  const copyLabel = useLabel(hubUiLabel("os.hub.agent.mcpCopy"));
  const copiedLabel = useLabel(hubUiLabel("os.hub.agent.mcpCopied"));
  const unavailableLabel = useLabel(hubUiLabel("os.hub.agent.mcpUnavailable"));
  const failedLabel = useLabel(hubUiLabel("os.hub.agent.mcpFailed"));
  const revocableLabel = useLabel(hubUiLabel("os.hub.agent.mcpRevocable"));
  const statusText = mcpClient.phase === "installing" ? installingLabel : mcpClient.phase === "copied" ? copiedLabel : "";
  return (
    <SharedBorderRow orientation="vertical" role="group" aria-labelledby={`${id}-title`} data-semio-hub-agent-mcp={mcpClient.phase}>
      <SharedBorderRowCell><p id={`${id}-title`} className="text-sm font-medium">{titleLabel}</p></SharedBorderRowCell>
      {mcpClient.phase === "idle" || mcpClient.phase === "installing" ? (
        <Button id="os.hub.agent.mcpInstall" icon="link" type="button" variant="outline" aria-label={installLabel} disabled={mcpClient.phase === "installing"} onClick={onInstall}>{installLabel}</Button>
      ) : null}
      <p role="status" aria-live="polite" data-shared-border-cell={statusText === "" ? undefined : ""} className={cn("text-xs text-muted-foreground", statusText === "" ? "sr-only" : "px-single py-1")}>{statusText}</p>
      {mcpClient.phase === "unavailable" ? <SharedBorderRowCell><p className="text-xs text-muted-foreground">{unavailableLabel}</p></SharedBorderRowCell> : null}
      {mcpClient.phase === "failed" ? <SharedBorderRowCell><p role="alert" className="text-xs text-destructive">{failedLabel}</p></SharedBorderRowCell> : null}
      {mcpClient.config === null ? null : (
        <>
          <SharedBorderRowCell><p className="text-xs text-muted-foreground">{readyLabel}</p></SharedBorderRowCell>
          <SharedBorderRowCell><pre data-semio-hub-agent-mcp-config className="max-h-64 min-w-0 overflow-auto whitespace-pre-wrap break-all bg-muted text-xs">{mcpClient.config}</pre></SharedBorderRowCell>
          <Button id="os.hub.agent.mcpCopy" icon="copy" type="button" variant="outline" aria-label={copyLabel} onClick={onCopy}>{copyLabel}</Button>
          <SharedBorderRowCell><p className="text-xs text-muted-foreground">{revocableLabel}</p></SharedBorderRowCell>
        </>
      )}
    </SharedBorderRow>
  );
}
//#endregion 🔖️Credential

//#region 🔖️Row
/** 🤖️ One delegation row, and the two-step withdrawal that guards it. The confirmation is inline and
 * focusable rather than a `window.confirm`: a shell that owns its own focus cannot hand it to the
 * browser, and a screen reader must hear what withdrawal costs before the control that does it. */
function DelegationRow({ row, busy, onRevoke }: {
  readonly row: AgentDelegationRowV1;
  readonly busy: boolean;
  readonly onRevoke: AgentDelegationsProps["onRevoke"];
}): ReactElement {
  const id = useId();
  const [confirming, setConfirming] = useState(false);
  const revokeLabel = useLabel(hubUiLabel("os.hub.agent.revoke"), { name: row.agentLabel });
  const confirmTitleLabel = useLabel(hubUiLabel("os.hub.agent.revokeConfirmTitle"), { name: row.agentLabel });
  const confirmBodyLabel = useLabel(hubUiLabel("os.hub.agent.revokeConfirmBody"));
  const confirmLabel = useLabel(hubUiLabel("os.hub.agent.revokeConfirm"));
  const cancelLabel = useLabel(hubUiLabel("os.hub.agent.revokeCancel"));
  const readLabel = useLabel(hubUiLabel("os.hub.agent.audienceRead"));
  const editLabel = useLabel(hubUiLabel("os.hub.agent.audienceEdit"));
  const liveLabel = useLabel(hubUiLabel("os.hub.agent.stateLive"));
  const expiredLabel = useLabel(hubUiLabel("os.hub.agent.stateExpired"));
  const revokedLabel = useLabel(hubUiLabel("os.hub.agent.stateRevoked"));
  const createdLabel = useLabel(hubUiLabel("os.hub.agent.created"), { when: agentDelegationInstantV1(row.createdAtMs) });
  const expiresLabel = useLabel(hubUiLabel("os.hub.agent.expires"), { when: agentDelegationInstantV1(row.expiresAtMs) });
  const lastUsedLabel = useLabel(hubUiLabel("os.hub.agent.lastUsed"), { when: row.lastUsedAtMs === null ? "" : agentDelegationInstantV1(row.lastUsedAtMs) });
  const neverUsedLabel = useLabel(hubUiLabel("os.hub.agent.lastUsedNever"));
  const stateText = row.state === "live" ? liveLabel : row.state === "expired" ? expiredLabel : revokedLabel;
  const usageText = row.lastUsedAtMs === null ? neverUsedLabel : lastUsedLabel;

  return (
    <HubRowItem orientation="vertical" data-delegation-id={row.delegationId} data-delegation-state={row.state} data-delegation-audience={row.audience}>
      <SharedBorderRow>
        <SharedBorderRowCell className="min-w-0 flex-1 flex-col items-start justify-center">
          <span id={`${id}-name`} className="max-w-full truncate text-sm font-medium">{row.agentLabel}</span>
          <span className="text-xs text-muted-foreground">{`${stateText} · ${row.audience === "edit" ? editLabel : readLabel} · ${usageText}`}</span>
          <span data-delegation-principal={row.agentPrincipalId} className="break-all text-xs text-muted-foreground">{row.agentPrincipalId}</span>
          <span className="text-xs text-muted-foreground">{`${createdLabel} · ${expiresLabel}`}</span>
        </SharedBorderRowCell>
        {agentDelegationRevocableV1(row) && !confirming ? (
          <Button icon="lock" type="button" variant="outline" disabled={busy} aria-label={revokeLabel} onClick={() => setConfirming(true)}>{revokeLabel}</Button>
        ) : null}
      </SharedBorderRow>
      {confirming ? (
        <SharedBorderRow orientation="vertical" tone="destructive" role="group" aria-labelledby={`${id}-confirm-title`} aria-describedby={`${id}-confirm-body`} data-semio-hub-agent-revoke-confirm={row.delegationId}>
          <SharedBorderRowCell><p id={`${id}-confirm-title`} className="text-sm font-medium">{confirmTitleLabel}</p></SharedBorderRowCell>
          <SharedBorderRowCell><p id={`${id}-confirm-body`} className="text-xs text-muted-foreground">{confirmBodyLabel}</p></SharedBorderRowCell>
          <SharedBorderRow>
            <Button
              icon="lock"
              type="button"
              variant="outline"
              disabled={busy}
              className="flex-1 min-w-0"
              aria-label={`${confirmLabel} — ${confirmTitleLabel}`}
              onClick={() => {
                setConfirming(false);
                onRevoke(row.delegationId);
              }}
            >
              {confirmLabel}
            </Button>
            <Button icon="x" type="button" variant="outline" aria-label={cancelLabel} className="flex-1 min-w-0" onClick={() => setConfirming(false)}>{cancelLabel}</Button>
          </SharedBorderRow>
        </SharedBorderRow>
      ) : null}
    </HubRowItem>
  );
}
//#endregion 🔖️Row

//#region 🔖️Pane
/** 🤖️ The surface. A live status line for the load/mutate phase, the one-time credential block when
 * a delegation was just minted, the create form where the caller may author, and a `role="list"` of
 * every delegation this human holds in this space — revoked ones included, because "what did I
 * withdraw" is as much a question as "what is live". Single column at phone width throughout. */
export function AgentDelegations({
  spaceId,
  rows,
  phase,
  error,
  credential,
  signedIn,
  canDelegate,
  onRefresh,
  onCreate,
  onDownloadCredential,
  onDismissCredential,
  onInstallMcpClient,
  onCopyMcpClientConfig,
  onRevoke,
}: AgentDelegationsProps): ReactElement {
  const id = useId();
  const title = useLabel(hubUiLabel("os.hub.agent.title"));
  const description = useLabel(hubUiLabel("os.hub.agent.description"));
  const signedOutLabel = useLabel(hubUiLabel("os.hub.agent.signedOut"));
  const noSpaceLabel = useLabel(hubUiLabel("os.hub.agent.noSpace"));
  const notAuthorLabel = useLabel(hubUiLabel("os.hub.agent.notAuthor"));
  const listLabel = useLabel(hubUiLabel("os.hub.agent.list"));
  const emptyLabel = useLabel(hubUiLabel("os.hub.agent.empty"));
  const loadingLabel = useLabel(hubUiLabel("os.hub.agent.loading"));
  const submittingLabel = useLabel(hubUiLabel("os.hub.agent.submitting"));
  const failedLabel = useLabel(hubUiLabel("os.hub.agent.failed"));
  const refreshLabel = useLabel(hubUiLabel("os.hub.agent.refresh"));
  const errorForbidden = useLabel(hubUiLabel("os.hub.agent.errorForbidden"));
  const errorRateLimited = useLabel(hubUiLabel("os.hub.agent.errorRateLimited"));
  const errorUnreachable = useLabel(hubUiLabel("os.hub.agent.errorUnreachable"));
  const errorRefused = useLabel(hubUiLabel("os.hub.agent.errorRefused"));
  const busy = phase === "submitting";
  const statusText = phase === "loading" ? loadingLabel : phase === "submitting" ? submittingLabel : phase === "failed" ? failedLabel : "";
  const errorText = error === null
    ? null
    : error === "forbidden" || error === "invalid-delegation"
      ? errorForbidden
      : error === "rate-limited"
        ? errorRateLimited
        : error === "unreachable" || error === "directory-unavailable"
          ? errorUnreachable
          : errorRefused;

  return (
    <SharedBorderRow as="section" orientation="vertical" aria-labelledby={`${id}-title`} data-semio-hub-agent-phase={phase} data-semio-hub-agent-space={spaceId ?? ""} className="w-full">
      <SharedBorderRow as="header">
        <SharedBorderRowCell className="flex-1"><h2 id={`${id}-title`} className="text-base font-medium">{title}</h2></SharedBorderRowCell>
        <Button icon="rotate-cw" type="button" variant="outline" aria-label={refreshLabel} disabled={busy || spaceId === null} onClick={onRefresh}>{refreshLabel}</Button>
      </SharedBorderRow>
      <SharedBorderRowCell><p className="text-xs text-muted-foreground">{description}</p></SharedBorderRowCell>
      <p role="status" aria-live="polite" data-semio-hub-agent-status={phase} data-shared-border-cell={statusText === "" ? undefined : ""} className={cn("text-xs text-muted-foreground", statusText === "" ? "sr-only" : "px-single py-1")}>{statusText}</p>
      {errorText === null ? null : <SharedBorderRowCell tone="destructive"><p role="alert" data-semio-hub-agent-error={error ?? ""} className="text-xs text-destructive">{errorText}</p></SharedBorderRowCell>}

      {!signedIn ? <SharedBorderRowCell><p className="text-sm text-muted-foreground">{signedOutLabel}</p></SharedBorderRowCell> : spaceId === null ? <SharedBorderRowCell><p className="text-sm text-muted-foreground">{noSpaceLabel}</p></SharedBorderRowCell> : null}

      {credential === null ? null : <CredentialBlock credential={credential} onDownload={onDownloadCredential} onDismiss={onDismissCredential} onInstallMcpClient={onInstallMcpClient} onCopyMcpClientConfig={onCopyMcpClientConfig} />}

      {signedIn && spaceId !== null ? (canDelegate ? <CreateDelegationForm disabled={busy} onCreate={onCreate} /> : <SharedBorderRowCell><p className="text-sm text-muted-foreground">{notAuthorLabel}</p></SharedBorderRowCell>) : null}

      {signedIn && spaceId !== null ? (
        rows.length === 0 ? (
          <SharedBorderRowCell><p className="text-sm text-muted-foreground">{emptyLabel}</p></SharedBorderRowCell>
        ) : (
          <HubRowList data-element-alias="os.hub.agent.list" aria-label={listLabel}>
            {rows.map((row) => (
              <DelegationRow key={row.delegationId} row={row} busy={busy} onRevoke={onRevoke} />
            ))}
          </HubRowList>
        )
      ) : null}
    </SharedBorderRow>
  );
}
//#endregion 🔖️Pane
