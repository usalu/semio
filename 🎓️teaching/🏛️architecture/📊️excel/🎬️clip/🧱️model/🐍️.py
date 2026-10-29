"""📊 Schema-first Excel teaching engine: A1 references, copy translation, formulas, events, en/de text."""

from __future__ import annotations

import copy
import json
import math
import re
from dataclasses import dataclass, field, replace
from decimal import ROUND_HALF_UP, Decimal, localcontext
from functools import lru_cache
from pathlib import Path

LANGUAGES = ("en", "de")
MAX_COL = 16384
MAX_ROW = 1048576
ERROR_CODES = ("#DIV/0!", "#REF!", "#NAME?", "#VALUE!", "#N/A", "#SPILL!", "#NUM!", "#NULL!")
DE_ERRORS = {"#DIV/0!": "#DIV/0!", "#REF!": "#BEZUG!", "#NAME?": "#NAME?", "#VALUE!": "#WERT!", "#N/A": "#NV", "#SPILL!": "#ÜBERLAUF!", "#NUM!": "#ZAHL!", "#NULL!": "#NULL!"}
DE_FUNCTIONS = {
    "SUM": "SUMME", "SUMPRODUCT": "SUMMENPRODUKT", "COUNT": "ANZAHL", "COUNTA": "ANZAHL2", "COUNTIF": "ZÄHLENWENN",
    "COUNTIFS": "ZÄHLENWENNS", "SUMIF": "SUMMEWENN", "SUMIFS": "SUMMEWENNS", "AVERAGE": "MITTELWERT", "MIN": "MIN",
    "MAX": "MAX", "ROUND": "RUNDEN", "IF": "WENN", "IFERROR": "WENNFEHLER", "XLOOKUP": "XVERWEIS", "INDEX": "INDEX",
    "MATCH": "VERGLEICH", "PI": "PI", "AND": "UND", "OR": "ODER", "ROWS": "ZEILEN", "COLUMNS": "SPALTEN",
}
EN_FUNCTIONS = {german: english for english, german in DE_FUNCTIONS.items()}
DE_SPECIALS = {"#headers": "#Kopfzeilen", "#all": "#Alle", "#data": "#Daten", "#this row": "#Diese Zeile", "#totals": "#Ergebnisse"}
EN_SPECIALS = {german.casefold(): {"#headers": "#Headers", "#all": "#All", "#data": "#Data", "#this row": "#This Row", "#totals": "#Totals"}[english] for english, german in DE_SPECIALS.items()}
DE_BOOLEANS = {"TRUE": "WAHR", "FALSE": "FALSCH"}
EN_BOOLEANS = {german: english for english, german in DE_BOOLEANS.items()}
EN_ERRORS = {german: english for english, german in DE_ERRORS.items()}


class FormulaError(ValueError):
    """⚠️ A formula text that Excel would refuse to accept."""


@dataclass(frozen=True)
class XlError:
    """🚩 An Excel error value such as #DIV/0!."""

    code: str

    def __str__(self):
        return self.code


ERR = {code: XlError(code) for code in ERROR_CODES}


class Signal(Exception):
    """🚦 Carries an error value up to the nearest cell or IFERROR."""

    def __init__(self, error: XlError):
        super().__init__(error.code)
        self.error = error


def fail(code: str):
    raise Signal(ERR[code])


@dataclass(frozen=True)
class Matrix:
    """🔢 A rectangular block of values produced by a range or an array operation."""

    rows: tuple

    @property
    def h(self):
        return len(self.rows)

    @property
    def w(self):
        return len(self.rows[0]) if self.rows else 0

    def flat(self):
        return [value for row in self.rows for value in row]


def column_number(letters: str) -> int:
    """🔤 Column letters to number: A=1, Z=26, AA=27 ([openpyxl](https://openpyxl.readthedocs.io/en/stable/api/openpyxl.utils.cell.html))."""
    number = 0
    for char in letters.upper():
        number = number * 26 + ord(char) - 64
    return number


def column_letters(number: int) -> str:
    """🔡 Column number to letters."""
    letters = ""
    while number:
        number, rest = divmod(number - 1, 26)
        letters = chr(65 + rest) + letters
    return letters


def split_address(address: str) -> tuple[int, int]:
    """📮 'B12' or 'Sheet!$B$12' to (column, row)."""
    match = re.fullmatch(r"(?:.*!)?\$?([A-Za-z]{1,3})\$?(\d+)", address)
    if not match or not (1 <= column_number(match[1]) <= MAX_COL and 1 <= int(match[2]) <= MAX_ROW):
        raise FormulaError("Not a cell address: " + address)
    return column_number(match[1]), int(match[2])


def address(col: int, row: int) -> str:
    """📮 (column, row) to 'B12'."""
    return column_letters(col) + str(row)


@dataclass(frozen=True)
class Ref:
    """📍 One A1 cell with its dollar flags and optional sheet span; col or row is None inside whole row/column ranges."""

    col: int | None
    row: int | None
    col_abs: bool = False
    row_abs: bool = False
    sheet: str | None = None
    sheet_end: str | None = None


@dataclass(frozen=True)
class Range:
    """🧱 A rectangular reference between two corners, possibly a whole column, a whole row or a 3D span."""

    start: Ref
    end: Ref
    sheet: str | None = None
    sheet_end: str | None = None


_SHEET = r"(?:'(?:[^']|'')+'|[^\W\d][\w.]*(?::[^\W\d][\w.]*)?)"
_CELL = r"\$?[A-Za-z]{1,3}\$?\d+"
_BODY = rf"(?:{_CELL}(?::{_CELL})?|\$?[A-Za-z]{{1,3}}:\$?[A-Za-z]{{1,3}}|\$?\d+:\$?\d+)"
_REF = re.compile(rf"(?:(?P<sheet>{_SHEET})!)?(?P<body>{_BODY})(?![\w(])")
_PLAIN_SHEET = re.compile(r"[^\W\d][\w.]*\Z")
_LOOKS_LIKE_REF = re.compile(r"(?:[A-Za-z]{1,3}\d+|[Rr]\d*[Cc]\d*|TRUE|FALSE)\Z", re.I)


def quote_sheet(name: str) -> str:
    """💬 Quote a sheet name only when Excel requires it: spaces, symbols, leading digits, or a look-alike of a cell."""
    if _PLAIN_SHEET.match(name) and not _LOOKS_LIKE_REF.match(name):
        return name
    return "'" + name.replace("'", "''") + "'"


def _part(text: str) -> Ref:
    cell = re.fullmatch(r"(\$?)([A-Za-z]{1,3})(\$?)(\d+)", text)
    if cell:
        return Ref(column_number(cell[2]), int(cell[4]), bool(cell[1]), bool(cell[3]))
    letters = re.fullmatch(r"(\$?)([A-Za-z]{1,3})", text)
    if letters:
        return Ref(column_number(letters[2]), None, bool(letters[1]), False)
    digits = re.fullmatch(r"(\$?)(\d+)", text)
    return Ref(None, int(digits[2]), False, bool(digits[1]))


def _valid(ref: Ref) -> bool:
    return (ref.col is None or 1 <= ref.col <= MAX_COL) and (ref.row is None or 1 <= ref.row <= MAX_ROW)


def _sheets(prefix: str | None) -> tuple[str | None, str | None]:
    if prefix is None:
        return None, None
    inner = prefix[1:-1].replace("''", "'") if prefix.startswith("'") else prefix
    first, _, last = inner.partition(":")
    return first, last or None


def _from_match(match) -> Ref | Range | None:
    sheet, sheet_end = _sheets(match["sheet"])
    corners = [_part(piece) for piece in match["body"].split(":")]
    if not all(_valid(corner) for corner in corners):
        return None
    if len(corners) == 1:
        return replace(corners[0], sheet=sheet, sheet_end=sheet_end)
    return Range(corners[0], corners[1], sheet, sheet_end)


def parse_ref(text: str) -> Ref | Range:
    """🔍 Parse "'Type A'!$B$3", "Climate!B2:B4", "B:B", "3:3" or "'Type A:Type C'!B12" into Ref or Range."""
    match = _REF.fullmatch(text.strip())
    result = _from_match(match) if match else None
    if result is None:
        raise FormulaError("Not a reference: " + text)
    return result


_F4 = ((False, False), (True, True), (False, True), (True, False))


def cycle_reference(text: str) -> str:
    """⌨️ F4: A1 to $A$1 to A$1 to $A1 and back; ranges cycle both corners together, the sheet prefix stays."""
    ref = parse_ref(text)
    start = _box(ref)[0]
    col_abs, row_abs = _F4[(_F4.index((start.col_abs, start.row_abs)) + 1) % 4]
    flip = lambda part: replace(part, col_abs=col_abs if part.col is not None else False, row_abs=row_abs if part.row is not None else False)
    if isinstance(ref, Range):
        return format_ref(replace(ref, start=flip(ref.start), end=flip(ref.end)))
    return format_ref(flip(ref))


def _format_part(ref: Ref) -> str:
    column = ("$" if ref.col_abs else "") + column_letters(ref.col) if ref.col is not None else ""
    row = ("$" if ref.row_abs else "") + str(ref.row) if ref.row is not None else ""
    return column + row


