import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "@semio-tech/framework/directory/io/text";
import { type ChildProcess, spawn } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ReactShell, Wasm32Shell, type Wasm32View } from "../🖥️shells/🟦️.ts";
import { HUB_COLLABORATION_BOUNDS, HubCollaborationBlocked, type HubCollaborationOptionsV1 } from "../📋️contract/🟦️.ts";

//#region 🔖️Kinds
/** 🧩️ How the wasm32 shell reads and edits one kind's document (controls through its accessibility projection). */
type Wasm32Kind = Readonly<{
  prepare: (shell: Wasm32Shell) => Promise<void>;
  count: (view: Wasm32View) => number;
  edit: (shell: Wasm32Shell) => Promise<string>;
  undo: (shell: Wasm32Shell) => Promise<string>;
}>;

export const HANDLE_KINDS = /(\d+) (?:Handle Kinds?|Griffarten?)/u;

/** ↩️ The shell's own History panel undo (opened first when it is closed). */
async function historyUndo(shell: Wasm32Shell): Promise<string> {
  if (!(await shell.keyEndingWith("framework.history.undo.run"))) await shell.activate("framework.panel.history", 2_500);
  return shell.activate(await shell.awaitKey("framework.history.undo.run", 10_000), 4_000);
}

/** 🧩️ note: the Artifact panel lists one `note-play-block:` row per block; Add Text authors one. block2d: the board paints its
 * Handle Kinds count; the window's Actions pane runs `addHandleKind`. Undo is the shell's own History verb on both. */
const WASM32_KINDS: Readonly<Record<string, Wasm32Kind>> = {
  note: {
    prepare: async (shell) => {
      for (let attempt = 0; attempt < 3 && !(await shell.keyEndingWith("note-play-blocks.add.text")); attempt += 1) {
        if ((await shell.projection()).find((node) => node.key === "framework.panel.artifact")?.checked !== true) await shell.activate("framework.panel.artifact", 3_000);
        await shell.awaitKey("note-play-blocks.add.text", 10_000);
      }
    },
    count: (view) => view.nodes.filter((node) => String(node.key).startsWith("note-play-block:")).length,
    edit: async (shell) => shell.activate(await shell.keyEndingWith("note-play-blocks.add.text"), 4_000),
    undo: historyUndo,
  },
  block2d: {
    prepare: async (shell) => {
      if (!(await shell.keyEndingWith("action.addHandleKind"))) await shell.activate(await shell.awaitKey("framework.window.block2dBoard.engagement.toggle", 30_000), 3_000);
      await shell.awaitKey("action.addHandleKind", 30_000);
    },
    count: (view) => Number(view.texts.map((text) => HANDLE_KINDS.exec(text)?.[1]).find((value) => value !== undefined) ?? Number.NaN),
    edit: async (shell) => shell.activate(await shell.keyEndingWith("action.addHandleKind"), 4_000),
    undo: historyUndo,
  },
};

export function wasm32Kind(variant: string): Wasm32Kind {
  const kind = WASM32_KINDS[variant];
  if (!kind) throw new Error(`no wasm32 document observable for ${variant}`);
  return kind;
}

//#endregion 🔖️Kinds

//#region 🔖️Hub
/** 🏘️ The journey's document: the given one, or a fresh private space of human A with B seated as an author and one document of
 * `schema` created by the hub's own creation saga. */
