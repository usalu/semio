// #region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — Repo server library: PostgreSQL, auth, events, parsing (Next.js API routes).
// #endregion 🧲️Header


import { insertDiscordDelivery, insertEvent, newId, type Event } from "../🗄️persistence/🟦️.ts";

// #region 🔖️events
// #region 🌡️Publish
// Publish an event: persist to DB and queue Discord delivery.

export async function publishEvent(kind: string, source: string, payload: unknown): Promise<Event> {
  const event: Event = {
    id: newId(),
    kind,
    source,
    payload_json: payload,
    created_at: new Date(),
  };
  await insertEvent(event);
  // Queue Discord delivery for every event
  try {
    await insertDiscordDelivery(event.id, "");
  } catch {
    // Non-critical - event is already persisted
  }
  return event;
}
// #endregion 🌡️Publish

// #region 🔷️Discord
// Discord notification helpers.

const DISCORD_WEBHOOK = process.env.DISCORD_WEBHOOK_URL || "";

export async function sendDiscordMessage(title: string, body: string): Promise<boolean> {
  if (!DISCORD_WEBHOOK) return false;
  try {
    const response = await fetch(DISCORD_WEBHOOK, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ content: `${title}\n${body}` }),
      signal: AbortSignal.timeout(5000),
    });
    return response.ok;
  } catch {
    return false;
  }
}

// 📡️Route event kind to Discord channel tier.
export function getDiscordChannel(eventKind: string): string {
  if (eventKind.startsWith("ticket.")) return "#tickets";
  if (eventKind.includes("warning") || eventKind.includes("breach")) return "#quality";
  if (eventKind.startsWith("goal.")) return "#goals";
  if (eventKind.startsWith("checkpoint")) return "#operations";
  return "#activity";
}
// #endregion 🔷️Discord
// #endregion 🔖️events