def format_ref(ref: Ref | Range) -> str:
    """🖨️ Print a reference in canonical A1 form with Excel's sheet quoting rules."""
    prefix = ""
    if ref.sheet is not None:
        names = ref.sheet if ref.sheet_end is None else ref.sheet + ":" + ref.sheet_end
        quoted = "'" + names.replace("'", "''") + "'" if any(quote_sheet(part) != part for part in names.split(":")) else names
        prefix = quoted + "!"
    if isinstance(ref, Range):
        return prefix + _format_part(ref.start) + ":" + _format_part(ref.end)
    return prefix + _format_part(ref)


_NUMBER = {"en": re.compile(r"(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?"), "de": re.compile(r"(?:\d+(?:,\d*)?|,\d+)(?:[eE][+-]?\d+)?")}
_STRING = re.compile(r'"(?:[^"]|"")*"')
_ERROR = re.compile("#(?:" + "|".join(re.escape(code[1:]) for code in sorted(set(ERROR_CODES) | set(DE_ERRORS.values()), key=len, reverse=True)) + ")")
_NAME = re.compile(r"[^\W\d][\w.]*")
_TABLE = re.compile(r"[^\W\d][\w.]*(?:\[(?:[^\[\]]|\[[^\]]*\])*\])+")
_SPACE = re.compile(r"\s+")
_BRACKET = re.compile(r"\[(?:[^\[\]]|\[[^\]]*\])*\]")


@dataclass(frozen=True)
class Token:
    """🧩 One lexical piece of a formula with its source offset."""

    kind: str
    text: str
    start: int


def tokenize(text: str, language: str = "en") -> list[Token]:
    """✂️ Split formula text into tokens; language "de" reads `;` as separator and `,` as decimal mark."""
    separator = ";" if language == "de" else ","
    tokens: list[Token] = []
    index = 0
    while index < len(text):
        char = text[index]
        match = None
        kind = None
        if char.isspace():
            match, kind = _SPACE.match(text, index), "space"
        elif char == '"':
            match, kind = _STRING.match(text, index), "str"
            if not match:
                raise FormulaError("Unterminated string")
        elif char == "#":
            match, kind = _ERROR.match(text, index), "err"
            if not match:
                raise FormulaError("Unknown error literal at " + str(index))
        if match is None:
            candidate = _REF.match(text, index)
            if candidate and _from_match(candidate) is not None:
                match, kind = candidate, "ref"
                candidate = None
        if match is None and (char.isdigit() or char == "." or (language == "de" and char == ",")):
            match, kind = _NUMBER[language].match(text, index), "num"
        if match is None and char == "[":
            match, kind = _BRACKET.match(text, index), "table"
        if match is None and (char.isalpha() or char in "_\\"):
            match = _TABLE.match(text, index)
            kind = "table"
            if match is None:
                match = _NAME.match(text, index)
                kind = None
            kind = kind or ("func" if text[match.end():match.end() + 1] == "(" else "name")
            if kind == "name" and match[0].upper() in {*DE_BOOLEANS, *EN_BOOLEANS} and (language == "de" or match[0].upper() in DE_BOOLEANS):
                kind = "bool"
        if match is not None:
            tokens.append(Token(kind, match[0], index))
            index = match.end()
            continue
        pair = text[index:index + 2]
        if pair in ("<>", "<=", ">="):
            tokens.append(Token("op", pair, index))
            index += 2
        elif char in "+-*/^&=<>%:":
            tokens.append(Token("op", char, index))
            index += 1
        elif char == "(":
            tokens.append(Token("open", char, index))
            index += 1
        elif char == ")":
            tokens.append(Token("close", char, index))
            index += 1
        elif char == separator:
            tokens.append(Token("sep", char, index))
            index += 1
        else:
            raise FormulaError("Unexpected character " + repr(char) + " at " + str(index))
    return tokens


@dataclass(frozen=True)
class Node:
    """🌳 Parse tree node: num str bool err ref name call neg pos pct bin union colon empty."""

    kind: str
    value: object = None
    args: tuple = ()

    def data(self):
        kind = self.kind
        if kind in ("num", "str", "bool", "name", "err", "table"):
            return [kind, self.value]
        if kind == "ref":
            return ["ref", format_ref(self.value)]
        if kind == "call":
            return ["call", self.value, [arg.data() for arg in self.args]]
        if kind == "bin":
            return ["bin", self.value, *[arg.data() for arg in self.args]]
        if kind == "union":
            return ["union", [arg.data() for arg in self.args]]
        return [kind, *[arg.data() for arg in self.args]]


class _Parser:
    LEVELS = (("=", "<>", "<", ">", "<=", ">="), ("&",), ("+", "-"), ("*", "/"), ("^",))

    def __init__(self, tokens, language):
        self.tokens = [token for token in tokens if token.kind != "space"]
        self.index = 0
        self.language = language

    def peek(self):
        return self.tokens[self.index] if self.index < len(self.tokens) else None

    def take(self):
        token = self.peek()
        if token is None:
            raise FormulaError("Unexpected end of formula")
        self.index += 1
        return token

    def at(self, kind, text=None):
        token = self.peek()
        return token is not None and token.kind == kind and (text is None or token.text == text)

    def binary(self, level=0):
        if level == len(self.LEVELS):
            return self.percent()
        left = self.binary(level + 1)
        while self.peek() and self.peek().kind == "op" and self.peek().text in self.LEVELS[level]:
            operator = self.take().text
            left = Node("bin", operator, (left, self.binary(level + 1)))
        return left

    def percent(self):
        node = self.unary()
        while self.at("op", "%"):
            self.take()
            node = Node("pct", None, (node,))
        return node

    def unary(self):
        if self.at("op", "-") or self.at("op", "+"):
            return Node("neg" if self.take().text == "-" else "pos", None, (self.unary(),))
        return self.colon()

    def colon(self):
        node = self.primary()
        while self.at("op", ":"):
            self.take()
            node = _span(node, self.primary())
        return node

    def primary(self):
        token = self.take()
        kind = token.kind
        if kind == "num":
            return Node("num", float(token.text.replace(",", ".")) if self.language == "de" else float(token.text))
        if kind == "str":
            return Node("str", token.text[1:-1].replace('""', '"'))
        if kind == "bool":
            return Node("bool", (EN_BOOLEANS.get(token.text.upper(), token.text.upper()) if self.language == "de" else token.text.upper()) == "TRUE")
        if kind == "err":
            return Node("err", EN_ERRORS.get(token.text, token.text) if self.language == "de" else token.text)
        if kind == "ref":
            return Node("ref", parse_ref(token.text))
        if kind == "name":
            return Node("name", token.text)
        if kind == "table":
            return Node("table", token.text)
        if kind == "func":
            name = token.text.upper()
            if self.language == "de":
                name = EN_FUNCTIONS[name] if name in EN_FUNCTIONS else "?" + name if DE_FUNCTIONS.get(name, "") not in ("", name) else name
            self.take()
            return Node("call", name, self.arguments())
        if kind == "open":
            members = [self.binary()]
            while self.at("sep"):
                self.take()
                members.append(self.binary())
            self.close()
            return members[0] if len(members) == 1 else Node("union", None, tuple(members))
        raise FormulaError("Unexpected token " + token.text)

    def close(self):
        if not self.at("close"):
            raise FormulaError("Missing closing parenthesis")
        self.take()

    def arguments(self):
        args = []
        if self.at("close"):
            self.take()
            return ()
        while True:
            args.append(Node("empty") if self.at("sep") or self.at("close") else self.binary())
            if self.at("sep"):
                self.take()
                continue
            self.close()
            return tuple(args)


def _box(ref):
    if isinstance(ref, Ref):
        return ref, ref
    return ref.start, ref.end


def _span(left: Node, right: Node) -> Node:
    if left.kind == "ref" and right.kind == "ref":
        first, last = _box(left.value)[0], _box(right.value)[1]
        if left.value.sheet == right.value.sheet or right.value.sheet is None:
            return Node("ref", Range(first, last, left.value.sheet, left.value.sheet_end))
    return Node("colon", None, (left, right))


@lru_cache(maxsize=2048)
def parse(formula: str, language: str = "en") -> Node:
    """🌲 Parse formula text (leading `=` optional) with Excel operator precedence: `-2^2` is 4, `2^3^2` is 64, `50%` is 0.5."""
    text = formula.strip()
    text = text[1:] if text.startswith("=") else text
    parser = _Parser(tokenize(text, language), language)
    node = parser.binary()
    if parser.peek() is not None:
        raise FormulaError("Unexpected token " + parser.peek().text)
    return node


def _rewrite(formula: str, change) -> str:
    out = []
    for token in tokenize(formula):
        if token.kind == "ref":
            replacement = change(parse_ref(token.text))
            out.append(token.text if replacement is None else replacement)
        else:
            out.append(token.text)
    return "".join(out)


def reference_spans(formula: str, wb: "Workbook | None" = None, sheet: str | None = None, cell: str | None = None) -> list[tuple[int, int, Ref | Range]]:
    """🖍️ Where each reference sits in the formula text, for coloured reference frames: (start, end, reference); with `wb` (and the host sheet/cell for `@`) structured references like `Capsules[Type]` resolve to their A1 Range."""
    spans = []
    for token in tokenize(formula):
        if token.kind == "ref":
            item = parse_ref(token.text)
        elif token.kind == "table" and wb is not None:
            try:
                item = structured_range(wb, token.text, sheet, cell)
            except Signal:
                continue
        else:
            continue
        spans.append((token.start, token.start + len(token.text), item))
    return spans


def references(formula: str, wb: "Workbook | None" = None, sheet: str | None = None, cell: str | None = None) -> list[Ref | Range]:
    """🧭 All references of a formula in reading order."""
    return [item[2] for item in reference_spans(formula, wb, sheet, cell)]


