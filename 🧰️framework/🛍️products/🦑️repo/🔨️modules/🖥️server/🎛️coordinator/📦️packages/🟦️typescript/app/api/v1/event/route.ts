// #region 🧲️Header
// 2025 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0
// Event ingestion and listing API.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { NextRequest, NextResponse } from "next/server";
import { ownedSchema as z } from "../../../../🟦️validation";
// #endregion 🔌️Adapters

import { listEvents } from "@/lib";
import { requireAuth, isAuthError } from "@/lib";
import { publishEvent } from "@/lib";

const EventSchema = z.object({
  kind: z.string().min(1),
  source: z.string().default(""),
  payload: z.unknown().default({}),
});

export async function GET(request: NextRequest) {
  const auth = await requireAuth(request);
  if (isAuthError(auth)) return auth;

  const kind = request.nextUrl.searchParams.get("kind") || undefined;
  const limit = parseInt(request.nextUrl.searchParams.get("limit") || "100");
  const events = await listEvents(kind, limit);
  return NextResponse.json(events);
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

  const parsed = EventSchema.safeParse(body);
  if (!parsed.success) {
    return NextResponse.json({ error: parsed.error.message }, { status: 400 });
  }

  const event = await publishEvent(parsed.data.kind, parsed.data.source, parsed.data.payload);
  return NextResponse.json({ status: "ok", event_id: event.id });
}
