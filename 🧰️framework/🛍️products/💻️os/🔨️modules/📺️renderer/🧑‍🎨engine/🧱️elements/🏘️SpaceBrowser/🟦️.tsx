// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🏘️SpaceBrowser/component.tsx
/** 🏘️ `🏘️SpaceBrowser` — the end-user spaces surface `🛂️SpaceAdministration` never was: list
 * the spaces I can reach, switch between them, create one, see who is a member and who is here right
 * now, issue an invitation, and redeem one someone sent me. Purely presentational: every mutation
 * leaves as an intent callback, and `useHubConnection` turns it into a closed `DirectoryCommand` on
 * the hub's command path (CQRS — this element never performs a CRUD write). Ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useId, useState, type FormEvent, type ReactElement } from "../⚛️react-port/🟦️.ts";
import { Button, Input, PresenceBar, SharedBorderRow, SharedBorderRowCell, useLabel, type PresencePeer } from "@semio-tech/ui-react";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "@semio-tech/framework/directory/schema";
import {
  INVITE_TTL_CHOICES_SECS_V1,
  spaceRowInvitableV1,
  spaceRowWritableV1,
  type SpaceBrowserPhaseV1,
  type SpaceMemberPresenceV1,
  type SpaceRowV1
} from "@semio-tech/framework/directory/spaces";
import { HubFieldRow, HubRowItem, HubRowList, HubSelectField } from "../🔗️HubConnection/🪢️hub-rows/🟦️.tsx";
import { hubUiLabel } from "../🔗️HubConnection/🌐️ui-labels/🟦️.ts";
import { type HubInviteCapabilityV1, type HubRedemptionStateV1 } from "../🔗️HubConnection/🔌️connection-contract/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Types
export interface SpaceBrowserProps {
  readonly rows: readonly SpaceRowV1[];
  readonly activeSpaceId: string | null;
  readonly phase: SpaceBrowserPhaseV1;
  readonly search: string;
  readonly members: readonly SpaceMemberPresenceV1[];
  readonly invite: HubInviteCapabilityV1 | null;
  readonly redemption: HubRedemptionStateV1;
  readonly signedIn: boolean;
  readonly onSearch: (query: string) => void;
  readonly onOpenSpace: (spaceId: string) => void;
  readonly onRefresh: () => void;
  readonly onCreateSpace: (name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) => void;
  readonly onArchiveSpace: (spaceId: string) => void;
  readonly onCreateInvite: (spaceId: string, role: DirectorySpaceRole, ttlSecs: number) => void;
  readonly onCopyInvite: () => void;
  readonly onDismissInvite: () => void;
  readonly onRedeemInvite: (typed: string) => void;
}
//#endregion 🔖️Types

//#region 🔖️Presence
/** 👥️ Projects the member roster onto the domain-neutral `PresenceBar` peer shape: only members who
 * are actually connected become peers, so the bar never claims presence it cannot observe. */
export function spaceBrowserPresencePeersV1(members: readonly SpaceMemberPresenceV1[]): readonly PresencePeer[] {
  return members.filter((member) => member.online).map((member, index) => ({ actor: `user:${member.userId}`, userId: member.userId, label: member.displayName, role: member.role, color: index }));
}
//#endregion 🔖️Presence

