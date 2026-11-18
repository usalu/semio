// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🏛️SpacesPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, SharedBorderRow, SharedBorderRowCell, SharedBorderRowLabel, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Table, uiDataLabel, useMediaQuery, UI_MOBILE_MEDIA_QUERY, type TableColumn } from "@semio-tech/ui-react";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, SpaceView } from "@semio-tech/framework-os";
import { AdminPage } from "../📄️AdminPage/🟦️.tsx";
import { useAdminT, type AdminI18nKey } from "../📚️I18n/🟦️.tsx";
import { useAdminSession, type AdminSpaceDetail } from "../🔑️AdminSession/🟦️.tsx";
// #endregion 🔌️Adapters

const SPACE_KINDS: readonly DirectorySpaceKind[] = ["atelier", "studio", "archive"];
const SPACE_ROLES: readonly DirectorySpaceRole[] = ["author", "spectator"];
const INVITE_TTL_SECS = 60 * 60 * 24 * 7;

//#region 🔖️CreateDialog
function CreateSpaceDialog({ open, onOpenChange, onCreate }: { readonly open: boolean; readonly onOpenChange: (open: boolean) => void; readonly onCreate: (name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility) => void }): React.ReactElement {
  const t = useAdminT();
  const [name, setName] = React.useState("");
  const [kind, setKind] = React.useState<DirectorySpaceKind>("studio");
  const [visibility, setVisibility] = React.useState<DirectorySpaceVisibility>("private");

  React.useEffect(() => {
    if (open) {
      setName("");
      setKind("studio");
      setVisibility("private");
    }
  }, [open]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("admin.spaces.createTitle")}</DialogTitle>
        </DialogHeader>
        <SharedBorderRow orientation="vertical" edges={["top"]}>
          <SharedBorderRow>
            <SharedBorderRowLabel htmlFor="admin-space-create-name">{t("admin.spaces.name")}</SharedBorderRowLabel>
            <Input id="admin-space-create-name" className="flex-1 min-w-0" value={name} onChange={(event) => setName(event.target.value)} />
          </SharedBorderRow>
          <SharedBorderRow>
            <SharedBorderRowLabel htmlFor="admin-space-create-kind">{t("admin.spaces.kind")}</SharedBorderRowLabel>
            <Select id="admin-space-create-kind-select" value={kind} onValueChange={(next) => setKind(next as DirectorySpaceKind)}>
              <SelectTrigger id="admin-space-create-kind" className="flex-1 min-w-0">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {SPACE_KINDS.map((candidate) => (
                  <SelectItem key={candidate} value={candidate}>
                    {t(`admin.spaces.kind${candidate[0].toUpperCase()}${candidate.slice(1)}` as AdminI18nKey)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </SharedBorderRow>
          <SharedBorderRow>
            <SharedBorderRowLabel htmlFor="admin-space-create-visibility">{t("admin.spaces.visibility")}</SharedBorderRowLabel>
            <Select id="admin-space-create-visibility-select" value={visibility} onValueChange={(next) => setVisibility(next as DirectorySpaceVisibility)}>
              <SelectTrigger id="admin-space-create-visibility" className="flex-1 min-w-0">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="private">{t("admin.spaces.visibilityPrivate")}</SelectItem>
                <SelectItem value="public">{t("admin.spaces.visibilityPublic")}</SelectItem>
              </SelectContent>
            </Select>
          </SharedBorderRow>
        </SharedBorderRow>
        <DialogFooter>
          <Button id="admin-space-create-cancel" icon="x" text={t("admin.spaces.cancel")} variant="ghost" onClick={() => onOpenChange(false)} />
          <Button
            id="admin-space-create-submit"
            icon="plus"
            text={t("admin.spaces.save")}
            disabled={name.trim().length === 0}
            onClick={() => {
              onCreate(name.trim(), kind, visibility);
              onOpenChange(false);
            }}
          />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
//#endregion 🔖️CreateDialog

//#region 🔖️MembersPanel
function MembersPanel({ spaceId, detail, loadingMore, onLoadMore, onChanged }: { readonly spaceId: string; readonly detail: AdminSpaceDetail; readonly loadingMore: boolean; readonly onLoadMore: (cursor: string) => void; readonly onChanged: () => void }): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [email, setEmail] = React.useState("");
  const [role, setRole] = React.useState<DirectorySpaceRole>("author");
  const [inviteToken, setInviteToken] = React.useState<string | null>(null);

  const memberColumns: TableColumn<AdminSpaceDetail["members"]["rows"][number]>[] = [
    { id: "email", header: t("admin.spaces.email"), accessor: (row) => row.email || row.userId },
    { id: "role", header: t("admin.spaces.role"), accessor: (row) => (row.role === "author" ? t("admin.spaces.roleAuthor") : t("admin.spaces.roleSpectator")) },
    {
      id: "actions",
      header: t("admin.spaces.actions"),
      cell: "row",
      accessor: (row) => (
        <SharedBorderRow framed={false} className="h-full w-fit">
          <Button
            id={`admin-space-member-remove-${row.userId}`}
            icon="trash"
            text={t("admin.spaces.remove")}
            variant="ghost"
            onClick={() => client.removeSpaceMember(spaceId, row.userId).then(onChanged)}
          />
        </SharedBorderRow>
      ),
    },
  ];

  return (
    <SharedBorderRow orientation="vertical" edges={["bottom"]} data-slot="admin-space-members">
      <SharedBorderRowCell>
        <h3 className="text-sm font-semibold text-emphasized">{t("admin.spaces.membersTitle")}</h3>
      </SharedBorderRowCell>
      <Table columns={memberColumns} data={[...detail.members.rows]} getRowId={(row) => `user:${row.userId}`} rowDragProps={(row) => ({ "data-row-id": `user:${row.userId}` }) as React.HTMLAttributes<HTMLTableRowElement>} rowHeight="compact" />
      {detail.members.nextCursor ? (
        <Button
          id={`admin-space-members-load-more-${spaceId}`}
          icon="arrow-down"
          text={t("admin.spaces.loadMoreMembers")}
          variant="ghost"
          disabled={loadingMore}
          onClick={() => onLoadMore(detail.members.nextCursor!)}
        />
      ) : null}
      <SharedBorderRow data-slot="admin-space-member-add-row">
        <Input id={`admin-space-member-email-${spaceId}`} aria-label={t("admin.spaces.email")} placeholder={t("admin.spaces.email")} className="flex-1 min-w-0" value={email} onChange={(event) => setEmail(event.target.value)} />
        <Select id={`admin-space-member-role-select-${spaceId}`} value={role} onValueChange={(next) => setRole(next as DirectorySpaceRole)}>
          <SelectTrigger id={`admin-space-member-role-${spaceId}`} aria-label={t("admin.spaces.role")} className="w-32">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {SPACE_ROLES.map((candidate) => (
              <SelectItem key={candidate} value={candidate}>
                {candidate === "author" ? t("admin.spaces.roleAuthor") : t("admin.spaces.roleSpectator")}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Button
          id={`admin-space-member-add-${spaceId}`}
          icon="plus"
          text={t("admin.spaces.addMember")}
          disabled={email.trim().length === 0}
          onClick={() => {
            client.upsertSpaceMember(spaceId, email.trim(), role).then(() => {
              setEmail("");
              onChanged();
            });
          }}
        />
      </SharedBorderRow>
      <SharedBorderRow data-slot="admin-space-invite-row">
        <Button
          id={`admin-space-invite-create-${spaceId}`}
          icon="link"
          text={t("admin.spaces.inviteCreate")}
          variant="ghost"
          onClick={() => {
            client.createSpaceInvite(spaceId, role, INVITE_TTL_SECS).then((response) => {
              const token = response.result?.inviteToken;
              setInviteToken(token ?? null);
            });
          }}
        />
        {inviteToken ? (
          <>
            <SharedBorderRowCell className="flex-1 text-sm text-muted-foreground">
              <code data-slot="admin-space-invite-token">{inviteToken}</code>
            </SharedBorderRowCell>
            <Button
              id={`admin-space-invite-copy-${spaceId}`}
              icon="copy"
              text={t("admin.spaces.inviteCopy")}
              variant="ghost"
              onClick={() => {
                void navigator.clipboard?.writeText(inviteToken);
              }}
            />
          </>
        ) : null}
      </SharedBorderRow>
    </SharedBorderRow>
  );
}
//#endregion 🔖️MembersPanel

/** 🏛️ `GET /admin/api/spaces` as a `Table`, a create dialog, per-row rename/visibility/archive/delete
 * actions (all via strict `POST /admin/api/intents` variants whose principal is server-derived),
 * and an expandable members sub-table
 * (`GET /admin/api/spaces/{id}`) with upsert-by-email, remove, and an invite-link action. Rows carry
 * `data-row-id="space:<id>"` per contract §C0's test-id grammar. */
export function SpacesPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [spaces, setSpaces] = React.useState<SpaceView[] | null>(null);
  const [spacesCursor, setSpacesCursor] = React.useState<string | null>(null);
  const [loadingSpaces, setLoadingSpaces] = React.useState(false);
  const [expandedId, setExpandedId] = React.useState<string | null>(null);
  const [detail, setDetail] = React.useState<AdminSpaceDetail | null>(null);
  const [loadingMembers, setLoadingMembers] = React.useState(false);
  const [createOpen, setCreateOpen] = React.useState(false);
  const narrow = useMediaQuery(UI_MOBILE_MEDIA_QUERY);

  const loadSpaces = React.useCallback((cursor?: string, append = false) => {
    setLoadingSpaces(true);
    void client
      .spaces(cursor)
      .then((page) => {
        setSpaces((current) => (append && current ? [...current, ...page.rows] : [...page.rows]));
        setSpacesCursor(page.nextCursor ?? null);
      })
      .catch(() => {
        if (!append) {
          setSpaces([]);
          setSpacesCursor(null);
        }
      })
      .finally(() => setLoadingSpaces(false));
  }, [client]);

  React.useEffect(() => {
    loadSpaces();
  }, [loadSpaces]);

  const loadDetail = React.useCallback(
    (spaceId: string, cursor?: string, append = false) => {
      setLoadingMembers(true);
      void client
        .space(spaceId, cursor)
        .then((next) => {
          setDetail((current) =>
            append && current?.id === next.id
              ? { ...next, members: { ...next.members, rows: [...current.members.rows, ...next.members.rows] } }
              : next,
          );
        })
        .catch(() => {
          if (!append) setDetail(null);
        })
        .finally(() => setLoadingMembers(false));
    },
    [client],
  );

  const toggleExpanded = React.useCallback(
    (spaceId: string) => {
      if (expandedId === spaceId) {
        setExpandedId(null);
        setDetail(null);
        return;
      }
      setExpandedId(spaceId);
      loadDetail(spaceId);
    },
    [expandedId, loadDetail],
  );

  const refreshExpanded = React.useCallback(() => {
    loadSpaces();
    if (expandedId) loadDetail(expandedId);
  }, [loadSpaces, loadDetail, expandedId]);

  const actionButtons = (row: SpaceView): React.ReactNode => (
    <>
      <Button
        id={`admin-space-rename-${row.id}`}
        icon="pencil"
        text={t("admin.spaces.rename")}
        variant="ghost"
        onClick={() => {
          const next = window.prompt(t("admin.spaces.rename"), row.name);
          if (next && next.trim().length > 0) client.renameSpace(row.id, next.trim()).then(refreshExpanded);
        }}
      />
      <Button
        id={`admin-space-visibility-${row.id}`}
        icon="eye"
        text={t("admin.spaces.setVisibility")}
        variant="ghost"
        onClick={() => {
          const next: DirectorySpaceVisibility = row.visibility === "public" ? "private" : "public";
          client.setSpaceVisibility(row.id, next).then(refreshExpanded);
        }}
      />
      <Button
        id={`admin-space-archive-${row.id}`}
        icon="trash"
        text={t("admin.spaces.archive")}
        variant="ghost"
        onClick={() => {
          if (window.confirm(t("admin.spaces.confirmArchive", { name: row.name }))) client.archiveSpace(row.id).then(refreshExpanded);
        }}
      />
      <Button
        id={`admin-space-delete-${row.id}`}
        icon="x"
        text={t("admin.spaces.delete")}
        variant="ghost"
        onClick={() => {
          if (window.confirm(t("admin.spaces.confirmDelete", { name: row.name }))) client.deleteSpace(row.id).then(refreshExpanded);
        }}
      />
    </>
  );

  const columns: TableColumn<SpaceView>[] = [
    { id: "name", header: t("admin.spaces.name"), accessor: (row) => row.name },
    { id: "kind", header: t("admin.spaces.kind"), accessor: (row) => row.kind },
    { id: "visibility", header: t("admin.spaces.visibility"), accessor: (row) => (row.visibility === "public" ? t("admin.spaces.visibilityPublic") : t("admin.spaces.visibilityPrivate")) },
    { id: "members", header: t("admin.spaces.members"), accessor: (row) => row.memberCount },
    { id: "documents", header: t("admin.spaces.documents"), accessor: (row) => row.documentCount },
    {
      id: "actions",
      header: t("admin.spaces.actions"),
      cell: "row",
      accessor: (row) => (
        <SharedBorderRow framed={false} className="h-full w-fit" onClick={(event) => event.stopPropagation()}>
          {actionButtons(row)}
        </SharedBorderRow>
      ),
    },
  ];

  const renderMobileRow = (row: SpaceView, _index: number, selected: boolean, onClick: (event: React.MouseEvent) => void): React.ReactNode => (
    <SharedBorderRow orientation="vertical" edges={["bottom"]} data-row-id={`space:${row.id}`} data-state={selected ? "selected" : undefined} data-slot="admin-space-row">
      <SharedBorderRowCell as="div" role="button" tabIndex={0} aria-expanded={selected} className="font-semibold" onClick={onClick} onKeyDown={(event) => (event.key === "Enter" || event.key === " ") && onClick(event as unknown as React.MouseEvent)}>
        {row.name}
      </SharedBorderRowCell>
      <SharedBorderRow framed={false}>
        <SharedBorderRowCell className="flex-1 text-xs text-muted-foreground">{row.kind}</SharedBorderRowCell>
        <SharedBorderRowCell className="flex-1 text-xs text-muted-foreground">{row.visibility === "public" ? t("admin.spaces.visibilityPublic") : t("admin.spaces.visibilityPrivate")}</SharedBorderRowCell>
        <SharedBorderRowCell className="text-xs text-muted-foreground">{t("admin.spaces.members")} {row.memberCount}</SharedBorderRowCell>
        <SharedBorderRowCell className="text-xs text-muted-foreground">{t("admin.spaces.documents")} {row.documentCount}</SharedBorderRowCell>
      </SharedBorderRow>
      <SharedBorderRow orientation="vertical" framed={false} className="[&_button]:w-full [&_button]:justify-start">
        {actionButtons(row)}
      </SharedBorderRow>
    </SharedBorderRow>
  );

  return (
    <AdminPage title={t("admin.spaces.title")} data-slot="admin-spaces" actions={<Button id="admin-space-create-open" icon="plus" text={t("admin.spaces.create")} onClick={() => setCreateOpen(true)} />}>
      <Table
        columns={columns}
        data={spaces ?? []}
        emptyMessage={uiDataLabel(t("admin.spaces.empty"))}
        getRowId={(row) => `space:${row.id}`}
        rowDragProps={(row) => ({ "data-row-id": `space:${row.id}` }) as React.HTMLAttributes<HTMLTableRowElement>}
        onRowClick={(row) => toggleExpanded(row.id)}
        selectedRows={expandedId ? new Set([`space:${expandedId}`]) : undefined}
        isMobile={narrow}
        renderMobileRow={renderMobileRow}
      />
      {spacesCursor ? (
        <SharedBorderRow edges={["bottom"]}>
          <Button id="admin-spaces-load-more" icon="arrow-down" text={t("admin.spaces.loadMore")} variant="ghost" disabled={loadingSpaces} onClick={() => loadSpaces(spacesCursor, true)} />
        </SharedBorderRow>
      ) : null}
      {expandedId && detail ? (
        <MembersPanel spaceId={expandedId} detail={detail} loadingMore={loadingMembers} onLoadMore={(cursor) => loadDetail(expandedId, cursor, true)} onChanged={refreshExpanded} />
      ) : null}
      <CreateSpaceDialog open={createOpen} onOpenChange={setCreateOpen} onCreate={(name, kind, visibility) => client.createSpace(name, kind, visibility).then(() => loadSpaces())} />
    </AdminPage>
  );
}
