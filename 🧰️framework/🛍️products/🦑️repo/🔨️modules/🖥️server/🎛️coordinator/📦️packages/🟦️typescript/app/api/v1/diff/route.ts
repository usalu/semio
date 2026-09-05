// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Diff ingestion API: parses patches, indexes changed files, maps claims, detects conflicts.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { NextRequest, NextResponse } from "next/server";
import { ownedSchema as z } from "../../../../✅️validation.ts";
// #endregion 🔌️Adapters

import { replaceScopes, upsertClaim, listConflicts, replaceWarnings, newId, type Warning, type Scope } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
import { parseUnifiedDiff, buildScopesForFile } from "@/lib";
import { readFileSync } from "fs";
import { join } from "path";

const REPO_ROOT = process.env.COMPOSE_SERVER_REPO_ROOT || process.cwd();

const DiffIngestSchema = z.object({
  ticket_id: z.string().min(1),
  repo_id: z.string().default(""),
  patch: z.string().min(1),
  snapshots: z.array(z.object({ path: z.string(), content: z.string() })).default([]),
});

export async function POST(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return NextResponse.json({ error: "invalid JSON" }, { status: 400 });
  }

  const parsed = DiffIngestSchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.message }, { status: 400 });
  }

  const { ticket_id, patch, snapshots } = parsed.data;
  const diffFiles = parseUnifiedDiff(patch);
  const changedFiles = [...new Set(diffFiles.map((f) => f.path).filter(Boolean))];

  await publishEvent("DiffIngested", "repo-cli", {
    ticket_id,
    files: changedFiles,
  });

  // 📸️Build snapshot map
  const contentByFile: Record<string, string> = {};
  for (const s of snapshots) {
    contentByFile[s.path] = s.content;
  }

  // ♻️Index changed files
  const allScopes: Scope[] = [];
  for (const file of changedFiles) {
    let content = contentByFile[file];
    if (!content) {
      try {
        content = readFileSync(join(REPO_ROOT, file), "utf-8");
      } catch {
        continue;
      }
    }
    const scopes = buildScopesForFile(file, content);
    await replaceScopes(file, scopes);
    allScopes.push(...scopes);
  }

  // 🗺️Map claims
  const claimedIds: string[] = [];
  for (const diffFile of diffFiles) {
    if (!diffFile.path) continue;
    const fileScopes = allScopes.filter((s) => s.file_path === diffFile.path);
    for (const hunk of diffFile.hunks) {
      if (hunk.newRange.end === 0) continue;
      for (const scope of fileScopes) {
        if (scope.start_line === 0 && scope.end_line === 0) continue;
        if (hunk.newRange.start <= scope.end_line && scope.start_line <= hunk.newRange.end) {
          if (scope.kind === "definition" || scope.kind === "section") {
            if (!claimedIds.includes(scope.id)) {
              claimedIds.push(scope.id);
            }
            await upsertClaim(ticket_id, scope.id, "touched");
          }
        }
      }
    }
  }

  // 🔷️Detect conflicts
  const conflicts = await listConflicts();
  const warnings: Warning[] = conflicts.map((c) => ({
    id: newId(),
    kind: "conflict",
    severity: "error",
    message: `conflict on ${c.scope_id} across tickets ${c.tickets.join(", ")}`,
    ticket_id: "",
    scope_id: c.scope_id,
    created_at: new Date(),
    acknowledged_at: null,
    ack_by: "",
  }));
  await replaceWarnings(warnings);

  const blockers = warnings.filter((w) => w.severity === "error").map((w) => w.message);

  return NextResponse.json({
    changed_files: changedFiles,
    claimed_scopes: claimedIds,
    warnings,
    breachs: [],
    blockers,
  });
}
