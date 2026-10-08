import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "../../🚪️io/📝️text/🟦️.ts";
import { randomBytes } from "node:crypto";
import { decodeServerFrame, encodeClientFrame } from "../../../../../../🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "../../../../🟦️.ts";
import { createSpaceCommandV1 } from "../../🏘️spaces/🟦️.ts";

import { sealSpaceArtifactCreateV1 } from "../../🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

//#region 🔖️ProbeClient
/** 🧑‍💻️ A hub probe client acting exactly as a signed-in human's browser does over HTTP and the document socket: credential
 * sign-in, the directory's `create-space` command, the server-owned artifact creation, the open plan, and chained edits
 * over one document socket. Shared by the hub's operational drills (backup/restore, residency). */
export type HubProbeAnswer = { readonly status: number; readonly text: string; readonly json: any; readonly bytes: Uint8Array };

/** 🌐️ One JSON call against `origin`, bounded to two minutes. */
export async function hubProbeCall(origin: string, method: string, path: string, token?: string, body?: string, accept?: string): Promise<HubProbeAnswer> {
  const response = await fetch(`${origin}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}), ...(accept ? { accept } : {}) }, ...(body === undefined ? {} : { body }), signal: AbortSignal.timeout(120_000) });
  const bytes = new Uint8Array(await response.arrayBuffer());
  const text = new TextDecoder().decode(bytes);
  let json: any = null;
  try {
    json = JSON.parse(text);
  } catch {
    json = null;
  }
  return { status: response.status, text, json, bytes };
}

/** 🔑️ Credential sign-in as a browser client; answers the session token. */
export async function hubProbeSignIn(origin: string, email: string, password: string, device: string): Promise<string> {
  const answer = await hubProbeCall(origin, "POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email, password, deviceInstanceId: `${device}${randomBytes(10).toString("hex")}`, clientClass: "browser" }));
  if (answer.status !== 200 || typeof answer.json?.token !== "string") throw new Error(`sign-in ${answer.status} ${answer.text.slice(0, 200)}`);
  return answer.json.token;
}

/** 🏘️ Creates a private studio space through the directory command and answers its id. */
export async function hubProbeCreateSpace(origin: string, token: string, name: string): Promise<string> {
  const answer = await hubProbeCall(origin, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(name, "studio", "private"))));
  const spaceId = answer.json?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (answer.status !== 202 || typeof spaceId !== "string") throw new Error(`create-space ${answer.status} ${answer.text.slice(0, 300)}`);
  return spaceId;
}

/** 🗂️ The space's creation catalog: its generation and creatable kinds. */
export async function hubProbeCreationCatalog(origin: string, token: string, spaceId: string): Promise<{ generationId: string; kinds: { kindId: string; schema: string }[] }> {
  const answer = await hubProbeCall(origin, "GET", `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`, token);
  if (answer.status !== 200) throw new Error(`creation catalog ${answer.status} ${answer.text.slice(0, 300)}`);
  return { generationId: String(answer.json?.catalogGenerationId ?? ""), kinds: (answer.json?.kinds ?? []) as { kindId: string; schema: string }[] };
}

/** 🌱️ Runs the server-owned creation of one `kindId` document to `ready` and answers its artifact id and duration. */
export async function hubProbeCreateArtifact(origin: string, token: string, spaceId: string, generationId: string, kindId: string, name: string, budgetMs = 1_800_000): Promise<{ artifactId: string; ms: number }> {
  const route = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const requestId = randomBytes(16).toString("hex");
  const started = Date.now();
  const accepted = await hubProbeCall(origin, "POST", route, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: generationId, kindId, name })));
  if (accepted.status !== 202) throw new Error(`creation ${accepted.status} ${accepted.text.slice(0, 300)}`);
  let state = accepted.json;
  while (["accepted", "preparing", "indeterminate"].includes(state?.phase) && Date.now() - started < budgetMs) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 500));
    state = (await hubProbeCall(origin, "GET", `${route}/${requestId}`, token)).json;
  }
  if (state?.phase !== "ready") throw new Error(`creation of ${kindId} ended ${JSON.stringify(state).slice(0, 300)}`);
  return { artifactId: String(state.ready.artifactId), ms: Date.now() - started };
}

/** 🧭️ Asks for the editor open plan of one document (the hub loads the kind's package to answer it). */
export async function hubProbeOpenPlan(origin: string, token: string, spaceId: string, documentId: string, client: string): Promise<HubProbeAnswer> {
  return hubProbeCall(origin, "POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, token, JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, clientInstanceId: client }));
}

/** 📡️ One open document socket: the plan it was opened under, its Welcome frame, one opaque command batch answered by its
 * `Ack` (`submit`, accepted or not — a crafted write of a read-only subject is refused here), chained opaque edits that must
 * be accepted, when another client's mutation reached this socket in a relayed `Commands` frame (`relayed`, epoch ms), how
 * many server frames this tree's wire codec could not decode (a hub built from another tree), how the socket ended when the
 * hub ended it (`ended`: its close code and whether the closing handshake completed, `null` while open past the budget), the
 * relayed envelopes themselves (`relayedEnvelopes`), any batch of envelopes sent as this socket's own actor and answered by
 * its `Ack` (`submitEnvelopes` — how a crafted history transition is tried), and close. */
export type HubProbeDocument = Readonly<{ plan: any; actorId: string; welcome: any; submit: (index: number, previous: string) => Promise<{ mutationId: string; accepted: boolean; ack: any }>; submitEnvelopes: (batchId: number, envelopes: readonly any[]) => Promise<{ accepted: boolean; ack: any }>; relayedEnvelopes: () => readonly any[]; edit: (index: number, previous: string) => Promise<string>; relayed: (mutationId: string, budgetMs?: number) => Promise<number>; undecodableFrames: () => number; ended: (budgetMs?: number) => Promise<{ code: number; clean: boolean; reason: string } | null>; close: () => void }>;

/** 📡️ Opens one document over the plan → socket grant → socket hello path and answers once it is welcomed. */
export async function hubProbeOpenDocument(origin: string, token: string, spaceId: string, documentId: string, client: string): Promise<HubProbeDocument> {
  const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
  const plan = await hubProbeOpenPlan(origin, token, spaceId, documentId, client);
  if (plan.status !== 200) throw new Error(`open-plan ${plan.status} ${plan.text.slice(0, 300)}`);
  const grant = await hubProbeCall(origin, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
  if (grant.status !== 200) throw new Error(`socket-grant ${grant.status} ${grant.text.slice(0, 300)}`);
  const granted = parseDocumentSocketGrantReceiptV1(grant.json);
  const socket = new WebSocket(`${origin.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
  socket.binaryType = "arraybuffer";
  const closing = new Promise<{ code: number; clean: boolean; reason: string }>((resolveClose) => socket.addEventListener("close", (event) => resolveClose({ code: event.code, clean: event.wasClean, reason: event.reason })));
  const frames: any[] = [];
  const waiters: ((frame: any) => void)[] = [];
  const relayedAt = new Map<string, number>();
  const relayedEnvelopes: any[] = [];
  const relayWaiters = new Set<() => void>();
  let undecodable = 0;
  socket.addEventListener("message", (event) => {
    if (!(event.data instanceof ArrayBuffer)) return;
    let frame: any;
    try {
      frame = decodeServerFrame(new Uint8Array(event.data)).frame;
    } catch {
      undecodable += 1;
      return;
    }
    if ("Commands" in frame) {
      for (const envelope of frame.Commands.envelopes ?? []) {
        if (!relayedAt.has(String(envelope.mutation_id))) relayedAt.set(String(envelope.mutation_id), Date.now());
        relayedEnvelopes.push(envelope);
      }
      for (const wake of [...relayWaiters]) wake();
      return;
    }
    if ("Presence" in frame) return;
    frames.push(frame);
    for (const waiter of [...waiters]) waiter(frame);
  });
  const waitFrame = (matches: (frame: any) => boolean, label: string, budgetMs = 60_000): Promise<any> =>
    new Promise((resolveFrame, rejectFrame) => {
      const hit = frames.find(matches);
      if (hit) return resolveFrame(hit);
      const onFrame = (frame: any): void => {
        if (!matches(frame)) return;
        clearTimeout(timer);
        waiters.splice(waiters.indexOf(onFrame), 1);
        resolveFrame(frame);
      };
      waiters.push(onFrame);
      const timer = setTimeout(() => rejectFrame(new Error(`missing ${label}; last frames ${JSON.stringify(frames.slice(-3)).slice(0, 400)}`)), budgetMs);
    });
  for (let tick = 0; tick < 400 && socket.readyState === WebSocket.CONNECTING; tick += 1) await new Promise((resolveDelay) => setTimeout(resolveDelay, 50));
  if (socket.readyState !== WebSocket.OPEN) throw new Error("document socket did not open");
  const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
  const welcome = await waitFrame((frame) => "Welcome" in frame || "Error" in frame, "Welcome");
  if ("Error" in welcome) throw new Error(`refused: ${JSON.stringify(welcome.Error)}`);
  const submitEnvelopes = async (batchId: number, envelopes: readonly any[]): Promise<{ accepted: boolean; ack: any }> => {
    socket.send(encodeClientFrame({ Commands: { batch_id: batchId, envelopes: envelopes.map((envelope) => ({ ...envelope, actor: granted.actorId })) } }, "command"));
    const acked = await waitFrame((frame) => "Ack" in frame && frame.Ack.batch_id === batchId, `Ack ${batchId}`);
    return { accepted: JSON.stringify(acked.Ack.stages).includes("Accepted"), ack: acked.Ack };
  };
  const submit = async (index: number, previous: string): Promise<{ mutationId: string; accepted: boolean; ack: any }> => {
    const mutationId = `probe-${documentId}-${index}`;
    socket.send(encodeClientFrame({ Commands: { batch_id: index + 1, envelopes: [{ mutation_id: mutationId, document_id: documentId, actor: granted.actorId, dependencies: previous ? [previous] : [], observed: null, target: [], diff: { schema: plan.json.artifact.schema, payload: Array.from(new TextEncoder().encode(`probe:${index}:${"b".repeat(512)}`)) }, inverse: { schema: plan.json.artifact.schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 }, transaction: null, verb: null, line: null }] } }, "command"));
    const acked = await waitFrame((frame) => "Ack" in frame && frame.Ack.batch_id === index + 1, `Ack ${index}`);
    return { mutationId, accepted: JSON.stringify(acked.Ack.stages).includes("Accepted"), ack: acked.Ack };
  };
  const edit = async (index: number, previous: string): Promise<string> => {
    const answered = await submit(index, previous);
    if (!answered.accepted) throw new Error(`edit ${index} not accepted: ${JSON.stringify(answered.ack)}`);
    return answered.mutationId;
  };
  const relayed = (mutationId: string, budgetMs = 60_000): Promise<number> =>
    new Promise((resolveRelay, rejectRelay) => {
      const settle = (): void => {
        const at = relayedAt.get(mutationId);
        if (at === undefined) return;
        clearTimeout(timer);
        relayWaiters.delete(settle);
        resolveRelay(at);
      };
      const timer = setTimeout(() => {
        relayWaiters.delete(settle);
        rejectRelay(new Error(`${mutationId} was never relayed to this socket; relayed ${JSON.stringify([...relayedAt.keys()].slice(-3))}`));
      }, budgetMs);
      relayWaiters.add(settle);
      settle();
    });
  const ended = (budgetMs = 30_000): Promise<{ code: number; clean: boolean; reason: string } | null> => Promise.race([closing, new Promise<null>((resolveOpen) => setTimeout(() => resolveOpen(null), budgetMs))]);
  return { plan: plan.json, actorId: granted.actorId, welcome: welcome.Welcome, submit, submitEnvelopes, relayedEnvelopes: () => [...relayedEnvelopes], edit, relayed, undecodableFrames: () => undecodable, ended, close: () => socket.close(1000, "probe") };
}
//#endregion 🔖️ProbeClient
