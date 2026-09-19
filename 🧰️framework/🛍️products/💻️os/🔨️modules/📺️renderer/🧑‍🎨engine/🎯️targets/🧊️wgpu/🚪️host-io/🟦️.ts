// #region 🧲️Header
/** @emoji 🚪️ The PAGE half of the wgpu renderer's file door — the one place a `<a download>` and an
 * `<input type="file">` are created for this target.
 *
 * 🐛️ Why it exists at all: the wgpu shell runs inside a dedicated Worker (`🎞️frame-worker/🟦️.ts` owns the
 * `OffscreenCanvas`), and a Worker has no `window` and no `document`. The shell's own browser halves
 * therefore answered `web_sys::window()` with `None` and returned silently — `Export Document…` produced
 * its bytes and handed them to nobody, and `Import Document…` opened nothing at all, on a renderer where
 * every other hop of both journeys was already correct (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-end-to-end-verification-2026-09-14.md` §C).
 *
 * ⚖️ ONE binding, two installs: the shell always calls `globalThis.semioWgpuHostIo`. The page installs
 * {@link createWgpuPageHostIo} directly (`🎬️renderer-boot/🟦️.ts`, main-thread mount); the Worker installs
 * a `postMessage` bridge to this same implementation running on the page
 * (`🎞️frame-worker/🟦️.ts` ⇄ `🚚️browser-frame-transport/🟦️.ts`). The shell has no idea which isolate it is
 * in, so neither journey can regress on one variant while passing on the other.
 */
// #endregion 🧲️Header

import { WGPU_HOST_STORAGE_VALUE_MAX_BYTES, wgpuHostStorageCarriesKey, type WgpuHostStorageScope } from "../🧭️boot-descriptor/🟦️.ts";

/** 🚪️ What the shell asks the page to do. `op` is the whole vocabulary — an unknown one is refused
 * loudly rather than answered with an empty pick, which is indistinguishable from a cancelled dialog. */
export type WgpuHostIoRequest =
  | { readonly op: "download-media-export"; readonly filename: string; readonly mimeType: string }
  | { readonly op: "request-file-open"; readonly accept: string; readonly readAs?: string; readonly multiple?: boolean }
  | { readonly op: "request-native-file-path"; readonly accept?: string }
  | { readonly op: "request-native-folder-path" }
  | { readonly op: "directory-http"; readonly method: string; readonly url: string; readonly bearer?: string; readonly body?: string }
  | { readonly op: "storage"; readonly verb: "get" | "set" | "remove"; readonly scope?: WgpuHostStorageScope; readonly key: string; readonly value?: string }
  | { readonly op: "socket"; readonly verb: "open"; readonly socketId: number; readonly url: string; readonly protocols?: readonly string[] }
  | { readonly op: "socket"; readonly verb: "send"; readonly socketId: number; readonly text?: string; readonly binary?: string }
  | { readonly op: "socket"; readonly verb: "poll"; readonly socketId: number; readonly maxMessages?: number; readonly maxBytes?: number }
  | { readonly op: "socket"; readonly verb: "close"; readonly socketId: number };

/** 📇️ One hub directory hop the shell asks the page to make. It is the SAME call the React shell's
 * TypeScript `DirectoryClient` makes — cookie session (`credentials: "include"`), JSON bodies, the
 * frozen paths `/auth/sessions/me`, `/directory/spaces[/{id}]`, `/directory/commands`,
 * `/directory/event-page/v1` — so both renderers reach one hub surface through one contract.
 *
 * 🐛️ Why the page and not the Worker: the shell's isolate is the Worker that owns the
 * `OffscreenCanvas`, which has no `document` and therefore no same-site cookie jar to send. Routing
 * the fetch through the page is what makes the session cookie travel at all. */
export type WgpuDirectoryHttpAnswer = { readonly status: number; readonly body: string } | { readonly error: string };

