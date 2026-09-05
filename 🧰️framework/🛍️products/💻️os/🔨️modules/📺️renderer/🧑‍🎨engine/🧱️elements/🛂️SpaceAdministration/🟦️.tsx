// #region 🧲️Header
/** @emoji 🛂️ `🛂️SpaceAdministration` — the Shell-owned administration pane for exactly one space
 * (ticket `26/09/02/COMPLETE-SEMIO-END-TO-END`, packet §4). It renders SOLELY from the hub's own
 * canonical `DirectorySpaceAdministrationPageV1`: the member roster, the author-only invite roster,
 * and the receipt/status region. Authority is never derived from a locally stored role — every
 * affordance is gated on the page's own server-filled `capabilities`, and a `member`/`public` page
 * structurally has neither invites nor capabilities, so there is nothing to mis-gate.
 *
 * The receipt/status region changes only after an accepted server receipt has been followed by a
 * page refresh (the worker's `receipt → refreshing → ready` turn). Nothing here dispatches an
 * optimistic mutation, retries a mutation, or holds an invite capability: the one-shot invite token
 * lives in the browser worker until an exact operation-bound clipboard success result.
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useEffect, useMemo, useRef, useState, type ReactElement } from "react";
import { Button, registerUiTranslationBundles, useLabel } from "@semio-tech/ui-react";
import type { DirectoryAdministrationInviteCapabilityStatusV1, DirectoryAdministrationPhaseV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationInviteRowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole } from "../../../../../🟦️.ts";
// #endregion 🔌️Adapters

//#region 🔖️Types
/** 🎬️ Every administration intent this pane can raise. The pane never builds a command itself —
 * `ShellHost` maps these onto the closed `DirectoryCommand` vocabulary and the retained operation. */
export type SpaceAdministrationIntentV1 =
  | { readonly kind: "set-role"; readonly userId: string; readonly role: DirectorySpaceRole }
  | { readonly kind: "remove-member"; readonly userId: string }
  | { readonly kind: "create-invite"; readonly role: DirectorySpaceRole }
  | { readonly kind: "revoke-invite"; readonly inviteId: string }
  | { readonly kind: "page"; readonly cursor: string }
  | { readonly kind: "copy-invite-capability" }
  | { readonly kind: "close" };

export interface SpaceAdministrationPaneProps {
  readonly spaceId: string;
  readonly phase: DirectoryAdministrationPhaseV1;
  readonly page: DirectorySpaceAdministrationPageV1 | null;
  readonly receiptSha256?: string;
  readonly code?: string;
  readonly inviteCapabilityPending?: boolean;
  readonly inviteCapabilityStatus?: DirectoryAdministrationInviteCapabilityStatusV1;
  readonly onIntent: (intent: SpaceAdministrationIntentV1) => void;
}
//#endregion 🔖️Types

