"""🧮 Inline TeX subset of narration clauses: one parser, one deterministic plain-text rendering.

A clause may contain ``$…$`` segments written in the TeX subset below. The same node tree
feeds the caption typesetter, the plain-text subtitle rendering (``to_unicode``) and the
spoken lexicon, so a symbol is defined exactly once.

Supported: ``x_{i}`` ``x^{2}`` ``\\frac{a}{b}`` ``\\dot{Q}`` ``\\sqrt{x}`` ``\\mathrm{kWh}`` ``\\text{words}``
``\\textcolor{#HEX}{…}`` ``\\left`` ``\\right``, the spaces ``\\, \\; \\quad \\!``, ``{,}`` as decimal comma
and every symbol in ``🔣️.json``.

https://www.unicode.org/charts/PDF/U0300.pdf
"""

from __future__ import annotations

import json
from functools import lru_cache
from pathlib import Path

_DATA = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
SYMBOLS: dict[str, dict] = _DATA["symbols"]
SPACES: dict[str, dict] = _DATA["spaces"]
_CHAR_TO_SYMBOL: dict[str, str] = {}
for _name, _entry in SYMBOLS.items():
    if "alias" not in _entry and _name not in {"sum", "circ"}:
        _CHAR_TO_SYMBOL.setdefault(_entry["char"], _name)
_ASCII_CHARACTERS = frozenset("+=<>/()[]|,;:.!?")
_UNICODE_CHARACTERS = frozenset("−")
_SUPERSCRIPTS = dict(zip("0123456789+−()", "⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁽⁾"))
_COMBINING_DOT = "̇"
_THIN = " "
_BINARY = frozenset("+−")
_RELATIONS = frozenset("=<>")
_OPENERS = frozenset("([")


class TexError(ValueError):
    """🚨 Raised for TeX the narration subset cannot represent; never swallowed."""


def split_segments(text: str) -> list[tuple[str, str]]:
    """✂️ Split a clause into ``("text", …)`` and ``("math", …)`` segments; ``\\$`` is a literal dollar."""
    segments: list[tuple[str, str]] = []
    buffer: list[str] = []
    math = False
    index = 0
    while index < len(text):
        char = text[index]
        if char == "\\" and not math and text[index + 1:index + 2] == "$":
            buffer.append("$")
            index += 2
            continue
        if char == "\\" and math:
            buffer.append(text[index:index + 2])
            index += 2
            continue
        if char == "$":
            content = "".join(buffer)
            if math:
                if not content.strip():
                    raise TexError(f"empty math segment in {text!r}")
                segments.append(("math", content))
            elif content:
                segments.append(("text", content))
            buffer = []
            math = not math
            index += 1
            continue
        buffer.append(char)
        index += 1
    if math:
        raise TexError(f"unbalanced $ in {text!r}")
    if buffer:
        segments.append(("text", "".join(buffer)))
    return segments


