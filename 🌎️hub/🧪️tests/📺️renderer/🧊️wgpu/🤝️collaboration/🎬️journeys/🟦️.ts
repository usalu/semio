import type { Browser } from "playwright";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { hubProbeCall } from "@semio-tech/framework-os/modules/directory/testkit/client-probe";
import { requireMcpBinary, spawnRawMcp } from "@semio-tech/framework-os-mcp";
import { HUB_COLLABORATION_BOUNDS, HUB_COLLABORATION_KINDS, HubCollaborationBlocked, type HubCollaborationJourneyV1, type HubCollaborationOptionsV1 } from "../📋️contract/🟦️.ts";
import { LIVE_SYNC, ReactShell, Wasm32Shell } from "../🖥️shells/🟦️.ts";
import { documentHead, followNativeCursors, followNativeEdits, HANDLE_KINDS, Handshake, spawnNativeLaw, wasm32Kind } from "../🧭️native-law-and-follow/🟦️.ts";
import { changedPixelCount, distinctColours, regionPixels, startSeverableRelay } from "../🧲️relay-and-pixels/🟦️.ts";

//#region 🔖️Journeys
/** 🧰️ What every journey body receives: the options, the browser, the step recorder, the report's measured map, a clock,
 * the document it runs on and cleanup registration. */
type JourneyRun = Readonly<{
  options: HubCollaborationOptionsV1;
  browser: Browser;
  record: (name: string, pass: boolean, detail: unknown) => void;
  measured: Record<string, number | string | boolean>;
  clock: () => number;
  out: (name: string) => string;
  document: { spaceId: string; documentId: string };
  onCleanup: (cleanup: () => Promise<void>) => void;
  token: string;
}>;

const rosterNames = (label: string): string[] => label.split(/\s*·\s*/u).filter(Boolean);

const GERMAN_LINK_LINE = /Verbindung|Zugriff/u;

const ENGLISH_LINK_LINE = /connection|access/iu;

function requireServe(url: string | null, flag: string): string {
  if (!url) throw new HubCollaborationBlocked(`this run holds no ${flag} serve`);
  return url;
}

async function signedIn(run: JourneyRun, shell: Wasm32Shell): Promise<void> {
  const outcome = await shell.signIn();
  run.record(`${shell.human.label} signs in through the wgpu hub workspace`, outcome.minted, outcome);
}

async function attached(run: JourneyRun, shell: Wasm32Shell): Promise<boolean> {
  const outcome = await shell.attach(run.document.spaceId, run.document.documentId);
  const live = await shell.waitFor((view) => LIVE_SYNC.test(view.sync) && view.sync, HUB_COLLABORATION_BOUNDS.liveMs);
  writeFileSync(run.out(`projection-${shell.human.label}-${run.clock()}.json`), JSON.stringify(live.view.nodes, null, 1));
  run.record(`${shell.human.label} attaches the hub document and is live`, live.value !== null, { ...outcome, sync: live.view.sync, afterMs: live.afterMs });
  return live.value !== null;
}

/** 👥️ Two wasm32 shells, each behind its own severable relay: attach, presence, edits both ways, short / medium / long cuts,
 * a late joiner. */