export async function prepareDocument(options: HubCollaborationOptionsV1, schema: string, token: string): Promise<{ spaceId: string; documentId: string; createdMs: number }> {
  if (options.documentId) {
    if (!options.spaceId) throw new HubCollaborationBlocked("--document needs --space");
    return { spaceId: options.spaceId, documentId: options.documentId, createdMs: 0 };
  }
  const spaceId = options.spaceId ?? (await hubProbeCreateSpace(options.hub, token, `wgpu collaboration ${options.journey} ${options.tag}`));
  const member = await hubProbeCall(options.hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomRequestId(), { kind: "upsert-member", spaceId, email: options.humans[1].email, role: "author" })));
  if (member.status !== 202 && member.status !== 200) throw new Error(`seating ${options.humans[1].label} as author answered ${member.status} ${member.text.slice(0, 200)}`);
  const catalog = await hubProbeCreationCatalog(options.hub, token, spaceId);
  const kind = catalog.kinds.find((row) => row.schema === schema);
  if (!kind) throw new HubCollaborationBlocked(`the hub's creation catalog has no ${schema} kind`);
  const created = await hubProbeCreateArtifact(options.hub, token, spaceId, catalog.generationId, kind.kindId, `wgpu ${options.journey} ${options.tag}`);
  return { spaceId, documentId: created.artifactId, createdMs: created.ms };
}

function randomRequestId(): string {
  return Array.from(crypto.getRandomValues(new Uint8Array(16)), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 📏️ The hub's committed head of one document, read over the hub's REST surface. */
export async function documentHead(hub: string, token: string, spaceId: string, documentId: string): Promise<number> {
  const answer = await hubProbeCall(hub, "GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token);
  return Number(answer.json?.head_seq ?? -1);
}

//#endregion 🔖️Hub

/** 🦀️ The native wgpu user: one ignored live law of the renderer's test binary (`--native-binary`) or of `cargo test`, seated as
 * human A on the hub with human B as its peer, stepping through the file handshake in `dir`. Output → `<out>/native.txt`. */
export function spawnNativeLaw(options: HubCollaborationOptionsV1, law: string, dir: string, out: string): { child: ChildProcess; exit: Promise<number> } {
  if (!options.nativeModules) throw new HubCollaborationBlocked("the native block2d runtime directory is not staged (--native-modules)");
  const env: NodeJS.ProcessEnv = {
    ...process.env,
    SEMIO_HUB_LIVE_ORIGIN: options.hub,
    SEMIO_HUB_LIVE_EMAIL: options.humans[0].email,
    SEMIO_HUB_LIVE_PASSWORD: options.humans[0].password,
    SEMIO_HUB_LIVE_PEER_EMAIL: options.humans[1].email,
    SEMIO_CROSS_SHELL_DIR: dir,
    SEMIO_PLUGIN: "block2d",
    SEMIO_PLUGIN_MODULES: options.nativeModules,
  };
  const path = `shell::hub_projection_workspace_tests::${law}`;
  const [command, args] = options.nativeBinary
    ? [options.nativeBinary, [path, "--exact", "--ignored", "--nocapture", "--test-threads=1"]]
    : ["cargo", ["test", "-p", "semio-framework-os-renderer-wgpu", "--lib", "--", path, "--exact", "--ignored", "--nocapture", "--test-threads=1"]];
  mkdirSync(out, { recursive: true });
  const log: string[] = [];
  const child = spawn(command, args, { cwd: options.repoRoot, env, stdio: ["ignore", "pipe", "pipe"] });
  const keep = (chunk: Buffer): void => {
    log.push(chunk.toString("utf8"));
    writeFileSync(join(out, "native.txt"), log.join(""));
  };
  child.stdout!.on("data", keep);
  child.stderr!.on("data", keep);
  const exit = new Promise<number>((resolveExit) => child.on("close", (code) => resolveExit(code ?? 1)));
  options.signal.addEventListener("abort", () => child.kill("SIGTERM"), { once: true });
  return { child, exit };
}

/** 🤝️ The browser half of the native law's file handshake (`native-<step>.json` ← native, `react-<step>.json` ← browser). */
export class Handshake {
  constructor(
    readonly dir: string,
    private readonly record: (name: string, pass: boolean, detail: unknown) => void,
  ) {
    rmSync(dir, { recursive: true, force: true });
    mkdirSync(dir, { recursive: true });
  }

  publish(step: string, value: Record<string, unknown>): void {
    const staged = join(this.dir, `.react-${step}.json`);
    writeFileSync(staged, JSON.stringify(value, null, 2));
    renameSync(staged, join(this.dir, `react-${step}.json`));
  }

  async awaitNative(step: string, budgetMs: number, alive: () => boolean): Promise<Record<string, unknown> | null> {
    const path = join(this.dir, `native-${step}.json`);
    const started = Date.now();
    while (Date.now() - started < budgetMs && alive()) {
      if (existsSync(path)) return JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
      await new Promise((resolveDelay) => setTimeout(resolveDelay, 200));
    }
    this.record(`native step ${step} arrived`, false, { budgetMs, nativeAlive: alive() });
    return null;
  }
}

/** 👣️ What a browser user must do to follow the native edits law: open, see a peer, read the document count, edit, undo, reload. */
type Follower = Readonly<{
  open: (spaceId: string, documentId: string) => Promise<{ liveAfterMs: number | null; detail: unknown }>;
  seesPeer: (actor: string, userId: string) => Promise<{ afterMs: number | null; peers: unknown }>;
  count: () => Promise<number>;
  edit: () => Promise<string>;
  undo: () => Promise<string>;
  reload: (spaceId: string, documentId: string) => Promise<{ liveAfterMs: number | null }>;
}>;

async function countUntil(follower: Follower, budgetMs: number, test: (count: number) => boolean): Promise<{ afterMs: number | null; count: number }> {
  const started = Date.now();
  let count = await follower.count();
  while (!test(count) && Date.now() - started < budgetMs) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 400));
    count = await follower.count();
  }
  return { afterMs: test(count) ? Date.now() - started : null, count };
}