def _move(ref: Ref, dc: int, dr: int) -> Ref | None:
    col = ref.col if ref.col is None or ref.col_abs else ref.col + dc
    row = ref.row if ref.row is None or ref.row_abs else ref.row + dr
    moved = replace(ref, col=col, row=row)
    return moved if _valid(moved) else None


def translate(formula: str, source: str, target: str) -> str:
    """📋 Copy/fill rule ([openpyxl Translator](https://openpyxl.readthedocs.io/en/stable/api/openpyxl.formula.translate.html)): relative parts shift by the offset, `$` parts stay, off-grid becomes #REF!."""
    (c0, r0), (c1, r1) = split_address(source), split_address(target)

    def change(item):
        if isinstance(item, Range):
            start, end = _move(item.start, c1 - c0, r1 - r0), _move(item.end, c1 - c0, r1 - r0)
            return "#REF!" if start is None or end is None else format_ref(replace(item, start=start, end=end))
        moved = _move(item, c1 - c0, r1 - r0)
        return "#REF!" if moved is None else format_ref(moved)

    return _rewrite(formula, change)


@dataclass(frozen=True)
class Cell:
    """🟩 A constant (value) or a formula (with leading `=`), a number format and optional bilingual labels."""

    value: object = None
    formula: str | None = None
    format: str | None = None
    labels: dict | None = None


@dataclass(frozen=True, eq=False)
class Sheet:
    """📄 A named worksheet (canonical English name, optional bilingual tab labels); cells are keyed by A1 address."""

    name: str
    cells: dict
    widths: dict
    labels: dict | None = None


@dataclass(frozen=True)
class Table:
    """🗃️ An Excel table: canonical English name, optional bilingual name labels (Capsules, Kapseln), top-left header cell, bilingual column headers, number of data rows."""

    name: str
    sheet: str
    col: int
    row: int
    headers: tuple
    nrows: int
    labels: dict | None = None


@dataclass(frozen=True, eq=False)
class Workbook:
    """📗 Ordered worksheets plus defined names (name to refers-to reference text) and tables."""

    sheets: tuple
    names: dict
    file: str = "Nakagin.xlsx"
    tables: dict = field(default_factory=dict)

    def table(self, name: str) -> Table | None:
        folded = name.casefold()
        return next((table for key, table in self.tables.items() if folded == key.casefold() or folded in {label.casefold() for label in (table.labels or {}).values()}), None)

    def table_at(self, sheet: str, col: int, row: int) -> Table | None:
        return next((t for t in self.tables.values() if t.sheet.casefold() == sheet.casefold() and t.col <= col < t.col + len(t.headers) and t.row <= row <= t.row + t.nrows), None)

    def sheet(self, name: str) -> Sheet | None:
        folded = name.casefold()
        return next((sheet for sheet in self.sheets if sheet.name.casefold() == folded), None)

    def order(self) -> list[str]:
        return [sheet.name for sheet in self.sheets]

    def cell(self, sheet: str, cell: str) -> Cell | None:
        found = self.sheet(sheet)
        return found.cells.get(cell) if found else None


@lru_cache(maxsize=64)
def _extent(sheet: Sheet) -> tuple[int, int]:
    cols = [split_address(key) for key in sheet.cells]
    return max((c for c, _ in cols), default=1), max((r for _, r in cols), default=1)


def _span_sheets(wb: Workbook, host: str | None, first: str | None, last: str | None) -> list[Sheet]:
    if first is None:
        found = wb.sheet(host) if host else None
        return [found] if found else []
    order = [name.casefold() for name in wb.order()]
    if first.casefold() not in order:
        return []
    start = order.index(first.casefold())
    end = order.index(last.casefold()) if last and last.casefold() in order else start
    if last and last.casefold() not in order:
        return []
    low, high = sorted((start, end))
    return list(wb.sheets[low:high + 1])


def _corners(ref, extent) -> tuple[int, int, int, int]:
    start, end = _box(ref)
    cols = [part.col if part.col is not None else None for part in (start, end)]
    rows = [part.row if part.row is not None else None for part in (start, end)]
    c1, c2 = (1, extent[0]) if cols[0] is None else sorted(cols)
    r1, r2 = (1, extent[1]) if rows[0] is None else sorted(rows)
    return c1, r1, c2, r2


_SPECIAL_ROWS = {"#all": "all", "#data": "data", "#headers": "headers", "#this row": "row", "#totals": "totals"}


def _structured_parts(text: str) -> tuple[str | None, str, list]:
    match = re.fullmatch(r"([^\W\d][\w.]*)?\[(.*)\]", text, re.S)
    name, inner = match[1], match[2]
    if inner.startswith("["):
        span = re.fullmatch(r"\[([^\[\]]*)\]:\[([^\[\]]*)\]", inner)
        groups = list(span.groups()) if span else re.findall(r"\[([^\[\]]*)\]", inner)
    else:
        groups = [re.sub(r"^@\[([^\]]*)\]$", r"@\1", inner)]
    rowspec, columns = "data", []
    for group in groups:
        if group.startswith("@"):
            rowspec = "row"
            if group[1:]:
                columns.append(group[1:])
        elif group.casefold() in _SPECIAL_ROWS:
            rowspec = _SPECIAL_ROWS[group.casefold()]
        else:
            columns.append(group)
    return name, rowspec, columns


def structured_range(wb: Workbook, text: str, sheet: str | None = None, cell: str | None = None) -> Range:
    """🗃️ Resolve `Capsules[Type]`, `Capsules[@Type]`, `Capsules[[#This Row],[Type]]`, `Capsules[#Headers]`, `Capsules[#All]`, `Capsules[[A]:[B]]` or `[@Type]` (inside the table) to an A1 Range; unknown table is #NAME?, unknown column #REF!, this-row outside the table #VALUE!."""
    name, rowspec, columns = _structured_parts(text)
    col, row = split_address(cell) if cell else (0, 0)
    table = wb.table(name) if name else (wb.table_at(sheet, col, row) if sheet and cell else None)
    if table is None:
        fail("#NAME?")
    heads = [[label.casefold() for label in head.values()] for head in table.headers]
    picked = []
    for column in columns:
        found = [index for index, labels in enumerate(heads) if column.casefold() in labels]
        if not found:
            fail("#REF!")
        picked.append(found[0])
    first, last = (min(picked), max(picked)) if picked else (0, len(heads) - 1)
    if rowspec == "data":
        top, bottom = table.row + 1, table.row + table.nrows
    elif rowspec == "headers":
        top = bottom = table.row
    elif rowspec == "all":
        top, bottom = table.row, table.row + table.nrows
    elif rowspec == "row":
        if not cell or sheet.casefold() != table.sheet.casefold() or not table.row < row <= table.row + table.nrows:
            fail("#VALUE!")
        top = bottom = row
    else:
        fail("#REF!")
    if bottom < top:
        fail("#REF!")
    return Range(Ref(table.col + first, top), Ref(table.col + last, bottom), table.sheet)


def _cells_of(wb: Workbook, host: str, node: Node, depth=0, cell: str | None = None) -> list[tuple[str, str]]:
    found = []
    if node.kind == "ref":
        ref = node.value
        for sheet in _span_sheets(wb, host, ref.sheet, ref.sheet_end):
            c1, r1, c2, r2 = _corners(ref, _extent(sheet))
            single = isinstance(ref, Ref)
            for row in range(r1, r2 + 1 if single else min(r2, max(r1, _extent(sheet)[1])) + 1):
                for col in range(c1, c2 + 1 if single else min(c2, max(c1, _extent(sheet)[0])) + 1):
                    found.append((sheet.name, address(col, row)))
    elif node.kind == "table" or (node.kind == "name" and wb.table(node.value)):
        try:
            found += _cells_of(wb, host, Node("ref", structured_range(wb, node.value if node.kind == "table" else node.value + "[#Data]", host, cell)), depth, cell)
        except Signal:
            pass
    elif node.kind == "name" and depth < 8:
        target = {key.casefold(): value for key, value in wb.names.items()}.get(node.value.casefold())
        if target:
            found += _cells_of(wb, host, parse(target), depth + 1, cell)
    for arg in node.args:
        found += _cells_of(wb, host, arg, depth, cell)
    return found


def _formula_cells(wb: Workbook) -> list[tuple[str, str]]:
    return [(sheet.name, key) for sheet in wb.sheets for key, cell in sorted(sheet.cells.items(), key=lambda item: (split_address(item[0])[1], split_address(item[0])[0])) if cell.formula]


def _direct(wb: Workbook, key: tuple[str, str]) -> list[tuple[str, str]]:
    cell = wb.cell(*key)
    try:
        found = _cells_of(wb, key[0], parse(cell.formula), 0, key[1])
    except FormulaError:
        return []
    seen = dict.fromkeys(found)
    order = {name.casefold(): index for index, name in enumerate(wb.order())}
    return sorted(seen, key=lambda item: (order[item[0].casefold()], split_address(item[1])[1], split_address(item[1])[0]))


