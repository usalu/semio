import type { OpenedArtifactV1, OpenedCheckV1, OpenedUiV1 } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/👥️two-human/🧩️opened/🟦️.ts";
import scenarios from "../🧫️fixtures/🔣️.json" with { type: "json" };
import pickerCases from "../../../🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🧪️tests/🧫️picker/🔣️.json" with { type: "json" };
import windowCases from "../🧫️fixtures/🪟️windows.json" with { type: "json" };
import { archiveBytesEqualV1, archiveWitnessV1, coldArchiveStatusV1, snapshotWitnessV1, snapshotUndoRedoObservedV1, corruptImportObservedV1, abortObservedV1, finalizationObservedV1, mediaImportObservedV1, remodelBoundEditorV1, remodelInputsV1, undoRedoObservedV1 } from "../📋️report/🟦️.ts";

const editor = "s.remodel.remodeling@1/*#editor";
const surfaceSelector = '[data-surface-app-id][data-surface-window-id]';
const attributes = ["data-surface-app-id", "data-surface-role", "data-surface-session-instance-id", "data-surface-window-id", "data-surface-window-kind-id", "data-surface-space-id", "data-surface-document-id"] as const;

/** 🪟 Exercises real duplicate windows and records their independently published controls. */
export async function inspectRemodelWindowsV1(context: OpenedArtifactV1): Promise<readonly string[]> {
  const evidence: string[] = [], ui = context.a;
  const escape = (value: string) => [...value].map(character => "\\" + character.codePointAt(0)!.toString(16) + " ").join("");
  try {
    for (const item of windowCases) {
      try {
      context.signal.throwIfAborted();
      console.log("[remodel] Inspecting " + item.mode + " window instances");
      const nav = '[id="playground.navbar.modes.' + item.mode + '"]';
      if (!(await ui.observe(nav, [], false)).some(row => row.visible)) { evidence.push(item.mode + ": mode unavailable"); continue; }
      await ui.press(nav, "Enter");
      const body = '[data-surface-app-id="' + escape(editor) + '"][data-surface-role="editor"][data-surface-space-id="' + escape(context.spaceId) + '"][data-surface-document-id="' + escape(context.documentId) + '"][data-surface-window-kind-id="' + escape(item.kind) + '"]';
      const active = '[data-slot="window"][data-active="true"] ' + body;
      const deadline = Date.now() + 15_000;
      let selected = await ui.observe(active, attributes, false);
      while (!selected.some(row => row.visible) && Date.now() < deadline) { context.signal.throwIfAborted(); await new Promise(resolve => setTimeout(resolve, 500)); selected = await ui.observe(active, attributes, false); }
      const original = selected.find(row => row.visible)?.attributes["data-surface-window-id"];
      if (!original) { evidence.push(item.mode + ": scoped active window unavailable"); continue; }
      const prior = (await ui.observe(body, attributes, false)).map(row => row.attributes["data-surface-window-id"]);
      await ui.press(nav, "ControlOrMeta+Shift+N");
      let duplicates = await ui.observe(body, attributes, false), added: string | null = null;
      const duplicateDeadline = Date.now() + 15_000;
      while (Date.now() < duplicateDeadline) {
        context.signal.throwIfAborted();
        added = duplicates.find(row => row.visible && row.attributes["data-surface-window-id"] && !prior.includes(row.attributes["data-surface-window-id"]))?.attributes["data-surface-window-id"] ?? null;
        if (added) break;
        await new Promise(resolve => setTimeout(resolve, 500)); duplicates = await ui.observe(body, attributes, false);
      }
      if (!added) { evidence.push(JSON.stringify({ mode: item.mode, original, duplicates, result: "duplicate unavailable" })); continue; }
      const roots = [original, added].map(id => '[data-slot="window"]:has(' + body + '[data-surface-window-id="' + escape(id) + '"])');
      for (const root of roots) {
        const toggle = root + ' [id$=".measures.unfold"]';
        if ((await ui.observe(toggle, [], false)).some(row => row.visible)) await ui.click(toggle);
      }
      const controls = [original, added].map((id, index) => roots[index] + ' [id="' + escape(id + "/" + item.measure) + '"]');
      const before = await Promise.all(controls.map(control => ui.observe(control, ["data-published-value"], false)));
      let after = before;
      if (before.every(rows => rows.length === 1 && rows[0]!.visible)) {
        if (item.control === "toggle") await ui.press(controls[0]!, "Space");
        else if (item.control === "slider") {
          const thumb = controls[0]! + ' [role="slider"]', current = await ui.observe(thumb, ["aria-valuenow", "aria-valuemax", "aria-disabled"], false);
          if (current[0]?.attributes["aria-disabled"] !== "true" && Number(current[0]?.attributes["aria-valuemax"] ?? 0) > 0) await ui.press(thumb, Number(current[0]?.attributes["aria-valuenow"] ?? 0) >= Number(current[0]?.attributes["aria-valuemax"] ?? 0) ? "ArrowLeft" : "ArrowRight");
        } else {
          await ui.click(controls[0]!);
          const cameras = before[0]?.[0]?.attributes["data-published-value"] === "gcps";
          const option = cameras ? (context.locale === "de" ? "Kameras" : "Cameras") : (context.locale === "de" ? "Passpunkte" : "Ground control points");
          const choice = '[role="option"]:text-is("' + option + '")';
          if ((await ui.observe(choice, [], false)).some(row => row.visible)) await ui.click(choice);
          else await ui.press(controls[0]!, "Escape");
        }
        const changedDeadline = Date.now() + 15_000;
        do { context.signal.throwIfAborted(); await new Promise(resolve => setTimeout(resolve, 500)); after = await Promise.all(controls.map(control => ui.observe(control, ["data-published-value"], false))); } while (JSON.stringify(before) === JSON.stringify(after) && Date.now() < changedDeadline);
      }
      const value = (rows: typeof before[number]) => rows[0]?.attributes["data-published-value"] ?? null;
      const independent = value(before[0]!) !== null && value(before[1]!) !== null && value(after[0]!) !== null && value(before[0]!) !== value(after[0]!) && value(before[1]!) === value(after[1]!);
      evidence.push(JSON.stringify({ mode: item.mode, kind: item.kind, original, added, before, after, independent }));
      } catch (error) { context.signal.throwIfAborted(); evidence.push(item.mode + ": " + String(error).slice(0, 500)); }
    }
  } catch (error) { context.signal.throwIfAborted(); evidence.push("Window observation interrupted: " + String(error).slice(0, 500)); }
  return evidence;
}

