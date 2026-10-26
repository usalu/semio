# EnergyBalance — Full Video

Plays the **EnergyBalance series** in curriculum order, prepended with the
series intro card ``Demo_Intro_Energiebilanz`` from ``🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/👋️intro/🐍️.py``:

0. Intro — Energiebilanz
1. Kapitel 1 — Die Energiebilanz des Gebäudes
2. Kapitel 2 — Regelwerk & Klimadaten
3. Kapitel 3 — Bedarf vs. Leistung
4. Kapitel 4 — Von der Nutzenergie zur Endenergie
5. Kapitel 5 — Primärenergie & das GEG
6. Kapitel 6 — Der Energieausweis
7. Kapitel 7 — Theorie vs. Praxis

Entry file: [`🐍️.py`](./🐍️.py)

This is the fifth video in the curriculum, after `1_physical_fundamentals/`,
`2_ventilation/`, `3_heating/`, and `4_cooling/`.

---

## The through-line

The series is one continuous calculation, not seven topics. Each chapter hands
the next one a number:

| Step | Quantity | Where it comes from |
|------|----------|---------------------|
| Bilanz | `Q_h = Q_Senke − η_h · Q_Quelle` | Kapitel 1, from the Heating/Cooling flows |
| Verfahren | 12 Monatsbilanzen, Klimadaten, `G_t` | Kapitel 2 |
| Leistung ≠ Energie | `Φ_HL [kW]` vs. `Q_h [kWh/a]` | Kapitel 3 |
| Anlagenkette | `Q_E = Q_h · e_p` | Kapitel 4 |
| Vorkette | `Q_P = Q_E · f_p` | Kapitel 5 |
| Ergebnis | Effizienzklasse auf dem Ausweis | Kapitel 6 |
| Realität | Bedarf ≠ Verbrauch | Kapitel 7 |

The building blocks of `Q_V` (airflow `V̇ = V·n`, `c_Luft`, `Δθ`, `η_WRG`) come
from the Lüftung video (`2_ventilation/`); Kapitel 1 only sums them into the loss side.

One schematic worked example runs through Kapitel 4 to 7 unchanged, so the
numbers on screen stay the same building throughout:

```
Q_h  = 100 kWh/(m²·a)
  · e_p = 1,04 · 1,08 · 1,06 · 1,18 ≈ 1,40 (Gaskessel)  → Q_E = 140 → · f_p = 1,1 → Q_P = 154 → Klasse E
  · e_p = 1,04 · 1,08 · 1,06 · 0,26 ≈ 0,31 (Wärmepumpe) → Q_E =  31 → · f_p = 1,8 → Q_P ≈  56 → Klasse A
```

Where each number lives, so a change is made once:

| Value | Source of truth | Mirrored in |
|-------|-----------------|-------------|
| `e_ü`, `e_d`, `e_s`, `e_g` and `Q_E` | `♨️system-losses/🐍️.py` (`E_*`, `Q_E_GAS`, `Q_E_WP`) — Beat 1's staircase is derived from the same factors | `🏭️primary-energy/🐍️.py` (`E_P_*`) |
| `f_p` (GEG Anlage 4), `Q_P`, example GEG limit | `🏭️primary-energy/🐍️.py` | `🧪️practice/🐍️.py` (`CHAIN_*`) |
| displayed `Q_E` and the efficiency class | `🏷️certificate/🐍️.py` (`GAS_END`, `WP_END`, `_class_letter`) | `🧪️practice/🐍️.py` |

Kapitel 5 Beat 4 compares these against a GEG limit of 66 kWh/(m²·a) — 55 % of
a **schematic** reference-building value of 120 (GEG § 15). The real reference
value is building-specific; the beat labels it as an example on screen.

Standards cited per beat via the `_din_ref()` chip, with editions where it
matters: GEG (§ 15/16, § 20, § 60c, § 71, § 79–87, Anlagen 1/4/10),
DIN V 18599:2018-09 (Teile 1/2/5/6/10) and its 2025 continuation DIN/TS 18599,
DIN EN 12831-1:2017-09 / DIN/TS 12831-1:2020-04, DIN 4108-2/-6, VDI 2078,
VDI 3807, VDI 4645, VDI 4650-1:2024, VOB/C DIN 18380.

---

## Render

From the workspace root, the repository entry provisions the pinned Python environment:

```sh
repo 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --video energy_balance --quality l --jobs 4 --output-root ./videos --no-play
```