@lru_cache(maxsize=32)
def _plan(wb: Workbook) -> tuple[tuple, frozenset, dict]:
    nodes = _formula_cells(wb)
    members = set(nodes)
    edges = {key: [item for item in _direct(wb, key) if item in members] for key in nodes}
    index, low, stack, on, order, cyclic = {}, {}, [], set(), [], set()

    def visit(key):
        index[key] = low[key] = len(index)
        stack.append(key)
        on.add(key)
        for other in edges[key]:
            if other not in index:
                visit(other)
                low[key] = min(low[key], low[other])
            elif other in on:
                low[key] = min(low[key], index[other])
        if low[key] == index[key]:
            group = []
            while True:
                top = stack.pop()
                on.discard(top)
                group.append(top)
                if top == key:
                    break
            if len(group) > 1 or key in edges[key]:
                cyclic.update(group)
            else:
                order.append(key)

    for key in nodes:
        if key not in index:
            visit(key)
    return tuple(order), frozenset(cyclic), edges


def recalculation_order(wb: Workbook) -> list[tuple[str, str]]:
    """🔁 Formula cells in dependency order: precedents are always calculated before their dependents."""
    return list(_plan(wb)[0])


def circular_cells(wb: Workbook) -> list[tuple[str, str]]:
    """♾️ Formula cells that depend on themselves, directly or through others (Excel warns and shows 0)."""
    return sorted(_plan(wb)[1])


def precedents(wb: Workbook, key: tuple[str, str]) -> list[tuple[str, str]]:
    """⬅️ Cells the formula in `key` reads (trace precedents); ranges expand to their cells."""
    cell = wb.cell(*key)
    return _direct(wb, key) if cell and cell.formula else []


def dependents(wb: Workbook, key: tuple[str, str]) -> list[tuple[str, str]]:
    """➡️ Formula cells that read `key` (trace dependents)."""
    target = (wb.sheet(key[0]).name, key[1]) if wb.sheet(key[0]) else key
    return [other for other in _formula_cells(wb) if target in _direct(wb, other)]


def _sc(value):
    if isinstance(value, Matrix) and value.h == 1 and value.w == 1:
        return value.rows[0][0]
    return value


def _num(value) -> float:
    value = _sc(value)
    if isinstance(value, XlError):
        raise Signal(value)
    if isinstance(value, Matrix):
        fail("#VALUE!")
    if value is None:
        return 0.0
    if isinstance(value, bool):
        return 1.0 if value else 0.0
    if isinstance(value, (int, float)):
        return float(value)
    try:
        return float(value.strip())
    except ValueError:
        fail("#VALUE!")


def _text(value) -> str:
    value = _sc(value)
    if isinstance(value, XlError):
        raise Signal(value)
    if value is None:
        return ""
    if isinstance(value, bool):
        return "TRUE" if value else "FALSE"
    if isinstance(value, float):
        return str(int(value)) if value == int(value) and abs(value) < 1e15 else format(value, ".15g")
    return str(value)


def _bool(value) -> bool:
    value = _sc(value)
    if isinstance(value, str):
        if value.upper() in ("TRUE", "FALSE"):
            return value.upper() == "TRUE"
        fail("#VALUE!")
    return _num(value) != 0


def _rank(value):
    return 2 if isinstance(value, bool) else 1 if isinstance(value, str) else 0


def _compare(a, b) -> int:
    for item in (a, b):
        if isinstance(item, XlError):
            raise Signal(item)
    if a is None:
        a = "" if isinstance(b, str) else False if isinstance(b, bool) else 0.0
    if b is None:
        b = "" if isinstance(a, str) else False if isinstance(a, bool) else 0.0
    if _rank(a) != _rank(b):
        return (_rank(a) > _rank(b)) - (_rank(a) < _rank(b))
    if isinstance(a, str):
        a, b = a.casefold(), b.casefold()
    elif not isinstance(a, bool) and math.isclose(a, b, rel_tol=1e-15, abs_tol=0.0):
        return 0
    return (a > b) - (a < b)


def _arith(op):
    def run(a, b):
        x, y = _num(a), _num(b)
        if op == "+":
            return x + y
        if op == "-":
            return x - y
        if op == "*":
            return x * y
        if op == "/":
            if y == 0:
                fail("#DIV/0!")
            return x / y
        if x == 0 and y == 0 or (x < 0 and y != int(y)):
            fail("#NUM!")
        try:
            return x ** y
        except (OverflowError, ZeroDivisionError):
            fail("#NUM!" if x else "#DIV/0!")
    return run


_COMPARE = {"=": lambda c: c == 0, "<>": lambda c: c != 0, "<": lambda c: c < 0, ">": lambda c: c > 0, "<=": lambda c: c <= 0, ">=": lambda c: c >= 0}
_BINARY = {**{op: _arith(op) for op in "+-*/^"}, "&": lambda a, b: _text(a) + _text(b), **{op: (lambda test: lambda a, b: test(_compare(a, b)))(test) for op, test in _COMPARE.items()}}


def _at(value, i, j):
    if not isinstance(value, Matrix):
        return value
    return value.rows[0 if value.h == 1 else i][0 if value.w == 1 else j]


def _lift(function, *values):
    values = [_sc(value) for value in values]
    shaped = [value for value in values if isinstance(value, Matrix)]
    if not shaped:
        return function(*values)
    h = max(value.h for value in shaped)
    w = max(value.w for value in shaped)
    if any(value.h not in (1, h) or value.w not in (1, w) for value in shaped):
        return Matrix(tuple(tuple(ERR["#N/A"] for _ in range(w)) for _ in range(h)))
    rows = []
    for i in range(h):
        row = []
        for j in range(w):
            try:
                row.append(function(*[_at(value, i, j) for value in values]))
            except Signal as signal:
                row.append(signal.error)
        rows.append(tuple(row))
    return Matrix(tuple(rows))


def _mat(value) -> Matrix:
    return value if isinstance(value, Matrix) else Matrix(((value,),))


def _numbers(args, strict=False):
    for arg in args:
        arg = _sc(arg) if not isinstance(arg, Matrix) or (arg.h == 1 and arg.w == 1 and False) else arg
        if isinstance(arg, Matrix):
            for value in arg.flat():
                if isinstance(value, XlError):
                    raise Signal(value)
                if isinstance(value, (int, float)) and not isinstance(value, bool):
                    yield float(value)
        elif arg is not None:
            yield _num(arg)


def _wildcard(pattern: str):
    out = ""
    index = 0
    while index < len(pattern):
        char = pattern[index]
        if char == "~" and index + 1 < len(pattern):
            out += re.escape(pattern[index + 1])
            index += 1
        else:
            out += ".*" if char == "*" else "." if char == "?" else re.escape(char)
        index += 1
    return re.compile(out, re.I | re.S)


def _criterion(criterion):
    criterion = _sc(criterion)
    if isinstance(criterion, XlError):
        raise Signal(criterion)
    operator, rest = "=", criterion
    if isinstance(criterion, str):
        match = re.match(r"(<>|>=|<=|=|>|<)?(.*)\Z", criterion, re.S)
        operator, rest = match[1] or "=", match[2]
    if isinstance(rest, bool) or isinstance(rest, (int, float)):
        number = float(rest)
    else:
        try:
            number = float(rest)
        except ValueError:
            number = None

    def test(value):
        if isinstance(value, XlError):
            return False
        if number is not None:
            candidate = value
            if isinstance(value, str):
                try:
                    candidate = float(value)
                except ValueError:
                    return operator == "<>"
            if isinstance(candidate, bool) or not isinstance(candidate, (int, float)):
                return operator == "<>"
            return _COMPARE[operator](_compare(float(candidate), number))
        if rest == "":
            blank = value is None or value == ""
            return blank if operator == "=" else not blank if operator == "<>" else False
        if not isinstance(value, str):
            return operator == "<>"
        if operator in ("=", "<>"):
            return bool(_wildcard(rest).fullmatch(value)) == (operator == "=")
        return _COMPARE[operator](_compare(value, rest))

    return test


FUNCTIONS: dict = {}
LAZY: dict = {}


def _function(name):
    def register(function):
        FUNCTIONS[name] = function
        return function
    return register


def _lazy(name):
    def register(function):
        LAZY[name] = function
        return function
    return register


@_function("SUM")
def _sum(run, args):
    return math.fsum(_numbers(args))


@_function("COUNT")
def _count(run, args):
    total = 0
    for arg in args:
        if isinstance(arg, Matrix):
            total += sum(1 for value in arg.flat() if isinstance(value, (int, float)) and not isinstance(value, bool))
        elif arg is not None and not isinstance(arg, XlError):
            try:
                _num(arg)
                total += 1
            except Signal:
                pass
    return float(total)


@_function("COUNTA")
def _counta(run, args):
    return float(sum(1 for arg in args for value in (arg.flat() if isinstance(arg, Matrix) else [arg]) if value is not None))


@_function("AVERAGE")
def _average(run, args):
    values = list(_numbers(args))
    if not values:
        fail("#DIV/0!")
    return math.fsum(values) / len(values)


@_function("MIN")
def _min(run, args):
    return min(_numbers(args), default=0.0)


@_function("MAX")
def _max(run, args):
    return max(_numbers(args), default=0.0)


@_function("PI")
def _pi(run, args):
    if args:
        fail("#VALUE!")
    return math.pi


def round_half_up(value: float, digits: int) -> float:
    """🎯 Excel ROUND: half away from zero on the decimal representation."""
    with localcontext() as context:
        context.prec = 60
        exact = Decimal(repr(value))
        quantum = Decimal(1).scaleb(-digits)
        return float(abs(exact).quantize(quantum, rounding=ROUND_HALF_UP).copy_sign(exact))


