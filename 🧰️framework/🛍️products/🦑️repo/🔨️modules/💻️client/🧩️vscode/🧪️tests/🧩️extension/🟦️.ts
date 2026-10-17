// #region 🧲️Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

/** 🧩️ The extension-host test entry: it activates the extension once, runs the RepoEvent parsing and command registration suites and mounts the suites of its children.
 *
 * @see ../🧰️extension-test-kit/🟦️.ts — the shared constants and helpers
 * @see ../🔎️repo-diagnostics-suites/🟦️.ts — kit validation, repo diagnostics and refresh diagnostics
 * @see ../🌊️view-and-provider-suites/🟦️.ts — views, tree providers and data structures
 * @see ../🌈️cli-tree-helper-suites/🟦️.ts — the CLI tree helpers
 * @see ../📮️uri-and-event-suites/🟦️.ts — extended RepoEvent parsing, URIs and navigation
 * @see ../🏷️entity-and-codelens-suites/🟦️.ts — entity emoji registry, entity ID regex and CodeLens
 */

// #region 🔌️Adapters
import * as assert from "assert";
import * as vscode from "vscode";
import { extractRepoResult, parseRepoEvents } from "../../🎼️utilities/🟦️.ts";
import { RepoEvent } from "../../⚙️data-models/🟦️.ts";
import { EXPECTED_COMMANDS, openFixture } from "../🧰️extension-test-kit/🟦️.ts";
import "../🔎️repo-diagnostics-suites/🟦️.ts";
import "../🌊️view-and-provider-suites/🟦️.ts";
import "../🌈️cli-tree-helper-suites/🟦️.ts";
import "../📮️uri-and-event-suites/🟦️.ts";
import "../🏷️entity-and-codelens-suites/🟦️.ts";

// #endregion 🔌️Adapters
// #region 🧪️OwnedExtensionHostTestContract
type OwnedExtensionHostTestContext = { timeout(milliseconds: number): void };
type OwnedExtensionHostTestBody = (this: OwnedExtensionHostTestContext) => void | Promise<void>;
declare function suite(title: string, body: OwnedExtensionHostTestBody): void;
declare function test(title: string, body: OwnedExtensionHostTestBody): void;
declare function suiteSetup(body: OwnedExtensionHostTestBody): void;

// #endregion 🧪️OwnedExtensionHostTestContract
// #region 🖇️Extension Activation
suiteSetup(async function () {
  this.timeout(30000);
  await openFixture("compose/metabolism/wip/initialKit/kit.compose.json");
  await new Promise((resolve) => setTimeout(resolve, 2000));
});

// #endregion 🖇️Extension Activation
// #region 🗺️RepoEvent Parsing Tests
suite("RepoEvent Parsing Test Suite", () => {
  test("parseRepoEvents handles result field correctly", () => {
    const output = '{"kind":"result","result":{"data":{"breachs":[{"id":"v1"}]}}}';
    const events = parseRepoEvents(output);
    assert.strictEqual(events.length, 1);
    assert.strictEqual(events[0].kind, "result");
    assert.ok(events[0].result);
    const result = events[0].result as any;
    assert.ok(result.data);
    assert.ok(result.data.breachs);
    assert.strictEqual(result.data.breachs.length, 1);
  });

  test("extractRepoResult extracts data from result field", () => {
    const events: RepoEvent[] = [{ kind: "result", result: { data: { breachs: [{ id: "v1" }] } } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs.length, 1);
    assert.strictEqual(data.breachs[0].id, "v1");
  });

  test("extractRepoResult falls back to data field if result is missing", () => {
    const events: RepoEvent[] = [{ kind: "result", data: { breachs: [{ id: "v2" }] } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs.length, 1);
    assert.strictEqual(data.breachs[0].id, "v2");
  });

  test("extractRepoResult prefers result over data field", () => {
    const events: RepoEvent[] = [{ kind: "result", result: { data: { breachs: [{ id: "from-result" }] } }, data: { breachs: [{ id: "from-data" }] } }];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
    const data = extracted.data as any;
    assert.ok(data.breachs);
    assert.strictEqual(data.breachs[0].id, "from-result");
  });

  test("extractRepoResult handles fatal errors", () => {
    const events: RepoEvent[] = [{ kind: "error", error: { message: "Fatal error occurred", fatal: true } }];
    assert.throws(() => extractRepoResult(events), /Fatal error occurred/);
  });

  test("extractRepoResult ignores non-fatal errors", () => {
    const events: RepoEvent[] = [
      { kind: "error", error: { message: "Non-fatal warning", fatal: false } },
      { kind: "result", result: { data: { breachs: [] } } },
    ];
    const extracted = extractRepoResult(events);
    assert.ok(extracted.data);
  });

  test("extractRepoResult uses last result when multiple result events", () => {
    const events: RepoEvent[] = [
      { kind: "result", result: { data: { breachs: [{ id: "first" }] } } },
      { kind: "result", result: { data: { breachs: [{ id: "last" }] } } },
    ];
    const extracted = extractRepoResult(events);
    const data = extracted.data as any;
    assert.strictEqual(data.breachs[0].id, "last");
  });
});

// #endregion 🗺️RepoEvent Parsing Tests
// #region 🌙️Command Registration Tests
suite("Command Registration Test Suite", () => {
  test("All expected commands are registered", async () => {
    const extension = vscode.extensions.getExtension("usalu.repo");
    assert.ok(extension, "Extension should be found");
    if (!extension.isActive) {
      await extension.activate();
    }
    const commands = await vscode.commands.getCommands(true);
    const missing = EXPECTED_COMMANDS.filter((cmd) => !commands.includes(cmd));
    assert.strictEqual(missing.length, 0, `Missing commands: ${missing.join(", ")}`);
  });
});
// #endregion 🌙️Command Registration Tests