async function wasm32Journey(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const kind = wasm32Kind(HUB_COLLABORATION_KINDS.wasm32.variant);
  const relays = [await startSeverableRelay(options.hub), await startSeverableRelay(options.hub)] as const;
  run.onCleanup(async () => {
    for (const relay of relays) await relay.close();
  });
  const documentId = (): string => run.document.documentId;
  const [a, b] = [await Wasm32Shell.boot(browser, options.humans[0], serve, relays[0].origin, options.locale, run.clock, documentId), await Wasm32Shell.boot(browser, options.humans[1], serve, relays[1].origin, options.locale, run.clock, documentId)];
  run.onCleanup(async () => {
    for (const shell of [a, b]) {
      writeFileSync(run.out(`console-${shell.human.label}.txt`), shell.lines.join("\n"));
      writeFileSync(run.out(`hub-${shell.human.label}.txt`), shell.hubLines.join("\n"));
    }
  });
  for (const shell of [a, b]) record(`${shell.human.label} paints a frame`, (await distinctColours(shell.page, run.out(`boot-${shell.human.label}.png`))) > 4, {});
  for (const shell of [a, b]) await signedIn(run, shell);
  for (const shell of [a, b]) await attached(run, shell);
  const presence = await Promise.all([a, b].map((shell) => shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs)));
  record("presence both ways (each roster names both humans)", presence.every((seen) => seen.value !== null), { a: presence[0]!.view.peers, b: presence[1]!.view.peers });
  for (const shell of [a, b]) await kind.prepare(shell);
  const head = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  for (const shell of [a, b]) {
    const shown = await shell.waitFor((view) => kind.count(view) >= head && view, 30_000);
    record(`${shell.human.label} shows the document's committed content after attach (hub head ${head})`, shown.value !== null, { count: kind.count(shown.view), head });
  }
  const crossing = async (author: Wasm32Shell, reader: Wasm32Shell): Promise<void> => {
    const before = kind.count(await reader.view());
    const started = Date.now();
    const edited = await kind.edit(author);
    const seen = await reader.waitFor((view) => kind.count(view) > before && view, HUB_COLLABORATION_BOUNDS.editSeenMs);
    const latencyMs = Date.now() - started;
    run.measured[`edit${author.human.label}to${reader.human.label}Ms`] = seen.value === null ? -1 : latencyMs;
    record(`${author.human.label}'s edit reaches ${reader.human.label} without a reload`, edited === "activated" && seen.value !== null, { edited, before, after: kind.count(seen.view), latencyMs });
  };
  await crossing(a, b);
  await crossing(b, a);
  for (const shell of [a, b]) {
    const sent = shell.documentFrames.filter((frame) => frame.direction === "sent");
    record(`${shell.human.label}'s document frames name the document`, sent.some((frame) => frame.namesDocument), { sent: sent.length, received: shell.documentFrames.length - sent.length });
  }
  const onlineCount = kind.count(await a.view());
  const bBefore = kind.count(await b.view());
  const cut = relays[0].sever();
  const severedAt = Date.now();
  await a.page.waitForTimeout(3_000);
  const beats = [await a.beat(), await a.beat()];
  record("a short cut never freezes A (the frame worker keeps answering)", beats.every((beat) => beat.answered && beat.latencyMs < HUB_COLLABORATION_BOUNDS.frozenBeatMs), { beats, connectionsCut: cut });
  const offline = await kind.edit(a);
  const queued = await a.view();
  const keptSince = Date.now();
  let kept = await a.link();
  while (!/pending|ausstehend/iu.test(kept.sync) && !kept.codes.includes("reconnecting") && Date.now() - keptSince < HUB_COLLABORATION_BOUNDS.keptLineMs) {
    await a.page.waitForTimeout(500);
    kept = await a.link();
  }
  record("A edits during the cut: admitted locally, and the shell says the edit is kept", offline === "activated" && kind.count(queued) > onlineCount && (/pending|ausstehend/iu.test(kept.sync) || kept.codes.includes("reconnecting")), { offline, count: kind.count(queued), saidAfterMs: Date.now() - keptSince, ...kept });
  await a.page.waitForTimeout(Math.max(0, HUB_COLLABORATION_BOUNDS.shortCutMs - (Date.now() - severedAt)));
  relays[0].heal();
  const healedAt = Date.now();
  const relinked = await a.waitFor((view) => LIVE_SYNC.test(view.sync) && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  const delivered = await b.waitFor((view) => kind.count(view) > bBefore && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  run.measured.shortCutRelinkMs = relinked.value === null ? -1 : Date.now() - healedAt;
  record("after the short cut A relinks in place and B receives the offline edit", relinked.value !== null && delivered.value !== null, { cutMs: healedAt - severedAt, relinkAfterMs: relinked.afterMs, syncA: relinked.view.sync, countB: kind.count(delivered.view) });
  relays[0].sever();
  const mediumAt = Date.now();
  let spoken = await a.link();
  while (!spoken.codes.includes("reconnecting") && Date.now() - mediumAt < HUB_COLLABORATION_BOUNDS.mediumCutMs) {
    await a.page.waitForTimeout(1_000);
    spoken = await a.link();
  }
  await a.page.screenshot({ path: run.out("medium-cut-A.png") });
  record("a medium cut is spoken as a short shortage", spoken.codes.includes("reconnecting"), { afterMs: Date.now() - mediumAt, ...spoken });
  await a.page.waitForTimeout(Math.max(0, HUB_COLLABORATION_BOUNDS.mediumCutMs - (Date.now() - mediumAt)));
  relays[0].heal();
  const cleared = await a.waitFor((view) => LIVE_SYNC.test(view.sync) && view, HUB_COLLABORATION_BOUNDS.relinkMs);
  const line = await a.link();
  record("after the medium cut A relinks and the line clears", cleared.value !== null && line.codes.length === 0, { ...line, relinkAfterMs: cleared.afterMs });
  relays[1].sever();
  const longAt = Date.now();
  let expired = await b.link();
  while (!expired.codes.includes("link-expired") && Date.now() - longAt < HUB_COLLABORATION_BOUNDS.expiryMs) {
    await b.page.waitForTimeout(2_000);
    expired = await b.link();
  }
  await b.page.screenshot({ path: run.out("long-cut-B.png") });
  run.measured.linkExpiredAfterMs = expired.codes.includes("link-expired") ? Date.now() - longAt : -1;
  record("a long cut expires B's link and says so", expired.codes.includes("link-expired"), { afterMs: Date.now() - longAt, ...expired });
  const tongue = options.locale === "de" ? GERMAN_LINK_LINE : ENGLISH_LINK_LINE;
  record(`B's expiry line speaks the session's tongue (${options.locale})`, expired.texts.some((text) => tongue.test(text)) && (options.locale === "en" || !expired.texts.some((text) => /connection/iu.test(text))), { texts: expired.texts });
  relays[1].heal();
  await b.page.waitForTimeout(15_000);
  const after = await b.link();
  record("an expired link never relinks by itself", !LIVE_SYNC.test(after.sync) && (after.codes.includes("link-expired") || /detached|getrennt/iu.test(after.sync)), after);
  const late = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => {
    writeFileSync(run.out("console-late.txt"), late.lines.join("\n"));
    writeFileSync(run.out("hub-late.txt"), late.hubLines.join("\n"));
  });
  await signedIn(run, late);
  await attached(run, late);
  await kind.prepare(late);
  const converged = kind.count(await a.view());
  const lateHead = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  const joined = await late.waitFor((view) => kind.count(view) === converged && view, 60_000);
  record("a late joiner attaching afterwards shows every committed edit", joined.value !== null, { lateCount: kind.count(joined.view), converged, hubHead: lateHead });
  await late.context.close();
}