/** 🗄️ One durable preference hop the shell asks the page to make against the SAME keys and value
 * encodings React's `StoragePort` uses (`🖥️platform/🟦️.ts`), so a preference survives a renderer
 * switch in either direction. `value` is `null` for a key the store does not hold and for every
 * `set`/`remove`, which answer only that they landed.
 *
 * 🐛️ Why the page and not the Worker: the shell's isolate is the Worker that owns the
 * `OffscreenCanvas` and has no `window`, so its `localStorage` read resolved to nothing and EVERY
 * `prefs_get`/`prefs_set` on the browser build was a silent no-op — appearance, locale, terminology,
 * themes, keybinding overrides, the compute worker count, the dock skeleton and the introduction
 * seen-flag all read as empty and written to nothing (`📓️w4a-boot-appearance-and-tour.md` §6). */
export type WgpuHostStorageAnswer = { readonly value: string | null } | { readonly error: string };

/** 🔌️ One duplex socket hop. The wgpu shell owns no socket of its own: it asks the page to dial,
 * hands it frames to write, and PAGES whatever the peer sent back, because the trait seams it has to
 * satisfy (`DirectoryWsConnection::try_recv_text`, and the MCP bridge's own byte-oriented consumer)
 * are synchronous and a wasm isolate may not block on a promise.
 *
 * 🐛️ Why the page and not `new WebSocket(...)` inside the Worker: `WebSocket` does exist in a
 * dedicated Worker, so the constructor alone would work — but the same-site session cookie the hub
 * authenticates the directory socket with travels only from the document, the page-mounted variant of
 * this renderer (`🎬️renderer-boot/🟦️.ts`) has no Worker at all, and one door is one place to audit
 * every origin this shell talks to. Both isolates therefore call the SAME `semioWgpuHostIo`.
 *
 * 🌊️ Bounded on BOTH counts ({@link WGPU_SOCKET_QUEUE_MAX_MESSAGES}/{@link WGPU_SOCKET_QUEUE_MAX_BYTES}):
 * a peer that outruns the shell's poll cadence loses the OLDEST frames and the loss is REPORTED as
 * `dropped` on the very next answer. A Worker cannot apply real TCP back-pressure (the browser owns
 * the receive side), so a bounded queue that says what it lost is the honest shape; an unbounded one
 * eventually takes the tab with it. */
export type WgpuSocketFrame = { readonly text: string } | { readonly binary: string };

export type WgpuSocketAnswer = { readonly state: "connecting" | "open" | "closed"; readonly messages?: readonly WgpuSocketFrame[]; readonly more?: boolean; readonly dropped?: number; readonly closeCode?: number } | { readonly error: string };

/** 🌊️ How many messages one socket's page-side queue holds before the oldest are dropped. */
export const WGPU_SOCKET_QUEUE_MAX_MESSAGES = 256;

/** 🌊️ How many payload bytes one socket's page-side queue holds before the oldest are dropped. */
export const WGPU_SOCKET_QUEUE_MAX_BYTES = 1024 * 1024;

/** 📃️ The ceilings ONE `poll` hop may carry back, whatever the request asks for. The Rust half
 * clamps its own request to the identical pair (`🔌️socket-door/🦀️.rs`); clamping here too is what
 * makes the door safe against any caller, not only against the one that exists today. */
export const WGPU_SOCKET_POLL_MAX_MESSAGES = 32;
export const WGPU_SOCKET_POLL_MAX_BYTES = 48 * 1024;

/** 📤️ The largest single frame this door writes — the Rust encoder refuses a larger one before it
 * is ever sealed, and this is the second half of that same law. */
export const WGPU_SOCKET_SEND_MAX_BYTES = 48 * 1024;

/** 📤️ One file a picker handed back. The NAME is half the payload: every import leaf in the repo
 * resolves the file's format from its extension, so contents alone can only be guessed at. */
export type WgpuOpenedFile = { readonly name: string; readonly contents: string };

/** 🚪️ The binding the wgpu shell calls. `bytes` carries a download's already-decoded payload (the
 * `(data, encoding) → bytes` rule is the kernel's, answered in the shell before this is reached), and the
 * resolved string is the request's answer — `[]`-shaped JSON for a pick, `""` for a download. */