class _Parser:
    def __init__(self, src: str) -> None:
        self.src = src
        self.pos = 0

    def fail(self, message: str) -> TexError:
        return TexError(f"{message} in {self.src!r}")

    def skip_spaces(self) -> None:
        while self.pos < len(self.src) and self.src[self.pos] in " \t\n":
            self.pos += 1

    def raw_group(self) -> str:
        self.skip_spaces()
        if self.src[self.pos:self.pos + 1] != "{":
            raise self.fail("expected {")
        depth, start = 0, self.pos
        while self.pos < len(self.src):
            if self.src[self.pos] == "\\":
                self.pos += 1
            elif self.src[self.pos] == "{":
                depth += 1
            elif self.src[self.pos] == "}":
                depth -= 1
                if depth == 0:
                    self.pos += 1
                    return self.src[start + 1:self.pos - 1]
            self.pos += 1
        raise self.fail("unbalanced braces")

    def argument(self) -> tuple:
        self.skip_spaces()
        if self.pos >= len(self.src):
            raise self.fail("missing argument")
        if self.src[self.pos] == "{":
            self.pos += 1
            inner = self.sequence(closing=True)
            self.expect_close()
            return inner
        node = self.item(single=True)
        if node is None:
            raise self.fail("missing argument")
        return (node,)

    def expect_close(self) -> None:
        if self.src[self.pos:self.pos + 1] != "}":
            raise self.fail("unbalanced braces")
        self.pos += 1

    def sequence(self, *, closing: bool) -> tuple:
        nodes: list = []
        while self.pos < len(self.src):
            char = self.src[self.pos]
            if char == "}":
                if closing:
                    break
                raise self.fail("unbalanced braces")
            if char in "_^":
                self.pos += 1
                script = self.argument()
                base = nodes.pop() if nodes and nodes[-1][0] != "space" else None
                if base is not None and base[0] == "group" and not base[1]:
                    base = None
                if base is not None and base[0] == "scripts" and base[2 if char == "_" else 3] is None:
                    sub, sup = (script, base[3]) if char == "_" else (base[2], script)
                    nodes.append(("scripts", base[1], sub, sup))
                else:
                    nodes.append(("scripts", base, script if char == "_" else None, script if char == "^" else None))
                continue
            node = self.item()
            if node is not None:
                nodes.append(node)
        return _tidy(nodes)

    def number(self, single: bool) -> tuple:
        src, raw = self.src, ""
        while True:
            start = self.pos
            while self.pos < len(src) and src[self.pos].isdigit() and src[self.pos].isascii():
                self.pos += 1
                if single:
                    break
            raw += src[start:self.pos]
            if single:
                break
            tail = src[self.pos:]
            after = lambda offset: src[self.pos + offset:self.pos + offset + 1].isdigit()
            if tail.startswith("{,}") and after(3):
                raw, self.pos = raw + ",", self.pos + 3
            elif tail.startswith(",") and after(1):
                raw, self.pos = raw + ",", self.pos + 1
            elif tail.startswith(".") and after(1):
                raw, self.pos = raw + ".", self.pos + 1
            elif tail.startswith("\\,") and "," not in raw and "." not in raw and src[self.pos + 2:self.pos + 5].isdigit() and len(src[self.pos + 2:self.pos + 5]) == 3:
                raw, self.pos = raw + _THIN, self.pos + 2
            else:
                break
        return ("num", raw)

    def item(self, *, single: bool = False):
        char = self.src[self.pos]
        if char in " \t\n":
            self.pos += 1
            return None
        if char == "{":
            self.pos += 1
            inner = self.sequence(closing=True)
            self.expect_close()
            return ("group", inner)
        if char == "\\":
            return self.command()
        if char.isascii() and char.isdigit():
            return self.number(single)
        self.pos += 1
        if char.isascii() and char.isalpha():
            return ("char", char)
        if char == "-":
            return ("char", "−")
        if char in _ASCII_CHARACTERS or char in _UNICODE_CHARACTERS:
            return ("char", char)
        if char == "°":
            return ("deg",)
        if char in _CHAR_TO_SYMBOL:
            return ("sym", _CHAR_TO_SYMBOL[char])
        raise self.fail(f"unsupported character {char!r}")

    def command(self) -> tuple | None:
        self.pos += 1
        if self.pos >= len(self.src):
            raise self.fail("dangling backslash")
        if not self.src[self.pos].isalpha():
            char = self.src[self.pos]
            self.pos += 1
            if char in SPACES:
                return ("space", char)
            if char == " ":
                return None
            if char in "%{}_&#":
                return ("char", char)
            raise self.fail(f"unsupported TeX command \\{char}")
        start = self.pos
        while self.pos < len(self.src) and self.src[self.pos].isalpha() and self.src[self.pos].isascii():
            self.pos += 1
        name = self.src[start:self.pos]
        if name in {"frac", "dfrac", "tfrac"}:
            return ("frac", self.argument(), self.argument())
        if name == "dot":
            return ("dot", self.argument())
        if name == "sqrt":
            return ("sqrt", self.argument())
        if name in {"mathrm", "textrm"}:
            return ("mathrm", self.argument())
        if name == "text":
            text = self.raw_group()
            if any(char in text for char in "\\${}_^"):
                raise self.fail(f"\\text takes plain words only: {text!r}")
            return ("text", text)
        if name == "textcolor":
            self.raw_group()
            return ("group", self.argument())
        if name in {"left", "right"}:
            if self.src[self.pos:self.pos + 1] == ".":
                self.pos += 1
            return None
        if name == "quad":
            return ("space", "quad")
        if name in SYMBOLS:
            return ("sym", SYMBOLS[name].get("alias", name))
        raise self.fail(f"unsupported TeX command \\{name}")