/** ⚛️ A wasm32 shell and a React shell on one block2d document: open, presence, edits and undo both ways, late joiners of both
 * shell types. */
async function wasm32ReactJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const [serve, reactServe] = [requireServe(options.wgpuServe, "--serve"), requireServe(options.reactServe, "--react-serve")];
  const kind = wasm32Kind(HUB_COLLABORATION_KINDS["wasm32-react"].variant);
  const documentId = (): string => run.document.documentId;
  const a = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  const b = await ReactShell.boot(browser, options.humans[1], reactServe, options.locale, run.clock);
  run.onCleanup(async () => {
    writeFileSync(run.out("console-A-wasm32.txt"), a.lines.join("\n"));
    writeFileSync(run.out("console-B-react.txt"), b.lines.join("\n"));
  });
  await signedIn(run, a);
  record("B signs in through the React hub workspace", await b.signIn(), {});
  await attached(run, a);
  await kind.prepare(a);
  const painted = await a.view();
  const counts = painted.nodes.find((node) => String(node.key).endsWith("block2d-play-board.counts"));
  record("the board's painted counts are named for assistive technology", HANDLE_KINDS.test(String(counts?.label ?? "")) && String(counts?.label) === painted.texts.find((text) => HANDLE_KINDS.test(text)), { accessibleName: counts?.label ?? null, painted: painted.texts });
  const opened = await b.open(run.document.spaceId, run.document.documentId, (reading) => Number.isFinite(reading.handleKinds));
  record("B opens the document in React and is live", opened.liveAfterMs !== null, { liveAfterMs: opened.liveAfterMs, sync: opened.reading.syncPill, handleKinds: opened.reading.handleKinds });
  const aSees = await a.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs);
  const bSees = await b.until(HUB_COLLABORATION_BOUNDS.presenceMs, (reading) => reading.peers.length >= 2);
  record("presence both ways across shell types", aSees.value !== null && bSees.afterMs !== null, { wasm32: aSees.view.peers, react: bSees.reading.peers.map((row) => row.label) });
  const wasmCount = async (): Promise<number> => kind.count(await a.view());
  const reactBefore = (await b.read()).handleKinds;
  const aEdit = await kind.edit(a);
  const reactSees = await b.until(HUB_COLLABORATION_BOUNDS.editSeenMs, (reading) => reading.handleKinds > reactBefore);
  run.measured.editWasm32ToReactMs = reactSees.afterMs ?? -1;
  record("the wasm32 edit reaches React without a reload", aEdit === "activated" && reactSees.afterMs !== null, { aEdit, handleKinds: [reactBefore, reactSees.reading.handleKinds], afterMs: reactSees.afterMs });
  const wasmBefore = await wasmCount();
  const bEdit = await b.runAction("addHandleKind");
  const wasmSees = await a.waitFor((view) => kind.count(view) > wasmBefore && view, HUB_COLLABORATION_BOUNDS.editSeenMs);
  run.measured.editReactToWasm32Ms = wasmSees.value === null ? -1 : wasmSees.afterMs;
  record("the React edit reaches wasm32 without a reload", bEdit === "ok" && wasmSees.value !== null, { bEdit, counts: [wasmBefore, kind.count(wasmSees.view)], afterMs: wasmSees.afterMs });
  const beforeAUndo = await wasmCount();
  const aUndo = await kind.undo(a);
  const aUndone = await a.waitFor((view) => kind.count(view) < beforeAUndo && view, 30_000);
  const reactSeesUndo = await b.until(30_000, (reading) => reading.handleKinds === kind.count(aUndone.view));
  record("wasm32 undoes its own edit and React follows", aUndone.value !== null && reactSeesUndo.afterMs !== null, { aUndo, wasm32: [beforeAUndo, kind.count(aUndone.view)], react: reactSeesUndo.reading.handleKinds });
  const beforeBUndo = (await b.read()).handleKinds;
  const bUndo = await b.runAction("undo");
  const bUndone = await b.until(30_000, (reading) => reading.handleKinds < beforeBUndo);
  const wasmSeesUndo = await a.waitFor((view) => kind.count(view) === bUndone.reading.handleKinds && view, 30_000);
  record("React undoes its own edit and wasm32 follows", bUndone.afterMs !== null && wasmSeesUndo.value !== null, { bUndo, react: [beforeBUndo, bUndone.reading.handleKinds], wasm32: kind.count(wasmSeesUndo.view) });
  const converged = (await b.read()).handleKinds;
  const lateReact = await ReactShell.boot(browser, options.humans[0], reactServe, options.locale, run.clock);
  await lateReact.signIn();
  const lateOpened = await lateReact.open(run.document.spaceId, run.document.documentId, (reading) => reading.handleKinds === converged);
  record("a late React joiner converges", lateOpened.liveAfterMs !== null, { converged, late: lateOpened.reading.handleKinds });
  await lateReact.context.close();
  const lateWasm = await Wasm32Shell.boot(browser, options.humans[1], serve, options.hub, options.locale, run.clock, documentId);
  await signedIn(run, lateWasm);
  await attached(run, lateWasm);
  await kind.prepare(lateWasm);
  const lateSeen = await lateWasm.waitFor((view) => kind.count(view) === converged && view, 60_000);
  record("a late wasm32 joiner converges", lateSeen.value !== null, { converged, late: kind.count(lateSeen.view) });
  await lateWasm.context.close();
}