export type WgpuHostIo = (requestJson: string, bytes: Uint8Array | null) => Promise<string>;

/** 🚪️ The global name the shell looks up. Declared here, beside both installs, so the three sites cannot
 * drift into two spellings. */
export const WGPU_HOST_IO_GLOBAL = "semioWgpuHostIo";

/** ⬇️ Hands the viewer a file. The anchor is mounted before the click: a detached anchor is not
 * guaranteed to start a download in every browser, and an off-screen mounted one is. */
function downloadBytes(filename: string, mimeType: string, bytes: Uint8Array): void {
  const blob = new Blob([bytes.slice().buffer as ArrayBuffer], { type: mimeType || "application/octet-stream" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.setAttribute("type", mimeType);
  anchor.style.position = "fixed";
  anchor.style.left = "-10000px";
  document.body.appendChild(anchor);
  try {
    anchor.click();
  } finally {
    anchor.remove();
    // 🧹️ Revoked on the next macrotask, never synchronously: Chromium starts the download from the
    // object URL asynchronously, and revoking it in the same task cancels the save it just began.
    setTimeout(() => URL.revokeObjectURL(url), 0);
  }
}

/** 📤️ Opens the native file picker; one entry per selected file, in selection order, empty on cancel.
 *
 * ⌛️ `cancel` is a real browser event on the input, so a dismissed dialog resolves rather than leaving
 * the shell's action drain awaiting a promise nothing will settle. */
function openFiles(accept: string, readAs: string | undefined, multiple: boolean): Promise<readonly WgpuOpenedFile[]> {
  if (typeof document === "undefined") return Promise.resolve([]);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = accept;
    input.multiple = multiple;
    input.style.position = "fixed";
    input.style.left = "-10000px";
    document.body.appendChild(input);
    let settled = false;
    const settle = async (): Promise<void> => {
      if (settled) return;
      settled = true;
      const files = input.files ? Array.from(input.files) : [];
      input.remove();
      const opened: WgpuOpenedFile[] = [];
      for (const file of files) {
        if (readAs === "dataUrl") {
          const contents = await new Promise<string | null>((resolveFile) => {
            const reader = new FileReader();
            reader.onload = () => resolveFile(typeof reader.result === "string" ? reader.result : null);
            reader.onerror = () => resolveFile(null);
            reader.readAsDataURL(file);
          });
          if (contents !== null) opened.push({ contents, name: file.name });
          continue;
        }
        opened.push({ contents: await file.text(), name: file.name });
      }
      resolve(opened);
    };
    input.onchange = () => void settle();
    input.oncancel = () => void settle();
    input.click();
  });
}

type FileWithNativePath = File & { readonly path?: string };

/** 📂 One absolute file path for backbone attach, or `null` on cancel / when the host hides paths. */
function pickNativeFilePath(accept: string | undefined): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    if (accept) input.accept = accept;
    input.style.position = "fixed";
    input.style.left = "-10000px";
    document.body.appendChild(input);
    let settled = false;
    const settle = (): void => {
      if (settled) return;
      settled = true;
      const file = input.files?.[0] as FileWithNativePath | undefined;
      input.remove();
      resolve(file?.path?.trim() ? file.path : null);
    };
    input.onchange = () => settle();
    input.oncancel = () => settle();
    input.click();
  });
}

/** 📂 One absolute folder path for backbone attach, or `null` on cancel / when the host hides paths. */
function pickNativeFolderPath(): Promise<string | null> {
  if (typeof document === "undefined") return Promise.resolve(null);
  return new Promise((resolve) => {
    const input = document.createElement("input");
    input.type = "file";
    input.webkitdirectory = true;
    input.style.position = "fixed";
    input.style.left = "-10000px";
    document.body.appendChild(input);
    let settled = false;
    const settle = (): void => {
      if (settled) return;
      settled = true;
      const first = input.files?.[0] as FileWithNativePath | undefined;
      input.remove();
      const native = first?.path?.trim();
      if (native) resolve(native.replace(/[/\\][^/\\]+$/, "") || native);
      else resolve(null);
    };
    input.onchange = () => settle();
    input.oncancel = () => settle();
    input.click();
  });
}

