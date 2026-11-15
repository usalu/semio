"""🗣️ Spoken lexicon: turns the inline TeX of a narration clause into words the voice can read.

Every pronunciation lives in ``🔣️.json`` (validated by ``🧬️schema.json``), per language and without a
default language. Whole-symbol overrides win over token composition; TeX the lexicon cannot speak raises
``SpeechError`` before anything is sent to a synthesizer.

https://json-schema.org/draft/2020-12/json-schema-core
"""

from __future__ import annotations

import importlib
import json
import re
from functools import lru_cache
from pathlib import Path

_narration = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🐍️")
_tex = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.📝️narration.🧮️tex.🐍️")
LEXICON: dict = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
LANGUAGES: tuple[str, ...] = tuple(LEXICON["languages"])
_PUNCTUATION = frozenset(",;:.!?")
_THIN = " "
_SEPARATOR = "|"


class SpeechError(ValueError):
    """⛔ A clause contains TeX or text the voice must not receive unconverted."""


class _UnitError(SpeechError):
    pass


def require_language(language: str) -> str:
    """🌐 Reject anything but an explicit supported language; there is no default."""
    if language not in LANGUAGES:
        raise SpeechError(f"language must be one of {LANGUAGES}, got {language!r}")
    return language


def _plural(entry, form: str) -> str:
    return entry if isinstance(entry, str) else entry[form]


def _unit_like(node) -> bool:
    return node[0] == "mathrm" or (node[0] == "scripts" and node[1] is not None and node[1][0] == "mathrm")


