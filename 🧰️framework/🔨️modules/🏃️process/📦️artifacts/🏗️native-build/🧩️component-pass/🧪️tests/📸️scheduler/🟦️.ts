/** 📸️ Contract: the host scheduler crate is compiled from the same source root as the guest passes: its manifest and its `--manifest-path` argument are rebased onto the snapshot, a selection outside the live origin is left alone. */
import { expect, test } from "bun:test";
import { nativeBuildSchedulerSelectionV1 } from "../../🟦️.ts";

const selected = { manifest: "/live/🧰️framework/s/Cargo.toml", args: ["build", "--locked", "--manifest-path", "/live/🧰️framework/s/Cargo.toml", "--bins"] };

test("the scheduler selection follows the source root", () => {
  expect(nativeBuildSchedulerSelectionV1(selected, { cwd: "/live" })).toBe(selected);
  expect(nativeBuildSchedulerSelectionV1(selected, { cwd: "/live", originCwd: "/live" })).toBe(selected);
  expect(nativeBuildSchedulerSelectionV1(selected, { cwd: "/snap", originCwd: "/live" })).toEqual({ manifest: "/snap/🧰️framework/s/Cargo.toml", args: ["build", "--locked", "--manifest-path", "/snap/🧰️framework/s/Cargo.toml", "--bins"] });
  expect(nativeBuildSchedulerSelectionV1(selected, { cwd: "/snap", originCwd: "/other" })).toBe(selected);
});