//#region 🌐️Labels
export const spaceAdministrationUiLabel = registerUiTranslationBundles({
  en: {
    translation: {
      os: {
        spaceAdministration: {
          title: { label: { normal: "Space administration", beginner: "Manage this space" } },
          close: { label: { normal: "Close administration", beginner: "Close" } },
          members: { label: { normal: "Members", beginner: "People" } },
          invites: { label: { normal: "Invitations", beginner: "Invitations" } },
          role: { label: { normal: "Role", beginner: "Role" } },
          author: { label: { normal: "Author", beginner: "Can edit" } },
          spectator: { label: { normal: "Spectator", beginner: "Can view" } },
          owner: { label: { normal: "Owner", beginner: "Owner" } },
          remove: { label: { normal: "Remove member", beginner: "Remove" } },
          issue: { label: { normal: "Issue invitation", beginner: "Invite someone" } },
          revoke: { label: { normal: "Revoke invitation", beginner: "Cancel invitation" } },
          copy: { label: { normal: "Copy invitation link", beginner: "Copy link" } },
          copyAvailable: { label: { normal: "The invitation link is ready to copy.", beginner: "The link is ready." } },
          copyCopying: { label: { normal: "Copying the invitation link…", beginner: "Copying…" } },
          copyFailed: { label: { normal: "Clipboard unavailable or denied. Try copying again.", beginner: "Could not copy. Try again." } },
          more: { label: { normal: "Show more", beginner: "Show more" } },
          revoked: { label: { normal: "Revoked", beginner: "Cancelled" } },
          accepted: { label: { normal: "Accepted", beginner: "Accepted" } },
          pending: { label: { normal: "Pending", beginner: "Waiting" } },
          statusLoading: { label: { normal: "Loading the administration page…", beginner: "Loading…" } },
          statusReady: { label: { normal: "Administration page is current.", beginner: "Up to date." } },
          statusSubmitting: { label: { normal: "Waiting for the server receipt…", beginner: "Working…" } },
          statusReceipt: { label: { normal: "Server receipt accepted.", beginner: "Done." } },
          statusRefreshing: { label: { normal: "Refreshing after the receipt…", beginner: "Refreshing…" } },
          statusCancelled: { label: { normal: "Administration was cancelled.", beginner: "Cancelled." } },
          statusDenied: { label: { normal: "Access to this space was withdrawn.", beginner: "No longer allowed." } },
          statusStale: { label: { normal: "This session changed; reopen administration.", beginner: "Please reopen." } },
          statusFailed: { label: { normal: "Unknown outcome — refresh required before retrying.", beginner: "Unknown result. Refresh first." } },
          spectatorNotice: { label: { normal: "You can view this space but not administer it.", beginner: "You can only look." } },
          publicNotice: { label: { normal: "This is a public space you are not a member of.", beginner: "You are not a member." } },
        },
      },
    },
  },
  de: {
    translation: {
      os: {
        spaceAdministration: {
          title: { label: { normal: "Space-Verwaltung", beginner: "Diesen Space verwalten" } },
          close: { label: { normal: "Verwaltung schließen", beginner: "Schließen" } },
          members: { label: { normal: "Mitglieder", beginner: "Personen" } },
          invites: { label: { normal: "Einladungen", beginner: "Einladungen" } },
          role: { label: { normal: "Rolle", beginner: "Rolle" } },
          author: { label: { normal: "Autor", beginner: "Darf bearbeiten" } },
          spectator: { label: { normal: "Betrachter", beginner: "Darf ansehen" } },
          owner: { label: { normal: "Eigentümer", beginner: "Eigentümer" } },
          remove: { label: { normal: "Mitglied entfernen", beginner: "Entfernen" } },
          issue: { label: { normal: "Einladung ausstellen", beginner: "Jemanden einladen" } },
          revoke: { label: { normal: "Einladung widerrufen", beginner: "Einladung abbrechen" } },
          copy: { label: { normal: "Einladungslink kopieren", beginner: "Link kopieren" } },
          copyAvailable: { label: { normal: "Der Einladungslink kann kopiert werden.", beginner: "Der Link ist bereit." } },
          copyCopying: { label: { normal: "Einladungslink wird kopiert…", beginner: "Wird kopiert…" } },
          copyFailed: { label: { normal: "Zwischenablage nicht verfügbar oder abgelehnt. Erneut versuchen.", beginner: "Kopieren fehlgeschlagen. Erneut versuchen." } },
          more: { label: { normal: "Mehr anzeigen", beginner: "Mehr anzeigen" } },
          revoked: { label: { normal: "Widerrufen", beginner: "Abgebrochen" } },
          accepted: { label: { normal: "Angenommen", beginner: "Angenommen" } },
          pending: { label: { normal: "Ausstehend", beginner: "Wartet" } },
          statusLoading: { label: { normal: "Verwaltungsseite wird geladen…", beginner: "Lädt…" } },
          statusReady: { label: { normal: "Verwaltungsseite ist aktuell.", beginner: "Aktuell." } },
          statusSubmitting: { label: { normal: "Warte auf die Serverquittung…", beginner: "Arbeitet…" } },
          statusReceipt: { label: { normal: "Serverquittung angenommen.", beginner: "Fertig." } },
          statusRefreshing: { label: { normal: "Aktualisierung nach der Quittung…", beginner: "Aktualisiert…" } },
          statusCancelled: { label: { normal: "Verwaltung wurde abgebrochen.", beginner: "Abgebrochen." } },
          statusDenied: { label: { normal: "Der Zugriff auf diesen Space wurde entzogen.", beginner: "Nicht mehr erlaubt." } },
          statusStale: { label: { normal: "Diese Sitzung hat sich geändert; Verwaltung erneut öffnen.", beginner: "Bitte erneut öffnen." } },
          statusFailed: { label: { normal: "Unbekanntes Ergebnis — vor einem erneuten Versuch aktualisieren.", beginner: "Unbekannt. Zuerst aktualisieren." } },
          spectatorNotice: { label: { normal: "Du kannst diesen Space ansehen, aber nicht verwalten.", beginner: "Du kannst nur zusehen." } },
          publicNotice: { label: { normal: "Dies ist ein öffentlicher Space, in dem du kein Mitglied bist.", beginner: "Du bist kein Mitglied." } },
        },
      },
    },
  },
});