/** 📇️ Makes one directory hop. A refusal is answered as `{error}` rather than a fabricated status:
 * a fetch that never reached the hub and a hub that answered 5xx are different outcomes, and the
 * shell's directory client branches its retry policy on exactly that distinction. */
async function directoryHttp(request: Extract<WgpuHostIoRequest, { op: "directory-http" }>): Promise<WgpuDirectoryHttpAnswer> {
  const headers: Record<string, string> = {};
  if (request.body !== undefined) headers["content-type"] = "application/json";
  if (request.bearer !== undefined) headers.authorization = `Bearer ${request.bearer}`;
  try {
    const response = await fetch(request.url, { body: request.body, credentials: "include", headers, method: request.method });
    return { body: await response.text(), status: response.status };
  } catch (error) {
    return { error: error instanceof Error ? error.message : String(error) };
  }
}

/** 🗄️ Services one preference hop against the page's real stores. Bounded on BOTH sides: the key must
 * be one the census carries and a written value must fit {@link WGPU_HOST_STORAGE_VALUE_MAX_BYTES}, so
 * the door can never become a wider hole into the origin's storage than the shell's own store already
 * is. A refusal is `{error}`, never a fabricated empty read: "this key holds nothing" and "this call
 * was refused" are different answers and the shell branches on the difference. */
function storageHop(request: Extract<WgpuHostIoRequest, { op: "storage" }>): WgpuHostStorageAnswer {
  if (!wgpuHostStorageCarriesKey(request.key)) return { error: `wgpu-host-io.storage: ${JSON.stringify(request.key)} is not a carried key` };
  const store = request.scope === "session" ? globalThis.sessionStorage : globalThis.localStorage;
  if (!store) return { error: `wgpu-host-io.storage: this realm owns no ${request.scope ?? "local"}Storage` };
  try {
    if (request.verb === "get") return { value: store.getItem(request.key) };
    if (request.verb === "remove") {
      store.removeItem(request.key);
      return { value: null };
    }
    const value = request.value ?? "";
    if (value.length > WGPU_HOST_STORAGE_VALUE_MAX_BYTES) return { error: `wgpu-host-io.storage: ${request.key} exceeds ${WGPU_HOST_STORAGE_VALUE_MAX_BYTES} bytes` };
    store.setItem(request.key, value);
    return { value: null };
  } catch (error) {
    return { error: error instanceof Error ? error.message : String(error) };
  }
}

/** 🔌️ One page-owned socket and its bounded receive queue. `state` is the socket's own readiness as
 * the shell sees it: `connecting` until `onopen`, `closed` from `onclose`/`onerror`/an explicit
 * `close`, and never back again — a redial is a NEW socket id, so a late frame from a dead socket can
 * never be mistaken for its successor's. */
type WgpuSocketEntry = {
  socket: WebSocket;
  state: "connecting" | "open" | "closed";
  queue: WgpuSocketFrame[];
  queueBytes: number;
  dropped: number;
  closeCode?: number;
  error?: string;
};

function socketFrameBytes(frame: WgpuSocketFrame): number {
  return "text" in frame ? frame.text.length : frame.binary.length;
}

/** 🌊️ Admits one frame the peer sent, dropping the OLDEST when either ceiling is reached. A live
 * view must show the newest traffic; what is lost is counted, never hidden. */
function admitSocketFrame(entry: WgpuSocketEntry, frame: WgpuSocketFrame): void {
  entry.queue.push(frame);
  entry.queueBytes += socketFrameBytes(frame);
  while (entry.queue.length > WGPU_SOCKET_QUEUE_MAX_MESSAGES || (entry.queueBytes > WGPU_SOCKET_QUEUE_MAX_BYTES && entry.queue.length > 1)) {
    const dropped = entry.queue.shift();
    if (!dropped) break;
    entry.queueBytes -= socketFrameBytes(dropped);
    entry.dropped += 1;
  }
}

