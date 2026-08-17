// #region 🧲️Header
// 💻️ hub/modules/admin/elements/🏛️SpacesPage/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { Button, Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Table, uiDataLabel, type TableColumn } from "@semio-tech/ui-react";
import type { DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, SpaceView } from "@semio-tech/framework-os";
import { useAdminT, type AdminI18nKey } from "../📚️I18n/🟦️component.tsx";
import { useAdminSession, type AdminSpaceDetail } from "../🔑️AdminSession/🟦️component.tsx";
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
        <div className="flex flex-col gap-single">
          <label className="flex flex-col gap-single text-sm" htmlFor="admin-space-create-name">
            {t("admin.spaces.name")}
            <Input id="admin-space-create-name" value={name} onChange={(event) => setName(event.target.value)} />
          </label>
          <label className="flex flex-col gap-single text-sm">
            {t("admin.spaces.kind")}
            <Select id="admin-space-create-kind-select" value={kind} onValueChange={(next) => setKind(next as DirectorySpaceKind)}>
              <SelectTrigger id="admin-space-create-kind">
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
          </label>
          <label className="flex flex-col gap-single text-sm">
            {t("admin.spaces.visibility")}
            <Select id="admin-space-create-visibility-select" value={visibility} onValueChange={(next) => setVisibility(next as DirectorySpaceVisibility)}>
              <SelectTrigger id="admin-space-create-visibility">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="private">{t("admin.spaces.visibilityPrivate")}</SelectItem>
                <SelectItem value="public">{t("admin.spaces.visibilityPublic")}</SelectItem>
              </SelectContent>
            </Select>
          </label>
        </div>
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
function MembersPanel({ spaceId, detail, onChanged }: { readonly spaceId: string; readonly detail: AdminSpaceDetail; readonly onChanged: () => void }): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [email, setEmail] = React.useState("");
  const [role, setRole] = React.useState<DirectorySpaceRole>("author");
  const [inviteToken, setInviteToken] = React.useState<string | null>(null);

  const memberColumns: TableColumn<AdminSpaceDetail["members"][number]>[] = [
    { id: "email", header: t("admin.spaces.email"), accessor: (row) => row.email || row.userId },
    { id: "role", header: t("admin.spaces.role"), accessor: (row) => (row.role === "author" ? t("admin.spaces.roleAuthor") : t("admin.spaces.roleSpectator")) },
    {
      id: "actions",
      header: t("admin.spaces.actions"),
      accessor: (row) => (
        <Button
          id={`admin-space-member-remove-${row.userId}`}
          icon="trash"
          text={t("admin.spaces.remove")}
          variant="ghost"
          onClick={() => client.command({ kind: "remove-member", spaceId, userId: row.userId }).then(onChanged)}
        />
      ),
    },
  ];

  return (
    <div className="flex flex-col gap-single border-t p-single" data-slot="admin-space-members">
      <h3 className="text-sm font-semibold text-emphasized">{t("admin.spaces.membersTitle")}</h3>
      <Table columns={memberColumns} data={[...detail.members]} getRowId={(row) => `user:${row.userId}`} rowDragProps={(row) => ({ "data-row-id": `user:${row.userId}` }) as React.HTMLAttributes<HTMLTableRowElement>} rowHeight="compact" />
      <div className="flex items-end gap-single">
        <label className="flex flex-col gap-single text-sm" htmlFor={`admin-space-member-email-${spaceId}`}>
          {t("admin.spaces.email")}
          <Input id={`admin-space-member-email-${spaceId}`} value={email} onChange={(event) => setEmail(event.target.value)} />
        </label>
        <Select id={`admin-space-member-role-select-${spaceId}`} value={role} onValueChange={(next) => setRole(next as DirectorySpaceRole)}>
          <SelectTrigger id={`admin-space-member-role-${spaceId}`} className="w-32">
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
            client.command({ kind: "upsert-member", spaceId, email: email.trim(), role }).then(() => {
              setEmail("");
              onChanged();
            });
          }}
        />
      </div>
      <div className="flex items-center gap-single">
        <Button
          id={`admin-space-invite-create-${spaceId}`}
          icon="link"
          text={t("admin.spaces.inviteCreate")}
          variant="ghost"
          onClick={() => {
            client.command({ kind: "create-invite", spaceId, role, ttlSecs: INVITE_TTL_SECS }).then((response) => {
              const token = (response.result as { inviteToken?: string } | undefined)?.inviteToken;
              setInviteToken(token ?? null);
            });
          }}
        />
        {inviteToken ? (
          <span className="flex items-center gap-single text-sm text-muted-foreground">
            <code data-slot="admin-space-invite-token">{inviteToken}</code>
            <Button
              id={`admin-space-invite-copy-${spaceId}`}
              icon="copy"
              text={t("admin.spaces.inviteCopy")}
              variant="ghost"
              onClick={() => {
                void navigator.clipboard?.writeText(inviteToken);
              }}
            />
          </span>
        ) : null}
      </div>
    </div>
  );
}
//#endregion 🔖️MembersPanel