Use `--quality m` for 720p30 or `--quality h` for 1080p60. The renderer constructs every authored beat independently, assembles ordered chapters, and joins the institutional intro and all chapters into the complete silent lesson. Source-aware complete-clip caches support cancellation and restarting; Manim partial caches stay disabled.

The final low-quality movie is `<output-root>/rendered/Full_EnergyBalance_480p15.mp4`. Private raw segment media and logical chapter clips stay under the chosen output root.

## Chapter Scenes and Beats

| Scene class | Content |
|-------------|---------|
| `EnergyBalance_01_Energiebilanz` | Kapitel 1 |
| `EnergyBalance_02_RegelwerkKlima` | Kapitel 2 |
| `EnergyBalance_03_BedarfVsLeistung` | Kapitel 3 |
| `EnergyBalance_04_NutzZuEndenergie` | Kapitel 4 |
| `EnergyBalance_05_Primaerenergie` | Kapitel 5 |
| `EnergyBalance_06_Energieausweis` | Kapitel 6 |
| `EnergyBalance_07_TheorieVsPraxis` | Kapitel 7 |

| Chapter | Beat | Idea |
|---------|------|------|
| 1 | `Beat1_Bilanzgrenze` | Bilanzgrenze & Energieerhaltung |
| 1 | `Beat2_Verlustseite` | `Q_T + Q_V`, Wärmebrückenzuschlag |
| 1 | `Beat3_Gewinnseite` | `Q_S + Q_I`, Ausnutzungsgrad `η_h` |
| 1 | `Beat4_WinterUndSommer` | dieselbe Bilanz, umgekehrtes Vorzeichen |
| 2 | `Beat1_GEGundNormen` | GEG → DIN V 18599 → Nachbarnormen |
| 2 | `Beat2_Monatsbilanz` | warum kein Jahresmittel |
| 2 | `Beat3_Klimadaten` | `θ_e`, `I_S`, Testreferenzjahr |
| 2 | `Beat4_Gradtagszahl` | Heizgrenze, `G_t`, Witterungsbereinigung |
| 3 | `Beat1_LeistungUndEnergie` | Höhe vs. Fläche einer Leistungskurve |
| 3 | `Beat2_Normheizlast` | DIN EN 12831-1, Gewinne bewusst ignoriert |
| 3 | `Beat3_Jahresdauerlinie` | Spitze, Fläche, Vollbenutzungsstunden |
| 3 | `Beat4_RichtigAuslegen` | Unter- und Überdimensionierung, Takten |
| 4 | `Beat1_Verlustkette` | vier Stufen vom Raum zum Zähler |
| 4 | `Beat2_UebergabeVerteilungSpeicherung` | Vorlauftemperatur, Rohr- und Speicherverluste |
| 4 | `Beat3_Erzeugung` | Wirkungsgrad, Brennwert, Jahresarbeitszahl |
| 4 | `Beat4_Anlagenaufwandszahl` | `e_p = e_ü · e_d · e_s · e_g` |
| 5 | `Beat1_DreiBilanzgrenzen` | Raum / Zähler / Quelle |
| 5 | `Beat2_Primaerenergiefaktor` | `f_p` je Energieträger |
| 5 | `Beat3_Rechenbeispiel` | ein Gebäude, zwei Anlagen |
| 5 | `Beat4_Referenzgebaeude` | der Grenzwert ist das eigene Gebäude |
| 6 | `Beat1_BedarfVsVerbrauch` | berechnet vs. gemessen |
| 6 | `Beat2_DieSkala` | A+ … H, beide Beispielgebäude verortet |
| 6 | `Beat3_PflichtangabenUndFolgen` | Inserat, Effizienzklasse, Empfehlungen |
| 7 | `Beat1_BerechnungVsAbrechnung` | genormte Randbedingungen, Prebound/Rebound |
| 7 | `Beat2_HydraulischerAbgleich` | Durchfluss folgt Widerstand, nicht Bedarf |
| 7 | `Beat3_SerienAbschluss` | die fünf Videos als eine Rechnung |

## Authoring

Each named chapter owns its scene at `🐍️.py`, optional narration tool at `🔊️speech/🐍️.py`, composition tool at `🏗️composition/🐍️.py`, and timing/trace JSON under their respective semantic directories. The language-neutral curriculum descriptor at `../../🎬️clip/🔣️.json` controls the complete source order. Shared fonts, visual primitives and tool interfaces live under `../../🏗️authoring`.

Narration synthesis requires explicitly configured provider credentials. The complete silent lesson needs no external TTS account. Keep measured narration timing aligned when changing a beat. Set `LAYOUT_CHECK=1` in the environment to enable the authored layout guard while rendering.