//#region 🔖️CreateSpace
function CreateSpaceForm({ disabled, onCreateSpace }: { readonly disabled: boolean; readonly onCreateSpace: SpaceBrowserProps["onCreateSpace"] }): ReactElement {
  const id = useId();
  const [name, setName] = useState("");
  const [kind, setKind] = useState<DirectorySpaceKind>("atelier");
  const [visibility, setVisibility] = useState<DirectorySpaceVisibility>("private");
  const titleLabel = useLabel(hubUiLabel("os.hub.spaces.createTitle"));
  const nameLabel = useLabel(hubUiLabel("os.hub.spaces.createName"));
  const kindLabel = useLabel(hubUiLabel("os.hub.spaces.createKind"));
  const visibilityLabel = useLabel(hubUiLabel("os.hub.spaces.createVisibility"));
  const atelierLabel = useLabel(hubUiLabel("os.hub.spaces.kindAtelier"));
  const studioLabel = useLabel(hubUiLabel("os.hub.spaces.kindStudio"));
  const privateLabel = useLabel(hubUiLabel("os.hub.spaces.visibilityPrivate"));
  const publicLabel = useLabel(hubUiLabel("os.hub.spaces.visibilityPublic"));
  const submitLabel = useLabel(hubUiLabel("os.hub.spaces.createSubmit"));
  const valid = name.trim().length > 0 && !/\p{Cc}/u.test(name);

  const submit = (event: FormEvent): void => {
    event.preventDefault();
    if (!disabled && valid) {
      onCreateSpace(name, kind, visibility);
      setName("");
    }
  };

  return (
    <SharedBorderRow as="form" orientation="vertical" aria-label={titleLabel} onSubmit={submit}>
      <SharedBorderRowCell><h3 className="text-sm font-medium">{titleLabel}</h3></SharedBorderRowCell>
        <HubFieldRow htmlFor={`${id}-name`} label={nameLabel}>
          <Input id={`${id}-name`} data-element-alias="os.hub.spaces.createName" name="spaceName" required value={name} disabled={disabled} onChange={(event) => setName(event.target.value)} />
        </HubFieldRow>
        <HubSelectField id={`${id}-kind`} label={kindLabel} value={kind} disabled={disabled} options={[{ value: "atelier", label: atelierLabel }, { value: "studio", label: studioLabel }]} onValueChange={(next) => setKind(next === "studio" ? "studio" : "atelier")} />
        <HubSelectField id={`${id}-visibility`} label={visibilityLabel} value={visibility} disabled={disabled} options={[{ value: "private", label: privateLabel }, { value: "public", label: publicLabel }]} onValueChange={(next) => setVisibility(next === "public" ? "public" : "private")} />
        <Button id="os.hub.spaces.createSubmit" icon="plus" type="submit" variant="outline" aria-label={submitLabel} disabled={disabled || !valid}>{submitLabel}</Button>
    </SharedBorderRow>
  );
}
//#endregion 🔖️CreateSpace

