// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Ticket lifecycle API: open, close, reopen, list, detail.

// Specs:
// - POST /api/v1/ticket with action field dispatches open/close/reopen.
// - GET /api/v1/ticket lists tickets, optionally filtered by status.
// - All mutating operations require authenticated trusted developer.
// - Events are published for all lifecycle transitions.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { NextRequest, NextResponse } from "next/server";
import { parseCommandAction, parseTicketCloseRequest, parseTicketOpenRequest, parseTicketReopenRequest } from "../../../../../../🧬️schema/🟦️.ts";
import { upsertTicket, getTicket, listTickets, insertTicketFiles, listClaimsByTicket, type Ticket } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";
// #endregion 🔌️Adapters

// 🎯️#region 🪄️Handlers
export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const status = request.nextUrl.searchParams.get("status") || undefined;
  const tickets = await listTickets(status);
  return NextResponse.json(tickets);
}

export async function POST(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  let body: unknown;
  try {
    body = await request.json();
  } catch {
    return NextResponse.json({ error: "invalid JSON" }, { status: 400 });
  }

  const actionCheck = parseCommandAction(body);
  if (!actionCheck.success) {
    return NextResponse.json({ error: actionCheck.error.message }, { status: 400 });
  }

  switch (actionCheck.data) {
    case "open": {
      const parsed = parseTicketOpenRequest(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const now = new Date();
      const ticket: Ticket = {
        id: data.ticket_id,
        status: "open",
        title: data.title,
        prompt: data.prompt,
        summary: "",
        llm: data.llm,
        client: data.client,
        author: data.author,
        github_issue: data.github_issue,
        goal: data.goal,
        parent: data.parent,
        created_at: now,
        closed_at: null,
      };
      await upsertTicket(ticket);
      await publishEvent("TicketOpened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    case "close": {
      const parsed = parseTicketCloseRequest(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json({ error: "ticket not found" }, { status: 404 });
      }
      const now = new Date();
      ticket.status = "closed";
      ticket.summary = data.summary;
      ticket.closed_at = now;
      await upsertTicket(ticket);
      if (data.files.length > 0) {
        await insertTicketFiles(data.ticket_id, data.files);
      }
      await publishEvent("TicketClosed", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    case "reopen": {
      const parsed = parseTicketReopenRequest(body);
      if (!parsed.success) {
        return NextResponse.json({ error: parsed.error.message }, { status: 400 });
      }
      const data = parsed.data;
      const ticket = await getTicket(data.ticket_id);
      if (!ticket) {
        return NextResponse.json({ error: "ticket not found" }, { status: 404 });
      }
      ticket.status = "open";
      ticket.prompt = data.prompt;
      ticket.llm = data.llm;
      if (data.title) ticket.title = data.title;
      if (data.client) ticket.client = data.client;
      ticket.closed_at = null;
      await upsertTicket(ticket);
      await publishEvent("TicketReopened", "repo-cli", ticket);
      return NextResponse.json(ticket);
    }

    default:
      return NextResponse.json({ error: "unknown action" }, { status: 400 });
  }
}
// #endregion 🪄️Handlers
