/** 🗣️ The board-specific localized texts, merged over the harness copy. */
import { COPY as FRAMEWORK_COPY } from "@semio-tech/framework/time-travel/testing/copy";
import { currentLocale } from "@semio-tech/framework/time-travel/testing/state";
import type { Locale } from "@semio-tech/framework/time-travel/testing/state";

/** 🌍️ The harness copy with the board-specific labels merged over it (drag, create, lock, rotate, scale and the example commands). */
export const COPY = {
  en: {
    ...FRAMEWORK_COPY.en,
    items: (count: number) => (count === 1 ? "1 item" : `${count} items`),
    drag: (count: number, dx: string, dy: string) => `Drag ${COPY.en.items(count)} by (${dx}, ${dy})`,
    createNode: (id: string) => `Create node "${id}"`,
    itemCount: (count: number) => `Items: ${count}`,
    changeLocked: (id: string) => `Change node "${id}" locked`,
    rotateCommand: /^Rotate\b/,
    scaleCommand: /^Scale\b/,
    rotate: (count: number, degrees: string) => `Rotate ${COPY.en.items(count)} by ${degrees}°`,
    scale: (count: number, factor: string) => `Scale ${COPY.en.items(count)} by a factor of ${factor}`,
    example: /^Set Active Example\b/,
    exampleQuery: "Set Active Example",
    manifest: /^Change manifest id\b/,
    createNodeRow: /^Create node "/,
  },
  de: {
    ...FRAMEWORK_COPY.de,
    items: (count: number) => (count === 1 ? "1 Element" : `${count} Elemente`),
    drag: (count: number, dx: string, dy: string) => `${COPY.de.items(count)} um (${dx}; ${dy}) ziehen`,
    createNode: (id: string) => `Knoten "${id}" erstellen`,
    itemCount: (count: number) => `Elemente: ${count}`,
    changeLocked: (id: string) => `Sperre von Knoten "${id}" ändern`,
    rotateCommand: /^Drehen\b/,
    scaleCommand: /^Skalieren\b/,
    rotate: (count: number, degrees: string) => `${COPY.de.items(count)} um ${degrees}° drehen`,
    scale: (count: number, factor: string) => `${COPY.de.items(count)} um den Faktor ${factor} skalieren`,
    example: /^Aktives Beispiel festlegen\b/,
    exampleQuery: "Aktives Beispiel",
    manifest: /^Manifest-ID ändern\b/,
    createNodeRow: /^Knoten "/,
  },
} as const;
//#region 🔖️Copy

/** 🔢️ `puzzle2d_selection_number`: two decimals, trailing zeros and a negative zero dropped, a decimal comma in German. */
export const labelNumber = (value: number, locale: Locale) => {
  const rounded = Math.round(value * 100) / 100;
  const text = (rounded === 0 ? 0 : rounded).toFixed(2).replace(/0+$/, "").replace(/\.$/, "");
  return locale === "de" ? text.replace(".", ",") : text;
};

export const dragLabel = (count: number, dx: number, dy: number, locale = currentLocale) => COPY[locale].drag(count, labelNumber(dx, locale), labelNumber(dy, locale));
//#endregion 🔖️Copy