//#region 🔖️Invite
function InvitePanel({ row, invite, redemption, disabled, onCreateInvite, onCopyInvite, onDismissInvite, onRedeemInvite }: {
  readonly row: SpaceRowV1 | null;
  readonly invite: SpaceBrowserProps["invite"];
  readonly redemption: HubRedemptionStateV1;
  readonly disabled: boolean;
  readonly onCreateInvite: SpaceBrowserProps["onCreateInvite"];
  readonly onCopyInvite: SpaceBrowserProps["onCopyInvite"];
  readonly onDismissInvite: SpaceBrowserProps["onDismissInvite"];
  readonly onRedeemInvite: SpaceBrowserProps["onRedeemInvite"];
}): ReactElement {
  const id = useId();
  const [role, setRole] = useState<DirectorySpaceRole>("spectator");
  const [ttlSecs, setTtlSecs] = useState<number>(INVITE_TTL_CHOICES_SECS_V1[1]);
  const [typed, setTyped] = useState("");
  const titleLabel = useLabel(hubUiLabel("os.hub.invite.title"));
  const createTitleLabel = useLabel(hubUiLabel("os.hub.invite.createTitle"));
  const roleLabel = useLabel(hubUiLabel("os.hub.invite.role"));
  const expiryLabel = useLabel(hubUiLabel("os.hub.invite.expiry"));
  const hourLabel = useLabel(hubUiLabel("os.hub.invite.expiryHour"));
  const dayLabel = useLabel(hubUiLabel("os.hub.invite.expiryDay"));
  const weekLabel = useLabel(hubUiLabel("os.hub.invite.expiryWeek"));
  const createLabel = useLabel(hubUiLabel("os.hub.invite.create"));
  const readyLabel = useLabel(hubUiLabel("os.hub.invite.ready"));
  const copyLabel = useLabel(hubUiLabel("os.hub.invite.copy"));
  const copiedLabel = useLabel(hubUiLabel("os.hub.invite.copied"));
  const copyFailedLabel = useLabel(hubUiLabel("os.hub.invite.copyFailed"));
  const dismissLabel = useLabel(hubUiLabel("os.hub.invite.dismiss"));
  const redeemTitleLabel = useLabel(hubUiLabel("os.hub.invite.redeemTitle"));
  const redeemFieldLabel = useLabel(hubUiLabel("os.hub.invite.redeemField"));
  const redeemLabel = useLabel(hubUiLabel("os.hub.invite.redeem"));
  const redeemingLabel = useLabel(hubUiLabel("os.hub.invite.redeeming"));
  const redeemedLabel = useLabel(hubUiLabel("os.hub.invite.redeemed"));
  const authorLabel = useLabel(hubUiLabel("os.hub.spaces.roleAuthor"));
  const spectatorLabel = useLabel(hubUiLabel("os.hub.spaces.roleSpectator"));
  const errorInvalid = useLabel(hubUiLabel("os.hub.invite.errorInvalid"));
  const errorExpired = useLabel(hubUiLabel("os.hub.invite.errorExpired"));
  const errorAlready = useLabel(hubUiLabel("os.hub.invite.errorAlreadyMember"));
  const errorUnauthorized = useLabel(hubUiLabel("os.hub.invite.errorUnauthorized"));
  const errorUnreachable = useLabel(hubUiLabel("os.hub.invite.errorUnreachable"));
  const errorRefused = useLabel(hubUiLabel("os.hub.invite.errorRefused"));
  const errorCancelled = useLabel(hubUiLabel("os.hub.invite.errorCancelled"));

  const redemptionError = redemption.error === null
    ? null
    : redemption.error === "invalid-invite"
      ? errorInvalid
      : redemption.error === "expired-invite"
        ? errorExpired
        : redemption.error === "already-member"
          ? errorAlready
          : redemption.error === "unauthorized"
            ? errorUnauthorized
            : redemption.error === "unreachable"
              ? errorUnreachable
              : redemption.error === "cancelled"
                ? errorCancelled
                : errorRefused;

  return (
    <SharedBorderRow as="section" orientation="vertical" aria-label={titleLabel}>
      <SharedBorderRowCell><h3 className="text-sm font-medium">{titleLabel}</h3></SharedBorderRowCell>

      {row === null || !spaceRowInvitableV1(row) ? null : (
        <SharedBorderRow
          as="form"
          orientation="vertical"
          aria-label={createTitleLabel}
          onSubmit={(event) => {
            event.preventDefault();
            if (!disabled) onCreateInvite(row.id, role, ttlSecs);
          }}
        >
            <HubSelectField id={`${id}-role`} alias="os.hub.invite.role" label={roleLabel} value={role} disabled={disabled} options={[{ value: "spectator", label: spectatorLabel }, { value: "author", label: authorLabel }]} onValueChange={(next) => setRole(next === "author" ? "author" : "spectator")} />
            <HubSelectField id={`${id}-ttl`} alias="os.hub.invite.expiry" label={expiryLabel} value={String(ttlSecs)} disabled={disabled} options={[{ value: String(INVITE_TTL_CHOICES_SECS_V1[0]), label: hourLabel }, { value: String(INVITE_TTL_CHOICES_SECS_V1[1]), label: dayLabel }, { value: String(INVITE_TTL_CHOICES_SECS_V1[2]), label: weekLabel }]} onValueChange={(next) => setTtlSecs(Number.parseInt(next, 10))} />
            <Button id="os.hub.invite.create" icon="users" type="submit" variant="outline" aria-label={createLabel} disabled={disabled}>{createLabel}</Button>
        </SharedBorderRow>
      )}

      {invite === null ? null : (
        <SharedBorderRow orientation="vertical" role="status" aria-live="polite" data-semio-hub-invite={invite.spaceId}>
          <SharedBorderRowCell><p className="text-xs text-muted-foreground">{readyLabel}</p></SharedBorderRowCell>
          <SharedBorderRowCell><output htmlFor={`${id}-invite`} className="break-all text-xs">{invite.link}</output></SharedBorderRowCell>
          <SharedBorderRow>
            <Button id="os.hub.invite.copy" icon="copy" type="button" variant="outline" aria-label={copyLabel} className="flex-1 min-w-0" onClick={onCopyInvite}>{copyLabel}</Button>
            <Button icon="x" type="button" variant="outline" aria-label={dismissLabel} className="flex-1 min-w-0" onClick={onDismissInvite}>{dismissLabel}</Button>
          </SharedBorderRow>
          {invite.copy === "copied" ? <SharedBorderRowCell><p className="text-xs">{copiedLabel}</p></SharedBorderRowCell> : null}
          {invite.copy === "failed" ? <SharedBorderRowCell><p role="alert" className="text-xs text-destructive">{copyFailedLabel}</p></SharedBorderRowCell> : null}
        </SharedBorderRow>
      )}

      <SharedBorderRow
        as="form"
        orientation="vertical"
        aria-label={redeemTitleLabel}
        onSubmit={(event) => {
          event.preventDefault();
          if (redemption.phase !== "redeeming") onRedeemInvite(typed);
        }}
      >
          <HubFieldRow htmlFor={`${id}-redeem`} label={redeemFieldLabel}>
            <Input
              id={`${id}-redeem`}
              data-element-alias="os.hub.invite.redeemField"
              name="invitation"
              required
              value={typed}
              disabled={redemption.phase === "redeeming"}
              aria-invalid={redemption.error !== null}
              aria-describedby={redemption.error === null ? undefined : `${id}-redeem-error`}
              onChange={(event) => setTyped(event.target.value)}
            />
          </HubFieldRow>
          <Button icon="unlock" type="submit" variant="outline" aria-busy={redemption.phase === "redeeming"} aria-label={redemption.phase === "redeeming" ? redeemingLabel : redeemLabel} disabled={redemption.phase === "redeeming" || typed.trim().length === 0}>
            {redemption.phase === "redeeming" ? redeemingLabel : redeemLabel}
          </Button>
        {redemption.phase === "redeemed" ? <SharedBorderRowCell><p role="status" aria-live="polite" className="text-xs">{redeemedLabel}</p></SharedBorderRowCell> : null}
        {redemptionError === null ? null : <SharedBorderRowCell tone="destructive"><p id={`${id}-redeem-error`} role="alert" data-semio-hub-redemption-error={redemption.error ?? ""} className="text-xs text-destructive">{redemptionError}</p></SharedBorderRowCell>}
      </SharedBorderRow>
    </SharedBorderRow>
  );
}
//#endregion 🔖️Invite

