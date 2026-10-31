/** 📏️ The native block geometry needed to keep a scrollport usable beside natural chrome. */
export interface ScrollportMeasurement {
  readonly viewport: number;
  readonly chrome: readonly number[];
  readonly controls: readonly number[];
  readonly padding: readonly [number, number];
}

/** 📐️ The minimum usable scrollport and the resulting outer vertical extent. */
export interface ScrollportGeometry {
  readonly minimum: number;
  readonly main: number;
  readonly total: number;
  readonly overflow: number;
}

/** 📏️ Fits natural chrome and one whole intrinsic control without clipping either. */
export function intrinsicScrollport(value: ScrollportMeasurement): ScrollportGeometry {
  if (value.padding.length !== 2 || [value.viewport, ...value.chrome, ...value.controls, ...value.padding].some(number => !Number.isFinite(number) || number < 0)) throw new RangeError("Intrinsic scrollport measurement required");
  const minimum = Math.max(0, ...value.controls) + value.padding[0] + value.padding[1];
  const chrome = value.chrome.reduce((sum, height) => sum + height, 0);
  const main = Math.max(minimum, value.viewport - chrome);
  const total = chrome + main;
  if (![minimum, main, total].every(Number.isFinite)) throw new RangeError("Finite intrinsic scrollport geometry required");
  return Object.freeze({ minimum, main, total, overflow: Math.max(0, total - value.viewport) });
}

/** 👁️ Decides whether changed owned layout clips the actual focused control. */
export function scrollportFocusReveal(value: { readonly changed: boolean; readonly eligible: boolean; readonly control: readonly number[]; readonly clips: readonly (readonly number[])[] }): boolean {
  for (const bounds of [value.control, ...value.clips]) if (bounds.length !== 4 || bounds.some(number => !Number.isFinite(number)) || bounds[0]! > bounds[2]! || bounds[1]! > bounds[3]!) throw new RangeError("Finite ordered focus bounds required");
  return value.changed && value.eligible && value.clips.some(clip => value.control[0]! < clip[0]! || value.control[1]! < clip[1]! || value.control[2]! > clip[2]! || value.control[3]! > clip[3]!);
}

/** 📐️ Adds one physical pixel of native alignment room to the four authored CSS margins. */
export function scrollportFocusMargins(value: { readonly margins: readonly number[]; readonly pixelRatio: number }): readonly number[] {
  if (value.margins.length !== 4 || value.margins.some(number => !Number.isFinite(number)) || !Number.isFinite(value.pixelRatio) || value.pixelRatio <= 0) throw new RangeError("Finite physical focus alignment required");
  const result = value.margins.map(margin => margin + 1 / value.pixelRatio);
  if (result.some(number => !Number.isFinite(number))) throw new RangeError("Finite guarded focus margins required");
  return Object.freeze(result);
}

const CONTROLS = "button,input:not([type=hidden]),select,textarea,a[href],[role=button],[tabindex]:not([tabindex='-1'])";
const pixels = (value: string): number => Number.parseFloat(value) || 0;
const eligible = (node: HTMLElement): boolean => node.closest("[inert],[hidden],[aria-hidden=true]") === null && node.getClientRects().length > 0;
const blockInsets = (node: HTMLElement): number => {
  const style = getComputedStyle(node);
  return pixels(style.paddingTop) + pixels(style.paddingBottom) + pixels(style.borderTopWidth) + pixels(style.borderBottomWidth);
};

/** 👁️ Scopes native nearest alignment without replacing focus or retaining temporary margins. */
export function revealScrollportFocus(focused: HTMLElement, pixelRatio: number): void {
  const edges = ["scroll-margin-top", "scroll-margin-right", "scroll-margin-bottom", "scroll-margin-left"];
  const before = edges.map(name => ({ name, value: focused.style.getPropertyValue(name), priority: focused.style.getPropertyPriority(name) }));
  const computed = getComputedStyle(focused);
  const margins = scrollportFocusMargins({ margins: edges.map(name => pixels(computed.getPropertyValue(name))), pixelRatio });
  try {
    edges.forEach((name, index) => focused.style.setProperty(name, margins[index] + "px", "important"));
    focused.scrollIntoView({ block: "nearest", inline: "nearest" });
  } finally {
    for (const { name, value, priority } of before) if (value) focused.style.setProperty(name, value, priority); else focused.style.removeProperty(name);
  }
}