class _Speaker:
    def __init__(self, language: str) -> None:
        self.language = language
        pick = lambda table: {key: value[language] for key, value in table.items()}
        self.names = pick(LEXICON["names"])
        self.characters = pick(LEXICON["characters"])
        self.words = pick(LEXICON["words"])
        self.powers = pick(LEXICON["powers"])
        self.index_words = pick(LEXICON["indexWords"])
        self.abbreviations = pick(LEXICON["abbreviations"])
        self.overrides = {_tex.canon(_tex.parse_tex(key)): value[language] for key, value in LEXICON["symbols"].items()}
        self.longest = max(len(_tex.parse_tex(key)) for key in LEXICON["symbols"]) + 3
        self.units = LEXICON["units"]

    def number(self, raw: str) -> str:
        digits = raw.replace(_THIN, "").replace(",", ".")
        return digits.replace(".", "," if self.language == "de" else ".")

    def sequence(self, nodes) -> str:
        out: list[str] = []
        count = None
        index = 0
        while index < len(nodes):
            node = nodes[index]
            if node[0] == "space":
                index += 1
                continue
            hit = self.override(nodes, index)
            if hit:
                out.append(hit[0])
                index += hit[1]
                count = None
                continue
            consumed = 1
            if _unit_like(node):
                while index + consumed + 1 < len(nodes) and nodes[index + consumed] == ("char", "/") and _unit_like(nodes[index + consumed + 1]):
                    consumed += 2
                text, punctuation = self.unit(nodes[index:index + consumed], count), False
            else:
                text, punctuation = self.node(nodes, index, count)
            if punctuation and out:
                out[-1] += text
            else:
                out.append(text)
            count = node[1] if node[0] == "num" else None
            index += consumed
        return " ".join(part for part in out if part)

    def override(self, nodes, index: int):
        for length in range(min(len(nodes) - index, self.longest), 0, -1):
            key = _tex.canon(nodes[index:index + length])
            if key and key in self.overrides:
                return self.overrides[key], length
        return None

    def neighbour(self, nodes, index: int, step: int):
        index += step
        while 0 <= index < len(nodes) and nodes[index][0] == "space":
            index += step
        return nodes[index] if 0 <= index < len(nodes) else None

    def node(self, nodes, index: int, count) -> tuple[str, bool]:
        node = nodes[index]
        kind = node[0]
        if kind == "char":
            char = node[1]
            if char in _PUNCTUATION:
                return char, True
            if char == "%":
                return self.unit([node], count), False
            if char == "/":
                before, after = self.neighbour(nodes, index, -1), self.neighbour(nodes, index, 1)
                if before and after and _unit_like(before) and _unit_like(after):
                    return self.words["per"], False
            if char.isalpha() and char.isascii():
                return char, False
            if char in self.characters:
                return self.characters[char], False
            raise SpeechError(f"no spoken form for {char!r}")
        if kind == "num":
            return self.number(node[1]), False
        if kind == "sym":
            if node[1] == "ldots" and (self.neighbour(nodes, index, -1) or ("",))[0] == "num" and (self.neighbour(nodes, index, 1) or ("",))[0] == "num":
                return self.words["range"], False
            if node[1] not in self.names:
                raise SpeechError(f"no spoken form for \\{node[1]}")
            return self.names[node[1]], False
        if kind == "deg":
            return self.words["degree"], False
        if kind == "text":
            return node[1], False
        if kind == "group":
            return self.sequence(node[1]), False
        if kind == "mathrm":
            return self.unit([node], count), False
        if kind == "frac":
            return self.frac(node, count), False
        if kind == "dot":
            return self.words["dot"].replace("{}", self.sequence(node[1])), False
        if kind == "sqrt":
            return self.words["sqrt"].replace("{}", self.sequence(node[1])), False
        if kind == "scripts":
            return self.scripts(node), False
        raise SpeechError(f"no spoken form for node {kind!r}")

    def frac(self, node, count) -> str:
        numerator, denominator = node[1], node[2]
        significant = lambda nodes: [item for item in nodes if item[0] != "space"]
        if significant(denominator) and all(_unit_like(item) for item in significant(denominator)):
            if significant(numerator) == [("num", "1")]:
                return self.unit_items_speech(["/", "(", *self.unit_items(denominator), ")"], count)
            if significant(numerator) and all(_unit_like(item) for item in significant(numerator)):
                return self.unit_items_speech([*self.unit_items(numerator), "/", "(", *self.unit_items(denominator), ")"], count)
        wrap = lambda nodes, spoken: f"{self.words['fractionOpen']} {spoken} {self.words['fractionClose']}" if _tex.has_operator(nodes) else spoken
        return f"{wrap(numerator, self.sequence(numerator))} {self.words['over']} {wrap(denominator, self.sequence(denominator))}"

    def scripts(self, node) -> str:
        if _unit_like(node):
            return self.unit([node], None)
        parts = []
        if node[1] is not None:
            parts.append(self.sequence((node[1],)))
        if node[2] is not None:
            parts.append(self.index(node[2]))
        if node[3] is not None:
            parts.append(self.power(node[3]))
        return " ".join(part for part in parts if part)

    def index(self, nodes) -> str:
        flat: list = []

        def walk(sequence) -> None:
            for item in sequence:
                if item[0] in {"group", "mathrm"}:
                    walk(item[1])
                else:
                    flat.append(item)

        walk(nodes)
        groups: list[list] = [[]]
        for item in flat:
            if item == ("char", ","):
                groups.append([])
            elif item[0] != "space":
                groups[-1].append(item)
        spoken = []
        for group in groups:
            if not group:
                continue
            key = _tex.canon(group)
            if key in self.index_words:
                spoken.append(self.index_words[key])
            else:
                spoken.append(" ".join(self.sequence((item,)) for item in group))
        return " ".join(spoken)

    def power(self, nodes) -> str:
        key = _tex.canon(nodes).replace("−", "-")
        if key in self.powers:
            return self.powers[key]
        if nodes in {(("sym", "circ"),), (("deg",),)}:
            return self.words["degree"]
        return self.words["power"].replace("{}", self.sequence(nodes))

    def unit(self, nodes, count) -> str:
        try:
            return self.unit_items_speech(self.unit_items(nodes), count)
        except _UnitError as error:
            raise _UnitError(f"{error} (write plain words with \\text{{…}})") from None

    def unit_items(self, nodes) -> list:
        items: list = []

        def add(char: str) -> None:
            if items and isinstance(items[-1], list) and not items[-1][2]:
                items[-1][0] += char
            else:
                items.append([char, 1, False])

        def walk(sequence) -> None:
            for node in sequence:
                kind = node[0]
                if kind in {"mathrm", "group"}:
                    walk(node[1])
                elif kind == "char" and (node[1].isalpha() or node[1] == "%"):
                    add(node[1])
                elif kind == "char" and node[1] in "/()":
                    items.append(node[1])
                elif kind == "deg":
                    add("°")
                elif kind == "sym" and node[1] == "cdot":
                    items.append(_SEPARATOR)
                elif kind == "space":
                    items.append(_SEPARATOR)
                elif kind == "scripts":
                    if node[1] is not None:
                        walk((node[1],))
                    last = items[-1] if items and isinstance(items[-1], list) else None
                    if last is None:
                        raise _UnitError(f"exponent without unit in {_tex.canon((node,))!r}")
                    if node[2] is not None:
                        last[0] += "_{" + _tex.canon(node[2]) + "}"
                        last[2] = True
                    if node[3] is not None:
                        exponent = _tex.canon(node[3]).replace("−", "-")
                        if not re.fullmatch(r"-?\d+", exponent):
                            raise _UnitError(f"unit exponent {exponent!r} is not an integer")
                        last[1] = int(exponent)
                        last[2] = True
                else:
                    raise _UnitError(f"{_tex.canon((node,))!r} is not a unit")

        walk(nodes)
        return items

    def split_run(self, run: str) -> list[str]:
        if run in self.abbreviations:
            return [run]
        best: list[list[str] | None] = [None] * (len(run) + 1)
        best[0] = []
        for end in range(1, len(run) + 1):
            for start in range(end):
                if best[start] is not None and run[start:end] in self.units:
                    candidate = [*best[start], run[start:end]]
                    if best[end] is None or len(candidate) < len(best[end]):
                        best[end] = candidate
        if best[len(run)] is None:
            raise _UnitError(f"unknown unit {run!r}")
        return best[len(run)]

    def unit_items_speech(self, items: list, count) -> str:
        terms: list[tuple[str, int, bool]] = []
        stack, pending = [False], False
        for item in items:
            if item == "/":
                pending = True
            elif item == "(":
                stack.append(stack[-1] or pending)
                pending = False
            elif item == ")":
                if len(stack) == 1:
                    raise _UnitError("unbalanced brackets in unit")
                stack.pop()
            elif item == _SEPARATOR:
                continue
            else:
                run, exponent, _ = item
                units = self.split_run(run)
                for position, unit in enumerate(units):
                    last = position == len(units) - 1
                    power = exponent if last else 1
                    terms.append((unit, abs(power), (stack[-1] or pending) != (power < 0)))
                pending = False
        if len(stack) != 1:
            raise _UnitError("unbalanced brackets in unit")
        form = "one" if count is not None and count.replace(_THIN, "") == "1" else "other"
        numerator = [self.unit_word(unit, power, form if position == 0 else "one") for position, (unit, power, den) in enumerate([t for t in terms if not t[2]])]
        denominator = [self.unit_word(unit, power, "one") for unit, power, den in terms if den]
        spoken = list(numerator)
        if denominator:
            spoken.append(self.words["per"] + " " + f" {self.words['and']} ".join(denominator))
        return " ".join(spoken)

    def unit_word(self, unit: str, power: int, form: str) -> str:
        if unit in self.abbreviations:
            return self.abbreviations[unit]
        entry = self.units[unit]
        special = entry.get("powers", {}).get(str(power))
        if special:
            return _plural(special[self.language], form)
        word = _plural(entry[self.language], form)
        if power == 1:
            return word
        phrase = self.powers.get(str(power))
        return f"{word} {phrase}" if phrase else f"{word} {self.words['power'].replace('{}', str(power))}"


