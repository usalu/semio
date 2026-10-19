# 🎤️ Presentation

The semio presentation product: a declarative, render-independent presentation model — chapters,
sequences, thoughts, participants, embodiments, dispositions, arrangements and morphs — together with
a React renderer that plays it through reveal.js. The model knows nothing about the DOM; the renderer
is one target among possible others. Consumers such as
`♻️mit-bestand/🎤️präsentation/📅️33.projektetage` declare their deck as a glob of slide files and
mount it through the renderer.

## Layout

| Path | What lives there |
|---|---|
| `📦️packages/🟦️typescript/` | The core: the declarative model, the slide-glob loader, resolution scopes, morph-run auto-animate id assignment and the deck templates. Zero runtime imports. |
| `🎯️targets/⚛️react/` | The React + reveal.js renderer implementation: the public barrel `🟦️.tsx` (the single source of truth, consumed by plugins such as animate through `@semio-tech/presentation-react`) and `🔨️modules/` — the markdown → HTML compiler, the pdf.js canvas port and the renderer's internal modules (deck, reveal morph and auto-animate, figure scroll and zoom, morph views, interaction, arrangement section, json tree). |
| `📦️packages/🟦️typescript/🎯️targets/⚛️react/` | The `@semio-tech/presentation-react` package: a thin entry `🟦️.tsx` that re-exports the target and loads the stylesheet `🎨️.css`, the manifest and the test script. |
| `🧪️tests/` | One directory per language-agnostic test case: `🥒️.feature`, `🟦️.ts`, `🧫️fixtures/`. |
| `🔮️oracles/🔣️.json` | The owner oracle registry: the third-party packages the tests compare against, and the justified no-oracle decisions. |

## Commands

Start `bun run dashboard`, choose **New Task** (`Ctrl+B n`) and search
`presentation test` or `projektetage dev/build`. Each owner target routes to its
`📜️script.ts` through the build graph.

```bash
bun run dashboard run @semio-tech/presentation:test -- quick
bun run dashboard run @semio-tech/presentation-react:test -- quick
bun run dashboard run @semio-tech/mit-bestand-praesentation-projektetage:dev --detach --wait-ready
bun run dashboard run @semio-tech/mit-bestand-praesentation-projektetage:build
```

The deck's task offers **Open ready address** (`Ctrl+B u`). Task overview retains
output and its menu offers restart, stop and cancellation.

## Domain model

### Presentation

A presentation consists of chapters.
The slides are presented by a grid.
When `escape` is pressed, the overview is shown.
The presentation is website under site `/{columnIndex}/{rowIndex}?chapter={chapterName}&sequence={sequenceName}&thought={thoughtName}&slide={slideName}`. The variables are only for humans and not used by the system.

### Column

A vertical list of slides.

### Row

The slide in a column.

### Chapter

A chapter of sequences.

### Sequence

A sequence of thoughts.
A sequence is presented as a column.

### Thought

A thought has one or many slides.

### Participant

A participant is either part of a presentation, a chapter, a sequence, a thought or a slide and is available only for the artifact and its children.
A participant appear on one or many slides.
A participant is represented with an embodiment inside a disposition.
Multiple participants can have the same embodiment.

### Embodiment

An embodiment is either part of a presentation, a chapter, a sequence, a thought or a slide and is available only for the artifact and its children.
An embodiment is a representation of a participant in a certain shape.
The embodiment has a default style which can be changed by dispositions.

### Figure

A figure embodiment of a participant.

### Video

A video embodiment of a participant.

### Text

A text embodiment of a participant.

### Pdf

A pdf embodiment of a participant.

### Demo

A demo embodiment of a pariticipant.
A demo is an iframe that when active has full control over keyboard etc.

### Disposition

A disposition is a concrete positioned, sized and styled embodiment.
A disposition is modifyable (hoverable, draggable, selectable, resizable and rotatetable).
Every disposition has two small buttons on the top right that appear when it is selected: reset, enlarge.
The reset button resets all modifications. The enlarge button turns the large.
A selection rectangle is started when the user clicks into empty stapce.
When the selection rectangle is going to the left partial inclusion is enough in order to select it.
When the selection rectangle is going to the right full inclusion is necessary in order to select it.
You MUST NOT distort an embodiment. It is always a filled, scaled to the shorter-side, centered and covers an arbitrary target size.

### Morph

A morph is when source dispositions (position, size, style) morphs into target dispositions (position, size, style).
You MUST morph from the actual disposition including modifications.

#### OneToOne

One disposition morphing into one disposition.

#### OneToMany

One disposition morphing into many dispositions.

#### ManyToOne

Many disposition morphinh into many dispositions.

### Arrangement

An arrangement of dispositions.

### Transition

A transition from an arrangement into another arrangement.

### Slide

A slide is an arragement with an optional transition to the next slide.

### Template

A template produces a set of matching artifacts (a presentation, a chapter, a sequence, a tought, a partcipant, embodiment, figure, video, text, pdf, disposition, split, morph, arrangement, transition, …)
Templates can use other templates.

#### Intro

The intro template produces an sequence along with participants: title, description, goal, authors, affiliations.

#### Tile

A tile template produces from a source figure a new figure embodiments.

#### Split

A spit template produces a grid of tiles.
