/** 🚀️ Smoke case of the archive app: it mounts on a root element and asks for a language in both languages when none is known.
 * @see ../../📦️packages/🟦️typescript/📚️app/🟦️.tsx */
import { waitFor } from "@testing-library/react";
import { expect, it } from "vitest";
import { memoryStorage } from "../../🔨️modules/💾️persistence/🟦️.ts";
import { mountArchive } from "@teaching/staffage-react/app";

it("renders a main landmark with an English and a German heading", async () => {
  const root = document.createElement("div");
  document.body.append(root);
  mountArchive(root, { storage: memoryStorage(), languages: [] });
  await waitFor(() => expect(root.querySelector("main")).not.toBeNull());
  expect([...root.querySelectorAll("h1")].map((heading) => heading.lang)).toEqual(["en", "de"]);
});
