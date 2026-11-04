import { formatMediaTime, mediaKeyCommand, mediaTimeValueText, seekedMediaPosition, trimValueForSelection } from "../../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { collectMediaTransportBytes, type MediaTransportPort } from "../../🚚️lifecycle/🟦️.ts";
import { mediaSlotAuthorityKey, type PresentedMediaSlot } from "../🟦️.ts";

type MediaHost = { readonly element: HTMLDivElement; readonly authority: string; place: (slot: PresentedMediaSlot) => void; dispose: () => void };

function createHost(slot: PresentedMediaSlot, port: MediaTransportPort, apply?: (action: string, args: Readonly<Record<string, unknown>>) => void): MediaHost {
  const props = slot.props;
  const element = document.createElement("div");
  element.dataset.mediaSlot = slot.token;
  element.dataset.mediaWindow = slot.windowId;
  element.dataset.mediaKind = props.kind;
  element.dataset.mediaType = props.mediaType;
  element.dataset.uiNodeId = slot.nodeId;
  element.dataset.uiNodeKey = slot.nodeKey;
  element.lang = props.locale;
  element.style.cssText = "position:absolute;pointer-events:auto;overflow:hidden;display:flex;flex-direction:column;gap:4px;padding:4px;box-sizing:border-box;background:var(--base,Canvas);color:var(--foreground,CanvasText);";
  for (const type of ["pointerdown", "pointerup", "pointermove", "pointercancel", "click", "dblclick", "wheel", "contextmenu", "input", "change"]) element.addEventListener(type, (event) => event.stopPropagation());
  let active = true;
  let cancelled = false;
  let url: string | null = null;
  let media: HTMLMediaElement | null = null;
  let position = props.positionMs;
  let duration = props.durationMs;
  let selection: [number, number] | null = props.selectionStartMs !== null && props.selectionEndMs !== null ? [props.selectionStartMs, props.selectionEndMs] : null;
  const release = (): void => {
    if (media) {
      media.pause();
      media.removeAttribute("src");
      media.load();
      media = null;
    }
    if (url) {
      URL.revokeObjectURL(url);
      url = null;
    }
  };
  const status = (state: string, label: string): void => {
    element.dataset.mediaState = state;
    element.setAttribute("role", "status");
    element.setAttribute("aria-live", "polite");
    element.replaceChildren(document.createTextNode(label));
  };
  const failed = (): void => {
    release();
    if (active) status("failed", props.labels.unsupported);
  };
  const ready = (): void => {
    if (!active || !url) return;
    const player = document.createElement(props.kind);
    media = player;
    player.preload = "metadata";
    player.autoplay = false;
    player.src = url;
    player.setAttribute("aria-label", props.kind === "audio" ? props.labels.audio : props.labels.video);
    if (props.kind === "video") player.style.cssText = "min-height:0;width:100%;flex:1;object-fit:contain;";
    const button = document.createElement("button");
    button.type = "button";
    const seek = document.createElement("input");
    seek.type = "range";
    seek.min = "0";
    seek.setAttribute("aria-label", props.labels.seek);
    const seekLabel = document.createElement("label");
    seekLabel.append(props.labels.seek, seek);
    const positionOutput = document.createElement("output");
    positionOutput.setAttribute("aria-label", props.labels.position);
    const durationOutput = document.createElement("span");
    durationOutput.setAttribute("aria-label", props.labels.duration);
    const start = document.createElement("input");
    start.type = "range";
    start.setAttribute("aria-label", props.labels.selectionStart);
    const end = document.createElement("input");
    end.type = "range";
    end.setAttribute("aria-label", props.labels.selectionEnd);
    const startLabel = document.createElement("label");
    startLabel.append(props.labels.selectionStart, start);
    const endLabel = document.createElement("label");
    endLabel.append(props.labels.selectionEnd, end);
    const selectionControls = document.createElement("div");
    selectionControls.append(startLabel, endLabel);
    const cut = document.createElement("button");
    cut.type = "button";
    cut.textContent = props.labels.cutSelection;
    cut.setAttribute("aria-label", props.labels.cutSelection);
    cut.hidden = !(props.trim && apply);
    cut.addEventListener("click", () => {
      const selected = selection ?? [0, duration ?? 1];
      const value = props.trim ? trimValueForSelection(props.trim, selected[0], selected[1]) : null;
      if (props.trim && apply && value !== null) apply(props.trim.actionId, { revision: props.trim.revision, value });
    });
    const update = (): void => {
      const maximum = Math.max(1, duration ?? 1);
      const selected = selection ?? [0, maximum];
      button.textContent = player.paused ? props.labels.play : props.labels.pause;
      button.setAttribute("aria-label", button.textContent);
      seek.max = String(maximum);
      seek.value = String(Math.min(maximum, position));
      seek.setAttribute("aria-valuetext", mediaTimeValueText(position, props.locale, props.labels));
      seek.disabled = duration === null;
      positionOutput.textContent = formatMediaTime(position, props.locale);
      positionOutput.setAttribute("aria-valuetext", mediaTimeValueText(position, props.locale, props.labels));
      durationOutput.textContent = duration === null ? props.labels.unknownDuration : formatMediaTime(duration, props.locale);
      if (duration === null) durationOutput.removeAttribute("aria-valuetext");
      else durationOutput.setAttribute("aria-valuetext", mediaTimeValueText(duration, props.locale, props.labels));
      selectionControls.hidden = duration === null;
      cut.disabled = !props.trim || duration === null || trimValueForSelection(props.trim, (selection ?? [0, duration])[0], (selection ?? [0, duration])[1]) === null;
      start.min = "0";
      start.max = String(Math.max(0, selected[1] - 1));
      start.value = String(selected[0]);
      start.setAttribute("aria-valuetext", mediaTimeValueText(selected[0], props.locale, props.labels));
      end.min = String(Math.min(maximum, selected[0] + 1));
      end.max = String(maximum);
      end.value = String(selected[1]);
      end.setAttribute("aria-valuetext", mediaTimeValueText(selected[1], props.locale, props.labels));
    };
    button.addEventListener("click", () => {
      if (player.paused) void player.play().catch((error: unknown) => { if (active && !(error instanceof DOMException && error.name === "AbortError")) failed(); });
      else player.pause();
    });
    seek.addEventListener("input", () => { position = Number(seek.value); player.currentTime = position / 1000; update(); });
    start.addEventListener("input", () => { selection = [Number(start.value), (selection ?? [0, duration!])[1]]; update(); });
    end.addEventListener("input", () => { selection = [(selection ?? [0, duration!])[0], Number(end.value)]; update(); });
    for (const type of ["play", "pause", "ended"]) player.addEventListener(type, update);
    player.addEventListener("timeupdate", () => { position = Math.max(0, Math.round(player.currentTime * 1000)); update(); });
    player.addEventListener("loadedmetadata", () => { player.currentTime = Math.min(position, duration ?? position) / 1000; }, { once: true });
    player.addEventListener("durationchange", () => {
      if (Number.isFinite(player.duration) && player.duration > 0) {
        duration = Math.max(1, Math.round(player.duration * 1000));
        if (selection) selection = [Math.min(duration - 1, selection[0]), Math.min(duration, selection[1])];
        update();
      }
    });
    player.addEventListener("error", failed);
    element.tabIndex = 0;
    element.setAttribute("aria-keyshortcuts", "Space ArrowLeft ArrowRight Home End");
    element.addEventListener("keydown", (event) => {
      const target = event.target as HTMLElement;
      const command = mediaKeyCommand({ key: event.key, shiftKey: event.shiftKey, ctrlKey: event.ctrlKey, altKey: event.altKey, metaKey: event.metaKey, targetTag: target.tagName, targetType: target instanceof HTMLInputElement ? target.type : null }, duration);
      if (!command) {
        if (target instanceof HTMLInputElement && target.type === "range" && ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Home", "End", "PageUp", "PageDown"].includes(event.key)) event.stopPropagation();
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      if (command.kind === "toggle") {
        if (player.paused) void player.play().catch((error: unknown) => { if (active && !(error instanceof DOMException && error.name === "AbortError")) failed(); });
        else player.pause();
        return;
      }
      position = seekedMediaPosition(command, Math.round(player.currentTime * 1000), duration);
      player.currentTime = position / 1000;
      update();
    });
    element.dataset.mediaState = "ready";
    element.removeAttribute("role");
    element.removeAttribute("aria-live");
    element.replaceChildren(player, button, seekLabel, positionOutput, durationOutput, selectionControls, cut);
    update();
  };
  if (props.capability.status !== "ready" || !props.resource) status(props.capability.status, props.capability.reason ?? (props.capability.status === "loading" ? props.labels.loading : props.labels.unsupported));
  else {
    status("loading", props.labels.loading);
    const progress = document.createElement("progress");
    progress.setAttribute("aria-label", props.labels.progress);
    const cancel = document.createElement("button");
    cancel.type = "button";
    cancel.textContent = props.labels.cancel;
    cancel.addEventListener("click", () => { cancelled = true; cancel.disabled = true; });
    element.append(progress, cancel);
    void collectMediaTransportBytes({ instanceId: slot.appInstanceId, parentDocumentId: slot.parentDocumentId, revision: BigInt(props.resource.revision), generation: BigInt(props.resource.generation), mediaType: props.mediaType, port, cancelled: () => cancelled || !active, progress: (value) => {
      if (active) progress.dataset.appliedProgress = String(value.applied_progress);
    } }).then((bytes) => {
      if (!active || cancelled || !bytes) { if (active) status("cancelled", props.labels.cancelled); return; }
      url = URL.createObjectURL(new Blob(bytes.chunks.map((chunk) => chunk.slice().buffer as ArrayBuffer), { type: bytes.mediaType }));
      if (!active) { release(); return; }
      ready();
    }).catch(() => { if (active) failed(); });
  }
  const place = (next: PresentedMediaSlot): void => {
    const { rect, clip } = next;
    element.style.left = `${rect.x}px`;
    element.style.top = `${rect.y}px`;
    element.style.width = `${rect.width}px`;
    element.style.height = `${rect.height}px`;
    element.style.zIndex = String(next.paintOrder);
    element.style.visibility = next.occluded ? "hidden" : "visible";
    element.style.pointerEvents = next.occluded ? "none" : "auto";
    element.inert = next.occluded;
    if (next.occluded) element.setAttribute("aria-hidden", "true");
    else element.removeAttribute("aria-hidden");
    const top = Math.max(0, clip.y - rect.y);
    const left = Math.max(0, clip.x - rect.x);
    const right = Math.max(0, rect.x + rect.width - clip.x - clip.width);
    const bottom = Math.max(0, rect.y + rect.height - clip.y - clip.height);
    element.style.clipPath = `inset(${top}px ${right}px ${bottom}px ${left}px)`;
  };
  place(slot);
  return { element, authority: mediaSlotAuthorityKey(slot), place, dispose: () => {
    if (!active) return;
    active = false;
    cancelled = true;
    release();
    element.remove();
  } };
}

/** 🎛️ Reconciles real media controls from the GPU-accepted slot list without remounting stable resources. */
export function createBrowserMediaOverlay(root: HTMLElement, portForSlot: (slot: PresentedMediaSlot) => MediaTransportPort, applyForSlot?: (slot: PresentedMediaSlot) => ((action: string, args: Readonly<Record<string, unknown>>) => void) | undefined): { readonly accept: (slots: readonly PresentedMediaSlot[]) => void; readonly owns: (windowId: string, nodeId: number, nodeKey: string) => boolean; readonly dispose: () => void } {
  const overlay = document.createElement("div");
  overlay.dataset.wgpuMediaOverlay = "true";
  overlay.style.cssText = "position:absolute;inset:0;pointer-events:none;overflow:hidden;";
  root.append(overlay);
  const hosts = new Map<string, MediaHost>();
  let slots: readonly PresentedMediaSlot[] = [];
  let disposed = false;
  const dispose = (): void => {
    for (const host of hosts.values()) host.dispose();
    hosts.clear();
    slots = [];
  };
  return {
    accept: (next) => {
      if (disposed) return;
      const tokens = new Set(next.map((slot) => slot.token));
      for (const [token, host] of hosts) if (!tokens.has(token)) { host.dispose(); hosts.delete(token); }
      for (const slot of next) {
        let host = hosts.get(slot.token);
        if (host && host.authority !== mediaSlotAuthorityKey(slot)) { host.dispose(); hosts.delete(slot.token); host = undefined; }
        if (!host) { host = createHost(slot, portForSlot(slot), applyForSlot?.(slot)); hosts.set(slot.token, host); overlay.append(host.element); }
        host.place(slot);
      }
      slots = next;
    },
    owns: (windowId, nodeId, nodeKey) => slots.some((slot) => slot.windowId === windowId && slot.nodeId === String(nodeId) && slot.nodeKey === nodeKey),
    dispose: () => { if (disposed) return; disposed = true; dispose(); overlay.remove(); },
  };
}