/** 🤝️ Follows `a_native_and_a_react_user_collaborate_on_one_hub_document` step by step; the native law judges the ledger, this
 * side records what the browser user saw. */
export async function followNativeEdits(handshake: Handshake, follower: Follower, alive: () => boolean, record: (name: string, pass: boolean, detail: unknown) => void): Promise<void> {
  const nativeOpen = await handshake.awaitNative("open", HUB_COLLABORATION_BOUNDS.nativeStepMs, alive);
  if (!nativeOpen) return;
  const [spaceId, documentId] = [String(nativeOpen.spaceId ?? ""), String(nativeOpen.documentId ?? "")];
  const opened = await follower.open(spaceId, documentId);
  handshake.publish("open", { live: opened.liveAfterMs !== null, liveAfterMs: opened.liveAfterMs });
  record("browser opens the native user's document and is live", opened.liveAfterMs !== null, { spaceId, documentId, ...(opened.detail as object) });
  const nativePresence = await handshake.awaitNative("presence", 300_000, alive);
  const seen = await follower.seesPeer(String(nativeOpen.actor ?? ""), String(nativeOpen.userId ?? ""));
  handshake.publish("presence", { seesNative: seen.afterMs !== null, afterMs: seen.afterMs });
  record("presence both ways", seen.afterMs !== null && nativePresence?.seesReact === true, { browserSeesNativeAfterMs: seen.afterMs, nativeSeesBrowser: nativePresence?.seesReact, peers: seen.peers });
  const beforeNative = await follower.count();
  await handshake.awaitNative("edit", 300_000, alive);
  const ingested = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeNative);
  handshake.publish("ingested", { ingested: ingested.afterMs !== null, afterMs: ingested.afterMs });
  record("the native edit reaches the browser", ingested.afterMs !== null, { before: beforeNative, after: ingested.count, afterMs: ingested.afterMs });
  const beforeOwn = await follower.count();
  const executed = await follower.edit();
  const own = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeOwn);
  handshake.publish("edit", { ok: own.afterMs !== null, executed });
  const nativeIngested = await handshake.awaitNative("ingested", 300_000, alive);
  record("the browser edit reaches the native user", own.afterMs !== null && nativeIngested?.ingested === true, { executed, before: beforeOwn, after: own.count, nativeIngested: nativeIngested?.ingested });
  const nativeUndo = await handshake.awaitNative("undo", 300_000, alive);
  const sawNativeUndo = await countUntil(follower, 30_000, (count) => count < own.count);
  const beforeUndo = await follower.count();
  const undo = await follower.undo();
  const undone = await countUntil(follower, 30_000, (count) => count < beforeUndo);
  handshake.publish("undo", { ok: undo === "ok" || undo === "activated", ownReverted: undone.afterMs !== null, sawNativeUndo: sawNativeUndo.afterMs !== null });
  record("each user undoes only their own edit", nativeUndo?.ownReverted === true && sawNativeUndo.afterMs !== null && undone.afterMs !== null, { nativeUndo, sawNativeUndoAfterMs: sawNativeUndo.afterMs, undo, counts: [own.count, sawNativeUndo.count, beforeUndo, undone.count] });
  const reloaded = await follower.reload(spaceId, documentId);
  const settled = await countUntil(follower, 120_000, (count) => count === undone.count);
  handshake.publish("reloaded", { live: reloaded.liveAfterMs !== null && settled.afterMs !== null, liveAfterMs: reloaded.liveAfterMs });
  const beforeLast = await follower.count();
  await handshake.awaitNative("after-reload-edit", 300_000, alive);
  const converged = await countUntil(follower, HUB_COLLABORATION_BOUNDS.editSeenMs, (count) => count > beforeLast);
  handshake.publish("converged", { converged: converged.afterMs !== null, afterMs: converged.afterMs });
  record("a reload converges and the next native edit arrives", reloaded.liveAfterMs !== null && settled.afterMs !== null && converged.afterMs !== null, { reloadLiveAfterMs: reloaded.liveAfterMs, settled: settled.count, converged: converged.count });
  await handshake.awaitNative("done", 300_000, alive);
}