@_function("ROUND")
def _round(run, args):
    if len(args) != 2:
        fail("#VALUE!")
    return _lift(lambda x, d: round_half_up(_num(x), int(math.trunc(_num(d)))), *args)


@_function("SUMPRODUCT")
def _sumproduct(run, args):
    if not args:
        fail("#VALUE!")
    arrays = [_mat(arg) for arg in args]
    if any((array.h, array.w) != (arrays[0].h, arrays[0].w) for array in arrays):
        fail("#VALUE!")
    total = 0.0
    for items in zip(*[array.flat() for array in arrays]):
        product = 1.0
        for item in items:
            if isinstance(item, XlError):
                raise Signal(item)
            product *= float(item) if isinstance(item, (int, float)) and not isinstance(item, bool) else 0.0
        total += product
    return total


def _logical(args):
    found = []
    for arg in args:
        for value in (arg.flat() if isinstance(arg, Matrix) else [arg]):
            if isinstance(value, XlError):
                raise Signal(value)
            if value is None or (isinstance(arg, Matrix) and isinstance(value, str)):
                continue
            found.append(_bool(value))
    if not found:
        fail("#VALUE!")
    return found


@_function("AND")
def _and(run, args):
    return all(_logical(args))


@_function("OR")
def _or(run, args):
    return any(_logical(args))


def _pairs(args):
    ranges = [_mat(args[i]) for i in range(0, len(args), 2)]
    tests = [_criterion(args[i]) for i in range(1, len(args), 2)]
    if any((r.h, r.w) != (ranges[0].h, ranges[0].w) for r in ranges):
        fail("#VALUE!")
    return ranges, tests


def _hits(ranges, tests):
    return [all(test(value) for test, value in zip(tests, values)) for values in zip(*[r.flat() for r in ranges])]


@_function("COUNTIF")
def _countif(run, args):
    if len(args) != 2:
        fail("#VALUE!")
    return float(sum(_hits(*_pairs(args))))


@_function("COUNTIFS")
def _countifs(run, args):
    if len(args) < 2 or len(args) % 2:
        fail("#VALUE!")
    return float(sum(_hits(*_pairs(args))))


def _summed(values, hits):
    total = []
    for value, hit in zip(values, hits):
        if hit:
            if isinstance(value, XlError):
                raise Signal(value)
            if isinstance(value, (int, float)) and not isinstance(value, bool):
                total.append(float(value))
    return math.fsum(total)


@_function("SUMIF")
def _sumif(run, args):
    if len(args) not in (2, 3):
        fail("#VALUE!")
    ranges, tests = _pairs(args[:2])
    target = _mat(args[2]).flat()[:len(ranges[0].flat())] if len(args) == 3 else ranges[0].flat()
    return _summed(target, _hits(ranges, tests))


@_function("SUMIFS")
def _sumifs(run, args):
    if len(args) < 3 or len(args) % 2 == 0:
        fail("#VALUE!")
    sums = _mat(args[0])
    ranges, tests = _pairs(args[1:])
    if (sums.h, sums.w) != (ranges[0].h, ranges[0].w):
        fail("#VALUE!")
    return _summed(sums.flat(), _hits(ranges, tests))


def _same(a, b, wildcard=False):
    try:
        if wildcard and isinstance(b, str) and isinstance(a, str):
            return bool(_wildcard(b).fullmatch(a))
        return _rank(a) == _rank(b) and a is not None and _compare(a, b) == 0
    except Signal:
        return False


@_function("XLOOKUP")
def _xlookup(run, args):
    if not 3 <= len(args) <= 6:
        fail("#VALUE!")
    key, look, ret = _sc(args[0]), _mat(args[1]), _mat(args[2])
    fallback = args[3] if len(args) > 3 else None
    mode = int(_num(args[4])) if len(args) > 4 and args[4] is not None else 0
    direction = int(_num(args[5])) if len(args) > 5 and args[5] is not None else 1
    if look.h != 1 and look.w != 1:
        fail("#VALUE!")
    items = look.flat()
    order = range(len(items) - 1, -1, -1) if direction in (-1, -2) else range(len(items))
    found = next((i for i in order if _same(items[i], key, mode == 2)), None)
    if found is None and mode in (-1, 1):
        near = [i for i in order if _rank(items[i]) == _rank(key) and not isinstance(items[i], XlError) and (_compare(items[i], key) < 0 if mode == -1 else _compare(items[i], key) > 0)]
        if near:
            found = (max if mode == -1 else min)(near, key=lambda i: items[i] if not isinstance(items[i], str) else items[i].casefold())
    if found is None:
        if fallback is None:
            fail("#N/A")
        return fallback
    vertical = look.w == 1
    if (ret.h if vertical else ret.w) != len(items):
        fail("#VALUE!")
    chosen = Matrix((ret.rows[found],)) if vertical else Matrix(tuple((row[found],) for row in ret.rows))
    return _sc(chosen)


@_function("MATCH")
def _match(run, args):
    if not 2 <= len(args) <= 3:
        fail("#VALUE!")
    key, look = _sc(args[0]), _mat(args[1])
    kind = int(_num(args[2])) if len(args) > 2 and args[2] is not None else 1
    if look.h != 1 and look.w != 1:
        fail("#N/A")
    items = look.flat()
    if kind == 0:
        for i, item in enumerate(items):
            if _same(item, key, True):
                return float(i + 1)
        fail("#N/A")
    best = None
    for i, item in enumerate(items):
        if _rank(item) != _rank(key) or isinstance(item, XlError):
            continue
        c = _compare(item, key)
        if (kind == 1 and c <= 0) or (kind == -1 and c >= 0):
            best = i
        else:
            break
    if best is None:
        fail("#N/A")
    return float(best + 1)


@_function("INDEX")
def _index(run, args):
    if not 2 <= len(args) <= 3:
        fail("#VALUE!")
    array = _mat(args[0])
    row = int(_num(args[1])) if args[1] is not None else 0
    col = int(_num(args[2])) if len(args) > 2 and args[2] is not None else None
    if col is None and array.h == 1 and array.w > 1:
        row, col = 0, row
    col = 0 if col is None else col
    if not (0 <= row <= array.h and 0 <= col <= array.w):
        fail("#REF!")
    rows = array.rows if row == 0 else (array.rows[row - 1],)
    picked = tuple(tuple(r) if col == 0 else (r[col - 1],) for r in rows)
    return _sc(Matrix(picked))


@_function("ROWS")
def _rows(run, args):
    if len(args) != 1:
        fail("#VALUE!")
    return float(_mat(args[0]).h)


@_function("COLUMNS")
def _columns(run, args):
    if len(args) != 1:
        fail("#VALUE!")
    return float(_mat(args[0]).w)


@_lazy("IF")
def _if(run, nodes):
    if not 2 <= len(nodes) <= 3:
        fail("#VALUE!")
    condition = _sc(run.ev(nodes[0]))
    if isinstance(condition, Matrix):
        yes, no = run.ev(nodes[1]), run.ev(nodes[2]) if len(nodes) > 2 else False
        return _lift(lambda c, a, b: a if _bool(c) else b, condition, yes, no)
    chosen = nodes[1] if _bool(condition) else (nodes[2] if len(nodes) > 2 else None)
    return False if chosen is None else run.ev(chosen)


@_lazy("IFERROR")
def _iferror(run, nodes):
    if len(nodes) != 2:
        fail("#VALUE!")
    try:
        value = run.ev(nodes[0])
    except Signal:
        return run.ev(nodes[1])
    value = _sc(value)
    if isinstance(value, XlError):
        return run.ev(nodes[1])
    if isinstance(value, Matrix):
        alternative = run.ev(nodes[1])
        return _lift(lambda v, a: a if isinstance(v, XlError) else v, value, alternative)
    return value


class _Run:
    def __init__(self, wb: Workbook, values: dict):
        self.wb = wb
        self.values = values
        self.sheet = None
        self.cell = None
        self.depth = 0

    def read(self, ref) -> Matrix:
        sheets = _span_sheets(self.wb, self.sheet, ref.sheet, ref.sheet_end)
        if not sheets:
            fail("#REF!")
        rows = []
        for sheet in sheets:
            c1, r1, c2, r2 = _corners(ref, _extent(sheet))
            for row in range(r1, r2 + 1):
                rows.append(tuple(self.values.get((sheet.name, address(col, row))) for col in range(c1, c2 + 1)))
        return Matrix(tuple(rows))

    def ev(self, node: Node):
        kind = node.kind
        if kind in ("num", "str", "bool"):
            return node.value
        if kind == "empty":
            return None
        if kind == "err":
            return ERR[node.value]
        if kind == "ref":
            return self.read(node.value)
        if kind == "table" or (kind == "name" and self.wb.table(node.value)):
            return self.read(structured_range(self.wb, node.value if kind == "table" else node.value + "[#Data]", self.sheet, self.cell))
        if kind == "name":
            target = {key.casefold(): value for key, value in self.wb.names.items()}.get(node.value.casefold())
            if target is None or self.depth > 8:
                fail("#NAME?")
            self.depth += 1
            try:
                return self.ev(parse(target))
            finally:
                self.depth -= 1
        if kind == "neg":
            return _lift(lambda x: -_num(x), self.ev(node.args[0]))
        if kind == "pos":
            return _sc(self.ev(node.args[0]))
        if kind == "pct":
            return _lift(lambda x: _num(x) / 100, self.ev(node.args[0]))
        if kind == "bin":
            return _lift(_BINARY[node.value], self.ev(node.args[0]), self.ev(node.args[1]))
        if kind == "union":
            return Matrix(tuple((value,) for arg in node.args for value in _mat(self.ev(arg)).flat()))
        if kind == "colon":
            fail("#VALUE!")
        name = node.value
        if name in LAZY:
            return LAZY[name](self, node.args)
        if name not in FUNCTIONS:
            fail("#NAME?")
        return FUNCTIONS[name](self, [self.ev(arg) for arg in node.args])


