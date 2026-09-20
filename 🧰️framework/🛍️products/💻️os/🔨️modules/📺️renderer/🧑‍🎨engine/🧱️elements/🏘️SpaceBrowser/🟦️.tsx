// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/🏘️SpaceBrowser/component.tsx
/** @emoji 🏘️ `🏘️SpaceBrowser` — the end-user spaces surface `🛂️SpaceAdministration` never was: list
 * the spaces I can reach, switch between them, create one, see who is a member and who is here right
 * now, issue an invitation, and redeem one someone sent me. Purely presentational: every mutation
 * leaves as an intent callback, and `useHubConnection` turns it into a closed `DirectoryCommand` on
 * the hub's command path (CQRS — this element never performs a CRUD write). Ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useId, useState, type FormEvent, type ReactElement } from "react";
import { Button, PresenceBar, useLabel, type PresencePeer } from "@semio-tech/ui-react";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility } from "@semio-tech/framework-os";
import {
  INVITE_TTL_CHOICES_SECS_V1,
  spaceRowInvitableV1,
  spaceRowWritableV1,
  type SpaceBrowserPhaseV1,
  type SpaceMemberPresenceV1,
  type SpaceRowV1,
} from "../../../../📇️directory/🏘️spaces/🟦️.ts";
import { hubUiLabel, type HubInviteCapabilityV1, type HubRedemptionStateV1 } from "../🔗️HubConnection/🟦️.tsx";
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
    <form aria-label={titleLabel} onSubmit={submit} className="flex flex-col gap-2">
      <h3 className="text-sm font-medium">{titleLabel}</h3>
      <div className="flex flex-col gap-1">
        <label htmlFor={`${id}-name`} className="text-xs text-muted-foreground">{nameLabel}</label>
        <input id={`${id}-name`} data-element-alias="os.hub.spaces.createName" name="spaceName" required value={name} disabled={disabled} onChange={(event) => setName(event.target.value)} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm" />
      </div>
      <div className="flex flex-col gap-2 sm:flex-row">
        <div className="flex min-w-0 flex-1 flex-col gap-1">
          <label htmlFor={`${id}-kind`} className="text-xs text-muted-foreground">{kindLabel}</label>
          <select id={`${id}-kind`} value={kind} disabled={disabled} onChange={(event) => setKind(event.target.value === "studio" ? "studio" : "atelier")} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm">
            <option value="atelier">{atelierLabel}</option>
            <option value="studio">{studioLabel}</option>
          </select>
        </div>
        <div className="flex min-w-0 flex-1 flex-col gap-1">
          <label htmlFor={`${id}-visibility`} className="text-xs text-muted-foreground">{visibilityLabel}</label>
          <select id={`${id}-visibility`} value={visibility} disabled={disabled} onChange={(event) => setVisibility(event.target.value === "public" ? "public" : "private")} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm">
            <option value="private">{privateLabel}</option>
            <option value="public">{publicLabel}</option>
          </select>
        </div>
      </div>
      <Button id="os.hub.spaces.createSubmit" icon="plus" type="submit" variant="outline" aria-label={submitLabel} disabled={disabled || !valid}>{submitLabel}</Button>
    </form>
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
    <section aria-label={titleLabel} className="flex flex-col gap-3 border-t pt-3">
      <h3 className="text-sm font-medium">{titleLabel}</h3>

      {row === null || !spaceRowInvitableV1(row) ? null : (
        <form
          aria-label={createTitleLabel}
          onSubmit={(event) => {
            event.preventDefault();
            if (!disabled) onCreateInvite(row.id, role, ttlSecs);
          }}
          className="flex flex-col gap-2"
        >
          <div className="flex flex-col gap-2 sm:flex-row">
            <div className="flex min-w-0 flex-1 flex-col gap-1">
              <label htmlFor={`${id}-role`} className="text-xs text-muted-foreground">{roleLabel}</label>
              <select id={`${id}-role`} data-element-alias="os.hub.invite.role" value={role} disabled={disabled} onChange={(event) => setRole(event.target.value === "author" ? "author" : "spectator")} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm">
                <option value="spectator">{spectatorLabel}</option>
                <option value="author">{authorLabel}</option>
              </select>
            </div>
            <div className="flex min-w-0 flex-1 flex-col gap-1">
              <label htmlFor={`${id}-ttl`} className="text-xs text-muted-foreground">{expiryLabel}</label>
              <select id={`${id}-ttl`} data-element-alias="os.hub.invite.expiry" value={String(ttlSecs)} disabled={disabled} onChange={(event) => setTtlSecs(Number.parseInt(event.target.value, 10))} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm">
                <option value={String(INVITE_TTL_CHOICES_SECS_V1[0])}>{hourLabel}</option>
                <option value={String(INVITE_TTL_CHOICES_SECS_V1[1])}>{dayLabel}</option>
                <option value={String(INVITE_TTL_CHOICES_SECS_V1[2])}>{weekLabel}</option>
              </select>
            </div>
          </div>
          <Button id="os.hub.invite.create" icon="users" type="submit" variant="outline" aria-label={createLabel} disabled={disabled}>{createLabel}</Button>
        </form>
      )}

      {invite === null ? null : (
        <div role="status" aria-live="polite" data-semio-hub-invite={invite.spaceId} className="flex flex-col gap-2 rounded-sm border px-single py-1">
          <p className="text-xs text-muted-foreground">{readyLabel}</p>
          <output htmlFor={`${id}-invite`} className="break-all text-xs">{invite.link}</output>
          <div className="flex flex-col gap-2 sm:flex-row">
            <Button id="os.hub.invite.copy" icon="copy" type="button" variant="outline" aria-label={copyLabel} onClick={onCopyInvite}>{copyLabel}</Button>
            <Button icon="x" type="button" variant="outline" aria-label={dismissLabel} onClick={onDismissInvite}>{dismissLabel}</Button>
          </div>
          {invite.copy === "copied" ? <p className="text-xs">{copiedLabel}</p> : null}
          {invite.copy === "failed" ? <p role="alert" className="text-xs text-red-400">{copyFailedLabel}</p> : null}
        </div>
      )}

      <form
        aria-label={redeemTitleLabel}
        onSubmit={(event) => {
          event.preventDefault();
          if (redemption.phase !== "redeeming") onRedeemInvite(typed);
        }}
        className="flex flex-col gap-2"
      >
        <label htmlFor={`${id}-redeem`} className="text-xs text-muted-foreground">{redeemFieldLabel}</label>
        <input
          id={`${id}-redeem`}
          data-element-alias="os.hub.invite.redeemField"
          name="invitation"
          required
          value={typed}
          disabled={redemption.phase === "redeeming"}
          aria-invalid={redemption.error !== null}
          aria-describedby={redemption.error === null ? undefined : `${id}-redeem-error`}
          onChange={(event) => setTyped(event.target.value)}
          className="w-full min-w-0 rounded-sm border px-single py-1 text-sm"
        />
        <Button icon="unlock" type="submit" variant="outline" aria-busy={redemption.phase === "redeeming"} aria-label={redemption.phase === "redeeming" ? redeemingLabel : redeemLabel} disabled={redemption.phase === "redeeming" || typed.trim().length === 0}>
          {redemption.phase === "redeeming" ? redeemingLabel : redeemLabel}
        </Button>
        {redemption.phase === "redeemed" ? <p role="status" aria-live="polite" className="text-xs">{redeemedLabel}</p> : null}
        {redemptionError === null ? null : <p id={`${id}-redeem-error`} role="alert" data-semio-hub-redemption-error={redemption.error ?? ""} className="text-xs text-red-400">{redemptionError}</p>}
      </form>
    </section>
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
    <li data-space-id={row.id} data-space-access={row.access} className="flex flex-col gap-1 rounded-sm border px-single py-1 sm:flex-row sm:items-center sm:justify-between">
      <button
        type="button"
        aria-current={active ? "true" : undefined}
        aria-label={`${openLabel} — ${roleText}${active ? ` (${currentLabel})` : ""}`}
        onClick={() => onOpenSpace(row.id)}
        className="min-w-0 flex-1 text-left text-sm"
      >
        <span className="block truncate font-medium">{row.name}</span>
        <span className="block text-xs text-muted-foreground">{`${roleText} · ${membersLabel} · ${documentsLabel}${row.activeConnections > 0 ? ` · ${onlineLabel}` : ""}`}</span>
      </button>
      {spaceRowWritableV1(row) ? (
        <Button icon="file-archive" type="button" variant="outline" disabled={busy} aria-label={archiveLabel} onClick={() => onArchiveSpace(row.id)}>
          {archiveLabel}
        </Button>
      ) : null}
    </li>
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
    <section aria-labelledby={`${id}-title`} data-semio-hub-spaces-phase={phase} className="flex w-full min-w-0 flex-col gap-4 p-4">
      <header className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
        <h2 id={`${id}-title`} className="text-base font-medium">{title}</h2>
        <Button icon="rotate-cw" type="button" variant="outline" aria-label={refreshLabel} disabled={busy} onClick={onRefresh}>{refreshLabel}</Button>
      </header>

      <p role="status" aria-live="polite" data-semio-hub-spaces-status={phase} className="text-xs text-muted-foreground">{statusText}</p>

      <div className="flex flex-col gap-1">
        <label htmlFor={`${id}-search`} className="text-xs text-muted-foreground">{searchLabel}</label>
        <input id={`${id}-search`} name="spaceSearch" type="search" value={search} onChange={(event) => onSearch(event.target.value)} className="w-full min-w-0 rounded-sm border px-single py-1 text-sm" />
      </div>

      <div className="flex flex-col gap-4 md:flex-row md:items-start">
        <div className="flex min-w-0 flex-1 flex-col gap-2">
          {rows.length === 0 ? (
            <p className="text-sm text-muted-foreground">{search.trim().length > 0 ? noMatchesLabel : emptyLabel}</p>
          ) : (
            <ul role="list" data-element-alias="os.hub.spaces.list" aria-label={listLabel} className="flex flex-col gap-1">
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
            </ul>
          )}
        </div>

        <aside aria-label={membersLabel} className="flex w-full min-w-0 flex-col gap-2 md:w-64">
          <h3 className="text-sm font-medium">{membersLabel}</h3>
          <PresenceBar peers={peers} />
          <ul role="list" className="flex flex-col gap-1">
            {members.map((member) => (
              <li key={member.userId} data-user-id={member.userId} data-online={member.online ? "true" : "false"} className="flex items-center justify-between gap-2 text-xs">
                <span className="min-w-0 truncate">{member.displayName}</span>
                <span className="shrink-0 text-muted-foreground">{`${member.owner ? `${ownerLabel} · ` : ""}${member.role === "author" ? authorLabel : spectatorLabel} · ${member.online ? onlineLabel : awayLabel}`}</span>
              </li>
            ))}
          </ul>
        </aside>
      </div>

      {signedIn ? <CreateSpaceForm disabled={busy} onCreateSpace={onCreateSpace} /> : null}

      <InvitePanel row={activeRow} invite={invite} redemption={redemption} disabled={busy} onCreateInvite={onCreateInvite} onCopyInvite={onCopyInvite} onDismissInvite={onDismissInvite} onRedeemInvite={onRedeemInvite} />
    </section>
  );
}
//#endregion 🔖️Browser