//#region 🔖️Row
/** 🏠️ One space row: an open button carrying the whole accessible name (space, role, and whether it
 * is the one currently open) plus an archive control only where the caller may write. */
function SpaceRow({ row, active, busy, roleText, currentLabel, onOpenSpace, onArchiveSpace }: {
  readonly row: SpaceRowV1;
  readonly active: boolean;
  readonly busy: boolean;
  readonly roleText: string;
  readonly currentLabel: string;
  readonly onOpenSpace: SpaceBrowserProps["onOpenSpace"];
  readonly onArchiveSpace: SpaceBrowserProps["onArchiveSpace"];
}): ReactElement {
  const openLabel = useLabel(hubUiLabel("os.hub.spaces.open"), { name: row.name });
  const archiveLabel = useLabel(hubUiLabel("os.hub.spaces.archive"), { name: row.name });
  const membersLabel = useLabel(hubUiLabel("os.hub.spaces.memberCount"), { count: row.memberCount });
  const documentsLabel = useLabel(hubUiLabel("os.hub.spaces.documentCount"), { count: row.documentCount });
  const onlineLabel = useLabel(hubUiLabel("os.hub.spaces.onlineCount"), { count: row.activeConnections });
  return (
    <HubRowItem data-space-id={row.id} data-space-access={row.access}>
      <button
        type="button"
        data-shared-border-cell=""
        aria-current={active ? "true" : undefined}
        aria-label={`${openLabel} — ${roleText}${active ? ` (${currentLabel})` : ""}`}
        onClick={() => onOpenSpace(row.id)}
        className="min-w-0 flex-1 px-single py-1 text-left text-sm"
      >
        <span className="block truncate font-medium">{row.name}</span>
        <span className="block text-xs text-muted-foreground">{`${roleText} · ${membersLabel} · ${documentsLabel}${row.activeConnections > 0 ? ` · ${onlineLabel}` : ""}`}</span>
      </button>
      {spaceRowWritableV1(row) ? (
        <Button icon="file-archive" type="button" variant="outline" disabled={busy} aria-label={archiveLabel} onClick={() => onArchiveSpace(row.id)}>
          {archiveLabel}
        </Button>
      ) : null}
    </HubRowItem>
  );
}
//#endregion 🔖️Row

//#region 🔖️Browser
/** 🏘️ The surface. A `role="list"` of spaces where each row's open control is a real button carrying
 * `aria-current` for the active space, a search box, a live status line for the load/mutate phase, a
 * member roster with presence, the create form, and the invitation panel. Single column at phone
 * width; the roster moves beside the list from `md:` up. */
