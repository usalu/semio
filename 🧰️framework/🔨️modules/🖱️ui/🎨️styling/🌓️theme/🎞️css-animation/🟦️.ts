
/** ⏱️ One rule that starts CSS animations, addressed by its resolved selector path. */
export interface CssClockRule {
  selector: string;
  keyframes: readonly string[];
}

/** 🖌️ One rule that PAINTS with animated custom properties — reads them from a declaration whose
 * own property is not itself a custom property, directly or through a custom-property chain. A rule that
 * only forwards the value into another custom property is not a paint and owns no clock. */
export interface CssPaintRule {
  selector: string;
  properties: readonly string[];
}

/** 🔬️ What {@link analyzeCssAnimationScope} reads out of one stylesheet. */
export interface CssAnimationScope {
  animatedCustomProperties: readonly string[];
  keyframesByProperty: Readonly<Record<string, readonly string[]>>;
  propertyInheritance: Readonly<Record<string, boolean>>;
  clocks: readonly CssClockRule[];
  paints: readonly CssPaintRule[];
  rootClocks: readonly CssClockRule[];
}

/** 🌳️ Selectors that address the document root, whose animated inherited custom properties
 * re-resolve every element's computed style on every frame. */
const CSS_DOCUMENT_ROOT_SELECTORS = new Set([":root", "html", ":root:root", "html:root"]);

interface CssBlock {
  prelude: string;
  declarations: readonly (readonly [string, string])[];
  children: readonly CssBlock[];
}