/** 🦀️ The native wgpu user (the live law) with a wasm32 or React browser user following its handshake. */
async function nativeJourney(run: JourneyRun, browserShell: "wasm32" | "react", cursors = false): Promise<void> {
  const { options, browser, record } = run;
  const handshake = new Handshake(join(options.outDir, options.tag, "handshake"), record);
  const law = cursors ? "a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board" : "a_native_and_a_react_user_collaborate_on_one_hub_document";
  const native = spawnNativeLaw(options, law, handshake.dir, join(options.outDir, options.tag));
  let alive = true;
  const nativeExit = native.exit.then((code) => {
    alive = false;
    return code;
  });
  run.onCleanup(async () => {
    if (alive) native.child.kill("SIGTERM");
  });
  if (browserShell === "react") {
    const react = await ReactShell.boot(browser, options.humans[1], requireServe(options.reactServe, "--react-serve"), options.locale, run.clock);
    run.onCleanup(async () => writeFileSync(run.out("console-B-react.txt"), react.lines.join("\n")));
    record("B signs in through the React hub workspace", await react.signIn(), {});
    if (cursors) await followNativeCursors(handshake, react, () => alive, record);
    else
      await followNativeEdits(
        handshake,
        {
          open: async (spaceId, documentId) => {
            const opened = await react.open(spaceId, documentId, (reading) => Number.isFinite(reading.handleKinds));
            return { liveAfterMs: opened.liveAfterMs, detail: { sync: opened.reading.syncPill } };
          },
          seesPeer: async (actor, userId) => {
            const seen = await react.until(HUB_COLLABORATION_BOUNDS.presenceMs, (reading) => reading.peers.some((row) => row.id.includes(actor) || (userId !== "" && row.id.includes(userId))));
            return { afterMs: seen.afterMs, peers: seen.reading.peers };
          },
          count: async () => (await react.read()).handleKinds,
          edit: () => react.runAction("addHandleKind"),
          undo: () => react.runAction("undo"),
          reload: async (spaceId, documentId) => {
            await react.page.reload({ waitUntil: "domcontentloaded", timeout: HUB_COLLABORATION_BOUNDS.bootMs });
            const restored = await react.until(120_000, (reading) => Number.isFinite(reading.handleKinds) || (reading.ready === "s" && (reading.home || reading.spaceIndex)));
            if (Number.isFinite(restored.reading.handleKinds)) return { liveAfterMs: restored.afterMs };
            return { liveAfterMs: (await react.open(spaceId, documentId, (reading) => Number.isFinite(reading.handleKinds))).liveAfterMs };
          },
        },
        () => alive,
        record,
      );
  } else {
    const kind = wasm32Kind("block2d");
    let shell = await Wasm32Shell.boot(browser, options.humans[1], requireServe(options.wgpuServe, "--serve"), options.hub, options.locale, run.clock, () => run.document.documentId);
    run.onCleanup(async () => writeFileSync(run.out("console-B-wasm32.txt"), shell.lines.join("\n")));
    await signedIn(run, shell);
    const attach = async (spaceId: string, documentId: string): Promise<number | null> => {
      run.document.spaceId = spaceId;
      run.document.documentId = documentId;
      const started = Date.now();
      const live = await attached(run, shell);
      await kind.prepare(shell);
      return live ? Date.now() - started : null;
    };
    await followNativeEdits(
      handshake,
      {
        open: async (spaceId, documentId) => ({ liveAfterMs: await attach(spaceId, documentId), detail: {} }),
        seesPeer: async () => {
          const seen = await shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs);
          return { afterMs: seen.value === null ? null : seen.afterMs, peers: seen.view.peers };
        },
        count: async () => kind.count(await shell.view()),
        edit: () => kind.edit(shell),
        undo: () => kind.undo(shell),
        reload: async (spaceId, documentId) => {
          await shell.context.close();
          shell = await Wasm32Shell.boot(browser, options.humans[1], requireServe(options.wgpuServe, "--serve"), options.hub, options.locale, run.clock, () => run.document.documentId);
          await signedIn(run, shell);
          return { liveAfterMs: await attach(spaceId, documentId) };
        },
      },
      () => alive,
      record,
    );
  }
  const code = await Promise.race([nativeExit, new Promise<number>((resolveLate) => setTimeout(() => resolveLate(-1), 300_000))]);
  run.measured.nativeExit = code;
  record("the native law's own ledger passes (exit 0)", code === 0, { exit: code, log: join(options.outDir, options.tag, "native.txt") });
}

