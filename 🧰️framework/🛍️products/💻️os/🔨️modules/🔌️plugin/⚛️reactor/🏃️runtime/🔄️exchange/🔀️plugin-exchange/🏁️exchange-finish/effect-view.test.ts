import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import law from "./effect-view.json";

test("pending effects reads the original surface view without creating an owner", () => {
  const db = new Database(":memory:");
  db.exec("CREATE TABLE surface (window TEXT PRIMARY KEY, utility TEXT)");
  db.query("INSERT INTO surface VALUES (?, ?)").run(law.window, law.utility);
  const count = db.query<{ owners: number }, []>("SELECT COUNT(*) AS owners FROM surface");
  const read = db.query("SELECT utility FROM surface WHERE window = ?");
  expect(count.get()!.owners).toBe(law.viewOwnersBefore);
  for (const owners of [law.viewOwnersAfterPending, law.viewOwnersAfterPublication]) {
    expect(db.transaction(() => read.get(law.window))()).toEqual({ utility: law.utility });
    expect(count.get()!.owners).toBe(owners);
  }
  db.close();
});

test("background and refresh pending effects borrow the view inside the instance guard", () => {
  const source = readFileSync(new URL("../../../🧰️ops/🧾️typed-operations/🦀️.rs", import.meta.url), "utf8");
  const drive = source.slice(source.indexOf("pub fn plugin_drive_pending_effects"));
  expect(drive).toMatch(/let active = &mut \*active;\s*let view = active\.surface_contexts\.view\(\);\s*match [^\n]*active\.app\.pending_effects\(view, cx\)/);
  const refresh = readFileSync(new URL("../../../🧰️ops/🖥️surface-render/🦀️.rs", import.meta.url), "utf8");
  expect(refresh).toMatch(/let instance = &mut \*instance;\s*[^\n]*instance\.app\.pending_effects\(instance\.surface_contexts\.view\(\), cx\)/);
});
