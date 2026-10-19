/** 🧰️ The scenario harness: the sign in, timed fetch, directory command and socket helpers the phases share.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | the two-client document e2e}: a phase of the scenario over the same bindings, moved verbatim. */
import { withLocalHubHostTaskV1, startLocalHub } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { setTimeout as sleep } from "node:timers/promises";
import { expect } from "vitest";
import type { Frame, Holder, Minted } from "./📐️scenario-types/🟦️.ts";

/** 🧪️ Waits for the hub and builds the sign in, timed fetch and directory command helpers. */
export async function prepareScenarioHarness({ waitReady, run, origin, context, directoryCommandRequestJson, sealDirectoryCommandRequestV1 }: {
  readonly waitReady: (hubRun: Awaited<ReturnType<typeof startLocalHub>>, label: string) => Promise<Record<string, any>>;
  readonly run: Awaited<ReturnType<typeof startLocalHub>>;
  readonly origin: string;
  readonly context: Parameters<Parameters<typeof withLocalHubHostTaskV1>[2]>[0];
  readonly directoryCommandRequestJson: any;
  readonly sealDirectoryCommandRequestV1: any;
}) {
  await waitReady(run, "boot");
  const signIn = async (email: string, password: string, device: string) => {
    const res = await fetchTimed(`${origin}/auth/sessions`, {
      method: "POST",
      headers: { "content-type": "application/json", origin: "http://127.0.0.1:6066" },
      body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: device, clientClass: "browser" }),
    });
    expect(res.status).toBe(200);
    return ((await res.json()) as { token: string }).token;
  };

  const fetchTimed = async (url: string, init: RequestInit = {}, ms = 30_000) => {
    const res = await fetch(url, { ...init, signal: AbortSignal.any([context.control.signal,AbortSignal.timeout(ms)]) });
    return res;
  };

  const cmd = async (token: string, body: unknown) => {
    const id = [...crypto.getRandomValues(new Uint8Array(16))].map((x) => x.toString(16).padStart(2, "0")).join("");
    const sealed = directoryCommandRequestJson(sealDirectoryCommandRequestV1(id, body as any));
    for (let attempt = 1; ; attempt += 1) {
      const res = await fetchTimed(`${origin}/directory/commands`, {
        method: "POST",
        headers: { "content-type": "application/json", authorization: `Bearer ${token}`, origin: "http://127.0.0.1:6066" },
        body: sealed,
      });
      if (res.status === 429 && attempt < 20) {
        await new Promise((resolveDelay) => setTimeout(resolveDelay, Math.max(100, Number(res.headers.get("retry-after") ?? "1") * 1000)));
        continue;
      }
      const text = await res.text();
      return { status: res.status, json: text ? JSON.parse(text) : null };
    }
  };
  return { signIn, cmd, fetchTimed };
}

