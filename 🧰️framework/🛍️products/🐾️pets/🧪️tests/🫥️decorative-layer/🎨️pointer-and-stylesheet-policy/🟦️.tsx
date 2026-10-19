/** 🎨️ The pointer event and transition stylesheet policy tests and the pet placement tests. */
import { render } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { PET_EDGE_INSET, PetLayer } from "@semio-tech/pets-react";
import { BUTTON, CARDS, HALF, MENAGERIE, VIEWPORT, WORDS, css, layerOf, page, standing } from "../🧰️decorative-layer-fixtures/🟦️.tsx";

/** 🖱 Registers the pointer event and transition stylesheet laws, the surface standing and the placement tests. */
export function registerPointerAndStylesheetPolicyTests(): void {
  it("takes no pointer events where the pointer is fine: the stylesheet says so for the layer and everything in it, and only a touch pad would take them", () => {
    page();
    const sheet = document.createElement("style");
    sheet.textContent = css;
    document.head.append(sheet);
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />);
    vi.advanceTimersByTime(3000);
    const layer = layerOf(container);
    expect(getComputedStyle(layer).pointerEvents).toBe("none");
    expect(getComputedStyle(layer).position).toBe("fixed");
    const pets = layer.querySelectorAll("svg.pet");
    expect(pets.length).toBe(3);
    for (const node of layer.querySelectorAll("*")) expect(getComputedStyle(node).pointerEvents, node.nodeName).toBe("none");
    expect(css).toMatch(/@media \(forced-colors: active\) \{\s*\.pet-layer \{\s*display: none;/);
    expect(css).toMatch(/@media print \{\s*\.pet-layer \{\s*display: none;/);
    expect(layer.querySelector(".pet-pad, .pet-pads")).toBeNull();
    const rules = [...css.replace(/\/\*[\s\S]*?\*\//g, "").matchAll(/([^{}]+)\{([^}]*)\}/g)];
    expect(rules.filter(([, , body]) => /pointer-events:\s*(?!none)\S/.test(body!)).map(([, selector]) => selector!.trim())).toEqual([".pet-pad"]);
    expect(css).not.toContain("animation-name:");
  });
  it("never transitions: the stylesheet says so for the layer and everything in it, also under a host that shortens every transition for reduced motion", () => {
    page();
    const host = document.createElement("style");
    host.textContent = ".host * { transition-duration: 0.01ms !important; }";
    const sheet = document.createElement("style");
    sheet.textContent = css;
    document.head.append(sheet, host);
    const { container } = render(
      <div className="host">
        <p>text</p>
        <PetLayer menagerie={MENAGERIE} scene="home" mode="calm" seed={7} />
      </div>,
    );
    vi.advanceTimersByTime(3000);
    const layer = container.querySelector<HTMLElement>(".pet-layer")!;
    expect(getComputedStyle(container.querySelector("p")!).transitionDuration).toBe("0.01ms");
    expect(getComputedStyle(container.querySelector("p")!).transitionProperty).not.toBe("none");
    expect(getComputedStyle(layer).transitionProperty).toBe("none");
    const nodes = [...layer.querySelectorAll("*")];
    expect(nodes.length).toBeGreaterThan(3);
    for (const node of nodes) expect(getComputedStyle(node).transitionProperty, node.nodeName).toBe("none");
    expect(css).toMatch(/\.pet-layer,\s*\.pet-layer \* \{\s*transition-property: none !important;/);
    host.remove();
    sheet.remove();
  });
  it("stands its pets on the top edges of the surfaces and the floor, never over what is kept free", () => {
    page();
    const { container } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={11} capacity={3} />);
    vi.advanceTimersByTime(100);
    const pets = standing(layerOf(container));
    expect(pets).toHaveLength(3);
    const grounds = [
      { y: CARDS.first.top, x0: CARDS.first.left, x1: CARDS.first.left + CARDS.first.width },
      { y: CARDS.second.top, x0: WORDS.left + WORDS.width + 4, x1: CARDS.second.left + CARDS.second.width },
      { y: VIEWPORT.height, x0: 0, x1: VIEWPORT.width },
    ];
    for (const pet of pets) {
      const ground = grounds.find((candidate) => candidate.y === pet.y);
      expect(ground, `${pet.species} at ${pet.x}, ${pet.y}`).toBeDefined();
      expect(pet.x - HALF, pet.species).toBeGreaterThanOrEqual(ground!.x0);
      expect(pet.x + HALF, pet.species).toBeLessThanOrEqual(ground!.x1);
      if (pet.y === VIEWPORT.height) expect(pet.x + HALF <= BUTTON.left - 4 || pet.x - HALF >= BUTTON.left + BUTTON.width + 4, `${pet.species} in front of the button`).toBe(true);
      expect(pet.opacity, pet.species).toBe(1);
    }
  });
  it("finds a place for every pet on any stage it is seeded for", () => {
    page();
    const places = new Set<string>();
    for (let seed = 1; seed <= 24; seed++) {
      const { container, unmount } = render(<PetLayer menagerie={MENAGERIE} scene="home" mode="still" seed={seed} />);
      vi.advanceTimersByTime(100);
      const pets = standing(layerOf(container));
      expect(pets, `seed ${seed}`).toHaveLength(3);
      for (const pet of pets) {
        expect([CARDS.first.top, CARDS.second.top, VIEWPORT.height], `seed ${seed} ${pet.species}`).toContain(pet.y);
        expect(pet.x - HALF, `seed ${seed} ${pet.species} at the left edge`).toBeGreaterThanOrEqual(PET_EDGE_INSET);
        expect(pet.x + HALF, `seed ${seed} ${pet.species} at the right edge`).toBeLessThanOrEqual(VIEWPORT.width - PET_EDGE_INSET);
        if (pet.y === CARDS.second.top) expect(pet.x - HALF, `seed ${seed} ${pet.species} in front of the words`).toBeGreaterThanOrEqual(WORDS.left + WORDS.width + 4);
        if (pet.y === VIEWPORT.height) expect(pet.x + HALF <= BUTTON.left - 4 || pet.x - HALF >= BUTTON.left + BUTTON.width + 4, `seed ${seed} ${pet.species} in front of the button`).toBe(true);
        places.add(`${pet.y}`);
      }
      unmount();
    }
    expect([...places].sort()).toEqual([`${CARDS.first.top}`, `${CARDS.second.top}`, `${VIEWPORT.height}`].sort());
  });
}
