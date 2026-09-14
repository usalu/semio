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

/** 🚪️ What the shell asks the page to do. `op` is the whole vocabulary — an unknown one is refused
 * loudly rather than answered with an empty pick, which is indistinguishable from a cancelled dialog. */
export type WgpuHostIoRequest =
  | { readonly op: "download-media-export"; readonly filename: string; readonly mimeType: string }
  | { readonly op: "request-file-open"; readonly accept: string; readonly readAs?: string; readonly multiple?: boolean };

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

/** 🚪️ The page-owned implementation of {@link WgpuHostIo}. */
export function createWgpuPageHostIo(): WgpuHostIo {
  return async (requestJson, bytes) => {
    const request = JSON.parse(requestJson) as WgpuHostIoRequest;
    if (request.op === "download-media-export") {
      if (!bytes) throw new Error("wgpu-host-io.download-media-export: no bytes");
      downloadBytes(request.filename, request.mimeType, bytes);
      console.log(`[DEBUG] wgpu-host-io download name=${request.filename} type=${request.mimeType} bytes=${bytes.byteLength}`);
      return "";
    }
    if (request.op === "request-file-open") {
      const opened = await openFiles(request.accept, request.readAs, Boolean(request.multiple));
      console.log(`[DEBUG] wgpu-host-io file-open accept=${request.accept} files=${opened.length} bytes=${opened.reduce((total, file) => total + file.contents.length, 0)}`);
      return JSON.stringify(opened);
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