/** 🔤️ `ArrayBuffer` → standard base64, the encoding the Rust half decodes. Chunked so a large frame
 * cannot overflow the argument list of `String.fromCharCode`. */
function socketBytesToBase64(buffer: ArrayBuffer): string {
  const bytes = new Uint8Array(buffer);
  let binary = "";
  for (let offset = 0; offset < bytes.length; offset += 0x8000) binary += String.fromCharCode(...bytes.subarray(offset, offset + 0x8000));
  return btoa(binary);
}

/** 🔤️ Standard base64 → the bytes a `send` writes to the socket. */
function socketBase64ToBytes(value: string): Uint8Array {
  const binary = atob(value);
  const bytes = new Uint8Array(binary.length);
  for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
  return bytes;
}

/** 🔌️ Services every socket verb against one table of page-owned sockets. The table belongs to the
 * door INSTANCE, not to the module, so two mounts on one page (and every test) own disjoint
 * sockets. */
function createSocketDoor(): (request: Extract<WgpuHostIoRequest, { op: "socket" }>) => WgpuSocketAnswer {
  const sockets = new Map<number, WgpuSocketEntry>();

  /** 🧾️ Builds one answer and CONSUMES the loss count in the same step, so `dropped` always means
   * "lost since you last looked" — a total re-reported on every hop would be counted again by the
   * shell-side lane on each one. */
  const answerFor = (entry: WgpuSocketEntry, messages: readonly WgpuSocketFrame[], more: boolean): WgpuSocketAnswer => {
    const dropped = entry.dropped;
    entry.dropped = 0;
    return { closeCode: entry.closeCode, dropped, messages, more, state: entry.state };
  };

  return (request) => {
    if (request.verb === "open") {
      if (sockets.has(request.socketId)) return { error: `wgpu-host-io.socket: ${request.socketId} is already open` };
      let socket: WebSocket;
      try {
        socket = new WebSocket(request.url, request.protocols ? [...request.protocols] : undefined);
      } catch (error) {
        return { error: error instanceof Error ? error.message : String(error) };
      }
      socket.binaryType = "arraybuffer";
      const entry: WgpuSocketEntry = { dropped: 0, queue: [], queueBytes: 0, socket, state: "connecting" };
      sockets.set(request.socketId, entry);
      socket.onopen = () => {
        if (entry.state === "connecting") entry.state = "open";
      };
      socket.onmessage = (event: MessageEvent<unknown>) => {
        const data = event.data;
        if (typeof data === "string") admitSocketFrame(entry, { text: data });
        else if (data instanceof ArrayBuffer) admitSocketFrame(entry, { binary: socketBytesToBase64(data) });
        // 🛟️ A `Blob` payload cannot appear: `binaryType` is pinned to `arraybuffer` above. Anything
        // else is counted rather than silently ignored, so a shell that sees `dropped` knows.
        else entry.dropped += 1;
      };
      socket.onerror = () => {
        entry.error = "wgpu-host-io.socket: socket error";
      };
      socket.onclose = (event: CloseEvent) => {
        entry.state = "closed";
        entry.closeCode = event.code;
      };
      return answerFor(entry, [], false);
    }

    const entry = sockets.get(request.socketId);
    if (!entry) return { error: `wgpu-host-io.socket: unknown socket ${request.socketId}` };

    if (request.verb === "close") {
      entry.state = "closed";
      sockets.delete(request.socketId);
      try {
        entry.socket.close();
      } catch {
        // 🛟️ best-effort — a socket already closing refuses a second close in some engines.
      }
      return answerFor(entry, [], false);
    }

    if (request.verb === "send") {
      if (entry.state !== "open") return { error: `wgpu-host-io.socket: ${request.socketId} is ${entry.state}` };
      try {
        if (request.text !== undefined) {
          if (request.text.length > WGPU_SOCKET_SEND_MAX_BYTES) return { error: `wgpu-host-io.socket: frame exceeds ${WGPU_SOCKET_SEND_MAX_BYTES} bytes` };
          entry.socket.send(request.text);
        } else if (request.binary !== undefined) {
          const bytes = socketBase64ToBytes(request.binary);
          if (bytes.byteLength > WGPU_SOCKET_SEND_MAX_BYTES) return { error: `wgpu-host-io.socket: frame exceeds ${WGPU_SOCKET_SEND_MAX_BYTES} bytes` };
          entry.socket.send(bytes);
        } else {
          return { error: "wgpu-host-io.socket: send carries neither text nor binary" };
        }
      } catch (error) {
        return { error: error instanceof Error ? error.message : String(error) };
      }
      return answerFor(entry, [], entry.queue.length > 0);
    }

    // 📥️ `poll`: one PAGE of what the peer sent, never the whole queue, with `more` telling the shell
    // whether another hop would return without waiting.
    const maxMessages = Math.max(1, Math.min(request.maxMessages ?? WGPU_SOCKET_POLL_MAX_MESSAGES, WGPU_SOCKET_POLL_MAX_MESSAGES));
    const maxBytes = Math.max(1, Math.min(request.maxBytes ?? WGPU_SOCKET_POLL_MAX_BYTES, WGPU_SOCKET_POLL_MAX_BYTES));
    const page: WgpuSocketFrame[] = [];
    let pageBytes = 0;
    while (page.length < maxMessages && entry.queue.length > 0) {
      const next = entry.queue[0];
      if (!next) break;
      const nextBytes = pageBytes + socketFrameBytes(next);
      if (page.length > 0 && nextBytes > maxBytes) break;
      pageBytes = nextBytes;
      page.push(next);
      entry.queue.shift();
      entry.queueBytes -= socketFrameBytes(next);
    }
    const answer = answerFor(entry, page, entry.queue.length > 0);
    // 🧯️ A closed socket is forgotten only once everything it really received has been handed over —
    // frames that arrived before the close are the consumer's, and dropping them with the entry would
    // lose the peer's last word (a `bye`, a revocation close reason) exactly when it matters most.
    if (entry.state === "closed" && entry.queue.length === 0) sockets.delete(request.socketId);
    return answer;
  };
}

