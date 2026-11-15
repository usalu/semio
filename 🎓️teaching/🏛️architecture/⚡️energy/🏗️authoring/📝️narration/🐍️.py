"""📝 Narration text model shared by captions, plain-text exports, read-time sizing and the voice.

A clause is plain prose with optional ``$…$`` TeX segments. This module owns the neutral view of
that text (segments, plain text, wrap clusters, line chunks); typesetting lives in the visuals and
pronunciation in the speech lexicon.
"""

from __future__ import annotations

import importlib
import re
from typing import Callable

_tex = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧮️tex.🐍️")
TexError = _tex.TexError
split_segments = _tex.split_segments
plain_text = _tex.plain_text
parse_tex = _tex.parse_tex
to_unicode = _tex.to_unicode
SYMBOLS = _tex.SYMBOLS
SPACES = _tex.SPACES
canon = _tex.canon

BREAK = "\n"
Cluster = tuple[bool, tuple[tuple[str, str], ...]]


def has_math(text: str) -> bool:
    """💲 Whether a clause carries at least one ``$…$`` segment."""
    return any(kind == "math" for kind, _ in split_segments(text))


def clusters(text: str) -> list[Cluster | str]:
    """🔗 Split a clause into unbreakable clusters ``(space_before, parts)`` and ``BREAK`` markers.

    A cluster is a run without whitespace, so punctuation glued to a formula (``$\\Delta\\theta$,``) never
    wraps away from it. ``parts`` mixes ``("text", …)`` and ``("math", …)``.
    """
    out: list[Cluster | str] = []
    current: list[tuple[str, str]] = []
    pending_space = False
    started_space = False

    def close() -> None:
        nonlocal current
        if current:
            out.append((started_space, tuple(current)))
            current = []

    for kind, content in split_segments(text):
        if kind == "math":
            if not current:
                started_space = pending_space and bool(out) and out[-1] != BREAK
                pending_space = False
            current.append(("math", content))
            continue
        for token in re.split(r"(\s+)", content):
            if not token:
                continue
            if token.isspace():
                close()
                if "\n" in token:
                    out.append(BREAK)
                    pending_space = False
                else:
                    pending_space = True
                continue
            if not current:
                started_space = pending_space and bool(out) and out[-1] != BREAK
                pending_space = False
            current.append(("text", token))
    close()
    return out


def wrap(items: list[Cluster | str], line_width: Callable[[list[Cluster]], float], max_width: float) -> list[list[Cluster]]:
    """📐 Greedy longest-prefix wrap by bisection; ``line_width`` measures a candidate line.

    A single cluster wider than ``max_width`` still gets its own line, and ``BREAK`` forces a new one.
    """
    lines: list[list[Cluster]] = []
    paragraph: list[Cluster] = []

    def flush() -> None:
        start = 0
        while start < len(paragraph):
            low, high = 1, len(paragraph) - start
            if line_width(paragraph[start:]) <= max_width:
                low = high
            while low < high:
                middle = (low + high + 1) // 2
                if line_width(paragraph[start:start + middle]) <= max_width:
                    low = middle
                else:
                    high = middle - 1
            lines.append(paragraph[start:start + low])
            start += low
        paragraph.clear()

    for item in items:
        if item == BREAK:
            flush()
        else:
            paragraph.append(item)
    flush()
    return lines or [[]]


def line_chunks(line: list[Cluster]) -> list[tuple[str, str, bool]]:
    """🧱 Merge a wrapped line into ``(kind, content, space_before)`` chunks: text runs and single formulas."""
    chunks: list[list] = []
    for index, (space, parts) in enumerate(line):
        for position, (kind, content) in enumerate(parts):
            gap = bool(space) if position == 0 and index > 0 else False
            if kind == "text" and chunks and chunks[-1][0] == "text":
                chunks[-1][1] += (" " if gap else "") + content
            else:
                chunks.append([kind, content, gap])
    return [(kind, content, gap) for kind, content, gap in chunks]


def caption_words(text: str) -> int:
    """🧾 Words a viewer reads: formulas count as the word they print as."""
    return len(plain_text(text).split())