/** 🔧️ Builds the frame wait, grant mint and document socket open helpers. */
export function prepareSocketHelpers({ scenario, run, fetchTimed, origin, parseDocumentSocketGrantReceiptV1, wsOrigin, decodeServerFrame, encodeClientFrame }: {
  readonly scenario: { directoryTimings: { spaceListMs: number; directoryPageMs: number[]; }; spaceId: string; documentId: string; surfaceId: string; artifactSchema: string; packSchemaHashHex: string; hubOutput: () => string; };
  readonly run: Awaited<ReturnType<typeof startLocalHub>>;
  readonly fetchTimed: (url: string, init?: RequestInit, ms?: number) => Promise<Response>;
  readonly origin: string;
  readonly parseDocumentSocketGrantReceiptV1: any;
  readonly wsOrigin: string;
  readonly decodeServerFrame: any;
  readonly encodeClientFrame: any;
}) {
  scenario.hubOutput = (): string => run.output();
  const waitFrame = async (holder: Holder, pred: (f: Frame) => boolean, label: string, ms = 15000) => {
    const until = Date.now() + ms;
    const hit = holder.frames.find(pred);
    if (hit) return hit;
    return await new Promise<Frame>((resolve, reject) => {
      const onFrame = (frame: Frame) => {
        if (!pred(frame)) return;
        clearInterval(timer);
        holder.waiters = holder.waiters.filter((w) => w !== onFrame);
        resolve(frame);
      };
      holder.waiters.push(onFrame);
      const timer = setInterval(() => {
        if (Date.now() < until) return;
        clearInterval(timer);
        holder.waiters = holder.waiters.filter((w) => w !== onFrame);
        const kinds = [...new Set(holder.frames.map((f) => Object.keys(f)[0]))].join(",");
        const errors = holder.frames.filter((f) => "Error" in f).map((f) => JSON.stringify(f.Error));
        reject(new Error(`${holder.label} missing ${label} within ${ms} ms: ${kinds} (n=${holder.frames.length}) errors=${errors.join(";")}\nhub-output:\n${scenario.hubOutput().slice(-8000)}`));
      }, 100);
    });
  };


  const mint = async (label: string, token: string): Promise<Minted> => {
    const headers = { authorization: `Bearer ${token}`, "content-type": "application/json" };
    const planRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(scenario.spaceId)}/documents/${encodeURIComponent(scenario.documentId)}/open-plan`, {
      method: "POST",
      headers,
      body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId: scenario.spaceId, documentId: scenario.documentId }, requestedSurfaceId: scenario.surfaceId, clientInstanceId: `tc-${label}` }),
    });
    const planText = await planRes.text();
    expect(planRes.status, planText.slice(0, 200)).toBe(200);
    const plan = JSON.parse(planText);
    const mintedSurface = plan.surface?.surfaceId ?? scenario.surfaceId;
    const mintedSchema = plan.artifact.schema;
    const mintedHash = plan.artifact.packSchemaHash;
    const grantRes = await fetchTimed(`${origin}/spaces/${encodeURIComponent(scenario.spaceId)}/documents/${encodeURIComponent(scenario.documentId)}/socket-grants`, {
      method: "POST",
      headers,
      body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt }),
    });
    const grantText = await grantRes.text();
    expect(grantRes.status, grantText.slice(0, 200)).toBe(200);
    const receipt = parseDocumentSocketGrantReceiptV1(JSON.parse(grantText));
    const scope = `${scenario.spaceId}/${scenario.documentId}`;
    const wsUrl = `${wsOrigin}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(mintedSurface)}`;
    return {
      label,
      token,
      actor: receipt.actorId,
      surfaceId: mintedSurface,
      artifactSchema: mintedSchema,
      packSchemaHashHex: mintedHash,
      wsUrl,
      protocols: ["semio.session.v1", token],
    };
  };

  const openSocket = async (minted: Minted, resumeToken: string | null = null, admitWithinMs = 15000): Promise<Holder> => {
    let closeCode: number | null = null;
    let closeReason = "";
    const socket = new WebSocket(minted.wsUrl, [...minted.protocols]);
    socket.binaryType = "arraybuffer";
    const closed = new Promise<{ code: number; reason: string }>((resolve) =>
      socket.addEventListener("close", (event) => {
        closeCode = event.code;
        closeReason = event.reason;
        resolve({ code: event.code, reason: event.reason });
      }),
    );
    const holder: Holder = { label: minted.label, actor: minted.actor, socket, frames: [], waiters: [], closed };
    socket.addEventListener("message", (event) => {
      if (!(event.data instanceof ArrayBuffer)) return;
      try {
        const frame = decodeServerFrame(new Uint8Array(event.data)).frame as Frame;
        if ("Welcome" in frame) {
          holder.welcome = frame.Welcome;
          holder.resumeToken = frame.Welcome.resume_token;
        }
        // Cap retained frames — framework document lane can flood heartbeats and starve the event loop.
        holder.frames.push(frame);
        if (holder.frames.length > 64) holder.frames.splice(0, holder.frames.length - 32);
        for (const waiter of [...holder.waiters]) waiter(frame);
      } catch { /* ignore */ }
    });
    for (let i = 0; i < 200 && socket.readyState === WebSocket.CONNECTING; i++) await sleep(50);
    expect(
      socket.readyState,
      `${minted.label} socket closed before open code=${closeCode} reason=${closeReason} protocols=${JSON.stringify(minted.protocols)} url=${minted.wsUrl}`,
    ).toBe(WebSocket.OPEN);
    const packSchemaHash = [...(minted.packSchemaHashHex.match(/../gu) ?? [])].map((p) => Number.parseInt(p, 16));
    socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: minted.artifactSchema, pack_schema_hash: packSchemaHash, resume_token: resumeToken, frontier: null } }, "command"));
    const admitted = await waitFrame(holder, (f) => "Welcome" in f || "Error" in f, "Welcome", admitWithinMs);
    if ("Error" in admitted) {
      socket.close();
      throw new Error(`${minted.label} refused: ${JSON.stringify(admitted.Error)}`);
    }
    expect(holder.welcome.server_frontier.document_id ?? holder.welcome.server_frontier.documentId).toBe(scenario.documentId);
    return holder;
  };
  return { mint, openSocket, waitFrame };
}