/** 🚪️ The page-owned implementation of {@link WgpuHostIo}. */
export function createWgpuPageHostIo(): WgpuHostIo {
  const socketDoor = createSocketDoor();
  return async (requestJson, bytes) => {
    const request = JSON.parse(requestJson) as WgpuHostIoRequest;
    if (request.op === "directory-http") return JSON.stringify(await directoryHttp(request));
    if (request.op === "storage") return JSON.stringify(storageHop(request));
    if (request.op === "socket") return JSON.stringify(socketDoor(request));
    if (request.op === "download-media-export") {
      if (!bytes) throw new Error("wgpu-host-io.download-media-export: no bytes");
      downloadBytes(request.filename, request.mimeType, bytes);
      return "";
    }
    if (request.op === "request-file-open") {
      const opened = await openFiles(request.accept, request.readAs, Boolean(request.multiple));
      return JSON.stringify(opened);
    }
    if (request.op === "request-native-file-path") {
      return JSON.stringify(await pickNativeFilePath(request.accept));
    }
    if (request.op === "request-native-folder-path") {
      return JSON.stringify(await pickNativeFolderPath());
    }
    throw new Error(`wgpu-host-io: unknown op ${JSON.stringify((request as { readonly op?: unknown }).op)}`);
  };
}

/** 🚪️ Installs {@link createWgpuPageHostIo} as the global the shell calls, when this isolate really owns
 * a document. Idempotent: a second mount on the same page keeps the first install. */
export function installWgpuPageHostIo(host: { [WGPU_HOST_IO_GLOBAL]?: WgpuHostIo } = globalThis as never): void {
  if (typeof document === "undefined" || host[WGPU_HOST_IO_GLOBAL]) return;
  host[WGPU_HOST_IO_GLOBAL] = createWgpuPageHostIo();
}
