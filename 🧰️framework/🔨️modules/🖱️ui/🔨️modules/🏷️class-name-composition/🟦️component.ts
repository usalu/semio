// #region 🧲️Header
// 💻️ framework/ui/modules/🏷️class-name-composition/component.ts
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🎨️ClassNameComposition
/** @emoji 🧬️ Repository-owned recursive input for CSS class composition. */
export type ClassNameInput = string | number | bigint | boolean | null | undefined | { readonly [className: string]: unknown } | ClassNameInput[];

type ClassGroup = readonly [id: string, pattern: RegExp];

const CLASS_GROUPS: readonly ClassGroup[] = [
  ["position", /^(?:static|fixed|absolute|relative|sticky)$/],
  ["display", /^(?:block|inline-block|inline|flex|inline-flex|table|inline-table|table-(?:caption|cell|column|column-group|footer-group|header-group|row-group|row)|flow-root|grid|inline-grid|contents|list-item|hidden)$/],
  ["inset-x", /^-?inset-x-/],
  ["inset-y", /^-?inset-y-/],
  ["inset-start", /^-?(?:top|right|bottom|left|start|end)-/],
  ["inset", /^-?inset-/],
  ["z", /^-?z-/],
  ["float", /^float-/],
  ["clear", /^clear-/],
  ["object-fit", /^object-(?:contain|cover|fill|none|scale-down)$/],
  ["object-position", /^object-(?:bottom|center|left|left-bottom|left-top|right|right-bottom|right-top|top|\[)/],
  ["overflow-x", /^overflow-x-/],
  ["overflow-y", /^overflow-y-/],
  ["overflow", /^overflow-/],
  ["overscroll-x", /^overscroll-x-/],
  ["overscroll-y", /^overscroll-y-/],
  ["overscroll", /^overscroll-/],
  ["basis", /^basis-/],
  ["flex-direction", /^flex-(?:row|row-reverse|col|col-reverse)$/],
  ["flex-wrap", /^flex-(?:wrap|wrap-reverse|nowrap)$/],
  ["flex", /^flex-(?:1|auto|initial|none|\[)/],
  ["grow", /^(?:(?:flex-)?grow|(?:flex-)?grow-.+)$/],
  ["shrink", /^(?:(?:flex-)?shrink|(?:flex-)?shrink-.+)$/],
  ["order", /^-?order-/],
  ["grid-cols", /^grid-cols-/],
  ["grid-rows", /^grid-rows-/],
  ["col", /^-?col-(?:auto|span-|start-|end-|\[)/],
  ["row", /^-?row-(?:auto|span-|start-|end-|\[)/],
  ["gap-x", /^gap-x-/],
  ["gap-y", /^gap-y-/],
  ["gap", /^gap-/],
  ["justify-content", /^justify-(?:normal|start|end|center|between|around|evenly|stretch)$/],
  ["justify-items", /^justify-items-/],
  ["justify-self", /^justify-self-/],
  ["content", /^content-(?:normal|center|start|end|between|around|evenly|baseline|stretch|none|\[)/],
  ["items", /^items-/],
  ["self", /^self-/],
  ["place-content", /^place-content-/],
  ["place-items", /^place-items-/],
  ["place-self", /^place-self-/],
  ["p-x", /^px-/],
  ["p-y", /^py-/],
  ["p-top", /^pt-/],
  ["p-right", /^(?:pr|pe)-/],
  ["p-bottom", /^pb-/],
  ["p-left", /^(?:pl|ps)-/],
  ["p", /^p-/],
  ["m-x", /^-?mx-/],
  ["m-y", /^-?my-/],
  ["m-top", /^-?mt-/],
  ["m-right", /^-?(?:mr|me)-/],
  ["m-bottom", /^-?mb-/],
  ["m-left", /^-?(?:ml|ms)-/],
  ["m", /^-?m-/],
  ["space-x", /^-?space-x-/],
  ["space-y", /^-?space-y-/],
  ["scroll-p-x", /^scroll-px-/],
  ["scroll-p-y", /^scroll-py-/],
  ["scroll-p-top", /^scroll-pt-/],
  ["scroll-p-right", /^scroll-p(?:r|e)-/],
  ["scroll-p-bottom", /^scroll-pb-/],
  ["scroll-p-left", /^scroll-p(?:l|s)-/],
  ["scroll-p", /^scroll-p-/],
  ["scroll-m-x", /^-?scroll-mx-/],
  ["scroll-m-y", /^-?scroll-my-/],
  ["scroll-m-top", /^-?scroll-mt-/],
  ["scroll-m-right", /^-?scroll-m(?:r|e)-/],
  ["scroll-m-bottom", /^-?scroll-mb-/],
  ["scroll-m-left", /^-?scroll-m(?:l|s)-/],
  ["scroll-m", /^-?scroll-m-/],
  ["size", /^size-/],
  ["min-w", /^min-w-/],
  ["max-w", /^max-w-/],
  ["w", /^w-/],
  ["min-h", /^min-h-/],
  ["max-h", /^max-h-/],
  ["h", /^h-/],
  ["aspect-ratio", /^aspect-(?:auto|square)$/],
  ["font-family", /^font-(?:sans|serif|mono|\[)/],
  ["font-weight", /^font-(?:thin|extralight|light|normal|medium|semibold|bold|extrabold|black|\d+)$/],
  ["text-size", /^text-(?:xs|sm|base|lg|xl|[2-9]xl|\[.+\])(?:\/.+)?$/],
  ["text-align", /^text-(?:left|center|right|justify|start|end)$/],
  ["text-color", /^text-/],
  ["leading", /^leading-/],
  ["tracking", /^-?tracking-/],
  ["whitespace", /^whitespace-/],
  ["break", /^break-/],
  ["hyphens", /^hyphens-/],
  ["list-style-position", /^list-(?:inside|outside)$/],
  ["list-style-type", /^list-(?:none|disc|decimal|\[)/],
  ["placeholder-color", /^placeholder-/],
  ["bg-attachment", /^bg-(?:fixed|local|scroll)$/],
  ["bg-clip", /^bg-clip-/],
  ["bg-origin", /^bg-origin-/],
  ["bg-position", /^bg-(?:bottom|center|left|left-bottom|left-top|right|right-bottom|right-top|top|\[position:)/],
  ["bg-repeat", /^bg-(?:repeat|no-repeat|repeat-x|repeat-y|repeat-round|repeat-space)$/],
  ["bg-size", /^bg-(?:auto|cover|contain|\[size:)/],
  ["bg-image", /^bg-(?:none|gradient-to-|\[image:)/],
  ["background-blend", /^bg-blend-/],
  ["bg-color", /^(?:bg-|ui-surface$|ui-glass$|ui-veil$)/],
  ["rounded-top", /^rounded-(?:t|tl|tr)-/],
  ["rounded-bottom", /^rounded-(?:b|bl|br)-/],
  ["rounded-left", /^rounded-(?:l|ss|es)-/],
  ["rounded-right", /^rounded-(?:r|se|ee)-/],
  ["rounded", /^(?:rounded|rounded-.+)$/],
  ["border-collapse", /^border-(?:collapse|separate)$/],
  ["border-x-width", /^border-x(?:-|$)(?:0|2|4|8|\[.+\])?$/],
  ["border-y-width", /^border-y(?:-|$)(?:0|2|4|8|\[.+\])?$/],
  ["border-side-width", /^border-(?:t|r|b|l|s|e)(?:-|$)(?:0|2|4|8|\[.+\])?$/],
  ["border-width", /^border(?:-(?:0|2|4|8|\[.+\]))?$/],
  ["border-style", /^border-(?:solid|dashed|dotted|double|hidden|none)$/],
  ["border-x-color", /^border-x-/],
  ["border-y-color", /^border-y-/],
  ["border-side-color", /^border-(?:t|r|b|l|s|e)-/],
  ["border-color", /^border-/],
  ["divide-x", /^divide-x(?:-|$)/],
  ["divide-y", /^divide-y(?:-|$)/],
  ["divide-style", /^divide-(?:solid|dashed|dotted|double|none)$/],
  ["divide-color", /^divide-/],
  ["outline-offset", /^-?outline-offset-/],
  ["outline-width", /^outline(?:-(?:0|1|2|4|8|\[.+\]))?$/],
  ["outline-style", /^outline-(?:none|solid|dashed|dotted|double|hidden)$/],
  ["outline-color", /^outline-/],
  ["ring-offset-width", /^ring-offset-(?:0|1|2|4|8|\[.+\])$/],
  ["ring-offset-color", /^ring-offset-/],
  ["ring-inset", /^ring-inset$/],
  ["ring-width", /^ring(?:-(?:0|1|2|4|8|\[.+\]))?$/],
  ["ring-color", /^ring-/],
  ["shadow", /^(?:shadow|shadow-.+)$/],
  ["opacity", /^opacity-/],
  ["mix-blend", /^mix-blend-/],
  ["blur", /^blur(?:-|$)/],
  ["brightness", /^brightness-/],
  ["contrast", /^contrast-/],
  ["drop-shadow", /^drop-shadow(?:-|$)/],
  ["grayscale", /^grayscale(?:-|$)/],
  ["hue-rotate", /^-?hue-rotate-/],
  ["invert", /^invert(?:-|$)/],
  ["saturate", /^saturate-/],
  ["sepia", /^sepia(?:-|$)/],
  ["backdrop-blur", /^backdrop-blur(?:-|$)/],
  ["table-layout", /^table-(?:auto|fixed)$/],
  ["caption-side", /^caption-/],
  ["transition-property", /^(?:transition|transition-.+)$/],
  ["duration", /^duration-/],
  ["ease", /^ease-/],
  ["delay", /^delay-/],
  ["animate", /^animate-/],
  ["transform-origin", /^origin-/],
  ["translate-x", /^-?translate-x-/],
  ["translate-y", /^-?translate-y-/],
  ["rotate", /^-?rotate-/],
  ["skew-x", /^-?skew-x-/],
  ["skew-y", /^-?skew-y-/],
  ["scale-x", /^-?scale-x-/],
  ["scale-y", /^-?scale-y-/],
  ["scale", /^-?scale-/],
  ["cursor", /^cursor-/],
  ["pointer-events", /^pointer-events-/],
  ["resize", /^(?:resize|resize-.+)$/],
  ["scroll-behavior", /^scroll-(?:auto|smooth)$/],
  ["select", /^select-/],
  ["touch", /^touch-/],
  ["will-change", /^will-change-/],
  ["appearance", /^(?:appearance-|\[-moz-appearance:)/],
  ["fill", /^fill-/],
  ["stroke-width", /^stroke-(?:0|1|2|\[.+\])$/],
  ["stroke-color", /^stroke-/],
] as const;

const CONFLICTS: Readonly<Record<string, readonly string[]>> = {
  inset: ["inset-x", "inset-y", "inset-start"],
  "inset-x": ["inset-start"],
  "inset-y": ["inset-start"],
  overflow: ["overflow-x", "overflow-y"],
  overscroll: ["overscroll-x", "overscroll-y"],
  gap: ["gap-x", "gap-y"],
  p: ["p-x", "p-y", "p-top", "p-right", "p-bottom", "p-left"],
  "p-x": ["p-right", "p-left"],
  "p-y": ["p-top", "p-bottom"],
  m: ["m-x", "m-y", "m-top", "m-right", "m-bottom", "m-left"],
  "m-x": ["m-right", "m-left"],
  "m-y": ["m-top", "m-bottom"],
  "scroll-p": ["scroll-p-x", "scroll-p-y", "scroll-p-top", "scroll-p-right", "scroll-p-bottom", "scroll-p-left"],
  "scroll-p-x": ["scroll-p-right", "scroll-p-left"],
  "scroll-p-y": ["scroll-p-top", "scroll-p-bottom"],
  "scroll-m": ["scroll-m-x", "scroll-m-y", "scroll-m-top", "scroll-m-right", "scroll-m-bottom", "scroll-m-left"],
  "scroll-m-x": ["scroll-m-right", "scroll-m-left"],
  "scroll-m-y": ["scroll-m-top", "scroll-m-bottom"],
  size: ["w", "h"],
  rounded: ["rounded-top", "rounded-bottom", "rounded-left", "rounded-right"],
  "border-width": ["border-x-width", "border-y-width", "border-side-width"],
  "border-x-width": ["border-side-width"],
  "border-y-width": ["border-side-width"],
  "border-color": ["border-x-color", "border-y-color", "border-side-color"],
  "border-x-color": ["border-side-color"],
  "border-y-color": ["border-side-color"],
  scale: ["scale-x", "scale-y"],
};

const splitModifiers = (className: string): readonly [modifier: string, base: string] => {
  const modifiers: string[] = [];
  let bracketDepth = 0;
  let parenthesisDepth = 0;
  let start = 0;
  for (let index = 0; index < className.length; index += 1) {
    const character = className[index];
    if (character === "[") bracketDepth += 1;
    else if (character === "]") bracketDepth -= 1;
    else if (character === "(") parenthesisDepth += 1;
    else if (character === ")") parenthesisDepth -= 1;
    else if (character === ":" && bracketDepth === 0 && parenthesisDepth === 0) {
      modifiers.push(className.slice(start, index));
      start = index + 1;
    }
  }
  const baseWithImportance = className.slice(start);
  const important = baseWithImportance.startsWith("!") || baseWithImportance.endsWith("!");
  const base = important ? (baseWithImportance.startsWith("!") ? baseWithImportance.slice(1) : baseWithImportance.slice(0, -1)) : baseWithImportance;
  return [`${modifiers.join(":")}${important ? "!" : ""}`, base];
};

const classGroup = (base: string): string | undefined => CLASS_GROUPS.find(([, pattern]) => pattern.test(base))?.[0];

const appendClassTokens = (input: ClassNameInput, tokens: string[]): void => {
  if (!input) return;
  if (typeof input === "string" || typeof input === "number") {
    tokens.push(...String(input).trim().split(/\s+/).filter(Boolean));
    return;
  }
  if (Array.isArray(input)) {
    for (const child of input) appendClassTokens(child, tokens);
    return;
  }
  if (typeof input === "object") {
    for (const classNames in input) if (input[classNames]) appendClassTokens(classNames, tokens);
  }
};

/** @emoji 🪢️ Composes the finite utility families used by repository UI source. */
export function cn(...inputs: ClassNameInput[]): string {
  const tokens: string[] = [];
  for (const input of inputs) appendClassTokens(input, tokens);
  const conflicts = new Set<string>();
  const result: string[] = [];
  for (let index = tokens.length - 1; index >= 0; index -= 1) {
    const token = tokens[index];
    const [modifier, base] = splitModifiers(token);
    const group = classGroup(base);
    if (!group) {
      result.push(token);
      continue;
    }
    const identity = `${modifier}:${group}`;
    if (conflicts.has(identity)) continue;
    conflicts.add(identity);
    for (const conflictingGroup of CONFLICTS[group] ?? []) conflicts.add(`${modifier}:${conflictingGroup}`);
    result.push(token);
  }
  return result.reverse().join(" ");
}
// #endregion 🎨️ClassNameComposition
