"""📚 One reference registry for all five energy videos: every standard, guideline, law, book and data set that appears on screen, with edition, status, verified source page and per-use locators.

The data live in ``🔣️.json`` (schema ``🧬️schema/🔣️.schema.json``); scenes, quizzes and the guard test only ever see the strings built here.

@see https://www.gesetze-im-internet.de/geg/BJNR172810020.html — the law as renamed GModG
"""

from __future__ import annotations

import json
import os
from functools import cache
from pathlib import Path

SEPARATOR = " · "
LINE_LIMIT = 72
LANGUAGES = ("de", "en")
STATUS_WORDS = {
    "withdrawn": {"de": "zurückgezogen", "en": "withdrawn"},
    "superseded": {"de": "ersetzt", "en": "superseded"},
}
CURRENT_WORDS = {"de": "aktuell", "en": "current"}


@cache
def registry() -> dict[str, dict]:
    """🗂️ Every entry of ``🔣️.json`` by id."""
    data = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
    return {entry["id"]: entry for entry in data["entries"]}


def entry(reference_id: str) -> dict:
    """🔎 The entry of ``reference_id``; an unknown id is a programming error."""
    try:
        return registry()[reference_id]
    except KeyError:
        raise KeyError(f"unknown reference id {reference_id!r}") from None


def short(reference_id: str) -> str:
    """✂️ The designation without edition, e.g. ``DIN 4108-2``; only for spoken narration and sentences that carry the edition elsewhere."""
    return entry(reference_id)["designation"]


def designation(reference_id: str) -> str:
    """🏷️ Designation with its edition, e.g. ``DIN 4108-2:2026-05``; laws and books return their citation name."""
    item = entry(reference_id)
    if item["kind"] in ("standard", "guideline"):
        return f"{item['designation']}:{item['edition']}"
    return item["designation"]


def _author_names(item: dict) -> str:
    return ", ".join(name.split(",")[0] for name in item["authors"])


def citation(reference_id: str, language: str) -> str:
    """📎 Short author-year citation of a book, article, thesis or data set, e.g. ``Willems, Schild, Stricker (2024), 8. Aufl.``."""
    item = entry(reference_id)
    if "citeAs" in item:
        return item["citeAs"][language]
    text = f"{_author_names(item)} ({item['year']})"
    if "editionNumber" in item and item["editionNumber"] > 1:
        text += f", {item['editionNumber']}. Aufl." if language == "de" else f", {item['editionNumber']}th ed."
    return text


def base(reference_id: str, language: str) -> str:
    """🧱 What opens a reference on screen: designation with edition, the law's full name, or the author-year citation."""
    item = entry(reference_id)
    if item["kind"] in ("standard", "guideline"):
        return designation(reference_id)
    if item["kind"] == "law":
        return item["full"][language]
    return citation(reference_id, language)


def ref(reference_id: str, locator: str | None = None, *, language: str) -> str:
    """📖 Full on-screen string of one use, e.g. ``DIN 4108-2:2026-05 · Abschn. 8 – Sommerlicher Wärmeschutz``."""
    if language not in LANGUAGES:
        raise ValueError(f"language must be one of {LANGUAGES}")
    text = base(reference_id, language)
    if locator is None:
        return text
    try:
        return f"{text}{SEPARATOR}{entry(reference_id)['locators'][locator][language]}"
    except KeyError:
        raise KeyError(f"unknown locator {locator!r} of {reference_id!r}") from None


def chip(*parts: str | tuple[str, str | None], language: str) -> str:
    """🔗 Several references on one line; a part is an id, an ``(id, locator)`` pair or ``!id`` for the status chip of a withdrawn entry."""
    rendered = [status_chip(part[1:], language) if isinstance(part, str) and part.startswith("!") else ref(*((part, None) if isinstance(part, str) else part), language=language) for part in parts]
    line = SEPARATOR.join(rendered)
    return line if len(line) <= LINE_LIMIT else "\n".join(rendered)


def cite(*parts: str | tuple[str, str | None]) -> str:
    """🌐 ``chip`` in the label language of the runtime (``SEMIO_TEACHING_LANGUAGE``, the contract of ``visuals.language``)."""
    selected = os.environ.get("SEMIO_TEACHING_LANGUAGE")
    if selected not in LANGUAGES:
        raise ValueError("Set SEMIO_TEACHING_LANGUAGE to en or de")
    return chip(*parts, language=selected)


def status_chip(reference_id: str, language: str) -> str:
    """🚩 Reference of a withdrawn or superseded entry with its status and the current successor, e.g. ``DIN V 4108-6:2003-06 (zurückgezogen) – Symbolik; aktuell: DIN/TS 18599``."""
    item = entry(reference_id)
    if item["status"] == "current":
        return base(reference_id, language)
    text = f"{designation(reference_id)} ({STATUS_WORDS[item['status']][language]})"
    if "role" in item:
        text += f" – {item['role'][language]}"
    return f"{text}; {CURRENT_WORDS[language]}: {short(item['successor'])}"


def bibliography(reference_id: str, language: str) -> str:
    """📕 Full bibliographic line of a book, article, thesis or data set: authors, title, edition, publisher, year and ISBN or DOI."""
    item = entry(reference_id)
    authors = "; ".join(item["authors"])
    title = item["title"][language]
    if item["kind"] == "article":
        return f"{authors} ({item['year']}): {title}. {item['journal']} {item['volume']}, {item['pages']}. DOI {item['doi']}"
    if item["kind"] == "thesis":
        return f"{authors} ({item['year']}): {title}. {'Diss.' if language == 'de' else 'Doctoral thesis,'} {item['institution']}"
    parts = [f"{authors} ({item['year']}): {title}"]
    if "editionNumber" in item:
        parts.append(f"{item['editionNumber']}. Aufl." if language == "de" else f"{item['editionNumber']}th ed.")
    if "publisher" in item:
        parts.append(f"{item['publisher']}, {item['place']}")
    if "isbn" in item:
        parts.append(f"ISBN {item['isbn']}")
    if "doi" in item:
        parts.append(f"DOI {item['doi']}")
    tail = ", ".join(parts[1:])
    return parts[0] + (f". {tail}" if tail else "")


def card(reference_id: str, language: str) -> dict[str, str]:
    """🪪 The lines of a bibliography card: ``who`` (author-year, edition), ``title``, ``subtitle``, ``imprint`` (publisher, place, ISBN) and a DOI ``link`` (empty without DOI)."""
    item = entry(reference_id)
    title, _, subtitle = item["title"][language].partition(": " if ": " in item["title"][language] else " – ")
    imprint = f"{item['publisher']}, {item['place']}" + (f" · ISBN {item['isbn']}" if "isbn" in item else "")
    return {"who": citation(reference_id, language), "title": title, "subtitle": subtitle, "imprint": imprint,
            "link": f"https://doi.org/{item['doi']}" if "doi" in item else ""}


def all_forms() -> frozenset[str]:
    """📜 Every string the registry can emit for a designation: with edition, the short form, locator references, status chips, citations and bibliography lines."""
    forms: set[str] = set()
    for reference_id, item in registry().items():
        forms.add(designation(reference_id))
        forms.add(short(reference_id))
        forms.update(item.get("aliases", ()))
        for language in LANGUAGES:
            forms.add(base(reference_id, language))
            forms.add(status_chip(reference_id, language))
            if item["kind"] not in ("standard", "guideline", "law"):
                forms.add(bibliography(reference_id, language))
            for locator in item["locators"]:
                forms.add(ref(reference_id, locator, language=language))
    return frozenset(forms)
