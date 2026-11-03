import { expect, test } from "bun:test";
import { localeRequestToApply } from "../🟦️.ts";

test("a new request is applied once", () => {
  expect(localeRequestToApply("en", "de", undefined)).toBe("de");
  expect(localeRequestToApply("de", "de", undefined)).toBeUndefined();
});

test("no request, or a locked language, applies nothing", () => {
  expect(localeRequestToApply("en", undefined, undefined)).toBeUndefined();
  expect(localeRequestToApply("en", "de", "en")).toBeUndefined();
});
