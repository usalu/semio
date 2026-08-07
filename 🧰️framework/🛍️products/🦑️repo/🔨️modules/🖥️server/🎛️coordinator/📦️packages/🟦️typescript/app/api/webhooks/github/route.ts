// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// GitHub webhook handler for issue events and push events.

// Specs:
// - Verifies HMAC-SHA256 signature when GITHUB_WEBHOOK_SECRET is set.
// - Caches issue comments for correlating close/reopen events.
// - Processes push events for contributor work tracking.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { ephemeralMap } from "@semio-tech/framework";
import { NextRequest, NextResponse } from "next/server";
import { createHmac, timingSafeEqual } from "crypto";
// #endregion 🔌️Adapters

import { publishEvent } from "@/lib";
import { removeContributorWorkForCheckpoint } from "@/lib";

const GITHUB_SECRET = process.env.GITHUB_WEBHOOK_SECRET || "";

// 💬️In-memory comment cache (same as Go server)
const commentCache = ephemeralMap<string, { body: string; actor: string; repo: string; issue: number; time: Date }>("framework.products.repo.modules.server.coordinator.packages.typescript.app.api.webhooks.github.route.ts.commentCache");

function verifySignature(body: string, signature: string): boolean {
  if (!GITHUB_SECRET) return true;
  const parts = signature.split("=");
  if (parts.length !== 2) return false;
  const mac = createHmac("sha256", GITHUB_SECRET).update(body).digest("hex");
  try {
    return timingSafeEqual(Buffer.from(mac), Buffer.from(parts[1]));
  } catch {
    return false;
  }
}

export async function POST(request: NextRequest) {
  const rawBody = await request.text();

  const signature = request.headers.get("X-Hub-Signature-256") || "";
  if (GITHUB_SECRET && !verifySignature(rawBody, signature)) {
    return NextResponse.json({ error: "invalid signature" }, { status: 401 });
  }

  const eventKind = request.headers.get("X-GitHub-Event") || "";
  let payload: Record<string, unknown>;
  try {
    payload = JSON.parse(rawBody);
  } catch {
    return NextResponse.json({ error: "invalid JSON" }, { status: 400 });
  }

  await publishEvent("GitHubIssueEventReceived", "github", { type: eventKind });

  if (eventKind === "issue_comment") {
    const issue = (payload.issue as Record<string, unknown>)?.number as number;
    const repo = (payload.repository as Record<string, unknown>)?.full_name as string;
    const actor = (payload.sender as Record<string, unknown>)?.login as string;
    const body = (payload.comment as Record<string, unknown>)?.body as string;
    if (issue && repo && actor && body) {
      const key = `${repo}#${issue}#${actor}`;
      commentCache.set(key, { body, actor, repo, issue, time: new Date() });
    }
  }

  if (eventKind === "issues") {
    const action = payload.action as string;
    const issue = (payload.issue as Record<string, unknown>)?.number as number;
    const repo = (payload.repository as Record<string, unknown>)?.full_name as string;
    const actor = (payload.sender as Record<string, unknown>)?.login as string;
    if (issue && repo && actor) {
      const key = `${repo}#${issue}#${actor}`;
      const cached = commentCache.get(key);
      if (cached && Date.now() - cached.time.getTime() < 90000) {
        if (action === "closed") {
          await publishEvent("TicketClosed", "github", {
            issue,
            comment: cached.body,
          });
        }
        if (action === "reopened") {
          await publishEvent("TicketReopened", "github", {
            issue,
            comment: cached.body,
          });
        }
      }
    }
  }

  if (eventKind === "push") {
    let actor = (payload.sender as Record<string, unknown>)?.login as string;
    if (!actor) {
      actor = (payload.pusher as Record<string, unknown>)?.name as string;
    }
    const files: string[] = [];
    const commits = payload.commits as Array<Record<string, unknown>> | undefined;
    if (commits) {
      for (const c of commits) {
        const added = c.added as string[] | undefined;
        const modified = c.modified as string[] | undefined;
        if (added) files.push(...added);
        if (modified) files.push(...modified);
      }
    }
    if (actor && files.length > 0) {
      await removeContributorWorkForCheckpoint(actor, files);
    }
  }

  return new NextResponse(null, { status: 200 });
}
