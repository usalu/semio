/** 🟤️ The lines and open faces tests of the 📡️ surface survey suite: a text block is kept free where its words are and a card is no solid unless it stands on an edge. */
import { afterEach, expect, it, vi } from "vitest";
import { LINE_GAP, PET_KEEPOUTS, PET_LINES, survey } from "@semio-tech/pets-react";
import { card, place, viewport } from "../🟣️harness/🟦️.tsx";

type Words = readonly (readonly [number, number, number, number])[];

/** 🔤️ Lets every text node of the page say where its words lie, as a layout engine would (jsdom lays nothing out). */
function worded(words: ReadonlyMap<Node, Words>): void {
  vi.spyOn(document, "createRange").mockImplementation(() => {
    let node: Node | null = null;
    return {
      selectNodeContents: (target: Node) => {
        node = target;
      },
      getClientRects: () => (words.get(node!) ?? []).map(([left, top, width, height]) => ({ left, top, width, height, right: left + width, bottom: top + height, x: left, y: top })),
    } as unknown as Range;
  });
}

/** 📝️ A block of `tag` with the text `text` in it, `box` wide, and the pieces `words` say it shows. */
function block(tag: string, box: { left: number; top: number; width: number; height: number }, words: Words, said: Map<Node, Words>, attributes: Record<string, string> = {}): HTMLElement {
  const element = place(document.createElement(tag), box);
  for (const [name, value] of Object.entries(attributes)) element.setAttribute(name, value);
  const text = document.createTextNode("words");
  element.append(text);
  said.set(text, words);
  document.body.append(element);
  return element;
}

afterEach(() => {
  vi.restoreAllMocks();
});

/** 🔷 Registers the lines and open faces tests of the 📡️ surface survey suite. */
export function registerLinesAndOpenFacesTests(): void {
  it("keeps a text block free where its words are, not across the whole box its container gives it", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    block("p", { left: 100, top: 100, width: 800, height: 40 }, [[100, 100, 200, 16], [100, 124, 120, 16]], said);
    expect(survey(document).keepouts).toEqual([
      { x: 96, y: 96, width: 208, height: 24 },
      { x: 96, y: 120, width: 128, height: 24 },
    ]);
  });

  it("joins the pieces of one line that lie a word space apart and keeps a column apart as two", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    block("li", { left: 100, top: 100, width: 800, height: 20 }, [[100, 100, 60, 16], [160 + LINE_GAP, 100, 40, 16], [700, 100, 100, 16]], said);
    expect(survey(document).keepouts).toEqual([
      { x: 96, y: 96, width: 60 + LINE_GAP + 40 + 8, height: 24 },
      { x: 696, y: 96, width: 108, height: 24 },
    ]);
  });

  it("keeps a block's pictures free beside its words", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    const row = block("li", { left: 100, top: 100, width: 800, height: 24 }, [[130, 100, 100, 16]], said);
    const icon = place(document.createElementNS("http://www.w3.org/2000/svg", "svg"), { left: 100, top: 100, width: 24, height: 24 });
    row.append(icon);
    expect(survey(document).keepouts).toEqual([{ x: 96, y: 96, width: 138, height: 32 }]);
  });

  it("keeps the whole box where the page cannot say where the words are, of a control, of what the host names or marks, and of a read-out", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    block("p", { left: 100, top: 100, width: 800, height: 40 }, [], said);
    block("li", { left: 100, top: 200, width: 800, height: 40 }, [[100, 200, 50, 16]], said, { "data-course-item": "" });
    block("p", { left: 100, top: 300, width: 800, height: 40 }, [[100, 300, 50, 16]], said, { "data-pet-keepout": "" });
    block("button", { left: 100, top: 400, width: 800, height: 40 }, [[100, 400, 50, 16]], said);
    block("output", { left: 100, top: 500, width: 800, height: 40 }, [[100, 500, 50, 16]], said);
    expect(survey(document, { keepouts: `${PET_KEEPOUTS}, [data-course-item]` }).keepouts).toEqual([
      { x: 96, y: 96, width: 808, height: 48 },
      { x: 96, y: 196, width: 808, height: 48 },
      { x: 96, y: 296, width: 808, height: 48 },
      { x: 96, y: 396, width: 808, height: 48 },
      { x: 96, y: 496, width: 808, height: 48 },
    ]);
    expect(PET_LINES).not.toContain("output");
    expect(PET_LINES).not.toContain("label");
  });

  it("lets a line begin under an edge without the margin reaching over it", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    card({ left: 100, top: 100, width: 400, height: 200 });
    block("p", { left: 100, top: 100, width: 400, height: 40 }, [[100, 100, 100, 16]], said);
    expect(survey(document).keepouts).toContainEqual({ x: 96, y: 100, width: 108, height: 20 });
  });

  it("stands a card in front of nobody: a part that stands on an edge is solid, a card that fills the cell above is not", () => {
    viewport(1000, 700);
    const above = card({ left: 0, top: 0, width: 400, height: 176 });
    const tab = card({ left: 0, top: 176, width: 100, height: 24 });
    const body = card({ left: 0, top: 200, width: 400, height: 200 });
    const surveyed = survey(document);
    expect(surveyed.surfaces.map((surface) => surface.y)).toEqual([0, 176, 200, 700]);
    expect(surveyed.keepouts).toEqual([{ x: -4, y: 176, width: 108, height: 24 }]);
    expect([above, tab, body].every((element) => element.isConnected)).toBe(true);
  });

  it("makes a card as wide as the one under it solid for that one: it stands on it", () => {
    viewport(1000, 700);
    card({ left: 0, top: 0, width: 400, height: 200 });
    card({ left: 0, top: 200, width: 400, height: 100 });
    expect(survey(document).keepouts).toEqual([{ x: -4, y: 0, width: 408, height: 200 }]);
  });

  it("keeps the whole face of a card solid that the host says acts on a press: no pet stands in front of it", () => {
    viewport(1000, 700);
    const above = card({ left: 0, top: 0, width: 400, height: 176 });
    above.setAttribute("data-acts", "");
    card({ left: 0, top: 176, width: 100, height: 24 });
    card({ left: 0, top: 200, width: 400, height: 200 });
    expect(survey(document, { controls: "[data-acts]" }).keepouts).toContainEqual({ x: -4, y: 0, width: 408, height: 176 });
    expect(survey(document).keepouts).not.toContainEqual({ x: -4, y: 0, width: 408, height: 176 });
  });

  it("makes a part solid that stands on the floor of the stage, so nobody stands in front of it at the bottom of the window", () => {
    viewport(1000, 700);
    const said = new Map<Node, Words>();
    worded(said);
    card({ left: 0, top: 676, width: 1000, height: 24 });
    block("p", { left: 8, top: 678, width: 200, height: 20 }, [[8, 678, 60, 16]], said);
    expect(survey(document).keepouts).toContainEqual({ x: -4, y: 676, width: 1008, height: 24 });
  });
}
