// #region 🧲️Header
/** @emoji ⌨️ The text-input model's TS half over `✍️editor/🧫️fixtures/⌨️text-input/🔣️.json`: the host's optimistic local echo
 * (`reconcileTextEditorEchoV1`, `refuseTextEditorEditV1`, the read-only refusals) replays `✏️TextEditor/🧫️fixtures/🔁️local-echo/🔣️.json`, and Chromium's native `<textarea>` editing — the third-party
 * oracle, driven with real key events — replays every typing `sequence` and must end at the fixture's text and selection,
 * the same answer the Rust `EditorHost` law gives under both token schedules. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { chromium, type Page } from "playwright";
import { afterAll, beforeAll, describe, expect, it } from "vitest";
import { TEXT_EDITOR_READ_ONLY_REFUSALS, reconcileTextEditorEchoV1, refuseTextEditorEditV1, type TextEditorEchoStateV1 } from "../../🧱️elements/✏️TextEditor/🟦️.tsx";
import localEcho from "../../🧱️elements/✏️TextEditor/🧫️fixtures/🔁️local-echo/🔣️.json";
import fixture from "../../../../../../../🔨️modules/✍️editor/🧫️fixtures/⌨️text-input/🔣️.json";
// #endregion 🔌️Adapters

// #region 🧪️Harness
type Step = { readonly type?: string; readonly key?: string; readonly select?: readonly [number, number]; readonly paste?: string; readonly compose?: string };
type Sequence = { readonly id: string; readonly text: string; readonly selection: readonly [number, number]; readonly steps: readonly Step[]; readonly expect: { readonly text: string; readonly selection: readonly [number, number] } };
type EchoEvent = { readonly typed?: string; readonly local?: string; readonly refuse?: string; readonly reason?: string; readonly echo?: string; readonly expect: string; readonly pending: readonly string[]; readonly readOnly?: boolean };

const lineEdgeKeys: Readonly<Record<string, string>> = process.platform === "darwin" ? { Home: "Meta+ArrowLeft", End: "Meta+ArrowRight" } : { Home: "Home", End: "End" };

/** 🔢️ UTF-16 offsets of the textarea → Unicode scalar values of the fixture. */
function scalarOffset(text: string, utf16: number): number {
  return Array.from(text.slice(0, utf16)).length;
}

function utf16Offset(text: string, scalars: number): number {
  return Array.from(text).slice(0, scalars).join("").length;
}

async function replay(page: Page, sequence: Sequence): Promise<{ readonly text: string; readonly selection: readonly [number, number] }> {
  await page.evaluate(
    ({ text, anchor, caret }) => {
      const area = document.querySelector("textarea")!;
      area.value = text;
      area.focus();
      area.setSelectionRange(Math.min(anchor, caret), Math.max(anchor, caret), caret < anchor ? "backward" : "forward");
    },
    { text: sequence.text, anchor: utf16Offset(sequence.text, sequence.selection[0]), caret: utf16Offset(sequence.text, sequence.selection[1]) },
  );
  for (const step of sequence.steps) {
    if (step.type !== undefined) for (const ch of Array.from(step.type)) await page.keyboard.press(ch === "\n" ? "Enter" : ch);
    else if (step.key !== undefined) await page.keyboard.press(lineEdgeKeys[step.key] ?? step.key);
    else if (step.paste !== undefined) await page.keyboard.insertText(step.paste);
    else if (step.compose !== undefined) await page.keyboard.insertText(step.compose);
    else if (step.select !== undefined) {
      const [anchor, caret] = step.select;
      await page.evaluate(({ anchor, caret }) => document.querySelector("textarea")!.setSelectionRange(Math.min(anchor, caret), Math.max(anchor, caret)), { anchor, caret });
    }
  }
  const state = await page.evaluate(() => {
    const area = document.querySelector("textarea")!;
    return { text: area.value, start: area.selectionStart, end: area.selectionEnd, backward: area.selectionDirection === "backward" };
  });
  const start = scalarOffset(state.text, state.start);
  const end = scalarOffset(state.text, state.end);
  return { text: state.text, selection: state.backward ? [end, start] : [start, end] };
}
// #endregion 🧪️Harness

// #region ⌨️TextInputLaws
describe("🔁️ the host's optimistic local echo, refusals and read-only mode (local-echo fixture)", () => {
  for (const testCase of localEcho.cases as readonly { readonly id: string; readonly initial: string; readonly events: readonly EchoEvent[] }[]) {
    it(testCase.id, () => {
      let guest = testCase.initial;
      let text = testCase.initial;
      let state: TextEditorEchoStateV1 = { pending: [], acknowledged: testCase.initial };
      let readOnly = false;
      for (const event of testCase.events) {
        if (event.typed !== undefined) {
          if (!readOnly) text = event.typed;
        } else if (event.local !== undefined) {
          if (!readOnly) {
            text = event.local;
            state = { ...state, pending: [...state.pending, event.local] };
          }
        } else if (event.refuse !== undefined) {
          if (TEXT_EDITOR_READ_ONLY_REFUSALS.has(event.reason ?? "dispatch-failed")) readOnly = true;
          const refused = refuseTextEditorEditV1(state, event.refuse);
          state = { pending: refused.pending, acknowledged: refused.acknowledged };
          if (refused.resync) {
            state = { pending: [], acknowledged: guest };
            text = guest;
          }
        } else {
          guest = event.echo!;
          const echo = reconcileTextEditorEchoV1(state, guest);
          state = { pending: echo.pending, acknowledged: echo.acknowledged };
          if (echo.external) text = guest;
        }
        expect(text, `${testCase.id}: text`).toBe(event.expect);
        expect(state.pending, `${testCase.id}: pending`).toEqual(event.pending);
        if (event.readOnly !== undefined) expect(readOnly, `${testCase.id}: read-only`).toBe(event.readOnly);
      }
    });
  }
});

describe("⌨️ Chromium's native textarea answers every typing sequence of the text-input fixture (third-party oracle)", () => {
  let browser: Awaited<ReturnType<typeof chromium.launch>>;
  let page: Page;
  beforeAll(async () => {
    browser = await chromium.launch({ headless: true });
    page = await browser.newPage();
    await page.setContent('<textarea style="font-family: monospace; font-size: 14px; width: 600px; height: 200px" spellcheck="false"></textarea>');
  }, 60_000);
  afterAll(async () => {
    await browser?.close();
  });

  for (const sequence of fixture.sequences as readonly Sequence[]) {
    it(sequence.id, async () => {
      const answer = await replay(page, sequence);
      expect(answer.text, `${sequence.id}: text`).toBe(sequence.expect.text);
      expect(answer.selection, `${sequence.id}: selection`).toEqual(sequence.expect.selection);
    });
  }
});
// #endregion ⌨️TextInputLaws