/** 🌐️ Every user-facing string this pane needs, pre-resolved for the pure builders below. */
export interface SpaceAdministrationLabels {
  readonly title: string;
  readonly close: string;
  readonly members: string;
  readonly invites: string;
  readonly role: string;
  readonly author: string;
  readonly spectator: string;
  readonly owner: string;
  readonly remove: string;
  readonly issue: string;
  readonly revoke: string;
  readonly copy: string;
  readonly copyStatus: Record<DirectoryAdministrationInviteCapabilityStatusV1, string>;
  readonly more: string;
  readonly revoked: string;
  readonly accepted: string;
  readonly pending: string;
  readonly status: Record<DirectoryAdministrationPhaseV1, string>;
  readonly spectatorNotice: string;
  readonly publicNotice: string;
}

export function useSpaceAdministrationLabels(): SpaceAdministrationLabels {
  return {
    title: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.title")),
    close: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.close")),
    members: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.members")),
    invites: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.invites")),
    role: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.role")),
    author: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.author")),
    spectator: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.spectator")),
    owner: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.owner")),
    remove: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.remove")),
    issue: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.issue")),
    revoke: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.revoke")),
    copy: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.copy")),
    copyStatus: {
      available: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.copyAvailable")),
      copying: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.copyCopying")),
      failed: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.copyFailed")),
    },
    more: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.more")),
    revoked: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.revoked")),
    accepted: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.accepted")),
    pending: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.pending")),
    status: {
      loading: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusLoading")),
      ready: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusReady")),
      submitting: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusSubmitting")),
      receipt: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusReceipt")),
      refreshing: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusRefreshing")),
      cancelled: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusCancelled")),
      denied: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusDenied")),
      stale: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusStale")),
      failed: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.statusFailed")),
    },
    spectatorNotice: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.spectatorNotice")),
    publicNotice: useLabel(spaceAdministrationUiLabel("os.spaceAdministration.publicNotice")),
  };
}
//#endregion 🌐️Labels

//#region 🔖️Capability
/** 🛂️ The pane's ONLY authority source: the page's own server-filled capability flags. A
 * `member`/`public` page carries none, so every administration affordance is structurally absent
 * rather than merely hidden. A phase that is not `ready` disables dispatch without changing which
 * affordances exist, so a control never silently changes meaning mid-turn. */
export function spaceAdministrationCapabilities(page: DirectorySpaceAdministrationPageV1 | null): DirectorySpaceAdministrationCapabilitiesV1 | null {
  return page !== null && page.access === "author" ? page.capabilities : null;
}

/** 🔒️ An owner row can never be removed: the server rejects it, so the control is disabled before
 * dispatch rather than offered and then refused. */
export function spaceAdministrationMemberRemovable(row: DirectorySpaceAdministrationMemberRowV1, capabilities: DirectorySpaceAdministrationCapabilitiesV1 | null): boolean {
  return capabilities !== null && capabilities.removeMember && !row.owner;
}

/** 🎟️ A revoked or accepted invitation is terminal; only a live one may be revoked. */
export function spaceAdministrationInviteRevocable(row: DirectorySpaceAdministrationInviteRowV1, capabilities: DirectorySpaceAdministrationCapabilitiesV1 | null): boolean {
  return capabilities !== null && capabilities.revokeInvite && !row.revoked && !row.accepted;
}

/** 🚦️ Dispatch is admitted only from a settled `ready` page — never while a receipt is pending. */
export function spaceAdministrationDispatchable(phase: DirectoryAdministrationPhaseV1): boolean {
  return phase === "ready";
}
//#endregion 🔖️Capability

//#region 🔖️Pane
/** 🏛️ The pane. Semantic buttons and a labelled select per row, one `role="status"` live region for
 * the receipt/status text, and focus restored to the pane heading whenever the phase settles, so a
 * keyboard operator is never stranded on a control that has just been removed. */
