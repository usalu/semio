/** 📡️ Collaboration e2e command frame recording: the command frame counter, the server frame commands tag, the frame count, the sent command recording and the editor text wait. */
import { decodeClientFrame } from "@semio-tech/framework-replication";

//#region 🔖️CollabE2e
/** 📡️ A live tally of the `ServerFrame::Commands` frames one page's DOCUMENT sockets have received.
 * Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice C1 — the audit's §7 step 3 asks for a
 * round-trip bound, not just "the text turned up eventually", so the harness has to be able to count
 * the frames that carry it. A server frame is `lane: u8 | tag: u8 | fields…`
 * (`📡️replication/📡️wire/🦀️.rs`'s `encode_server_frame`) and `Commands` is tag `3`; the directory
 * socket is excluded by path, since its command relay is a different lane entirely. */
export type CollabCommandFrameCounter = { count: number };

//#region 🔖️CollabE2eDom
/** 🔢️ The `ServerFrame` tag byte for `Commands`, mirrored from `encode_server_frame`'s own match arm
 * (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`, `out.push(3)`). */
const COLLAB_SERVER_FRAME_COMMANDS_TAG = 3;

/** 📡️ Attaches a `ServerFrame::Commands` tally to every document socket `page` opens from now on.
 * Only `/scopes/{space}%2F{doc}/document/ws` counts — the directory socket carries the space/artifact
 * listing lane, whose frames would otherwise inflate the round-trip bound STEP 8 asserts. */
export function collabCountCommandFrames(page: import("playwright").Page): CollabCommandFrameCounter {
  const counter: CollabCommandFrameCounter = { count: 0 };
  page.on("websocket", (ws) => {
    const url = ws.url();
    if (url.includes("/directory/") || !url.includes("/scopes/") || !url.includes("/document/ws")) return;
    ws.on("framereceived", (frame) => {
      const payload = frame.payload;
      if (typeof payload === "string" || payload.length < 2) return;
      if (payload[1] === COLLAB_SERVER_FRAME_COMMANDS_TAG) counter.count += 1;
    });
  });
  return counter;
}

/** 🪪️ Every `ClientFrame::Commands` envelope one page SENDS on its document sockets, in send order: the live witness that
 * a door-created artifact's editor authors under the artifact's own id from its very first edit (its genesis arrives
 * with the hub's canonical checkpoint pair before the actor activates — ticket 26/09/23 C10 item 1). A frame this
 * harness cannot decode is recorded, never skipped. */
export type CollabSentCommands = { readonly documentIds: string[]; readonly socketDocumentIds: string[]; readonly undecodable: string[] };

export function collabRecordSentCommands(page: import("playwright").Page): CollabSentCommands {
  const sent: CollabSentCommands = { documentIds: [], socketDocumentIds: [], undecodable: [] };
  page.on("websocket", (ws) => {
    const url = ws.url();
    if (url.includes("/directory/") || !url.includes("/scopes/") || !url.includes("/document/ws")) return;
    const segments = new URL(url).pathname.split("/"),
      scope = decodeURIComponent(segments[segments.indexOf("scopes") + 1] ?? "");
    ws.on("framesent", (frame) => {
      if (typeof frame.payload === "string") return;
      try {
        const decoded = decodeClientFrame(new Uint8Array(frame.payload)).frame;
        if (typeof decoded !== "object" || !("Commands" in decoded)) return;
        for (const envelope of decoded.Commands.envelopes) {
          sent.documentIds.push(envelope.document_id);
          sent.socketDocumentIds.push(scope.slice(scope.indexOf("/") + 1));
        }
      } catch (error) {
        sent.undecodable.push(error instanceof Error ? error.message : String(error));
      }
    });
  });
  return sent;
}

/** ⏳️ Polls `editor` until it shows `text`, returning how many `ServerFrame::Commands` frames the page
 * received between the call and the moment the text was first observed. Throws with the last value it
 * saw when the budget runs out, so a failure names what the editor actually held. */
export async function collabWaitForEditorText(
  page: import("playwright").Page,
  editor: import("playwright").Locator,
  counter: CollabCommandFrameCounter,
  text: string,
  deadlineMs: number,
): Promise<number> {
  const before = counter.count;
  const deadline = Date.now() + deadlineMs;
  let seen = "";
  while (Date.now() < deadline) {
    seen = (await editor.inputValue().catch(() => editor.innerText().catch(() => ""))) ?? "";
    if (seen.includes(text)) return counter.count - before;
    await page.waitForTimeout(100);
  }
  throw new Error(`timeout waiting for ${JSON.stringify(text)} in the peer's editor (last seen: ${JSON.stringify(seen.slice(-200))}, ${counter.count - before} Commands frame(s) received meanwhile)`);
}
//#endregion 🔖️CollabE2eDom
//#endregion 🔖️CollabE2e
