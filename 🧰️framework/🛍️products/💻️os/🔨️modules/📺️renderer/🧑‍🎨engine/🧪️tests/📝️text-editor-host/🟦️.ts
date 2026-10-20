/** 📝 Text editor host, selection retirement and span edits. */
import { describe } from "vitest";
import { registerRendersTextEditorHostTests } from "./🔶️renders-text-editor-host/🟦️.ts";
import { registerFrameworkRendererHostsTests } from "./🔸️framework-renderer-hosts/🟦️.ts";
import { registerRendersTextEditorHostTestsMore } from "./🟠️renders-text-editor-host/🟦️.ts";
import { registerMultispanreplaceRenamesOccurrenceRemapsTests } from "./🟢️multispanreplace-renames-occurrence-rema/🟦️.ts";

describe("framework renderer hosts", () => {
  registerRendersTextEditorHostTests();
  registerFrameworkRendererHostsTests();
  registerRendersTextEditorHostTestsMore();
  registerMultispanreplaceRenamesOccurrenceRemapsTests();
});