@lru_cache(maxsize=None)
def _speaker(language: str) -> _Speaker:
    return _Speaker(require_language(language))


@lru_cache(maxsize=8192)
def spoken_math(src: str, language: str) -> str:
    """🎤 Spoken form of one ``$…$`` body; raises ``SpeechError`` for TeX that cannot be spoken."""
    try:
        return _speaker(language).sequence(_tex.parse_tex(src))
    except _tex.TexError as error:
        raise SpeechError(str(error)) from None
    except SpeechError as error:
        raise SpeechError(f"{error} in {src!r}") from None


def spoken_text(text: str, language: str) -> str:
    """🗨️ A whole clause as the voice reads it: prose unchanged, every ``$…$`` segment spoken."""
    try:
        segments = _tex.split_segments(text)
    except _tex.TexError as error:
        raise SpeechError(str(error)) from None
    return re.sub(r"[ \t]{2,}", " ", "".join(content if kind == "text" else spoken_math(content, language) for kind, content in segments)).strip()


def spoken_clauses(narration, *, language: str, source: str | None = None) -> list[str]:
    """🎙️ Spoken text of every clause in ``language``; errors name the file and clause key."""
    require_language(language)
    column = {"en": 1, "de": 2}[language]
    spoken = []
    for clause in narration:
        try:
            spoken.append(spoken_text(clause[column], language))
        except SpeechError as error:
            raise SpeechError(f"{source or 'narration'}: clause {clause[0]!r} ({language}): {error}") from None
    return spoken


def forbidden_findings(text: str) -> list[tuple[str, str, str]]:
    """🛑 Spelled-out symbol names and bare Unicode math letters outside ``$…$``: ``(rule, match, hint)``."""
    rules = LEXICON["forbidden"]
    outside = " ".join(content for kind, content in _tex.split_segments(text) if kind == "text")
    findings = [(rule["id"], match.group(0), rule["hint"]) for rule in rules["patterns"] for match in re.finditer(rule["pattern"], outside, re.IGNORECASE)]
    findings += [("math-letter", match.group(0), "put Greek letters inside $…$") for match in re.finditer(rules["characters"], outside)]
    return findings