export function SpaceAdministrationPane({ spaceId, phase, page, receiptSha256, code, inviteCapabilityPending, inviteCapabilityStatus, onIntent }: SpaceAdministrationPaneProps): ReactElement {
  const labels = useSpaceAdministrationLabels();
  const headingRef = useRef<HTMLHeadingElement | null>(null);
  const [inviteRole, setInviteRole] = useState<DirectorySpaceRole>("spectator");
  const capabilities = useMemo(() => spaceAdministrationCapabilities(page), [page]);
  const dispatchable = spaceAdministrationDispatchable(phase);
  const members = page !== null && page.access !== "public" ? page.members : null;
  const invites = page !== null && page.access === "author" ? page.invites : null;

  useEffect(() => {
    if (phase === "ready" || phase === "denied" || phase === "stale" || phase === "failed" || phase === "cancelled") headingRef.current?.focus();
  }, [phase]);

  const roleLabel = (role: DirectorySpaceRole): string => (role === "author" ? labels.author : labels.spectator);
  const status = `${labels.status[phase]}${code === undefined ? "" : ` (${code})`}${inviteCapabilityStatus === undefined ? "" : ` ${labels.copyStatus[inviteCapabilityStatus]}`}`;

  return (
    <section aria-labelledby="os-space-administration-title" data-space-id={spaceId} data-phase={phase} className="flex flex-col gap-4 p-4">
      <header className="flex items-center justify-between gap-2">
        <h2 id="os-space-administration-title" ref={headingRef} tabIndex={-1}>{`${labels.title} — ${spaceId}`}</h2>
        <Button type="button" variant="outline" aria-label={labels.close} onClick={() => onIntent({ kind: "close" })}>{labels.close}</Button>
      </header>

      <p role="status" aria-live="polite" data-testid="os-space-administration-status" data-receipt={receiptSha256 ?? ""}>{status}</p>

      {page !== null && page.access === "public" ? <p data-testid="os-space-administration-notice">{labels.publicNotice}</p> : null}
      {page !== null && page.access === "member" ? <p data-testid="os-space-administration-notice">{labels.spectatorNotice}</p> : null}

      {members === null ? null : (
        <section aria-label={labels.members}>
          <h3>{labels.members}</h3>
          <ul>
            {members.rows.map((row) => (
              <li key={row.userId} data-user-id={row.userId} className="flex items-center gap-2">
                <span>{row.displayName.length > 0 ? row.displayName : row.email}</span>
                {row.owner ? <span data-testid={`os-space-administration-owner-${row.userId}`}>{labels.owner}</span> : null}
                <label htmlFor={`os-space-administration-role-${row.userId}`}>{labels.role}</label>
                <select
                  id={`os-space-administration-role-${row.userId}`}
                  value={row.role}
                  disabled={capabilities === null || !capabilities.upsertMember || !dispatchable}
                  onChange={(event) => onIntent({ kind: "set-role", userId: row.userId, role: event.target.value === "author" ? "author" : "spectator" })}
                >
                  <option value="author">{labels.author}</option>
                  <option value="spectator">{labels.spectator}</option>
                </select>
                <Button
                  type="button"
                  variant="outline"
                  aria-label={`${labels.remove}: ${row.userId}`}
                  disabled={!spaceAdministrationMemberRemovable(row, capabilities) || !dispatchable}
                  onClick={() => onIntent({ kind: "remove-member", userId: row.userId })}
                >
                  {labels.remove}
                </Button>
              </li>
            ))}
          </ul>
          {members.nextCursor === undefined ? null : (
            <Button type="button" variant="outline" aria-label={`${labels.more}: ${labels.members}`} disabled={!dispatchable} onClick={() => onIntent({ kind: "page", cursor: members.nextCursor as string })}>
              {labels.more}
            </Button>
          )}
        </section>
      )}

      {invites === null || capabilities === null ? null : (
        <section aria-label={labels.invites}>
          <h3>{labels.invites}</h3>
          <label htmlFor="os-space-administration-invite-role">{labels.role}</label>
          <select id="os-space-administration-invite-role" value={inviteRole} disabled={!capabilities.createInvite || !dispatchable} onChange={(event) => setInviteRole(event.target.value === "author" ? "author" : "spectator")}>
            <option value="author">{labels.author}</option>
            <option value="spectator">{labels.spectator}</option>
          </select>
          <Button type="button" variant="outline" aria-label={labels.issue} disabled={!capabilities.createInvite || !dispatchable} onClick={() => onIntent({ kind: "create-invite", role: inviteRole })}>
            {labels.issue}
          </Button>
          {inviteCapabilityPending === true ? (
            <Button type="button" variant="outline" aria-label={labels.copy} disabled={inviteCapabilityStatus === "copying"} onClick={() => onIntent({ kind: "copy-invite-capability" })}>
              {labels.copy}
            </Button>
          ) : null}
          <ul>
            {invites.rows.map((row) => (
              <li key={row.inviteId} data-invite-id={row.inviteId} className="flex items-center gap-2">
                <span>{`${row.inviteId} · ${roleLabel(row.role)} · ${row.revoked ? labels.revoked : row.accepted ? labels.accepted : labels.pending}`}</span>
                <Button
                  type="button"
                  variant="outline"
                  aria-label={`${labels.revoke}: ${row.inviteId}`}
                  disabled={!spaceAdministrationInviteRevocable(row, capabilities) || !dispatchable}
                  onClick={() => onIntent({ kind: "revoke-invite", inviteId: row.inviteId })}
                >
                  {labels.revoke}
                </Button>
              </li>
            ))}
          </ul>
          {invites.nextCursor === undefined ? null : (
            <Button type="button" variant="outline" aria-label={`${labels.more}: ${labels.invites}`} disabled={!dispatchable} onClick={() => onIntent({ kind: "page", cursor: invites.nextCursor as string })}>
              {labels.more}
            </Button>
          )}
        </section>
      )}
    </section>
  );
}
//#endregion 🔖️Pane