function stripCssComments(css: string): string {
  return css.replace(/\/\*[\s\S]*?\*\//g, "");
}

function parseCssBlocks(css: string): readonly CssBlock[] {
  const root: CssBlock = { prelude: "", declarations: [], children: [] };
  const stack: { block: CssBlock; declarations: [string, string][]; children: CssBlock[] }[] = [{ block: root, declarations: [], children: [] }];
  let buffer = "";
  let depth = 0;
  for (let index = 0; index < css.length; index += 1) {
    const character = css[index]!;
    if (character === "'" || character === '"') {
      const end = css.indexOf(character, index + 1);
      const slice = end === -1 ? css.slice(index) : css.slice(index, end + 1);
      buffer += slice;
      index += slice.length - 1;
      continue;
    }
    if (character === "(") depth += 1;
    else if (character === ")") depth = Math.max(0, depth - 1);
    if (depth > 0) { buffer += character; continue; }
    if (character === "{") {
      const frame = { block: { prelude: buffer.trim(), declarations: [], children: [] } as CssBlock, declarations: [] as [string, string][], children: [] as CssBlock[] };
      stack.push(frame);
      buffer = "";
      continue;
    }
    if (character === "}") {
      const declaration = buffer.trim();
      const frame = stack.pop();
      if (!frame) { buffer = ""; continue; }
      if (declaration.length > 0) {
        const colon = declaration.indexOf(":");
        if (colon > 0) frame.declarations.push([declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()]);
      }
      const closed: CssBlock = { prelude: frame.block.prelude, declarations: frame.declarations, children: frame.children };
      stack.at(-1)?.children.push(closed);
      buffer = "";
      continue;
    }
    if (character === ";") {
      const declaration = buffer.trim();
      const colon = declaration.indexOf(":");
      if (colon > 0) stack.at(-1)?.declarations.push([declaration.slice(0, colon).trim(), declaration.slice(colon + 1).trim()]);
      buffer = "";
      continue;
    }
    buffer += character;
  }
  return stack[0]?.children ?? [];
}

function cssVarReads(value: string): readonly string[] {
  return [...value.matchAll(/var\(\s*(--[\w-]+)/g)].map((match) => match[1]!);
}

function cssAnimationNames(value: string): readonly string[] {
  const nonNames = new Set(["none", "infinite", "linear", "ease", "ease-in", "ease-out", "ease-in-out", "alternate", "alternate-reverse", "reverse", "normal", "forwards", "backwards", "both", "running", "paused", "step-start", "step-end", "important"]);
  return value.split(",").flatMap((layer) => layer.replace(/!important/g, " ").split(/\s+/).filter((token) => /^[A-Za-z_-][\w-]*$/.test(token) && !nonNames.has(token) && !token.startsWith("--") && !token.startsWith("cubic-bezier") && !token.startsWith("steps")));
}

/** 🔬️ Reads a stylesheet's animation scope: which custom properties any `@keyframes` animates,
 * how each is registered, which rules start those clocks, and which rules paint with them.
 *
 * Exists because the cost of a CSS animation is the size of the style invalidation it causes, not the
 * number of animations: an animated registered custom property declared `inherits: true` and started on
 * the document root re-resolves the computed style of EVERY element on EVERY frame. Measured on the
 * generation3d procedural example (ticket 2026/09/09, `📓️host-reconcile-silence-2026-09-12.md`): nine
 * such clocks on `:root` cost 112.3 s of style recalculation out of 129.0 s of main-thread task time and
 * starved the plugin host's continuation pump to one macrotask per frame.
 *
 * Nested rules (`@utility x { &::after { … } }`, `@media … { … }`) resolve to a space-joined selector
 * path, so a clock and the paint it drives are comparable by identity.
 * See https://drafts.css-houdini.org/css-properties-values-api/#inherits-descriptor. */
export function analyzeCssAnimationScope(css: string): CssAnimationScope {
  const blocks = parseCssBlocks(stripCssComments(css));
  const keyframesByProperty = new Map<string, Set<string>>();
  const propertyInheritance: Record<string, boolean> = {};
  const clocks: CssClockRule[] = [];
  const directReads = new Map<string, Set<string>>();
  const customPropertyReads = new Map<string, Set<string>>();
  const indirectReads = new Map<string, Set<string>>();
  const walk = (block: CssBlock, path: readonly string[]): void => {
    const prelude = block.prelude;
    if (/^@keyframes\s/.test(prelude)) {
      const name = prelude.replace(/^@keyframes\s+/, "").trim();
      const collect = (node: CssBlock): void => {
        for (const [property] of node.declarations) if (property.startsWith("--")) (keyframesByProperty.get(property) ?? keyframesByProperty.set(property, new Set()).get(property)!).add(name);
        for (const child of node.children) collect(child);
      };
      collect(block);
      return;
    }
    if (/^@property\s/.test(prelude)) {
      const name = prelude.replace(/^@property\s+/, "").trim();
      const inherits = block.declarations.find(([property]) => property === "inherits")?.[1];
      propertyInheritance[name] = inherits?.trim() === "true";
      return;
    }
    const scoping = /^@(media|supports|layer|container|scope)\b/.test(prelude) || prelude.length === 0;
    const selectorPath = scoping ? path : [...path, prelude];
    const selector = selectorPath.join(" ");
    for (const [property, value] of block.declarations) {
      if (property === "animation" || property === "animation-name") {
        const keyframes = cssAnimationNames(value);
        if (keyframes.length > 0) clocks.push({ selector, keyframes });
      }
      const reads = cssVarReads(value);
      if (reads.length === 0) continue;
      const sink = property.startsWith("--") ? customPropertyReads.get(property) ?? customPropertyReads.set(property, new Set()).get(property)! : directReads.get(selector) ?? directReads.set(selector, new Set()).get(selector)!;
      for (const read of reads) sink.add(read);
      if (!property.startsWith("--")) for (const read of reads) (indirectReads.get(selector) ?? indirectReads.set(selector, new Set()).get(selector)!).add(read);
    }
    for (const child of block.children) walk(child, selectorPath);
  };
  for (const block of blocks) walk(block, []);
  const animated = [...keyframesByProperty.keys()].sort();
  const animatedSet = new Set(animated);
  const resolve = (name: string, seen: Set<string>): readonly string[] => {
    if (seen.has(name)) return [];
    seen.add(name);
    if (animatedSet.has(name)) return [name];
    return [...(customPropertyReads.get(name) ?? [])].flatMap((next) => resolve(next, seen));
  };
  const paints: CssPaintRule[] = [];
  for (const [selector, reads] of directReads) {
    const resolved = [...new Set([...reads].flatMap((read) => resolve(read, new Set())))].sort();
    if (resolved.length > 0) paints.push({ selector, properties: resolved });
  }
  return {
    animatedCustomProperties: animated,
    keyframesByProperty: Object.fromEntries(animated.map((name) => [name, [...keyframesByProperty.get(name)!].sort()])),
    propertyInheritance,
    clocks,
    paints: paints.sort((left, right) => left.selector.localeCompare(right.selector)),
    rootClocks: clocks.filter((clock) => clock.selector.split(" ").every((part) => CSS_DOCUMENT_ROOT_SELECTORS.has(part.trim()))),
  };
}
/** ⚖️ The three laws of {@link analyzeCssAnimationScope}, in the order the fixture states them.
 * `no-document-root-clock` — no rule whose whole selector path addresses the document root may start an
 * animation; `animated-custom-properties-are-non-inherited` — a property any `@keyframes` animates must be
 * registered `inherits: false`, so its frames invalidate one element instead of the whole document;
 * `every-paint-owns-its-clock` — a rule that paints with an animated property must start the animation
 * itself, because a non-inherited phase never reaches it from an ancestor. */
export const CSS_ANIMATION_SCOPE_LAWS = ["no-document-root-clock", "animated-custom-properties-are-non-inherited", "every-paint-owns-its-clock"] as const;
export type CssAnimationScopeLaw = (typeof CSS_ANIMATION_SCOPE_LAWS)[number];

/** ⚖️ Names which of {@link CSS_ANIMATION_SCOPE_LAWS} a stylesheet's scope breaks, in law order. */
export function cssAnimationScopeViolations(scope: CssAnimationScope): readonly CssAnimationScopeLaw[] {
  const animated = new Set(scope.animatedCustomProperties);
  const started = new Map<string, Set<string>>();
  for (const clock of scope.clocks) for (const name of clock.keyframes) (started.get(clock.selector) ?? started.set(clock.selector, new Set()).get(clock.selector)!).add(name);
  const broken: CssAnimationScopeLaw[] = [];
  if (scope.rootClocks.length > 0) broken.push("no-document-root-clock");
  if ([...animated].some((name) => scope.propertyInheritance[name] !== false)) broken.push("animated-custom-properties-are-non-inherited");
  const unclocked = scope.paints.some((paint) => paint.properties.some((name) => !(scope.keyframesByProperty[name] ?? []).some((keyframes) => started.get(paint.selector)?.has(keyframes) === true)));
  if (unclocked) broken.push("every-paint-owns-its-clock");
  return broken;
}

/** 🖌️ Every paint that does not start the clock its animated property needs, named with the
 * keyframes it is missing — the actionable half of `every-paint-owns-its-clock`. */
export function cssAnimationScopeUnclockedPaints(scope: CssAnimationScope): readonly { selector: string; property: string; keyframes: readonly string[] }[] {
  const started = new Map<string, Set<string>>();
  for (const clock of scope.clocks) for (const name of clock.keyframes) (started.get(clock.selector) ?? started.set(clock.selector, new Set()).get(clock.selector)!).add(name);
  return scope.paints.flatMap((paint) => paint.properties.filter((name) => !(scope.keyframesByProperty[name] ?? []).some((keyframes) => started.get(paint.selector)?.has(keyframes) === true)).map((property) => ({ selector: paint.selector, property, keyframes: scope.keyframesByProperty[property] ?? [] })));
}
