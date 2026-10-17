/** 📤️ Text editor outbox and scene packets: the edit outbox, the echoed lane, the scene without echoed text and the scene packet with its equality test. */
import { TEXT_EDITOR_SCENE_LANES, type TextEditorScene } from "@semio-tech/framework";
import { encodePackValue } from "@semio-tech/framework/packs";
import { parseSceneJsonField } from "../../🗣️Interpreter/🌉️scene-host-contract/🟦️.ts";

//#region 🔖️TextEditorHost
//#region EditingHelpers
/** 📮️ What the editor owes the guest: its latest text and selection (byte offsets of the session). */
export type TextEditorOutboxV1 = { readonly text: string; readonly start: number; readonly end: number };

/** 📮️ The editor's outbox: at most ONE round trip in flight, and each delivery carries the editor's state AS IT IS WHEN THE
 * DELIVERY GOES OUT (`read`), never a snapshot taken when it was owed — a scene that folded a collaborator's run into the editor
 * in between is part of what goes out (ticket 26/09/23 C12, live wave p33: a splice computed from a snapshot older than that fold
 * deleted the collaborator's run and re-inserted everything around it, so both editors duplicated and lost runs). `notify` after
 * every change; a state equal to the last one delivered is not sent again; a refused round trip frees the lane like a settled one.
 * @see ../🧫️fixtures/📮️delivery/🔣️.json */
export function createTextEditorOutboxV1<T>(read: () => T | null, deliver: (value: T) => unknown, isEqual: (a: T, b: T) => boolean): () => void {
  let inFlight = false;
  let owed = false;
  let lastSent: T | undefined;
  const flush = () => {
    if (inFlight || !owed) return;
    owed = false;
    const next = read();
    if (next === null || (lastSent !== undefined && isEqual(lastSent, next))) return;
    lastSent = next;
    inFlight = true;
    const release = () => {
      inFlight = false;
      flush();
    };
    void Promise.resolve(deliver(next)).then(release, release);
  };
  return () => {
    owed = true;
    flush();
  };
}

/** 🧮️ Most edits one editor keeps in flight before an echo acknowledges them. */
export const TEXT_EDITOR_PENDING_EDIT_LIMIT = 256;

/** 🪞️ The scene fields an echo of the editor's own state carries back — its text and its selection. */
const TEXT_EDITOR_ECHOED_FIELDS: ReadonlySet<string> = new Set(["buffer", "selectionJson"]);

/** 🧾️ Whether a paged-carrier lane transports an echoed field: its ref (`bytes` + `hash` of the buffer) changes with
 * every keystroke, so an echo that kept it would re-sync — and repaint — an unchanged editor once per key. Read at call time:
 * the lane table is imported through a module cycle and is not initialised while this module evaluates. */
function textEditorEchoedLaneV1(lane: string): boolean {
  return TEXT_EDITOR_SCENE_LANES.some((entry) => entry.lane === lane && TEXT_EDITOR_ECHOED_FIELDS.has(entry.field));
}

/** ✂️ A scene without what an echo of the editor's own state must not apply — its buffer, its selection and the
 * lane refs that describe the buffer; every other field (tokens, diagnostics, completions…) stays and syncs. */
export function sceneWithoutEchoedTextV1(scene: TextEditorScene): Record<string, unknown> {
  const lanes = scene.lanes?.filter((ref) => !textEditorEchoedLaneV1(ref.lane));
  return Object.fromEntries(Object.entries(scene).flatMap(([key, value]): [string, unknown][] => (TEXT_EDITOR_ECHOED_FIELDS.has(key) ? [] : key === "lanes" ? (lanes?.length ? [[key, lanes]] : []) : [[key, value]])));
}

/** 📦️ The pack the editor syncs for `scene`: the whole scene when it is external (another author's text), else the
 * scene without the echo of its own state (ticket 26/09/23 F3: the buffer's lane ref made every echo re-sync). */
export function textEditorScenePacket(scene: TextEditorScene, external: boolean): Uint8Array {
  const source = external ? scene : sceneWithoutEchoedTextV1(scene);
  const field = <T,>(raw: unknown): T | undefined => {
    if (raw === undefined) return undefined;
    if (typeof raw !== "string") throw new TypeError("Editor scene transport field must be encoded text");
    return parseSceneJsonField<T>(raw);
  };
  const occurrences = field<{ readonly hover?: string; readonly selection?: string }>(source.occurrencesJson);
  const hover = field<{ readonly start: number; readonly end: number } | null>(source.hoverJson);
  const camera = field<{ readonly y: number }>(source.cameraJson);
  const overlays = field<{ readonly deadLineY?: number }>(source.overlaysJson);
  const input = {
    buffer: source.buffer,
    selection: field(source.selectionJson),
    tokens: field(source.tokensJson),
    diagnostics: field(source.diagnosticsJson),
    placeholders: field(source.placeholdersJson),
    occurrences: occurrences === undefined ? undefined : { hover: field(occurrences.hover), selection: field(occurrences.selection) },
    extraCarets: field(source.extraCaretsJson),
    selectableSpans: field(source.selectableSpansJson),
    settings: field(source.settingsJson),
    camera: camera === undefined ? undefined : { y: camera.y },
    overlays: overlays === undefined ? undefined : { deadLineY: overlays.deadLineY },
    hover: hover === undefined ? undefined : hover === null ? { kind: "clear" } : { kind: "range", start: hover.start, end: hover.end },
  };
  return new Uint8Array(encodePackValue(Object.fromEntries(Object.entries(input).filter(([, value]) => value !== undefined))));
}

/** 🟰️ Whether two encoded scene packs carry the same bytes. */
export function sameScenePackV1(left: Uint8Array, right: Uint8Array): boolean {
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index++) if (left[index] !== right[index]) return false;
  return true;
}

export function parseJsonOr<T>(json: string | undefined, fallback: T): T {
  if (!json) return fallback;
  try {
    return parseSceneJsonField<T>(json);
  } catch {
    return fallback;
  }
}

export function exactU64(value: unknown): value is string {
  return typeof value === "string" && /^(0|[1-9][0-9]{0,19})$/u.test(value) && BigInt(value) <= 0xffff_ffff_ffff_ffffn;
}
//#endregion EditingHelpers
//#endregion 🔖️TextEditorHost
