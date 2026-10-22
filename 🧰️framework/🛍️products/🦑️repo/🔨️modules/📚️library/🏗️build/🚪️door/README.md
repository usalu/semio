# Build CLI front door

## English

The front door is the entry the `repo` launcher starts for every build command (`🚪️door/📜️script.ts`). It reads the neutral operations catalogue and answers what needs no build implementation: help of every command, refusals of unknown commands, unknown subcommands of closed sets and values outside an option's enumeration, and the light commands declared in `🔣️.json` (`leases`, `capacity`, `doctor`). It loads 11 modules (about 64 KB) for help and a refusal, the command's own closure for a light command, and the build implementation (`../📜️script.ts`) only when a command needs it.

- **Declared, not hand-listed.** Commands, subcommands, options, languages and help come from `⚙️operations/🔣️catalog.json`; the door's own data is `🔣️.json` (closed schema `🧬️schema/🔣️.json`): the light operations, which subcommand sets are closed, the target prefixes that must reach the full implementation, and the EN/DE refusal texts. There is no default language: the shared language rule decides, and a refusal is bilingual.
- **Exit codes.** A refusal prints to stderr and exits with `refusedExitCode` (2); a light command exits with its own code.
- **Law.** `🧪️tests/🚦️startup` fails when help, a refusal or a light command loads a heavy module group (graph, native build, test harness, caller, invocation, selection, resident service, tool bootstrap, store receipts, tools); the traced module list equals the static closure that esbuild computes for the door.
- **Adding a light command.** Declare it in `🔣️.json`, add one literal loader to `buildDoorLightLoadersV1` and its row to `🧪️tests/🚦️startup/🔣️.json` with the groups it may not load.

## Deutsch

Die Vordertür ist der Einstieg, den der `repo`-Starter für jeden Build-Befehl startet (`🚪️door/📜️script.ts`). Sie liest den werkzeugneutralen Operationskatalog und beantwortet, was keine Build-Implementierung braucht: die Hilfe jedes Befehls, die Ablehnung unbekannter Befehle, unbekannter Unterbefehle geschlossener Mengen und von Werten ausserhalb der Aufzählung einer Option sowie die in `🔣️.json` deklarierten leichten Befehle (`leases`, `capacity`, `doctor`). Sie lädt für Hilfe und Ablehnung 8 Module, für einen leichten Befehl dessen eigene Hülle und die Build-Implementierung (`../📜️script.ts`) nur, wenn ein Befehl sie braucht.

- **Deklariert, nicht von Hand gelistet.** Befehle, Unterbefehle, Optionen, Sprachen und Hilfe stammen aus `⚙️operations/🔣️catalog.json`; die eigenen Daten der Tür sind `🔣️.json` (geschlossenes Schema `🧬️schema/🔣️.json`): die leichten Operationen, welche Unterbefehlsmengen geschlossen sind, die Ziel-Präfixe, die zur vollen Implementierung müssen, und die EN/DE-Texte der Ablehnungen. Es gibt keine Standardsprache: die gemeinsame Sprachregel entscheidet, und eine Ablehnung ist zweisprachig.
- **Exit-Codes.** Eine Ablehnung geht nach stderr und endet mit `refusedExitCode` (2); ein leichter Befehl endet mit seinem eigenen Code.
- **Gesetz.** `🧪️tests/🚦️startup` schlägt fehl, wenn Hilfe, eine Ablehnung oder ein leichter Befehl eine schwere Modulgruppe lädt (Graph, nativer Build, Testgeschirr, Aufrufer, Invocation, Selektion, residenter Dienst, Werkzeug-Bootstrap, Store-Belege, Werkzeuge); die verfolgte Modulliste entspricht der statischen Hülle, die esbuild für die Tür berechnet.
- **Leichten Befehl hinzufügen.** In `🔣️.json` deklarieren, einen literalen Lader zu `buildDoorLightLoadersV1` und seine Zeile zu `🧪️tests/🚦️startup/🔣️.json` mit den Gruppen, die er nicht laden darf, ergänzen.