export function SpaceBrowser({
  rows,
  activeSpaceId,
  phase,
  search,
  members,
  invite,
  redemption,
  signedIn,
  onSearch,
  onOpenSpace,
  onRefresh,
  onCreateSpace,
  onArchiveSpace,
  onCreateInvite,
  onCopyInvite,
  onDismissInvite,
  onRedeemInvite,
}: SpaceBrowserProps): ReactElement {
  const id = useId();
  const title = useLabel(hubUiLabel("os.hub.spaces.title"));
  const listLabel = useLabel(hubUiLabel("os.hub.spaces.list"));
  const searchLabel = useLabel(hubUiLabel("os.hub.spaces.search"));
  const currentLabel = useLabel(hubUiLabel("os.hub.spaces.current"));
  const emptyLabel = useLabel(hubUiLabel("os.hub.spaces.empty"));
  const noMatchesLabel = useLabel(hubUiLabel("os.hub.spaces.noMatches"));
  const loadingLabel = useLabel(hubUiLabel("os.hub.spaces.loading"));
  const staleLabel = useLabel(hubUiLabel("os.hub.spaces.stale"));
  const submittingLabel = useLabel(hubUiLabel("os.hub.spaces.submitting"));
  const failedLabel = useLabel(hubUiLabel("os.hub.spaces.failed"));
  const refreshLabel = useLabel(hubUiLabel("os.hub.spaces.refresh"));
  const authorLabel = useLabel(hubUiLabel("os.hub.spaces.roleAuthor"));
  const spectatorLabel = useLabel(hubUiLabel("os.hub.spaces.roleSpectator"));
  const publicLabel = useLabel(hubUiLabel("os.hub.spaces.accessPublic"));
  const membersLabel = useLabel(hubUiLabel("os.hub.spaces.members"));
  const onlineLabel = useLabel(hubUiLabel("os.hub.spaces.online"));
  const awayLabel = useLabel(hubUiLabel("os.hub.spaces.away"));
  const ownerLabel = useLabel(hubUiLabel("os.hub.spaces.owner"));
  const activeRow = rows.find((row) => row.id === activeSpaceId) ?? null;
  const busy = phase === "submitting";
  const statusText = phase === "loading" ? loadingLabel : phase === "stale" ? staleLabel : phase === "submitting" ? submittingLabel : phase === "failed" ? failedLabel : "";
  const peers = spaceBrowserPresencePeersV1(members);

  return (
    <SharedBorderRow as="section" orientation="vertical" aria-labelledby={`${id}-title`} data-semio-hub-spaces-phase={phase} className="w-full">
      <SharedBorderRow as="header">
        <SharedBorderRowCell className="flex-1 flex-wrap gap-x-4">
          <h2 id={`${id}-title`} className="text-base font-medium">{title}</h2>
          <p role="status" aria-live="polite" data-semio-hub-spaces-status={phase} className="text-xs text-muted-foreground">{statusText}</p>
        </SharedBorderRowCell>
        <Button icon="rotate-cw" type="button" variant="outline" aria-label={refreshLabel} disabled={busy} onClick={onRefresh}>{refreshLabel}</Button>
      </SharedBorderRow>

      <HubFieldRow htmlFor={`${id}-search`} label={searchLabel}>
        <Input id={`${id}-search`} name="spaceSearch" type="search" value={search} onChange={(event) => onSearch(event.target.value)} />
      </HubFieldRow>

      {rows.length === 0 ? (
        <SharedBorderRowCell><p className="text-sm text-muted-foreground">{search.trim().length > 0 ? noMatchesLabel : emptyLabel}</p></SharedBorderRowCell>
      ) : (
        <HubRowList data-element-alias="os.hub.spaces.list" aria-label={listLabel}>
          {rows.map((row) => (
            <SpaceRow
              key={row.id}
              row={row}
              active={row.id === activeSpaceId}
              busy={busy}
              currentLabel={currentLabel}
              roleText={row.access === "public" ? publicLabel : row.role === "author" ? authorLabel : spectatorLabel}
              onOpenSpace={onOpenSpace}
              onArchiveSpace={onArchiveSpace}
            />
          ))}
        </HubRowList>
      )}

      <SharedBorderRow as="aside" orientation="vertical" aria-label={membersLabel}>
        <SharedBorderRowCell><h3 className="text-sm font-medium">{membersLabel}</h3></SharedBorderRowCell>
        <PresenceBar peers={peers} />
        {members.length === 0 ? null : <HubRowList>
          {members.map((member) => (
            <SharedBorderRowCell as="li" key={member.userId} data-user-id={member.userId} data-online={member.online ? "true" : "false"} className="justify-between gap-2 text-xs">
              <span className="min-w-0 truncate">{member.displayName}</span>
              <span className="shrink-0 text-muted-foreground">{`${member.owner ? `${ownerLabel} · ` : ""}${member.role === "author" ? authorLabel : spectatorLabel} · ${member.online ? onlineLabel : awayLabel}`}</span>
            </SharedBorderRowCell>
          ))}
        </HubRowList>}
      </SharedBorderRow>

      {signedIn ? <CreateSpaceForm disabled={busy} onCreateSpace={onCreateSpace} /> : null}

      <InvitePanel row={activeRow} invite={invite} redemption={redemption} disabled={busy} onCreateInvite={onCreateInvite} onCopyInvite={onCopyInvite} onDismissInvite={onDismissInvite} onRedeemInvite={onRedeemInvite} />
    </SharedBorderRow>
  );
}
//#endregion 🔖️Browser