/** 🪟️ Keeps an actual main scrollport reachable without replacing its controls or hiding natural chrome. */
export function bindIntrinsicScrollport(main: HTMLElement): () => void {
  const app = main.parentElement;
  if (app === null) throw new Error("Intrinsic scrollport application required");
  let active = true, frame = 0;
  let previousPort: HTMLElement | undefined, previousGeometry: string | undefined;
  const observed = new Set<Element>();
  const minima = new Map<HTMLElement, { before: string; value: string }>();
  const restore = (node: HTMLElement) => {
    const entry = minima.get(node);
    if (entry !== undefined && node.style.minBlockSize === entry.value) node.style.minBlockSize = entry.before;
    minima.delete(node);
  };
  const minimum = (node: HTMLElement, value: number) => {
    const next = value + "px", previous = minima.get(node);
    if (previous === undefined) minima.set(node, { before: node.style.minBlockSize, value: next });
    else previous.value = next;
    if (node.style.minBlockSize !== next) node.style.minBlockSize = next;
  };
  const resize = new ResizeObserver(() => schedule());
  const schedule = () => {
    if (active && frame === 0) frame = requestAnimationFrame(measure);
  };
  const measure = () => {
    frame = 0;
    if (!active || !main.isConnected) return;
    const port = [...main.querySelectorAll<HTMLElement>("[data-course-scrollport]")].find(eligible) ?? main;
    const controls = [...port.querySelectorAll<HTMLElement>(CONTROLS)].filter(eligible);
    const saved = controls.map(node => ({ node, block: node.style.blockSize, flex: node.style.flex, align: node.style.alignSelf }));
    const targets = new Set<Element>([app, main, port, ...controls]);
    let sizes: number[];
    try {
      for (const { node } of saved) {
        node.style.blockSize = "max-content";
        node.style.flex = "none";
        node.style.alignSelf = "start";
      }
      sizes = controls.map(node => {
        let extent = node.getBoundingClientRect().height;
        for (let parent = node.parentElement; parent !== null && parent !== port; parent = parent.parentElement) extent += blockInsets(parent);
        return Math.max(0, extent);
      });
    } finally {
      for (const { node, block, flex, align } of saved) {
        node.style.blockSize = block;
        node.style.flex = flex;
        node.style.alignSelf = align;
      }
    }
    const flow = (node: Element): number => {
      if (!(node instanceof HTMLElement)) return 0;
      const style = getComputedStyle(node);
      if (style.display === "none" || style.display === "contents" || style.position === "absolute" || style.position === "fixed") return 0;
      targets.add(node);
      return Math.max(0, node.getBoundingClientRect().height + pixels(style.marginTop) + pixels(style.marginBottom));
    };
    let inside = 0;
    for (let child: HTMLElement = port; child !== main;) {
      const parent = child.parentElement;
      if (parent === null) throw new Error("Intrinsic scrollport containment required");
      const style = getComputedStyle(parent);
      const siblings = [...parent.children].filter(node => node !== child).map(flow).filter(value => value > 0);
      if (style.display !== "contents") {
        inside += blockInsets(parent);
        if (style.display !== "flex" || style.flexDirection.startsWith("column")) inside += siblings.reduce((sum, height) => sum + height, 0) + pixels(style.rowGap) * siblings.length;
      }
      targets.add(parent);
      child = parent;
    }
    const chrome = [...app.children].filter(node => node !== main).map(flow);
    const inset = blockInsets(port);
    const geometry = intrinsicScrollport({ viewport: app.clientHeight, chrome: [...chrome, inside], controls: sizes, padding: [inset, 0] });
    for (const node of minima.keys()) if (node !== main && node !== port) restore(node);
    minimum(port, geometry.minimum);
    minimum(main, inside + geometry.minimum);
    for (const node of observed) if (!targets.has(node)) { resize.unobserve(node); observed.delete(node); }
    for (const node of targets) if (!observed.has(node)) { resize.observe(node); observed.add(node); }
    const signature = JSON.stringify([window.innerWidth, window.innerHeight, app.clientWidth, app.clientHeight, main.clientWidth, main.clientHeight, port.clientWidth, port.clientHeight, inside, inset, chrome, sizes, geometry.minimum, controls.map(node => { const rect = node.getBoundingClientRect(); return [rect.width, rect.height]; })]);
    const changed = port !== previousPort || signature !== previousGeometry;
    previousPort = port;
    previousGeometry = signature;
    const focused = document.activeElement;
    if (!changed || !(focused instanceof HTMLElement) || !main.contains(focused) || !focused.matches(CONTROLS) || !eligible(focused)) return;
    const bounds = focused.getBoundingClientRect();
    const clips = [[0, 0, window.innerWidth, window.innerHeight]];
    for (let ancestor = focused.parentElement; ancestor !== null; ancestor = ancestor.parentElement) {
      const style = getComputedStyle(ancestor);
      const x = /^(auto|scroll|hidden|clip)$/u.test(style.overflowX), y = /^(auto|scroll|hidden|clip)$/u.test(style.overflowY);
      if (!x && !y) continue;
      const rect = ancestor.getBoundingClientRect();
      clips.push([x ? rect.left : 0, y ? rect.top : 0, x ? rect.right : window.innerWidth, y ? rect.bottom : window.innerHeight]);
    }
    if (scrollportFocusReveal({ changed, eligible: true, control: [bounds.left, bounds.top, bounds.right, bounds.bottom], clips })) revealScrollportFocus(focused, window.devicePixelRatio);

  };
  const mutations = new MutationObserver(schedule);
  mutations.observe(main, { childList: true, subtree: true, characterData: true, attributes: true, attributeFilter: ["class", "hidden", "inert", "lang", "data-grid-active"] });
  const appearance = new MutationObserver(schedule);
  appearance.observe(document.documentElement, { attributes: true, attributeFilter: ["style", "class", "lang"] });
  window.addEventListener("resize", schedule);
  document.fonts?.addEventListener("loadingdone", schedule);
  void document.fonts?.ready.then(schedule);
  measure();
  return () => {
    active = false;
    cancelAnimationFrame(frame);
    resize.disconnect();
    mutations.disconnect();
    appearance.disconnect();
    window.removeEventListener("resize", schedule);
    document.fonts?.removeEventListener("loadingdone", schedule);
    for (const node of minima.keys()) restore(node);
  };
}
