/** 🧭️ Live directory event page and command helpers shared by the directory process gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { createHash } from "node:crypto";
import { type LocalHubRun } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { liveDirectoryCommandRequestId, postLiveDirectoryCommand } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";

export type LiveDirectoryEventPageV1 = {
  readonly schema: "semio.directory.event-page.v1";
  readonly sessionBindingSha256: string;
  readonly authorizationGeneration: number;
  readonly afterSeqExclusive: number;
  readonly throughSeqInclusive: number;
  readonly hasMore: boolean;
  readonly events: readonly Record<string, any>[];
  readonly receiptSha256: string;
};

function liveDirectoryEventPageBinding(envelope: Record<string, any>, user: Record<string, any>): string {
  const session = Buffer.from(String(envelope.sessionId), "utf8");
  const userId = Buffer.from(String(user.userId), "utf8");
  const u32 = (value: number): Buffer => {
    const bytes = Buffer.alloc(4);
    bytes.writeUInt32BE(value);
    return bytes;
  };
  const u64 = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigUInt64BE(BigInt(value));
    return bytes;
  };
  const i64 = (value: number): Buffer => {
    const bytes = Buffer.alloc(8);
    bytes.writeBigInt64BE(BigInt(value));
    return bytes;
  };
  return createHash("sha256")
    .update(Buffer.concat([Buffer.from("semio/hub/directory-event-page/session-binding/v1\0"), u32(session.length), session, u32(userId.length), userId, u64(user.authorizationGeneration), i64(user.expiresAt)]))
    .digest("hex");
}

export async function liveDirectoryEventPageUser(run: LocalHubRun, envelope: Record<string, any>): Promise<Record<string, any>> {
  const response = await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, {
    headers: { authorization: `Bearer ${envelope.capability}` },
    signal: AbortSignal.timeout(2_000),
  });
  const user = (await response.json().catch(() => undefined)) as Record<string, any> | undefined;
  if (
    !response.ok ||
    !user ||
    typeof user.userId !== "string" ||
    user.userId.length === 0 ||
    typeof user.displayName !== "string" ||
    user.displayName.length === 0 ||
    user.expiresAt !== envelope.expiresAt ||
    user.authorizationGeneration !== envelope.authorizationGeneration
  ) {
    throw new Error("directory event page process session identity did not match its inherited envelope");
  }
  return user;
}

export async function fetchLiveDirectoryEventPage(run: LocalHubRun, envelope: Record<string, any>, user: Record<string, any>, after: number): Promise<LiveDirectoryEventPageV1> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/event-page/v1?after=${after}`, {
    headers: { authorization: `Bearer ${envelope.capability}` },
    signal: AbortSignal.timeout(2_000),
  });
  const source = await response.text();
  if (!response.ok || response.headers.get("content-type")?.split(";", 1)[0] !== "application/json" || Buffer.byteLength(source, "utf8") > 64 * 1024) {
    throw new Error(`directory event page process read failed: ${response.status}:${Buffer.byteLength(source, "utf8")}`);
  }
  const page = JSON.parse(source) as LiveDirectoryEventPageV1;
  if (JSON.stringify(page) !== source || JSON.stringify(Object.keys(page)) !== JSON.stringify(["schema", "sessionBindingSha256", "authorizationGeneration", "afterSeqExclusive", "throughSeqInclusive", "hasMore", "events", "receiptSha256"])) {
    throw new Error("directory event page process response was not exact canonical JSON");
  }
  const unsigned = {
    schema: page.schema,
    sessionBindingSha256: page.sessionBindingSha256,
    authorizationGeneration: page.authorizationGeneration,
    afterSeqExclusive: page.afterSeqExclusive,
    throughSeqInclusive: page.throughSeqInclusive,
    hasMore: page.hasMore,
    events: page.events,
  };
  const receipt = createHash("sha256").update(JSON.stringify(unsigned)).digest("hex");
  if (
    page.schema !== "semio.directory.event-page.v1" ||
    page.afterSeqExclusive !== after ||
    page.authorizationGeneration !== user.authorizationGeneration ||
    page.sessionBindingSha256 !== liveDirectoryEventPageBinding(envelope, user) ||
    page.receiptSha256 !== receipt
  ) {
    throw new Error("directory event page process receipt/session binding did not verify independently");
  }
  return page;
}

export async function submitLiveDirectoryCommand(run: LocalHubRun, envelope: Record<string, any>, command: Record<string, unknown>): Promise<readonly Record<string, any>[]> {
  const requestId = liveDirectoryCommandRequestId();
  const { status, text } = await postLiveDirectoryCommand(run, envelope.capability, requestId, command);
  const receipt = status === 202 ? (JSON.parse(text) as Record<string, any>) : undefined;
  if (!receipt || receipt.schema !== "semio.directory.command-receipt.v1" || receipt.requestId !== requestId || receipt.outcome !== "accepted" || !Array.isArray(receipt.events) || receipt.events.length === 0) {
    throw new Error(`directory process command failed: ${status}`);
  }
  return receipt.events;
}

export function createdLiveDirectorySpace(events: readonly Record<string, any>[]): string {
  const id = events.find((event) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (typeof id !== "string" || id.length === 0) throw new Error("directory event page process create-space result lacked its exact id");
  return id;
}
