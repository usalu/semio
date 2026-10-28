# Cooling Load — Full Video

Presents the **Cooling load series** (Kühllast, VDI2078 design power rather than annual cooling energy) inside the shared virtual semio UI. Its lesson window zooms in from the five-window canvas and returns to the identical overview; navbar and footer remain visible.

1. Teil 1 — Heizwärmebedarf vs. Kühllast
2. Teil 2 — Interne Wärmegewinne
3. Teil 3 — Transmission & Lüftungslast
4. Teil 4 — Solare Einstrahlung
5. Teil 5 — Systemauslegung
6. Teil 6 — Sommerstrategie

Entry file: [`🐍️.py`](./🐍️.py)

This is the fourth video in the curriculum: Physical Fundamentals → **Lüftung** (`../2_ventilation/`) → Heizung →
**Kühlung** → Energiebilanz. Ventilation is not explained here any more — see the note at the end.

---

## Render

From the workspace root, the repository entry provisions the pinned Python environment:

```sh
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video cooling --quality l --jobs 4 --output-root ./videos --no-play
```

Use `--quality m` for 720p30 or `--quality h` for 1080p60. The renderer constructs each canonical source beat once, selects the declared narration sequences, assembles ordered chapters, and presents the complete lesson inside its virtual window. Video and optional source audio use identical cut boundaries; silent zoom transitions pad both ends. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled. Add `--chapter <chapter-identifier>` to render a selected chapter inside the same virtual window. Pass --language en for English chrome; the authored teaching content retains its recorded language. Chapter identifiers are declared in the curriculum JSON; lesson entry files expose `compose(language="de", chapters=[...], **settings)`.

The final low-quality movie is `<output-root>/rendered/Full_Cooling_Demand_de_480p15.mp4`. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Identifiers and Beats

| Chapter identifier | Content |
|-------------|---------|
| `Cooling_01_HeatingVsCooling` | Teil 1 |
| `Cooling_02_InternalGains` | Teil 2 |
| `Cooling_03_TransmissionHumidity` | Teil 3 — opake Transmission, canonical physics thermal-mass excerpts, Brückenbeat `Beat3_LueftungslastImSommer` |
| `Cooling_04_SolarRadiation` | Teil 4 |
| `Cooling_05_Systemauslegung` | Teil 5 — `Beat2_VolumeFlowEquation`, `Beat3_IsolateAirflow` (Last → Luftstrom → Kanal, gegen den Hygieneluftstrom) |
| `Cooling_06_Sommerstrategie` | Teil 6 — `Beat1_LastZuerstSenken`, `Beat2_SorptionsKuehlung`, `Beat3_KomfortStrategie` |

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