def _finish(value):
    value = _sc(value)
    return 0.0 if value is None else value


def _spill_blocked(wb, sheet, values, spilled, cells) -> bool:
    return any(key in spilled or sheet.cells.get(key) is not None for key in cells)


def evaluate(wb: Workbook) -> dict:
    """🧮 Calculate every cell in dependency order: {(sheet, 'B4'): value}; errors are XlError, multi-cell results spill or give #SPILL!."""
    values = {(sheet.name, key): cell.value for sheet in wb.sheets for key, cell in sheet.cells.items() if not cell.formula}
    order, cyclic, _ = _plan(wb)
    for key in cyclic:
        values[key] = 0.0
    run = _Run(wb, values)
    spilled: set = set()
    for key in order:
        sheet, cell = key
        run.sheet = sheet
        run.cell = cell
        try:
            result = _finish(run.ev(parse(wb.cell(sheet, cell).formula)))
        except Signal as signal:
            result = signal.error
        except FormulaError:
            result = ERR["#NAME?"]
        if isinstance(result, Matrix):
            col, row = split_address(cell)
            targets = [address(col + j, row + i) for i in range(result.h) for j in range(result.w)]
            blocked = any(t != cell and (t in spilled or wb.cell(sheet, t) is not None) for t in targets)
            if blocked:
                values[key] = ERR["#SPILL!"]
                continue
            for i, line in enumerate(result.rows):
                for j, item in enumerate(line):
                    values[(sheet, address(col + j, row + i))] = 0.0 if item is None else item
                    spilled.add(address(col + j, row + i))
            spilled.discard(cell)
            continue
        values[key] = result
    return values


def value_data(value):
    """📤 Portable form of a value: errors as {"error": code}, matrices as lists."""
    if isinstance(value, XlError):
        return {"error": value.code}
    if isinstance(value, Matrix):
        return [[value_data(item) for item in row] for row in value.rows]
    return value


def calculate(formula: str, cells: dict | None = None, language: str = "en"):
    """🧪 Evaluate one formula (UI language `language`) on a scratch sheet `Sheet1` filled from {address: value-or-"=formula"}; the formula sits in Z99."""
    sheet = {key: (Cell(formula=item) if isinstance(item, str) and item.startswith("=") else Cell(value=float(item) if isinstance(item, int) and not isinstance(item, bool) else item)) for key, item in (cells or {}).items()}
    sheet["Z99"] = Cell(formula="=" + canonicalize(formula, language).lstrip("="))
    return evaluate(Workbook((Sheet("Sheet1", sheet, {}),), {}))[("Sheet1", "Z99")]


def localize_error(code: str, language: str) -> str:
    """🌍 Error literal in the UI language: #REF! to #BEZUG! ([Microsoft](https://support.microsoft.com/de-de/excel))."""
    return DE_ERRORS.get(code, code) if language == "de" else code


@lru_cache(maxsize=1)
def _default_titles() -> tuple:
    return tuple((spec["name"], dict(spec["title"])) for spec in load()["sheets"] if "title" in spec)


def sheet_titles(language: str, wb: Workbook | None = None) -> dict:
    """🗂️ Canonical sheet name to tab label in a language: Program to Programm, Type A to Typ A; sheets without labels keep their name."""
    pairs = [(s.name, s.labels) for s in wb.sheets if s.labels] if wb is not None else list(_default_titles())
    return {name: labels[language] for name, labels in pairs}


@lru_cache(maxsize=1)
def _default_tables() -> tuple:
    return tuple(build(load()).tables.values())


def table_titles(language: str, wb: Workbook | None = None) -> dict:
    """🏷️ Canonical table name to its name in a language: Capsules to Kapseln; tables without labels keep their name."""
    tables = wb.tables.values() if wb is not None else _default_tables()
    return {table.name: table.labels[language] for table in tables if table.labels}


def table_headers(language: str, wb: Workbook | None = None) -> dict:
    """🔠 Canonical (English) table column header to its header text in a language: Type to Typ, Net area to Nettofläche."""
    tables = wb.tables.values() if wb is not None else _default_tables()
    return {head["en"]: head[language] for table in tables for head in table.headers}


def _map_structured(text: str, language: str, wb: Workbook | None, back: bool) -> str:
    columns = table_headers(language, wb)
    columns = {german.casefold(): english for english, german in columns.items()} if back else {english.casefold(): german for english, german in columns.items()}
    specials = EN_SPECIALS if back else DE_SPECIALS
    titles = table_titles(language, wb)
    titles = {label.casefold(): name for name, label in titles.items()} if back else {name.casefold(): label for name, label in titles.items()}

    def part(match):
        content = match[1]
        prefix = "@" if content.startswith("@") else ""
        content = content[len(prefix):]
        table = specials if content.startswith("#") else columns
        return "[" + prefix + table.get(content.casefold(), content) + "]"

    head = re.match(r"[^\W\d][\w.]*", text)
    if head:
        text = titles.get(head[0].casefold(), head[0]) + text[head.end():]
    out = re.sub(r"\[([^\[\]]*)\]", part, text)
    return out.replace("];[", "],[") if back else out.replace("],[", "];[")


def tab_name(wb: Workbook, sheet: str, language: str) -> str:
    """🏷️ The text on a sheet tab in the UI language."""
    found = wb.sheet(sheet)
    return found.labels[language] if found and found.labels else sheet


def _map_sheets(text: str, mapping: dict) -> str:
    folded = {name.casefold(): label for name, label in mapping.items()}
    ref = parse_ref(text)
    swap = lambda name: folded.get(name.casefold(), name) if name else name
    if ref.sheet is None or (swap(ref.sheet), swap(ref.sheet_end)) == (ref.sheet, ref.sheet_end):
        return text
    return format_ref(replace(ref, sheet=swap(ref.sheet), sheet_end=swap(ref.sheet_end)))


def localize(formula: str, language: str, wb: Workbook | None = None) -> str:
    """🌍 Canonical English formula to UI text: `de` uses German function names, sheet tab names (`'Typ A'!B3`), `;` and decimal comma; defined names stay."""
    if language == "en":
        return formula
    titles = sheet_titles(language, wb)
    out = []
    for token in tokenize(formula, "en"):
        text = token.text
        if token.kind == "func":
            text = DE_FUNCTIONS.get(text.upper(), text)
        elif token.kind == "sep":
            text = ";"
        elif token.kind == "num":
            text = text.replace(".", ",")
        elif token.kind == "bool":
            text = DE_BOOLEANS[text.upper()]
        elif token.kind == "err":
            text = DE_ERRORS.get(text, text)
        elif token.kind == "ref":
            text = _map_sheets(text, titles)
        elif token.kind == "table":
            text = _map_structured(text, language, wb, False)
        elif token.kind == "name":
            text = table_titles(language, wb).get(text, text) if text in table_titles("en", wb) else text
        out.append(text)
    return "".join(out)


def canonicalize(formula: str, language: str, wb: Workbook | None = None) -> str:
    """🏛️ UI text back to the canonical English formula (inverse of localize, tab names included)."""
    if language == "en":
        return formula
    titles = {label: name for name, label in sheet_titles(language, wb).items()}
    out = []
    for token in tokenize(formula, "de"):
        text = token.text
        if token.kind == "func":
            text = EN_FUNCTIONS.get(text.upper()) or ("_" + text if DE_FUNCTIONS.get(text.upper(), text.upper()) != text.upper() else text)
        elif token.kind == "sep":
            text = ","
        elif token.kind == "num":
            text = text.replace(",", ".")
        elif token.kind == "bool":
            text = EN_BOOLEANS.get(text.upper(), text.upper())
        elif token.kind == "err":
            text = EN_ERRORS.get(text, text)
        elif token.kind == "ref":
            text = _map_sheets(text, titles)
        elif token.kind == "table":
            text = _map_structured(text, language, wb, True)
        elif token.kind == "name":
            text = {label.casefold(): name for name, label in table_titles(language, wb).items()}.get(text.casefold(), text)
        out.append(text)
    return "".join(out)


def _sections(number_format: str) -> list[str]:
    parts, current, quoted = [], "", False
    for char in number_format:
        if char == '"':
            quoted = not quoted
        if char == ";" and not quoted:
            parts.append(current)
            current = ""
        else:
            current += char
    return parts + [current]


def _scan(section: str):
    items, index, used = [], 0, False
    while index < len(section):
        char = section[index]
        if char == '"':
            end = section.index('"', index + 1)
            items.append(("lit", section[index + 1:end]))
            index = end + 1
        elif char in "\\_":
            items.append(("lit", section[index + 1] if char == "\\" else " "))
            index += 2
        elif char == "*":
            index += 2
        elif char == "[":
            index = section.index("]", index) + 1
        elif char in "0#?.,":
            end = index
            while end < len(section) and section[end] in "0#?.,":
                end += 1
            items.append(("lit", section[index:end]) if used else ("num", section[index:end]))
            used = True
            index = end
        elif char == "%":
            items.append(("pct", "%"))
            index += 1
        else:
            items.append(("lit", char))
            index += 1
    return items