def _tidy(nodes: list) -> tuple:
    """🧹 Turn a lone raised ring into a degree node and fuse ``°`` with a following C or F into one unit."""
    merged: list = []
    index = 0
    while index < len(nodes):
        node = nodes[index]
        if node[0] == "scripts" and node[1] is None and node[2] is None and node[3] in {(("sym", "circ"),), (("deg",),)}:
            node = ("deg",)
        if node == ("deg",) and index + 1 < len(nodes):
            follower = nodes[index + 1]
            if follower[0] == "mathrm":
                merged.append(("mathrm", (("deg",), *follower[1])))
                index += 2
                continue
            if follower[0] == "char" and follower[1] in {"C", "F"}:
                merged.append(("mathrm", (("deg",), follower)))
                index += 2
                continue
        merged.append(node)
        index += 1
    return tuple(merged)


@lru_cache(maxsize=4096)
def parse_tex(src: str) -> tuple:
    """🌳 Parse one math segment into an immutable node tree; raises ``TexError`` on anything unsupported."""
    parser = _Parser(src)
    nodes = parser.sequence(closing=False)
    if not nodes:
        raise TexError(f"empty math in {src!r}")
    return nodes


def canon(nodes) -> str:
    """🔑 Flat canonical TeX of a node sequence: no spaces, no ``\\mathrm`` wrappers, scripts always braced."""
    parts: list[str] = []
    for node in nodes:
        kind = node[0]
        if kind in {"char", "num"}:
            parts.append(node[1])
        elif kind == "sym":
            parts.append("\\" + node[1])
        elif kind == "deg":
            parts.append("\\circ")
        elif kind in {"group", "mathrm"}:
            parts.append(canon(node[1]))
        elif kind == "text":
            parts.append("\\text{" + node[1] + "}")
        elif kind == "frac":
            parts.append("\\frac{" + canon(node[1]) + "}{" + canon(node[2]) + "}")
        elif kind in {"dot", "sqrt"}:
            parts.append("\\" + kind + "{" + canon(node[1]) + "}")
        elif kind == "scripts":
            parts.append((canon((node[1],)) if node[1] is not None else "") + ("_{" + canon(node[2]) + "}" if node[2] is not None else "") + ("^{" + canon(node[3]) + "}" if node[3] is not None else ""))
    return "".join(parts)


def _is_letterlike(node) -> bool:
    return node[0] in {"sym", "scripts", "dot"} or (node[0] == "char" and node[1].isalpha()) or node[0] == "num"


def atom_count(nodes, *, upright: bool = False) -> int:
    """⚛️ Count operands in a sequence: an upright letter run is one atom, italic letters are one each."""
    count, in_run = 0, False
    for node in nodes:
        if node[0] == "mathrm":
            count += atom_count(node[1], upright=True)
            in_run = False
        elif node[0] == "group":
            count += atom_count(node[1], upright=upright)
            in_run = False
        elif node[0] == "char" and node[1].isalpha() and upright:
            count += 0 if in_run else 1
            in_run = True
        else:
            count += 1
            in_run = False
    return count


def has_operator(nodes) -> bool:
    for node in nodes:
        if node[0] in {"char"} and node[1] in _BINARY | _RELATIONS:
            return True
        if node[0] == "sym" and SYMBOLS[node[1]]["kind"] in {"binary", "relation"}:
            return True
        if node[0] in {"group", "mathrm"} and has_operator(node[1]):
            return True
    return False


def _script_text(text: str, mapping: dict[str, str] | None, marker: str) -> str:
    if mapping and text and all(char in mapping for char in text):
        return "".join(mapping[char] for char in text)
    plain = text.replace(_THIN, "").replace(" ", "")
    return marker + (plain if plain.isalnum() or len(plain) == 1 else "(" + plain + ")")