/** 🖱️ Two wasm32 shells on one puzzle2d board: each pointer over its own board is painted as a peer cursor on the other's. */
async function cursorsJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const documentId = (): string => run.document.documentId;
  const [a, b] = [await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId), await Wasm32Shell.boot(browser, options.humans[1], serve, options.hub, options.locale, run.clock, documentId)];
  for (const shell of [a, b]) await signedIn(run, shell);
  for (const shell of [a, b]) await attached(run, shell);
  const presence = await Promise.all([a, b].map((shell) => shell.waitFor((view) => view.peers.some((label) => rosterNames(label).length >= 2) && view.peers, HUB_COLLABORATION_BOUNDS.presenceMs)));
  record("presence both ways", presence.every((seen) => seen.value !== null), { a: presence[0]!.view.peers, b: presence[1]!.view.peers });
  const boardClip = async (shell: Wasm32Shell): Promise<{ x: number; y: number; width: number; height: number } | null> => {
    const box = await shell.page.evaluate(() => {
      const rect = document.querySelector("canvas")?.getBoundingClientRect();
      return rect ? { x: rect.x, y: rect.y, width: rect.width, height: rect.height } : null;
    });
    return box ? { x: Math.round(box.x + box.width * 0.3), y: Math.round(box.y + box.height * 0.25), width: Math.round(box.width * 0.4), height: Math.round(box.height * 0.5) } : null;
  };
  for (const [mover, watcher] of [[a, b], [b, a]] as const) {
    const label = `${mover.human.label} moves, ${watcher.human.label} sees`;
    const [moverClip, watcherClip] = [await boardClip(mover), await boardClip(watcher)];
    if (!moverClip || !watcherClip) {
      record(`${label}: peer cursor painted on the watcher's board`, false, { reason: "no canvas" });
      continue;
    }
    await mover.page.mouse.move(moverClip.x + moverClip.width * 0.2, moverClip.y + moverClip.height * 0.2);
    await watcher.page.waitForTimeout(4_000);
    const control = [await regionPixels(watcher.page, watcherClip)];
    await watcher.page.waitForTimeout(2_500);
    control.push(await regionPixels(watcher.page, watcherClip));
    const noise = changedPixelCount(control[0]!, control[1]!);
    const deltas: number[] = [];
    for (const [fx, fy] of [[0.8, 0.8], [0.5, 0.3], [0.2, 0.7]] as const) {
      await mover.page.mouse.move(moverClip.x + moverClip.width * fx, moverClip.y + moverClip.height * fy, { steps: 8 });
      await watcher.page.waitForTimeout(HUB_COLLABORATION_BOUNDS.cursorSettleMs);
      deltas.push(changedPixelCount(control[1]!, await regionPixels(watcher.page, watcherClip)));
    }
    await watcher.page.screenshot({ path: run.out(`cursor-${mover.human.label}-${watcher.human.label}.png`), clip: watcherClip });
    record(`${label}: peer cursor painted on the watcher's board`, deltas.every((delta) => delta > Math.max(40, noise * 3)), { noise, deltas });
  }
}