def _general(number: float, language: str) -> str:
    text = str(int(number)) if number == int(number) and abs(number) < 1e11 else format(number, ".10g")
    return text.replace(".", ",") if language == "de" else text


def _format_number(number: float, number_format: str, language: str) -> str:
    sections = _sections(number_format)
    negative_section = len(sections) >= 2 and number < 0
    zero_section = len(sections) >= 3 and number == 0
    section = sections[1] if negative_section else sections[2] if zero_section else sections[0]
    if section.strip().casefold() == "general":
        return _general(number, language)
    items = _scan(section)
    percent = sum(1 for kind, _ in items if kind == "pct")
    out = "-" if number < 0 and not negative_section else ""
    for kind, text in items:
        if kind != "num":
            out += text
            continue
        integer, _, decimals = text.partition(".")
        scale = len(integer) - len(integer.rstrip(","))
        grouped = "," in integer.rstrip(",")
        minimum = integer.count("0")
        low, high = decimals.count("0"), len(decimals.replace(",", ""))
        with localcontext() as context:
            context.prec = 60
            scaled = Decimal(repr(abs(number))) * (Decimal(100) ** percent) / (Decimal(1000) ** scale)
            digits = format(scaled.quantize(Decimal(1).scaleb(-high), rounding=ROUND_HALF_UP), "f")
        whole, _, fraction = digits.partition(".")
        fraction = fraction.rstrip("0").ljust(low, "0")
        whole = whole.lstrip("0") if minimum == 0 else whole
        whole = whole.rjust(minimum, "0")
        thousands, point = (".", ",") if language == "de" else (",", ".")
        if grouped:
            groups = []
            while len(whole) > 3:
                groups.insert(0, whole[-3:])
                whole = whole[:-3]
            whole = thousands.join([whole, *groups]) if whole else thousands.join(groups)
        out += whole + (point + fraction if "." in text else "")
    return out


def display(value, number_format: str | None = None, language: str = "en") -> str:
    """🖼️ What the cell shows: number formats such as `0.00 "m"`, `#,##0 "kWh/a"`, `0%` with en/de punctuation; the stored value is unchanged."""
    if isinstance(value, XlError):
        return localize_error(value.code, language)
    if isinstance(value, bool):
        return (DE_BOOLEANS if language == "de" else {"TRUE": "TRUE", "FALSE": "FALSE"})["TRUE" if value else "FALSE"]
    if value is None:
        return ""
    if isinstance(value, str):
        return value
    return _format_number(float(value), number_format or "General", language)


def shown(wb: Workbook, values: dict, sheet: str, cell: str, language: str) -> str:
    """👁️ The text a cell displays in a language: its label, or the formatted value."""
    item = wb.cell(sheet, cell)
    if item is None:
        return ""
    if item.labels:
        return item.labels[language]
    return display(values.get((wb.sheet(sheet).name, cell)), item.format, language)


def formula_text(wb: Workbook, sheet: str, cell: str, language: str) -> str:
    """✏️ What the formula bar shows: the formula in the UI language, or the constant."""
    item = wb.cell(sheet, cell)
    if item is None:
        return ""
    if item.formula:
        return localize(item.formula, language, wb)
    return shown(wb, {(wb.sheet(sheet).name, cell): item.value}, sheet, cell, language)


def _clone(wb: Workbook) -> Workbook:
    return Workbook(tuple(Sheet(s.name, dict(s.cells), dict(s.widths), s.labels) for s in wb.sheets), dict(wb.names), wb.file, dict(wb.tables))


def _replace_sheet(wb: Workbook, name: str, change) -> Workbook:
    new = _clone(wb)
    sheet = new.sheet(name)
    if sheet is None:
        raise KeyError("No sheet " + name)
    change(sheet)
    return new


def _reference_range(text: str, sheet: str):
    parsed = parse_ref(text if "!" in text else "'" + sheet + "'!" + text if sheet else text)
    sheet_name = parsed.sheet or sheet
    start, end = _box(parsed)
    return sheet_name, (start.col, start.row), (end.col, end.row)


def _move_sheet(wb, event):
    old = wb.order()
    names = [name.casefold() for name in old]
    moved = wb.sheet(event["sheet"]).name
    sheets = [sheet for sheet in wb.sheets if sheet.name != moved]
    sheets.insert(event["index"], wb.sheet(moved))
    new_order = [sheet.name for sheet in sheets]
    cf = moved.casefold()

    def change(item):
        if item.sheet is None or item.sheet_end is None or cf not in (item.sheet.casefold(), item.sheet_end.casefold()):
            return None
        low, high = sorted((names.index(item.sheet.casefold()), names.index(item.sheet_end.casefold())))
        rest = [name for name in old[low:high + 1] if name.casefold() != cf]
        if not rest:
            return "#REF!"
        keep = [new_order.index(name) for name in rest]
        if min(keep) < new_order.index(moved) < max(keep):
            return None
        ends = sorted(rest, key=new_order.index)
        return format_ref(replace(item, sheet=ends[0], sheet_end=None if ends[0] == ends[-1] else ends[-1]))

    rewritten = tuple(Sheet(s.name, {k: replace(c, formula=_rewrite(c.formula, change) if c.formula else None) for k, c in s.cells.items()}, s.widths, s.labels) for s in sheets)
    return Workbook(rewritten, {name: _rewrite(text, change) for name, text in wb.names.items()}, wb.file, dict(wb.tables))


def _clear_cell(wb, event):
    return _set_cell(wb, {**event, "clear": True})


def _set_cell(wb, event):
    def change(sheet):
        key = event["cell"]
        split_address(key)
        old = sheet.cells.get(key)
        keep = event.get("format", old.format if old else None)
        if event.get("clear"):
            sheet.cells.pop(key, None)
        elif "formula" in event:
            sheet.cells[key] = Cell(formula=event["formula"], format=keep)
        else:
            value = event["value"]
            sheet.cells[key] = Cell(value=float(value) if isinstance(value, int) and not isinstance(value, bool) else value, format=keep, labels=old.labels if old and old.value == value else None)
    return _replace_sheet(wb, event["sheet"], change)


def _copy(wb, event):
    sheet_name, (c1, r1), (c2, r2) = _reference_range(event["source"], event.get("sheet"))
    target_sheet, (t1, u1), (t2, u2) = _reference_range(event["target"], event.get("sheet"))
    c2, r2 = c2 or c1, r2 or r1
    height, width = r2 - r1 + 1, c2 - c1 + 1
    t2, u2 = (t1 + width - 1, u1 + height - 1) if (t2, u2) == (t1, u1) else (t2, u2)
    source = wb.sheet(sheet_name)

    def change(sheet):
        for row in range(u1, u2 + 1):
            for col in range(t1, t2 + 1):
                origin = address(c1 + (col - t1) % width, r1 + (row - u1) % height)
                cell = source.cells.get(origin)
                key = address(col, row)
                if cell is None:
                    sheet.cells.pop(key, None)
                elif cell.formula:
                    sheet.cells[key] = replace(cell, formula=translate(cell.formula, origin, key))
                else:
                    sheet.cells[key] = cell
    return _replace_sheet(wb, target_sheet, change)


def _paste_values(wb, event):
    values = evaluate(wb)
    sheet_name, (c1, r1), (c2, r2) = _reference_range(event["range"], event.get("sheet"))

    def change(sheet):
        for row in range(r1, (r2 or r1) + 1):
            for col in range(c1, (c2 or c1) + 1):
                key = address(col, row)
                cell = sheet.cells.get(key)
                if cell and cell.formula:
                    sheet.cells[key] = Cell(value=values[(sheet.name, key)], format=cell.format)
    return _replace_sheet(wb, sheet_name, change)


def _axis_adjust(low, high, index, count):
    if count > 0:
        return (low + count if low >= index else low, high + count if high >= index else high)
    last = index - count - 1
    if low >= index and high <= last:
        return None
    return (low if low < index else index if low <= last else low + count, high if high < index else index - 1 if high <= last else high + count)


def _structural_change(wb, target, axis, index, count):
    order = [name.casefold() for name in wb.order()]

    def covers(host, ref):
        if ref.sheet is None:
            return host is not None and host.casefold() == target.casefold()
        names = [ref.sheet.casefold(), (ref.sheet_end or ref.sheet).casefold()]
        if not all(name in order for name in names):
            return False
        low, high = sorted(order.index(name) for name in names)
        return low <= order.index(target.casefold()) <= high

    def make(host):
        def change(item):
            if not covers(host, item):
                return None
            start, end = _box(item)
            if getattr(start, axis) is None:
                return None
            low, high = sorted((getattr(start, axis), getattr(end, axis)))
            adjusted = _axis_adjust(low, high, index, count)
            if adjusted is None:
                return "#REF!"
            if isinstance(item, Ref):
                return format_ref(replace(item, **{axis: adjusted[0]}))
            return format_ref(replace(item, start=replace(start, **{axis: adjusted[0]}), end=replace(end, **{axis: adjusted[1]})))
        return change
    return make