/** 🏛️ `GET /admin/api/spaces` as a `Table`, a create dialog, per-row rename/visibility/archive/delete
 * actions (all via `POST /admin/api/commands`, actor kind `admin` — bypasses the member-authorship
 * authz `/directory/commands` enforces, per contract §C2), and an expandable members sub-table
 * (`GET /admin/api/spaces/{id}`) with upsert-by-email, remove, and an invite-link action. Rows carry
 * `data-row-id="space:<id>"` per contract §C0's test-id grammar. */
export function SpacesPage(): React.ReactElement {
  const t = useAdminT();
  const { client } = useAdminSession();
  const [spaces, setSpaces] = React.useState<SpaceView[] | null>(null);
  const [expandedId, setExpandedId] = React.useState<string | null>(null);
  const [detail, setDetail] = React.useState<AdminSpaceDetail | null>(null);
  const [createOpen, setCreateOpen] = React.useState(false);

  const loadSpaces = React.useCallback(() => {
    client.spaces().then((rows) => setSpaces([...rows])).catch(() => setSpaces([]));
  }, [client]);

  React.useEffect(loadSpaces, [loadSpaces]);

  const loadDetail = React.useCallback(
    (spaceId: string) => {
      client.space(spaceId).then(setDetail).catch(() => setDetail(null));
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

  const columns: TableColumn<SpaceView>[] = [
    { id: "name", header: t("admin.spaces.name"), accessor: (row) => row.name },
    { id: "kind", header: t("admin.spaces.kind"), accessor: (row) => row.kind },
    { id: "visibility", header: t("admin.spaces.visibility"), accessor: (row) => (row.visibility === "public" ? t("admin.spaces.visibilityPublic") : t("admin.spaces.visibilityPrivate")) },
    { id: "members", header: t("admin.spaces.members"), accessor: (row) => row.memberCount },
    { id: "documents", header: t("admin.spaces.documents"), accessor: (row) => row.documentCount },
    {
      id: "actions",
      header: t("admin.spaces.actions"),
      accessor: (row) => (
        <div className="flex items-center gap-single" onClick={(event) => event.stopPropagation()}>
          <Button
            id={`admin-space-rename-${row.id}`}
            icon="pencil"
            text={t("admin.spaces.rename")}
            variant="ghost"
            onClick={() => {
              const next = window.prompt(t("admin.spaces.rename"), row.name);
              if (next && next.trim().length > 0) client.command({ kind: "rename-space", spaceId: row.id, name: next.trim() }).then(refreshExpanded);
            }}
          />
          <Button
            id={`admin-space-visibility-${row.id}`}
            icon="eye"
            text={t("admin.spaces.setVisibility")}
            variant="ghost"
            onClick={() => {
              const next: DirectorySpaceVisibility = row.visibility === "public" ? "private" : "public";
              client.command({ kind: "set-visibility", spaceId: row.id, visibility: next }).then(refreshExpanded);
            }}
          />
          <Button
            id={`admin-space-archive-${row.id}`}
            icon="trash"
            text={t("admin.spaces.archive")}
            variant="ghost"
            onClick={() => {
              if (window.confirm(t("admin.spaces.confirmArchive", { name: row.name }))) client.command({ kind: "archive-space", spaceId: row.id }).then(refreshExpanded);
            }}
          />
          <Button
            id={`admin-space-delete-${row.id}`}
            icon="x"
            text={t("admin.spaces.delete")}
            variant="ghost"
            onClick={() => {
              if (window.confirm(t("admin.spaces.confirmDelete", { name: row.name }))) client.command({ kind: "delete-space", spaceId: row.id }).then(refreshExpanded);
            }}
          />
        </div>
      ),
    },
  ];

  return (
    <div className="flex h-full w-full flex-col gap-single overflow-auto p-single">
      <div className="flex items-center justify-between">
        <h1 className="text-lg font-semibold text-emphasized">{t("admin.spaces.title")}</h1>
        <Button id="admin-space-create-open" icon="plus" text={t("admin.spaces.create")} onClick={() => setCreateOpen(true)} />
      </div>
      <Table
        columns={columns}
        data={spaces ?? []}
        emptyMessage={uiDataLabel(t("admin.spaces.empty"))}
        getRowId={(row) => `space:${row.id}`}
        rowDragProps={(row) => ({ "data-row-id": `space:${row.id}` }) as React.HTMLAttributes<HTMLTableRowElement>}
        onRowClick={(row) => toggleExpanded(row.id)}
        selectedRows={expandedId ? new Set([`space:${expandedId}`]) : undefined}
      />
      {expandedId && detail ? <MembersPanel spaceId={expandedId} detail={detail} onChanged={refreshExpanded} /> : null}
      <CreateSpaceDialog open={createOpen} onOpenChange={setCreateOpen} onCreate={(name, kind, visibility) => client.command({ kind: "create-space", name, spaceKind: kind, visibility }).then(loadSpaces)} />
    </div>
  );
}