class _Uni:
    def run(self, nodes, *, upright: bool = False, scripted: bool = False) -> str:
        out, previous = "", None
        plain = upright or scripted
        for node in nodes:
            text, role = self.node(node, upright, scripted)
            if plain and role in {"binary", "relation"}:
                role = "plain"
            if role == "binary" and previous not in {None, "binary", "relation", "open", "comma"}:
                out = out.rstrip() + " " + text + " "
            elif role == "relation":
                out = (out.rstrip() + " " if out else "") + text + " "
            else:
                out += text
            if node[0] != "space":
                previous = role
        return out.strip()

    def node(self, node, upright: bool, scripted: bool) -> tuple[str, str]:
        kind = node[0]
        if kind == "char":
            char = node[1]
            if char in _BINARY:
                return char, "binary"
            if char in _RELATIONS:
                return char, "relation"
            if char in _OPENERS:
                return char, "open"
            if char in ",;":
                return (char, "comma") if scripted else (char + " ", "comma")
            return char, "plain"
        if kind == "sym":
            entry = SYMBOLS[node[1]]
            return entry["char"], entry["kind"] if entry["kind"] in {"binary", "relation"} else "plain"
        if kind == "num":
            return node[1], "plain"
        if kind == "deg":
            return "°", "plain"
        if kind == "space":
            return SPACES[node[1]]["char"], "plain"
        if kind == "text":
            return node[1], "plain"
        if kind == "group":
            return self.run(node[1], upright=upright, scripted=scripted), "plain"
        if kind == "mathrm":
            return self.run(node[1], upright=True, scripted=scripted), "plain"
        if kind == "frac":
            return self.frac(node, upright), "plain"
        if kind == "dot":
            return self.dot(node, upright), "plain"
        if kind == "sqrt":
            inner = self.run(node[1], upright=upright)
            single = atom_count(node[1], upright=upright) == 1 and not has_operator(node[1])
            return "√" + (inner if single else "(" + inner + ")"), "plain"
        if kind == "scripts":
            return self.scripts(node, upright), "plain"
        raise TexError(f"unknown node {kind!r}")

    def operand(self, nodes, upright: bool) -> str:
        text = self.run(nodes, upright=upright)
        if atom_count(nodes, upright=upright) > 1 or has_operator(nodes):
            return "(" + text + ")"
        return text

    def frac(self, node, upright: bool) -> str:
        return self.operand(node[1], upright) + "/" + self.operand(node[2], upright)

    def dot(self, node, upright: bool) -> str:
        inner = self.run(node[1], upright=upright)
        if len(inner) != 1:
            raise TexError(f"\\dot needs a single symbol, got {inner!r}")
        return inner + _COMBINING_DOT

    def scripts(self, node, upright: bool) -> str:
        base = self.run((node[1],), upright=upright) if node[1] is not None else ""
        out = base
        if node[2] is not None:
            out += _script_text(self.run(node[2], upright=upright, scripted=True), None, "_")
        if node[3] is not None:
            if node[3] in {(("sym", "circ"),), (("deg",),)}:
                out += "°"
            else:
                out += _script_text(self.run(node[3], upright=upright, scripted=True), _SUPERSCRIPTS, "^")
        return out


def to_unicode(nodes) -> str:
    """🔤 Deterministic plain text of a node tree: Greek letters, ``Q̇``, ``m³``, ``h⁻¹``, ``x_i`` and ``a/(b c)``.

    Superscripts made of digits, signs and parentheses use Unicode superscripts, every
    other script stays ASCII-marked (``Q̇_V``, ``R_si``, ``x^n``) so the output never mixes glyph families.
    """
    return _Uni().run(nodes)


def math_to_unicode(src: str) -> str:
    """🔠 Plain text of one ``$…$`` body."""
    return to_unicode(parse_tex(src))


def plain_text(text: str) -> str:
    """📄 A whole clause as plain text: ``$…$`` segments become Unicode, nothing raw TeX ever remains."""
    return "".join(content if kind == "text" else math_to_unicode(content) for kind, content in split_segments(text))