def _shift_table(table, axis, index, count):
    if axis == "row":
        top, size = table.row, table.nrows
        if count > 0:
            return replace(table, row=top + count) if index <= top else replace(table, nrows=size + count) if index <= top + size else table
        deleted = -count
        end = index + deleted - 1
        if end < top:
            return replace(table, row=top - deleted)
        if index > top + size:
            return table
        if index <= top:
            return None
        return replace(table, nrows=size - (min(end, top + size) - index + 1))
    left, width = table.col, len(table.headers)
    if count > 0:
        if index <= left:
            return replace(table, col=left + count)
        if index < left + width:
            heads = list(table.headers)
            at = index - left
            heads[at:at] = [{"en": f"Column{n}", "de": f"Spalte{n}"} for n in range(width + 1, width + count + 1)]
            return replace(table, headers=tuple(heads))
        return table
    deleted = -count
    end = index + deleted - 1
    if index >= left + width:
        return table
    shift = max(0, min(end, left - 1) - index + 1)
    low, high = max(index, left) - left, min(end, left + width - 1) - left
    heads = [head for i, head in enumerate(table.headers) if not low <= i <= high]
    return replace(table, col=left - shift, headers=tuple(heads)) if heads else None


def _shift_tables(wb, target, axis, index, count):
    moved = {}
    for name, table in wb.tables.items():
        shifted = _shift_table(table, axis, index, count) if table.sheet.casefold() == target.casefold() else table
        if shifted is not None:
            moved[name] = shifted
    return moved


def _structural(wb, event, axis, count):
    target, index = event["sheet"], event["index"]
    count = count * event.get("count", 1)
    make = _structural_change(wb, target, axis, index, count)
    new = _clone(wb)
    for sheet in new.sheets:
        cells = {}
        for key, cell in sheet.cells.items():
            col, row = split_address(key)
            if sheet.name.casefold() == target.casefold():
                position = _axis_adjust(row if axis == "row" else col, row if axis == "row" else col, index, count)
                if position is None:
                    continue
                key = address(col, position[0]) if axis == "row" else address(position[0], row)
            cells[key] = replace(cell, formula=_rewrite(cell.formula, make(sheet.name)) if cell.formula else None)
        sheet.cells.clear()
        sheet.cells.update(cells)
    return Workbook(new.sheets, {name: _rewrite(text, make(None)) for name, text in new.names.items()}, new.file, _shift_tables(wb, target, axis, index, count))


def _rename_sheet(wb, event):
    old, new_name = event["sheet"], event["name"]
    if wb.sheet(new_name) or wb.sheet(old) is None:
        raise KeyError("Cannot rename " + old)

    def change(item):
        if item.sheet is None or old.casefold() not in (item.sheet.casefold(), (item.sheet_end or "").casefold()):
            return None
        swap = lambda name: new_name if name and name.casefold() == old.casefold() else name
        return format_ref(replace(item, sheet=swap(item.sheet), sheet_end=swap(item.sheet_end)))

    new = _clone(wb)
    sheets = tuple(Sheet(new_name if s.name == wb.sheet(old).name else s.name, {k: replace(c, formula=_rewrite(c.formula, change) if c.formula else None) for k, c in s.cells.items()}, s.widths, None if s.name == wb.sheet(old).name else s.labels) for s in new.sheets)
    return Workbook(sheets, {name: _rewrite(text, change) for name, text in new.names.items()}, new.file, {k: (replace(t, sheet=new_name) if t.sheet.casefold() == old.casefold() else t) for k, t in wb.tables.items()})


def _delete_sheet(wb, event):
    gone = event["sheet"]
    order = [name.casefold() for name in wb.order()]
    index = order.index(gone.casefold())
    names = wb.order()

    def change(item):
        if item.sheet is None:
            return None
        first, last = item.sheet, item.sheet_end or item.sheet
        if gone.casefold() not in (first.casefold(), last.casefold()):
            return None
        if first.casefold() == last.casefold():
            return "#REF!"
        if first.casefold() == gone.casefold():
            first = names[index + 1]
        else:
            last = names[index - 1]
        return format_ref(replace(item, sheet=first, sheet_end=None if first.casefold() == last.casefold() else last))

    sheets = tuple(Sheet(s.name, {k: replace(c, formula=_rewrite(c.formula, change) if c.formula else None) for k, c in s.cells.items()}, s.widths, s.labels) for s in wb.sheets if s.name.casefold() != gone.casefold())
    return Workbook(sheets, {name: _rewrite(text, change) for name, text in wb.names.items()}, wb.file, {k: t for k, t in wb.tables.items() if t.sheet.casefold() != gone.casefold()})


def _add_sheet(wb, event):
    if wb.sheet(event["name"]):
        raise KeyError("Sheet exists: " + event["name"])
    sheets = list(_clone(wb).sheets)
    sheets.insert(event.get("index", len(sheets)), Sheet(event["name"], {}, {}))
    return Workbook(tuple(sheets), dict(wb.names), wb.file, dict(wb.tables))


def _define_table(wb, event):
    sheet = event["sheet"]
    (first, top), (last, bottom) = [split_address(part) for part in event["ref"].split(":")]
    cells = wb.sheet(sheet).cells
    heads = []
    for col in range(first, last + 1):
        cell = cells.get(address(col, top))
        heads.append(dict(cell.labels) if cell and cell.labels else {"en": str(cell.value), "de": str(cell.value)} if cell else {"en": f"Column{col - first + 1}", "de": f"Spalte{col - first + 1}"})
    new = _clone(wb)
    new.tables[event["name"]] = Table(event["name"], wb.sheet(sheet).name, first, top, tuple(heads), bottom - top)
    return new


def _delete_table(wb, event):
    new = _clone(wb)
    new.tables.pop(event["name"])
    return new


def _define_name(wb, event):
    new = _clone(wb)
    new.names[event["name"]] = event["ref"]
    return new


def _delete_name(wb, event):
    new = _clone(wb)
    new.names.pop(event["name"])
    return new


EVENTS = {
    "set-cell": _set_cell,
    "clear-cell": _clear_cell,
    "move-sheet": _move_sheet,
    "copy": _copy,
    "paste-values": _paste_values,
    "insert-row": lambda wb, e: _structural(wb, e, "row", 1),
    "delete-row": lambda wb, e: _structural(wb, e, "row", -1),
    "insert-column": lambda wb, e: _structural(wb, e, "col", 1),
    "delete-column": lambda wb, e: _structural(wb, e, "col", -1),
    "rename-sheet": _rename_sheet,
    "delete-sheet": _delete_sheet,
    "add-sheet": _add_sheet,
    "define-table": _define_table,
    "delete-table": _delete_table,
    "define-name": _define_name,
    "delete-name": _delete_name,
}


def apply(wb: Workbook, events) -> Workbook:
    """🎞️ Event-sourced edit: fold events (set-cell, copy, paste-values, insert/delete-row/column, rename/delete/add-sheet, define/delete-name) into a new workbook."""
    for event in events:
        if event["type"] not in EVENTS:
            raise KeyError("Unknown event " + event["type"])
        wb = EVENTS[event["type"]](wb, event)
    return wb


@lru_cache(maxsize=1)
def load() -> dict:
    """📥 The authored workbook data (JSON, schema-first)."""
    return json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))


def _constant(spec: dict, resolver):
    value = resolver(spec["from"]) if resolver and "from" in spec else spec.get("value")
    return float(value) if isinstance(value, (int, float)) and not isinstance(value, bool) else value


def _build_cell(spec: dict, resolver, language: str = "en") -> Cell:
    if "text" in spec:
        return Cell(value=spec["text"][language], format=spec.get("format"), labels=dict(spec["text"]))
    if "formula" in spec:
        return Cell(formula=spec["formula"], format=spec.get("format"))
    return Cell(value=_constant(spec, resolver), format=spec.get("format"))


def _table_cells(table: dict) -> dict:
    col0, row0 = split_address(table["at"])
    cells = {}
    for offset, column in enumerate(table["columns"]):
        header = {"text": column["header"]}
        cells[address(col0 + offset, row0)] = header
    for index, row in enumerate(table["rows"]):
        line = row0 + 1 + index
        given = iter(row)
        for offset, column in enumerate(table["columns"]):
            key = address(col0 + offset, line)
            if "formula" in column:
                cells[key] = {"formula": column["formula"].replace("{row}", str(line)), **({"format": column["format"]} if "format" in column else {})}
            else:
                item = next(given)
                cells[key] = {"text": column["labels"][item]} if "labels" in column else {"value": item, "basis": "derived", **({"format": column["format"]} if "format" in column else {})}
    return cells


def build(data: dict, resolver=None, language: str = "en") -> Workbook:
    """🏗️ Build a Workbook from authored data; `resolver(path)` rebinds `from` inputs to the shared Nakagin model; `language` picks which label text the text cells hold (criteria and look-ups then match in that language)."""
    sheets = []
    for spec in data["sheets"]:
        cells = {**spec["cells"], **(_table_cells(spec["table"]) if "table" in spec else {})}
        sheets.append(Sheet(spec["name"], {key: _build_cell(item, resolver, language) for key, item in cells.items()}, dict(spec.get("widths", {})), spec.get("title")))
    tables = {}
    for spec in data["sheets"]:
        table = spec.get("table")
        if table:
            col, row = split_address(table["at"])
            tables[table["name"]] = Table(table["name"], spec["name"], col, row, tuple(dict(column["header"]) for column in table["columns"]), len(table["rows"]), dict(table["title"]) if "title" in table else None)
    return Workbook(tuple(sheets), dict(data["names"]), data["file"], tables)


def workbook(resolver=None, language: str = "en") -> Workbook:
    """📗 The Nakagin teaching workbook: Program, one sheet per capsule type, Climate, Energy; `language="de"` holds the German texts (Typ A, Programm tabs stay canonical)."""
    return build(load(), resolver, language)
