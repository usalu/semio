/** 📢️ What speaks by itself and what stays quiet: the connection indicator is plain visible text — never a live region,
 * judged by the roles and implicit `aria-live` values of `aria-query` (the WAI-ARIA role model `@testing-library` itself
 * queries with) as the third-party oracle — and one polite status speaks only when the connection enters an alert state
 * and when it recovers (shared vectors, both languages).
 *
 * @see ../../🧫️fixtures/📢️live-regions/🔣️.json
 * @see https://www.w3.org/WAI/WCAG22/Understanding/status-messages.html
 */

import { render } from "@testing-library/react";
import { roles } from "aria-query";
import { describe, expect, it } from "vitest";
import { ConnectionStatus, QUIZ_LOCALES, connectionMessage, quizText, type QuizConnection } from "@semio-tech/quiz-react";
import regions from "../../🧫️fixtures/📢️live-regions/🔣️.json";

type Locale = (typeof QUIZ_LOCALES)[number];

interface Step {
  readonly connection: { readonly online: boolean; readonly reachability: string; readonly pending: number };
  readonly tone: string;
  readonly shown: Readonly<Record<Locale, string>>;
  readonly announced: Readonly<Record<Locale, string>>;
}

const steps: readonly Step[] = regions.steps;

function connection(step: Step): QuizConnection {
  return { ...step.connection, activity: step.connection.pending > 0 ? "sending" : "idle", deputy: false } as QuizConnection;
}

/** 🔮️ The `aria-live` value an element has by its own attribute or, per `aria-query`, by its role. */
function liveness(element: Element): string {
  const explicit = element.getAttribute("aria-live");
  if (explicit !== null) return explicit;
  const role = element.getAttribute("role");
  const implicit = role === null ? undefined : (roles.get(role as never)?.props as Readonly<Record<string, unknown>> | undefined)?.["aria-live"];
  return typeof implicit === "string" ? implicit : "off";
}

function spoken(element: Element): boolean {
  for (let node: Element | null = element; node !== null; node = node.parentElement) if (liveness(node) !== "off") return true;
  return false;
}

describe("📢️ live regions", () => {
  it("knows the roles that speak by themselves", () => {
    expect(roles.get("status")?.props["aria-live"]).toBe("polite");
    expect(roles.get("alert")?.props["aria-live"]).toBe("assertive");
  });

  for (const locale of QUIZ_LOCALES) {
    it(`shows every connection state and speaks only of an outage and its end (${locale})`, () => {
      const text = quizText(locale);
      const { container, rerender } = render(<ConnectionStatus connection={connection(steps[0]!)} text={text} />);
      for (const [index, step] of steps.entries()) {
        rerender(<ConnectionStatus connection={connection(step)} text={text} />);
        const shown = connectionMessage(connection(step), text);
        expect([shown.message, shown.tone], `step ${index}`).toEqual([step.shown[locale], step.tone]);
        const indicator = container.querySelector<HTMLElement>("[data-tone]")!;
        expect(indicator.getAttribute("data-tone"), `step ${index}`).toBe(step.tone);
        expect(indicator.textContent, `step ${index}`).toContain(step.shown[locale]);
        expect(spoken(indicator), `step ${index}`).toBe(false);
        const live = [...container.querySelectorAll("*")].filter((element) => liveness(element) !== "off");
        expect(live, `step ${index}`).toHaveLength(1);
        expect(liveness(live[0]!)).toBe("polite");
        expect(live[0]!.textContent, `step ${index}`).toBe(step.announced[locale]);
        expect(live[0]!.contains(indicator) || indicator.contains(live[0]!)).toBe(false);
      }
    });
  }
});