/** 🤖️ A human in the wasm32 shell and a delegated agent over the stdio semio MCP on one hub note: the roster names the agent AS an
 * agent, its block is decoded into the Artifact panel without a reload, and the live frame equals a cold reference render. */
async function agentPixelsJourney(run: JourneyRun): Promise<void> {
  const { options, browser, record } = run;
  const serve = requireServe(options.wgpuServe, "--serve");
  const kind = wasm32Kind("note");
  const documentId = (): string => run.document.documentId;
  const human = await Wasm32Shell.boot(browser, options.humans[0], serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => writeFileSync(run.out("console-human.txt"), human.lines.join("\n")));
  await signedIn(run, human);
  await attached(run, human);
  await kind.prepare(human);
  if (!(await human.closeSyncCard())) throw new Error("the human's Sync card does not close — the frames would differ in chrome");
  const parked = async (shell: Wasm32Shell): Promise<void> => {
    await shell.page.mouse.move(2, 996);
    await shell.page.waitForTimeout(1_500);
  };
  await parked(human);
  const body = { x: 0, y: 36, width: 1600, height: 1000 - 36 - 36 };
  const before = await regionPixels(human.page, body, run.out("agent-before.png"));
  await human.page.waitForTimeout(2_500);
  const noise = changedPixelCount(before, await regionPixels(human.page, body));
  const countBefore = kind.count(await human.view());
  const headBefore = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  const agentWord = options.locale === "de" ? "KI-Agent" : "AI agent";
  const agentLabel = `wgpu acceptance agent ${options.tag}`;
  const delegation = await hubProbeCall(options.hub, "POST", "/auth/agent-delegations", run.token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: run.document.spaceId, agentLabel, audience: "edit", ttlSecs: 1800 }));
  const delegationId = String(delegation.json?.delegationId ?? "");
  if (!delegationId) throw new Error(`agent delegation answered ${delegation.status} ${delegation.text.slice(0, 200)}`);
  run.onCleanup(async () => {
    await hubProbeCall(options.hub, "DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId)}`, run.token).catch(() => undefined);
  });
  const privateDir = join(options.outDir, options.tag, "agent");
  mkdirSync(privateDir, { recursive: true, mode: 0o700 });
  const credentialPath = join(privateDir, "credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: options.hub, spaceId: run.document.spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  run.onCleanup(async () => rmSync(credentialPath, { force: true }));
  const agent = spawnRawMcp(requireMcpBinary(options.repoRoot), ["stdio", "--hub", options.hub, "--space", run.document.spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"]);
  run.onCleanup(async () => {
    await agent.close().catch(() => undefined);
  });
  await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wgpu-hub-collaboration", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const call = async (name: string, args: Record<string, unknown>): Promise<{ isError?: boolean; structuredContent?: any }> => ((await agent.request("tools/call", { name, arguments: args }, 600_000)).result ?? {}) as { isError?: boolean; structuredContent?: any };
  const context = await call("context_resolve", {});
  const opened = await call("artifact_open", { artifactId: run.document.documentId });
  record("the agent is its own delegated principal and opens the document", opened.isError !== true && String(context.structuredContent?.principal ?? "") === `agent:${delegationId}`, { principal: context.structuredContent?.principal });
  const roster = await human.waitFor((view) => view.peers.some((label) => label.includes(agentWord) && label.includes(agentLabel)) && view.peers, 30_000);
  record(`the human's wgpu roster names the agent AS an agent (${options.locale})`, roster.value !== null, { peers: roster.view.peers });
  const search = await call("capabilities_search", { query: "add a block", kind: ["mutation"] });
  const capabilityId = ((search.structuredContent?.results ?? []) as { capabilityId?: string; id?: string }[]).map((hit) => String(hit.capabilityId ?? hit.id)).find((id) => id.endsWith(".addBlock")) ?? "";
  const prepared = await call("action_prepare", { capabilityId, input: { kind: "text" } });
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  const committedAt = Date.now();
  record("the agent commits an edit through the semio MCP", invoked.structuredContent?.status === "SUCCEEDED", { capabilityId, status: invoked.structuredContent?.status });
  const decoded = await human.waitFor((view) => kind.count(view) > countBefore && view, HUB_COLLABORATION_BOUNDS.agentSeenMs);
  run.measured.agentBlockSeenMs = decoded.value === null ? -1 : Date.now() - committedAt;
  record("the human's shell decodes the agent's block without a reload", decoded.value !== null, { count: [countBefore, kind.count(decoded.view)], afterMs: Date.now() - committedAt });
  const headAfter = await documentHead(options.hub, run.token, run.document.spaceId, run.document.documentId);
  record("the hub ledger advanced with the agent's commit", headAfter > headBefore, { head: [headBefore, headAfter] });
  if (!(await human.closeSyncCard())) throw new Error("the human's Sync card reopened — the frames would differ in chrome");
  await parked(human);
  let live = await regionPixels(human.page, body, run.out("agent-live.png"));
  for (let settle = 0; settle < 5; settle += 1) {
    await human.page.waitForTimeout(1_500);
    const next = await regionPixels(human.page, body);
    const moving = changedPixelCount(live, next) > noise;
    live = next;
    if (!moving) break;
  }
  await agent.close().catch(() => undefined);
  await human.context.close();
  const reference = await Wasm32Shell.boot(browser, { ...options.humans[0], label: `${options.humans[0].label} (cold reference)` }, serve, options.hub, options.locale, run.clock, documentId);
  run.onCleanup(async () => writeFileSync(run.out("console-reference.txt"), reference.lines.join("\n")));
  await signedIn(run, reference);
  await attached(run, reference);
  await kind.prepare(reference);
  if (!(await reference.closeSyncCard())) throw new Error("the reference's Sync card does not close — the frames would differ in chrome");
  const referenceDecoded = await reference.waitFor((view) => kind.count(view) === kind.count(decoded.view) && view, 60_000);
  record("a fresh session decodes the same committed state (cold reference)", referenceDecoded.value !== null, { count: [kind.count(decoded.view), kind.count(referenceDecoded.view)] });
  await parked(reference);
  const cold = await regionPixels(reference.page, body, run.out("agent-reference.png"));
  await reference.context.close();
  const differsFromBefore = changedPixelCount(before, live);
  const differsFromReference = changedPixelCount(live, cold);
  const edited = Math.max(4 * noise, HUB_COLLABORATION_BOUNDS.agentEditMinPixels);
  const same = Math.max(2 * noise, HUB_COLLABORATION_BOUNDS.agentSameMaxPixels);
  Object.assign(run.measured, { agentFrameNoise: noise, agentFrameChangedFromBefore: differsFromBefore, agentFrameDiffFromReference: differsFromReference, agentFrameEditedMin: edited, agentFrameSameMax: same });
  record("the agent's edit changes the rendered frame", differsFromBefore >= edited, { differsFromBefore, edited, noise });
  record("the live frame equals a cold reference render of the same committed state", referenceDecoded.value !== null && differsFromReference <= same, { differsFromReference, same, noise, evidence: [run.out("agent-before.png"), run.out("agent-live.png"), run.out("agent-reference.png")] });
}

//#endregion 🔖️Journeys

export const JOURNEY_BODIES: Readonly<Record<HubCollaborationJourneyV1, (run: JourneyRun) => Promise<void>>> = {
  wasm32: wasm32Journey,
  "wasm32-react": wasm32ReactJourney,
  "wasm32-native": (run) => nativeJourney(run, "wasm32"),
  "native-react": (run) => nativeJourney(run, "react"),
  "native-react-cursors": (run) => nativeJourney(run, "react", true),
  cursors: cursorsJourney,
  "agent-pixels": agentPixelsJourney,
};