/** 👁️ Observes the viewer body for the same shared artifact. */
export async function inspectRemodelViewerV1(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> {
  const [a, b] = await Promise.all([context.a.observe(surfaceSelector, attributes, false), context.b.observe(surfaceSelector, attributes, false)]);
  const matches = (item: typeof a[number], role: string) => item.visible && item.attributes["data-surface-app-id"] === `s.remodel.remodeling@1/*#${role}` && item.attributes["data-surface-role"] === role && item.attributes["data-surface-space-id"] === context.spaceId && item.attributes["data-surface-document-id"] === context.documentId && ["data-surface-session-instance-id", "data-surface-window-id", "data-surface-window-kind-id"].every(key => !!item.attributes[key]?.trim());
  const scoped = a.some(item => matches(item, "editor")) && b.some(item => matches(item, "viewer"));
  const evidence = [JSON.stringify({ phase: "viewer", actor: "B", spaceId: context.spaceId, documentId: context.documentId, scoped, a, b })];
  if (scoped) {
    try {
      const escape = (value: string) => [...value].map(character => "\\" + character.codePointAt(0)!.toString(16) + " ").join("");
      const window = (role: string) => '[data-slot="window"][data-active="true"]:has([data-surface-app-id="' + escape(`s.remodel.remodeling@1/*#${role}`) + '"][data-surface-space-id="' + escape(context.spaceId) + '"][data-surface-document-id="' + escape(context.documentId) + '"])';
      for (const [ui, role] of [[context.a, "editor"], [context.b, "viewer"]] as const) {
        const root = window(role), toggles = await ui.observe(root + ' [data-folded="true"] [id$=".engagement.toggle"]', ["id"], false);
        for (const toggle of toggles) if (toggle.visible && toggle.attributes.id) await ui.click(root + ' [id="' + escape(toggle.attributes.id) + '"]');
      }
      const suffix = ' [data-slot="window-action-pane"] [id="action.exportArtifactDocument"]', editorAction = window("editor") + suffix, viewerAction = window("viewer") + suffix;
      if ((await context.a.observe(editorAction, [], false)).some(row => row.visible) && (await context.b.observe(viewerAction, [], false)).some(row => row.visible)) {
        const [left, right] = await Promise.all([context.a.download(editorAction), context.b.download(viewerAction)]);
        const original = archiveWitnessV1(left.name, left.bytes, left.mime), viewed = archiveWitnessV1(right.name, right.bytes, right.mime);
        evidence.push(JSON.stringify({ original: { sha256: original.sha256, bytes: original.bytes, members: original.members }, viewed: { sha256: viewed.sha256, bytes: viewed.bytes, members: viewed.members }, same: archiveBytesEqualV1(original.raw, viewed.raw) }));
      } else evidence.push("Document export was not visible on both surfaces.");
    } catch (error) { context.signal.throwIfAborted(); evidence.push("Viewer document export unavailable: " + String(error).slice(0, 500)); }
  }
  evidence.push("Persisted per-window configuration and viewer content remain unverified.");
  return [{ id: "windows-and-viewer", status: "unexecuted", evidence }];
}

/** ❄️ Compares the real exported document from the original and freshly opened profiles. */
export async function inspectRemodelColdV1(context: OpenedArtifactV1): Promise<readonly OpenedCheckV1[]> {
  const id = "cold-reopen", escape = (value: string) => [...value].map(character => "\\" + character.codePointAt(0)!.toString(16) + " ").join("");
  try {
    const [a, b] = await Promise.all([context.a.observe(surfaceSelector, attributes, false), context.b.observe(surfaceSelector, attributes, false)]);
    const owns = (item: typeof a[number]) => item.visible && item.attributes["data-surface-app-id"] === editor && item.attributes["data-surface-role"] === "editor" && item.attributes["data-surface-space-id"] === context.spaceId && item.attributes["data-surface-document-id"] === context.documentId && !!item.attributes["data-surface-session-instance-id"]?.trim() && !!item.attributes["data-surface-window-id"]?.trim();
    if (!a.some(owns) || !b.some(owns)) return [{ id, status: "unexecuted", evidence: ["The original and fresh editor bodies were not both bound to the actual artifact."] }];
    const window = '[data-slot="window"][data-active="true"]:has([data-surface-app-id="' + escape(editor) + '"][data-surface-space-id="' + escape(context.spaceId) + '"][data-surface-document-id="' + escape(context.documentId) + '"])';
    for (const ui of [context.a, context.b]) {
      const toggles = await ui.observe(window + ' [data-folded="true"] [id$=".engagement.toggle"]', ["id"], false);
      for (const toggle of toggles) if (toggle.visible && toggle.attributes.id) await ui.click(window + ' [id="' + escape(toggle.attributes.id) + '"]');
    }
    const action = window + ' [data-slot="window-action-pane"] [id="action.exportArtifactDocument"]';
    const [aAction, bAction] = await Promise.all([context.a.observe(action, [], false), context.b.observe(action, [], false)]);
    if (!aAction.some(item => item.visible) || !bAction.some(item => item.visible)) return [{ id, status: "unexecuted", evidence: ["The document export action was not visible in both profiles."] }];
    const [left, right] = await Promise.all([context.a.download(action), context.b.download(action)]);
    const original = archiveWitnessV1(left.name, left.bytes, left.mime), reopened = archiveWitnessV1(right.name, right.bytes, right.mime);
    const same = archiveBytesEqualV1(original.raw, reopened.raw);
    return [{ id, status: coldArchiveStatusV1(original.raw, reopened.raw, original.members), evidence: [JSON.stringify({ spaceId: context.spaceId, documentId: context.documentId, a, b, original: { sha256: original.sha256, bytes: original.bytes, members: original.members }, reopened: { sha256: reopened.sha256, bytes: reopened.bytes, members: reopened.members }, same })] }];
  } catch (error) {
    context.signal.throwIfAborted();
    return [{ id, status: "unexecuted", evidence: [String(error).slice(0, 500)] }];
  }
}

/** 🪟 Verifies both visible editor bodies belong to the actual shared document. */
export async function inspectRemodelOpenedV1(context: OpenedArtifactV1, repoRoot: string): Promise<readonly OpenedCheckV1[]> {
  const rows: { id: string; status: "unexecuted" | "passed" | "failed" | "unsupported"; evidence: string[] }[] = scenarios.map(row => ({ id: row.id, status: "unexecuted", evidence: [] }));
  const row = (id: string) => rows.find(item => item.id === id)!;
  let stage = "identity";
  const escape = (value: string) => [...value].map(character => "\\" + character.codePointAt(0)!.toString(16) + " ").join("");
  try {
  const identify = async (ui: OpenedUiV1) => {
    const surfaces = (await ui.observe(surfaceSelector, attributes, false)).filter(item => item.visible && item.attributes["data-surface-app-id"] === editor && item.attributes["data-surface-role"] === "editor");
    const populated = await ui.observe('[data-surface-app-id="s.remodel.remodeling@1/*#editor"] [data-ui-node-key]:not([aria-busy="true"]):not([data-unknown-component])', []);
    return { surfaces, populated: populated.some(item => item.visible && item.text.trim().length > 0) };
  };
  context.signal.throwIfAborted();
  const [a, b] = await Promise.all([identify(context.a), identify(context.b)]);
  const matches = (side: typeof a) => side.surfaces.every(item => item.attributes["data-surface-space-id"] === context.spaceId && item.attributes["data-surface-document-id"] === context.documentId) && remodelBoundEditorV1(side.surfaces.map(item => ({ app: item.attributes["data-surface-app-id"] ?? null, role: item.attributes["data-surface-role"] ?? null, instance: item.attributes["data-surface-session-instance-id"] ?? null, window: item.attributes["data-surface-window-id"] ?? null, kind: item.attributes["data-surface-window-kind-id"] ?? null, space: item.attributes["data-surface-space-id"] ?? null, document: item.attributes["data-surface-document-id"] ?? null, visible: item.visible, populated: true })));
  row("identity").evidence.push(JSON.stringify({ spaceId: context.spaceId, documentId: context.documentId, a, b }));
  if (!matches(a) || !matches(b)) { row("identity").status = "failed"; return rows; }
  row("identity").evidence.push("Matching body scopes observed; content and artifact bytes remain unverified.");
  stage = "locales-and-modes";
  const modes = row(stage), settings = '[id="os.openSettings"]', language = '[id="framework.settings.language"][role="combobox"]';
  let localeModesPassed = true;
  for (const locale of ["en", "de", context.locale]) {
    context.signal.throwIfAborted();
    if (!(await context.a.observe(settings, [], false)).some(item => item.visible)) { modes.status = "unsupported"; localeModesPassed = false; break; }
    await context.a.click(settings);
    if (!(await context.a.observe(language, [], false)).some(item => item.visible)) { modes.status = "unsupported"; localeModesPassed = false; break; }
    await context.a.click(language);
    const option = '[role="option"][data-value="' + locale + '"]';
    if (!(await context.a.observe(option, [], false)).some(item => item.visible)) { modes.status = "unsupported"; localeModesPassed = false; break; }
    await context.a.click(option);
    const actual = await context.a.observe("html", ["lang"], false);
    await context.a.press(language, "Escape");
    const observedLocale = actual[0]?.attributes.lang ?? "";
    let layoutPass = new RegExp("^" + locale + "(?:-|$)", "iu").test(observedLocale);
    for (const [id, kind] of [["capture", "remodeling-frames"], ["model", "remodeling-main"], ["analyze", "remodeling-report"]] as const) {
      const selector = '[id="playground.navbar.modes.' + id + '"]';
      if (!(await context.a.observe(selector, [], false)).some(item => item.visible)) { layoutPass = false; break; }
      await context.a.press(selector, "Enter");
      const deadline = Date.now() + 15_000;
      let button = await context.a.observe(selector, ["aria-pressed"], false);
      const windowAttributes = ["data-surface-window-kind-id", "data-surface-app-id", "data-surface-role", "data-surface-space-id", "data-surface-document-id"];
      const windowOk = (item: Awaited<ReturnType<OpenedUiV1["observe"]>>[number]) => item.visible && item.attributes["data-surface-window-kind-id"] === kind && item.attributes["data-surface-app-id"] === editor && item.attributes["data-surface-role"] === "editor" && item.attributes["data-surface-space-id"] === context.spaceId && item.attributes["data-surface-document-id"] === context.documentId;
      let windows = await context.a.observe(surfaceSelector, windowAttributes, false);
      while ((!button.some(item => item.visible && item.attributes["aria-pressed"] === "true") || !windows.some(windowOk)) && Date.now() < deadline) {
        context.signal.throwIfAborted();
        await new Promise(resolve => setTimeout(resolve, 500));
        button = await context.a.observe(selector, ["aria-pressed"], false);
        windows = await context.a.observe(surfaceSelector, windowAttributes, false);
      }
      const passed = button.some(item => item.visible && item.attributes["aria-pressed"] === "true") && windows.some(windowOk);
      modes.evidence.push(JSON.stringify({ locale, observedLocale, mode: id, kind, button, windows }));
      layoutPass &&= passed;
    }
    localeModesPassed &&= layoutPass;
  }
  if (localeModesPassed) modes.status = "passed";
  stage = "examples";
  const examples = row(stage);
  for (const id of ["demo", "demo-session", "synthetic-orbit"]) {
    context.signal.throwIfAborted();
    const picker = '[id="playground.navbar.fixture"][role="combobox"]';
    if (!(await context.a.observe(picker, [])).some(item => item.visible)) { examples.status = "unsupported"; break; }
    await context.a.click(picker);
    const option = '[role="option"][data-value="' + id + '"]';
    const observed = await context.a.observe(option, []);
    if (!observed.some(item => item.visible)) { examples.status = "unsupported"; break; }
    await context.a.click(option);
    const expected = pickerCases.examples.find(item => item.id === id)!;
    const media = '[data-slot="panel-tab-button"][id="remodeling.media"]', calibration = '[data-slot="panel-tab-button"][id="remodeling.calibration"]';
    let mediaSummary: Awaited<ReturnType<OpenedUiV1["observe"]>> = [], streamRows: Awaited<ReturnType<OpenedUiV1["observe"]>> = [], cameraRows: Awaited<ReturnType<OpenedUiV1["observe"]>> = [];
    if ((await context.a.observe(media, [], false)).some(item => item.visible)) {
      await context.a.click(media);
      const deadline = Date.now() + 30_000;
      do {
        context.signal.throwIfAborted();
        mediaSummary = await context.a.observe('[data-ui-node-key="remodeling-media.summary"]', []);
        streamRows = await context.a.observe('[data-ui-node-key^="remodeling-media.stream."]', ["data-ui-node-key"]);
        const counts = mediaSummary.find(item => item.visible)?.text.match(/:\s*(\d+)\s*-\s*[^:]+:\s*(\d+)/u);
        if (counts && Number(counts[1]) === expected.expected.streamIds.length && Number(counts[2]) === expected.expected.frames && expected.expected.streamIds.every(stream => streamRows.some(item => item.attributes["data-ui-node-key"] === "remodeling-media.stream." + stream))) break;
        await new Promise(resolve => setTimeout(resolve, 500));
      } while (Date.now() < deadline);
    }
    if ((await context.a.observe(calibration, [], false)).some(item => item.visible)) {
      await context.a.click(calibration);
      cameraRows = await context.a.observe('[data-ui-node-key^="remodeling-calibration.camera."]', ["data-ui-node-key"]);
    }
    examples.evidence.push(JSON.stringify({ selected: id, option: observed, picker: await context.a.observe(picker, []), expected: expected.expected, mediaSummary, streamRows, cameraRows }));
  }
  const activeWindow = '[data-slot="window"][data-active="true"]:has([data-surface-app-id="' + escape(editor) + '"][data-surface-space-id="' + escape(context.spaceId) + '"][data-surface-document-id="' + escape(context.documentId) + '"])';
  stage = "identity";
  if (row("identity").status !== "passed") {
    const capture = '[id="playground.navbar.modes.capture"]';
    if ((await context.a.observe(capture, [], false)).some(item => item.visible)) {
      await context.a.press(capture, "Enter");
      const canvas = '[data-surface-app-id="' + escape(editor) + '"][data-surface-space-id="' + escape(context.spaceId) + '"][data-surface-document-id="' + escape(context.documentId) + '"][data-surface-window-kind-id="remodeling-frames"] canvas';
      const deadline = Date.now() + 30_000;
      let pixels: Awaited<ReturnType<OpenedUiV1["canvas"]>> = null;
      while (Date.now() < deadline) {
        context.signal.throwIfAborted();
        const found = await context.a.observe(canvas, [], false);
        if (found.length === 1 && found[0]!.visible) {
          pixels = await context.a.canvas(canvas);
          if (pixels !== null && pixels.visiblePixels > 0 && pixels.colors > 1) break;
        }
        await new Promise(resolve => setTimeout(resolve, 500));
      }
      row("identity").evidence.push(JSON.stringify({ frameCanvas: pixels }));
      if (pixels !== null && pixels.visiblePixels > 0 && pixels.colors > 1) row("identity").evidence.push("A scoped canvas contains varied visible pixels; their artifact content is not yet verified.");
    }
  }
  const unfold = async (ui: OpenedUiV1) => {
    const toggles = await ui.observe(activeWindow + ' [data-folded="true"] [id$=".engagement.toggle"]', ["id"], false);
    for (const toggle of toggles) if (toggle.visible && toggle.attributes.id) await ui.click(activeWindow + ' [id="' + escape(toggle.attributes.id) + '"]');
  };
  const archive = async (ui: OpenedUiV1) => {
    await unfold(ui);
    const action = activeWindow + ' [data-slot="window-action-pane"] [id="action.exportArtifactDocument"]';
    if (!(await ui.observe(action, [], false)).some(item => item.visible)) return null;
    const file = await ui.download(action);
    return archiveWitnessV1(file.name, file.bytes, file.mime);
  };
  const snapshot = async (ui: OpenedUiV1) => {
    await unfold(ui);
    const action = activeWindow + ' [data-slot="window-action-pane"] [id="action.exportSnapshotFile"]';
    if (!(await ui.observe(action, [], false)).some(item => item.visible)) return null;
    try { const file = await ui.download(action); return await snapshotWitnessV1(file.name, file.bytes, file.mime, context.signal); }
    catch (error) { context.signal.throwIfAborted(); row(stage).evidence.push("Snapshot download unavailable: " + String(error).slice(0, 500)); return null; }
  };
  const mediaTab = '[data-slot="panel-tab-button"][id="remodeling.media"]';
  const tab = await context.a.observe(mediaTab, ["aria-selected"]);
  if (tab.some(item => item.visible) && tab[0]?.attributes["aria-selected"] !== "true") await context.a.click(mediaTab);
  const peerTab = await context.b.observe(mediaTab, ["aria-selected"]);
  if (peerTab.some(item => item.visible) && peerTab[0]?.attributes["aria-selected"] !== "true") await context.b.click(mediaTab);
  const summary = (ui: OpenedUiV1) => ui.observe('[data-ui-node-key="remodeling-media.summary"]', []);
  const streams = () => context.a.observe('[data-ui-node-key^="remodeling-media.stream."]', ["data-ui-node-key"]);
  const counts = (items: Awaited<ReturnType<typeof summary>>): number[] => {
    const match = items.find(item => item.visible)?.text.match(/:\s*(\d+)\s*-\s*[^:]+:\s*(\d+)/u);
    return match ? [Number(match[1]), Number(match[2])] : [];
  };
  await unfold(context.a);
  const choose = async (id: string, actionId: string, assets: string) => {
    stage = id;
    const target = row(id), action = activeWindow + ' [data-slot="window-action-pane"] [id="action.' + actionId + '"]';
    if (!(await context.a.observe(action, [])).some(item => item.visible)) { target.status = "unsupported"; return; }
    const before = await summary(context.a), beforeStreams = await streams();
    const inputs = remodelInputsV1(repoRoot, assets);
    const name = inputs[0]?.split(/[\\/]/u).at(-1) ?? "";
    const invalidImport = context.locale === "de" ? `„${name}“ kann nicht importiert werden: nicht unterstützte oder beschädigte Bild- oder Videodaten.` : `Cannot import ${name}: unsupported or corrupt image or video data.`;
    const archiveBefore = id === "import-corrupt" ? await archive(context.a) : null;
    const alertsBefore = id === "import-corrupt" ? await context.a.observe('[role="alert"],[data-semio-transient-notice]', []) : [];
    try {
      try { await context.a.chooseFiles(action, inputs); }
      catch (error) {
        context.signal.throwIfAborted();
        const execute = activeWindow + ' [data-slot="window-action-pane"] [id$=".action.' + actionId + '.execute"]';
        if (!(await context.a.observe(execute, [])).some(item => item.visible)) throw error;
        await context.a.chooseFiles(execute, inputs);
      }
      const deadline = Date.now() + (id === "import-video" ? 120_000 : 60_000);
      const frames = id === "import-images" || id === "import-append" ? 3 : id === "import-video" ? 45 : id === "import-jpeg" ? 1 : 0;
      let progressAt = Date.now(), after = await summary(context.a), newAlert = "";
      while ((frames === 0 ? !newAlert : counts(before).length !== 2 || counts(after)[1] !== counts(before)[1]! + frames) && Date.now() < deadline) {
        context.signal.throwIfAborted();
        if (Date.now() - progressAt >= 10_000) { console.log("[remodel] Waiting for " + id); progressAt = Date.now(); }
        await new Promise(resolve => setTimeout(resolve, 500));
        after = await summary(context.a);
        if (id === "import-corrupt") {
          const alerts = await context.a.observe('[role="alert"],[data-semio-transient-notice]', []);
          newAlert = alerts.find(item => item.visible && name.length > 0 && item.text.includes(invalidImport) && !alertsBefore.some(old => old.text === item.text))?.text ?? "";
        }
      }
      const afterStreams = await streams();
      let peer = await summary(context.b);
      while (JSON.stringify(counts(peer)) !== JSON.stringify(counts(after)) && Date.now() < deadline) {
        context.signal.throwIfAborted();
        await new Promise(resolve => setTimeout(resolve, 500));
        peer = await summary(context.b);
      }
      const archiveAfter = id === "import-corrupt" ? await archive(context.a) : null;
      const corruptUnchanged = archiveBefore !== null && archiveAfter !== null && archiveBytesEqualV1(archiveBefore.raw, archiveAfter.raw);
      target.evidence.push(JSON.stringify({ before, after, peer, beforeStreams, afterStreams, selection: assets, newAlert, corruptUnchanged, archiveBefore: archiveBefore?.sha256 ?? null, archiveAfter: archiveAfter?.sha256 ?? null }));
      if (id === "import-corrupt" && corruptImportObservedV1(newAlert, corruptUnchanged, counts(before), counts(after), counts(peer))) target.status = "passed";
      target.evidence.push(JSON.stringify({ sharedCountsAndStream: id !== "import-jpeg" && frames > 0 && mediaImportObservedV1(counts(before), counts(after), counts(peer), frames, beforeStreams.map(item => item.attributes["data-ui-node-key"] ?? ""), afterStreams.map(item => ({ id: item.attributes["data-ui-node-key"] ?? "", text: item.text }))) }));
    } catch (error) {
      context.signal.throwIfAborted();
      target.status = "failed";
      target.evidence.push(String(error).slice(0, 500));
    }
  };
  await choose("import-images", "importFrames", "import-images");
  await choose("import-jpeg", "importFrames", "import-jpeg");
  await choose("import-corrupt", "importFrames", "import-corrupt");
  await choose("import-append", "importFrames", "import-images");
  await choose("import-video", "importVideo", "import-video");
  stage = "run-controls-and-abort";
  const run = row(stage);
  const tool = '[data-slot="panel-tab-button"][id="tool.reconstruction"]';
  const documentTab = '[data-slot="panel-tab-button"][id="framework.panel.document"]';
  const reconstructionTab = '[data-slot="panel-tab-button"][id="framework.panel.document.reconstruction"]';
  if (!(await context.a.observe(tool, [])).some(item => item.visible)) run.status = "unsupported";
  else {
    await context.a.click(tool);
    if ((await context.a.observe(documentTab, [])).some(item => item.visible)) await context.a.click(documentTab);
    if (!(await context.a.observe(reconstructionTab, [])).some(item => item.visible)) run.status = "unsupported";
    else {
      await context.a.click(reconstructionTab);
      const state = () => context.a.observe('[data-ui-node-key="remodeling-pipeline.run"]', []);
      const result = () => context.a.observe('[data-ui-node-key="remodeling-pipeline.result"]', []);
      const waitState = async (pattern: RegExp, duration: number) => {
        const deadline = Date.now() + duration;
        let last = await state(), progressAt = Date.now();
        while (!last.some(item => pattern.test(item.text)) && Date.now() < deadline) {
          context.signal.throwIfAborted();
          if (Date.now() - progressAt >= 10_000) { console.log("[remodel] Waiting for reconstruction state"); progressAt = Date.now(); }
          await new Promise(resolve => setTimeout(resolve, 500));
          last = await state();
        }
        return last;
      };
      const productsTab = '[data-slot="panel-tab-button"][id="remodeling.results"]';
      const products = async (ui: OpenedUiV1) => {
        if (!(await ui.observe(productsTab, [])).some(item => item.visible)) return null;
        await ui.click(productsTab);
        const values = await ui.observe('[data-ui-node-key^="remodeling-results."]', ["data-ui-node-key"]);
        const expected = ["mesh", "sparse", "dense", "trajectory", "geo"].map(id => "remodeling-results." + id);
        const selected = expected.map(id => values.find(item => item.visible && item.attributes["data-ui-node-key"] === id));
        return selected.every(Boolean) ? selected.map(item => item!.text) : null;
      };
      const beforeProducts = await products(context.a);
      const archiveBeforeA = await archive(context.a), archiveBeforeB = await archive(context.b);
      if ((await context.a.observe(documentTab, [])).some(item => item.visible)) await context.a.click(documentTab);
      if ((await context.a.observe(reconstructionTab, [])).some(item => item.visible)) await context.a.click(reconstructionTab);
      const runIdentity = (text: string) => text.match(/(?:run|lauf)\s+(\d+)/iu)?.[1] ?? null;
      const before = await state();
      await context.a.press(reconstructionTab, "ControlOrMeta+Enter");
      const started = await waitState(/running|läuft|starting|startet/iu, 30_000);
      const runId = runIdentity(started.find(item => item.visible)?.text ?? "");
      const previousId = runIdentity(before.find(item => item.visible)?.text ?? "");
      run.evidence.push(JSON.stringify({ before, started, runId }));
      if (runId !== null && runId !== previousId && started.some(item => /running|läuft/iu.test(item.text))) {
        await context.a.press(reconstructionTab, "ControlOrMeta+Alt+Enter");
        const paused = await waitState(/paused|pausiert/iu, 15_000);
        run.evidence.push(JSON.stringify({ paused }));
        let stepped = false, resumed = false;
        if (paused.some(item => /paused|pausiert/iu.test(item.text) && runIdentity(item.text) === runId)) {
          const runTab = '[data-slot="panel-tab-button"][id="framework.panel.document.run"]';
          if ((await context.a.observe(runTab, [])).some(item => item.visible)) {
            await context.a.click(runTab);
            const progress = () => context.a.observe('[role="progressbar"][data-ui-node-key="framework.toolRun.' + runId + '.progress"]', ["aria-valuenow", "aria-valuetext"]);
            const priorProgress = await progress();
            await context.a.press(runTab, "ControlOrMeta+Alt+ArrowRight");
            const stepDeadline = Date.now() + 15_000;
            let afterStep = await progress();
            while (JSON.stringify(afterStep) === JSON.stringify(priorProgress) && Date.now() < stepDeadline) {
              context.signal.throwIfAborted();
              await new Promise(resolve => setTimeout(resolve, 500));
              afterStep = await progress();
            }
            stepped = priorProgress.length > 0 && JSON.stringify(afterStep) !== JSON.stringify(priorProgress);
            run.evidence.push(JSON.stringify({ priorProgress, afterStep }));
            await context.a.press(runTab, "ControlOrMeta+Alt+Enter");
            await context.a.click(reconstructionTab);
            const resumedState = await waitState(/running|läuft|complete|abgeschlossen/iu, 15_000);
            resumed = resumedState.some(item => /running|läuft/iu.test(item.text) && runIdentity(item.text) === runId);
            run.evidence.push(JSON.stringify({ resumed: resumedState }));
          }
        }
        await context.a.press(reconstructionTab, "ControlOrMeta+.");
        const aborted = await waitState(/aborted|abgebrochen/iu, 30_000), abortedResult = await result();
        await new Promise(resolve => setTimeout(resolve, 5000));
        context.signal.throwIfAborted();
        const afterAbort = await products(context.a), peerAfterAbort = await products(context.b);
        const archiveAfterA = await archive(context.a), archiveAfterB = await archive(context.b);
        const displayedProductsUnchanged = abortObservedV1(stepped, resumed, aborted.some(item => /aborted|abgebrochen/iu.test(item.text) && runIdentity(item.text) === runId), beforeProducts, afterAbort, peerAfterAbort);
        const archivesUnchanged = archiveBeforeA !== null && archiveBeforeB !== null && archiveAfterA !== null && archiveAfterB !== null && archiveBytesEqualV1(archiveBeforeA.raw, archiveAfterA.raw) && archiveBytesEqualV1(archiveBeforeB.raw, archiveAfterB.raw);
        const receipt = (value: typeof archiveBeforeA) => value === null ? null : { sha256: value.sha256, bytes: value.bytes, parentBytes: value.parentBytes, members: value.members, mime: value.mime };
        run.evidence.push(JSON.stringify({ aborted, result: abortedResult, beforeProducts, afterAbort, peerAfterAbort, archiveBeforeA: receipt(archiveBeforeA), archiveBeforeB: receipt(archiveBeforeB), archiveAfterA: receipt(archiveAfterA), archiveAfterB: receipt(archiveAfterB), displayedProductsUnchanged, archivesUnchanged }));
        if (displayedProductsUnchanged && archivesUnchanged) run.status = "passed";
        if ((await context.a.observe(documentTab, [])).some(item => item.visible)) await context.a.click(documentTab);
        if ((await context.a.observe(reconstructionTab, [])).some(item => item.visible)) await context.a.click(reconstructionTab);
      }
      stage = "reconstruct-and-finalize";
      const final = row(stage);
      const beforeSecond = await state(), previousRun = runIdentity(beforeSecond.find(item => item.visible)?.text ?? "");
      await context.a.press(reconstructionTab, "ControlOrMeta+Enter");
      const completed = await waitState(/complete|abgeschlossen/iu, 300_000);
      const completedId = runIdentity(completed.find(item => item.visible)?.text ?? "");
      const beforeFinalize = await products(context.a), peerBeforeFinalize = await products(context.b);
      const snapshotBefore = await snapshot(context.a);
      final.evidence.push(JSON.stringify({ completed, beforeFinalize, peerBeforeFinalize }));
      if (completedId !== null && completedId !== previousRun && completed.some(item => /complete|abgeschlossen/iu.test(item.text))) {
        if ((await context.a.observe(documentTab, [])).some(item => item.visible)) await context.a.click(documentTab);
        if ((await context.a.observe(reconstructionTab, [])).some(item => item.visible)) await context.a.click(reconstructionTab);
        await context.a.press(reconstructionTab, "ControlOrMeta+Shift+Enter");
        const finalized = await waitState(/finalized|übernommen/iu, 30_000), finalResult = await result();
        const afterFinalize = await products(context.a), peerFinal = await products(context.b);
        const snapshotFinal = await snapshot(context.a);
        final.evidence.push(JSON.stringify({ finalized, result: finalResult, afterFinalize, peerFinal }));
        const displayedFinalization = finalizationObservedV1(beforeProducts, beforeFinalize, peerBeforeFinalize, afterFinalize, peerFinal, finalized.some(item => /finalized|übernommen/iu.test(item.text) && runIdentity(item.text) === completedId));
        final.evidence.push(JSON.stringify({ displayedFinalization }));
        stage = "undo-redo";
        const history = row(stage), undo = activeWindow + ' [data-slot="window-action-pane"] [id="action.undo"]';
        if ((await context.a.observe(undo, [])).some(item => item.visible)) {
          const beforeUndo = await products(context.a);
          await context.a.click(undo);
          const afterUndo = await products(context.a), peerUndo = await products(context.b);
          const snapshotUndo = await snapshot(context.a);
          const redo = activeWindow + ' [data-slot="window-action-pane"] [id="action.redo"]';
          if ((await context.a.observe(redo, [])).some(item => item.visible)) {
            await context.a.click(redo);
            const afterRedo = await products(context.a), peerRedo = await products(context.b);
            const snapshotRedo = await snapshot(context.a);
            history.evidence.push(JSON.stringify({ beforeUndo, afterUndo, peerUndo, afterRedo, peerRedo }));
            const displayedProductsRestored = undoRedoObservedV1(displayedFinalization, beforeUndo, afterUndo, peerUndo, afterRedo, peerRedo);
            const snapshotsRestored = snapshotBefore !== null && snapshotFinal !== null && snapshotUndo !== null && snapshotRedo !== null && snapshotUndoRedoObservedV1(snapshotBefore.raw, snapshotFinal.raw, snapshotUndo.raw, snapshotRedo.raw);
            history.evidence.push(JSON.stringify({ displayedProductsRestored, snapshotsRestored, snapshots: [snapshotBefore, snapshotFinal, snapshotUndo, snapshotRedo].map(value => value === null ? null : { sha256: value.sha256, bytes: value.bytes }) }));
            if (displayedProductsRestored && snapshotsRestored) history.status = "passed";
          } else history.status = "unsupported";
        } else history.status = "unsupported";
        stage = "mesh-and-qc-export";
        const exports = row(stage), qcAction = activeWindow + ' [data-slot="window-action-pane"] [id="action.exportQcReport"]';
        if ((await context.a.observe(qcAction, [])).some(item => item.visible)) {
          try {
            const file = await context.a.download(qcAction);
            const decoded = new TextDecoder("utf-8", { fatal: true }).decode(file.bytes), qc: unknown = JSON.parse(decoded);
            exports.evidence.push(JSON.stringify({ name: file.name, mime: file.mime, bytes: file.bytes.length, jsonObject: qc !== null && typeof qc === "object" && !Array.isArray(qc), expectedName: file.name === "remodeling-qc-report.json", expectedMime: file.mime === "application/json" }));
          } catch (error) {
            context.signal.throwIfAborted();
            exports.evidence.push("QC download unavailable: " + String(error).slice(0, 500));
          }
        } else exports.evidence.push("QC action not observed");
      }
    }
  }
  return rows;
  } catch (error) {
    context.signal.throwIfAborted();
    row(stage).status = "failed";
    row(stage).evidence.push(String(error).slice(0, 500));
    return rows;
  }
}

