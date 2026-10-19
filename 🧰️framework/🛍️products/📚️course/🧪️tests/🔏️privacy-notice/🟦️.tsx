/** 🔏️ Every screen ends with a footer that opens the notice of what is stored — on the proctor, in this browser, and
 * what is only shown live — and links the site's imprint and privacy policy, each only when the site names an absolute
 * http(s) address for it (judged by the WHATWG `URL` parser). The notice is a modal dialog that returns focus to its
 * opener, and while no language is known the footer names the links in every offered language.
 *
 * @see https://url.spec.whatwg.org/#url-parsing
 */

import { render, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { LegalFooter, COURSE_LOCALES, CourseApp, legalLink, memoryStorageOrigin, courseText, type PresenceConnect } from "@semio-tech/course-react";

const QUIET_PRESENCE: PresenceConnect = () => ({ readyState: 0, onopen: null, onmessage: null, onclose: null, onerror: null, send: () => undefined, close: () => undefined });
const SILENT = { send: () => new Promise<never>(() => undefined) };
const LEGAL = { imprint: "https://example.org/impressum", privacy: "https://example.org/datenschutz" };

const NOTICE = {
  en: {
    open: "What is stored",
    title: "What this site stores",
    told: [/^On the course server: the pseudonym or name you entered/u, /are public on the leaderboard\.$/u, /^In this browser's storage: your learner id – the only key to your progress, there are no passwords/u, /^While you are online, the others see that you are here/u, /shown live and not saved\.$/u],
    contact: "For questions, or to have your entries removed, contact the operator named in the imprint.",
    links: ["Imprint (opens in a new tab)", "Privacy policy (opens in a new tab)"],
    close: "Close",
    nav: "Legal and privacy",
  },
  de: {
    open: "Was gespeichert wird",
    title: "Was diese Seite speichert",
    told: [/^Auf dem Kursserver: das Pseudonym oder der Name, den du eingegeben hast/u, /sind in der Rangliste öffentlich\.$/u, /^Im Speicher dieses Browsers: deine Lern-ID – der einzige Schlüssel zu deinem Fortschritt, es gibt keine Passwörter/u, /^Solange du online bist, sehen die anderen, dass du da bist/u, /live angezeigt und nicht gespeichert\.$/u],
    contact: "Bei Fragen oder wenn deine Einträge entfernt werden sollen, wende dich an die im Impressum genannte Person.",
    links: ["Impressum (öffnet in einem neuen Tab)", "Datenschutzerklärung (öffnet in einem neuen Tab)"],
    close: "Schließen",
    nav: "Rechtliches und Datenschutz",
  },
} as const;

describe("🔏️ legal links", () => {
  it("renders only absolute http(s) addresses as links", () => {
    expect(legalLink("https://example.org/impressum")).toBe(new URL("https://example.org/impressum").href);
    expect(legalLink("http://example.org")).toBe("http://example.org/");
    for (const refused of [undefined, "", "impressum.html", "/impressum", "javascript:alert(1)", "data:text/html,x", "mailto:a@example.org", "//example.org/x"]) expect(legalLink(refused), String(refused)).toBeUndefined();
  });

  for (const locale of COURSE_LOCALES) {
    it(`shows the notice of what is stored and the site's legal pages (${locale})`, async () => {
      const expected = NOTICE[locale];
      const user = userEvent.setup();
      render(
        <div className="course-app" lang={locale}>
          <main />
          <LegalFooter legal={LEGAL} locale={locale} text={courseText(locale)} />
        </div>,
      );
      const footer = screen.getByRole("contentinfo");
      const nav = within(footer).getByRole("navigation", { name: expected.nav });
      const links = within(nav).getAllByRole("link");
      expect(links.map((link) => link.textContent)).toEqual(expected.links);
      expect(links.map((link) => [link.getAttribute("href"), link.getAttribute("target"), link.getAttribute("rel")])).toEqual([
        [LEGAL.imprint, "_blank", "noopener noreferrer"],
        [LEGAL.privacy, "_blank", "noopener noreferrer"],
      ]);
      const opener = within(nav).getByRole("button", { name: expected.open });
      await user.click(opener);
      const dialog = screen.getByRole("dialog", { name: expected.title });
      expect(dialog.getAttribute("aria-modal")).toBe("true");
      expect(dialog.closest(".course-app")).not.toBeNull();
      expect(document.querySelector("main")?.hasAttribute("inert")).toBe(true);
      const body = document.getElementById(dialog.getAttribute("aria-describedby") ?? "")!;
      const paragraphs = [...body.querySelectorAll("p")].map((paragraph) => paragraph.textContent ?? "");
      for (const sentence of expected.told) expect(paragraphs.some((paragraph) => sentence.test(paragraph)), String(sentence)).toBe(true);
      expect(paragraphs).toContain(expected.contact);
      expect(within(dialog).getAllByRole("link").map((link) => link.getAttribute("href"))).toEqual([LEGAL.imprint, LEGAL.privacy]);
      expect(document.activeElement).toBe(within(dialog).getByRole("button", { name: expected.close }));
      await user.keyboard("{Escape}");
      expect(screen.queryByRole("dialog")).toBeNull();
      expect(document.activeElement).toBe(opener);
    });
  }

  it("shows no link the site does not name and does not point at an imprint that is not there", async () => {
    const user = userEvent.setup();
    render(<LegalFooter legal={{ imprint: "javascript:alert(1)" }} locale="en" text={courseText("en")} />);
    expect(screen.queryAllByRole("link")).toEqual([]);
    await user.click(screen.getByRole("button", { name: "What is stored" }));
    const dialog = screen.getByRole("dialog");
    expect(within(dialog).queryAllByRole("link")).toEqual([]);
    expect(dialog.textContent).not.toContain("imprint");
  });

  it("ends every screen of the client with the footer, also the first one, and names the links in every language while none is known", async () => {
    const { unmount } = render(<CourseApp proctor="" tenant="legal" presence={QUIET_PRESENCE} transport={() => SILENT} storage={memoryStorageOrigin().tab()} languages={["en"]} legal={LEGAL} />);
    const footer = await screen.findByRole("contentinfo");
    expect(within(footer).getByRole("button", { name: "What is stored" })).toBeTruthy();
    expect(within(footer).getAllByRole("link")).toHaveLength(2);
    expect(footer.previousElementSibling?.tagName).toBe("MAIN");
    unmount();

    render(<CourseApp proctor="" tenant="legal" presence={QUIET_PRESENCE} transport={() => SILENT} storage={memoryStorageOrigin().tab()} languages={["fr"]} legal={LEGAL} />);
    const neutral = await screen.findByRole("contentinfo");
    expect(neutral.hasAttribute("lang")).toBe(false);
    expect(within(neutral).queryByRole("button")).toBeNull();
    const links = within(neutral).getAllByRole("link");
    expect(links.map((link) => [...link.querySelectorAll(":scope > [lang]")].map((part) => [part.getAttribute("lang"), part.textContent]))).toEqual([
      [
        ["en", "Imprint"],
        ["de", "Impressum"],
      ],
      [
        ["en", "Privacy policy"],
        ["de", "Datenschutzerklärung"],
      ],
    ]);
  });

  it("keeps the page where it is and centres the notice on the glass", async () => {
    const user = userEvent.setup();
    const seen: Array<boolean | undefined> = [];
    const original = HTMLElement.prototype.focus;
    HTMLElement.prototype.focus = function (this: HTMLElement, options?: FocusOptions) {
      seen.push(options?.preventScroll);
      return original.call(this, options);
    };
    try {
      render(
        <div className="course-app" lang="de">
          <main />
          <div data-testid="page" style={{ height: "80px", overflow: "auto" }}>
            <div style={{ height: "400px" }} />
          </div>
          <LegalFooter legal={LEGAL} locale="de" text={courseText("de")} />
        </div>,
      );
      const page = screen.getByTestId("page");
      page.scrollTop = 120;
      const before = page.scrollTop;
      await user.click(screen.getByRole("button", { name: "Was gespeichert wird" }));
      expect(page.scrollTop).toBe(before);
      const dialog = screen.getByRole("dialog", { name: "Was diese Seite speichert" });
      const glass = dialog.closest("[data-course-dialog]");
      expect(glass?.className).toContain("fixed");
      expect(glass?.className).toContain("overflow-y-auto");
      expect(glass?.className.includes("place-items-center")).toBe(false);
      expect(glass?.firstElementChild?.className).toContain("min-h-full");
      expect(glass?.firstElementChild?.className).toContain("items-center");
      expect(seen).toContain(true);
      page.dispatchEvent(new Event("scroll", { bubbles: true }));
      expect(page.scrollTop).toBe(before);
      await user.keyboard("{Escape}");
      expect(screen.queryByRole("dialog")).toBeNull();
      expect(page.scrollTop).toBe(before);
      expect(document.activeElement).toBe(screen.getByRole("button", { name: "Was gespeichert wird" }));
    } finally {
      HTMLElement.prototype.focus = original;
    }
  });
});
