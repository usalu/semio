import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import Ajv from "ajv";
import equal from "fast-deep-equal";
import { afterEach, describe, expect, it } from "vitest";
import fixture from "../../../../📇️directory/🪪️session-refresh/🔣️.json";
import schema from "../../../../📇️directory/🪪️session-refresh/🧬️.schema.json";
import { DIRECTORY_SESSION_AUTHORITY_TEXT_V1, directorySessionAuthorityTextV1 } from "../../../../📇️directory/🪪️session-refresh/🟦️.ts";
import { SessionAuthorityNotice } from "../../../../📇️directory/🪪️session-refresh/🪪️notice/🟦️.tsx";

afterEach(cleanup);

describe("session authority progress and cancellation", () => {
  it("validates the neutral bilingual presentation with independent schema and equality oracles", () => {
    expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(equal(DIRECTORY_SESSION_AUTHORITY_TEXT_V1, fixture.presentation.text)).toBe(true);
    for (const locale of fixture.presentation.locales) expect(equal(directorySessionAuthorityTextV1(locale), fixture.presentation.text[locale])).toBe(true);
    expect(() => directorySessionAuthorityTextV1("fr")).toThrow("directory.session-authority.locale-unsupported");
  });

  it("announces pending work with one cancellation and terminal unavailability without invented counts", () => {
    for (const locale of fixture.presentation.locales) {
      let cancellations = 0;
      const view = render(<SessionAuthorityNotice state="pending" locale={locale} onCancel={() => { cancellations += 1; }} />);
      const pending = screen.getByRole("status");
      expect(pending.getAttribute("aria-live")).toBe("polite");
      expect(pending.getAttribute("aria-busy")).toBe("true");
      expect(pending.textContent).toContain(fixture.presentation.text[locale].pending);
      expect(view.container.querySelector('[role="progressbar"]')).toBeNull();
      fireEvent.click(screen.getByRole("button", { name: fixture.presentation.text[locale].cancel }));
      expect(cancellations).toBe(1);
      view.rerender(<SessionAuthorityNotice state="unavailable" locale={locale} onCancel={() => { cancellations += 1; }} />);
      const unavailable = screen.getByRole("alert");
      expect(unavailable.getAttribute("aria-live")).toBe("assertive");
      expect(unavailable.textContent).toContain(fixture.presentation.text[locale].unavailable);
      expect(view.container.querySelector("button")).toBeNull();
      view.unmount();
    }
  });
});