/** 🖱️ Follows `a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board` in the React shell. */
export async function followNativeCursors(handshake: Handshake, react: ReactShell, alive: () => boolean, record: (name: string, pass: boolean, detail: unknown) => void): Promise<void> {
  const nativeOpen = await handshake.awaitNative("open", HUB_COLLABORATION_BOUNDS.nativeStepMs, alive);
  if (!nativeOpen) return;
  const opened = await react.open(String(nativeOpen.spaceId), String(nativeOpen.documentId), (reading) => reading.boards.some((box) => box.w > 100 && box.h > 100));
  handshake.publish("open", { live: opened.liveAfterMs !== null });
  record("React opens the native user's board and is live", opened.liveAfterMs !== null, { liveAfterMs: opened.liveAfterMs, sync: opened.reading.syncPill });
  await handshake.awaitNative("presence", 300_000, alive);
  const seen = await react.until(30_000, (reading) => reading.peers.some((row) => row.id.includes(String(nativeOpen.actor)) || row.id.includes(String(nativeOpen.userId))));
  handshake.publish("presence", { seesNative: seen.afterMs !== null });
  await handshake.awaitNative("cursor", 300_000, alive);
  const nativeCursor = await react.until(20_000, (reading) => reading.cursors.some((cursor) => cursor.actor === nativeOpen.actor));
  const board = [...nativeCursor.reading.boards].filter((box) => box.w > 100 && box.h > 100).sort((left, right) => right.w * right.h - left.w * left.h)[0];
  handshake.publish("cursor", { seesNativeCursor: nativeCursor.afterMs !== null });
  for (let step = 0; step < 40 && !existsSync(join(handshake.dir, "native-cursor-seen.json")); step += 1) {
    await react.page.mouse.move(board ? board.x + board.w * (0.35 + (step % 6) * 0.05) : 700, board ? board.y + board.h * 0.5 : 450);
    await react.page.waitForTimeout(250);
  }
  const nativeSeen = await handshake.awaitNative("cursor-seen", 60_000, alive);
  record("cursors both ways over one board", nativeCursor.afterMs !== null && nativeSeen?.seesReactCursor === true, { reactSeesNativeAfterMs: nativeCursor.afterMs, nativeSeesReact: nativeSeen?.seesReactCursor });
  await handshake.awaitNative("done", 300_000, alive);
}
