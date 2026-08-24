"""🐍️ Python oracle adapter for the PDF 1.4 text layer.

The reference is `pypdf`, registered by this plugin's 🧪️oracle contribution and put on this host's
import path by the `python` entry in its `oracleHostPackages`. Nothing in this file knows where the
interpreter came from — the coordinator provisions it.
"""

from __future__ import annotations

# region 🔖️Imports
import io

import pypdf

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixture
DOCUMENT = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf"


def reader(ctx: Context) -> pypdf.PdfReader:
    """📄️ The committed document, read through the reference implementation without copying it."""
    return pypdf.PdfReader(io.BytesIO(ctx.fixture_bytes(DOCUMENT)))


def claims(ctx: Context) -> list[dict[str, str]]:
    """🧫️ The scenario's `| page | contains |` table — the feature owns the vectors."""
    table = next((step["dataTable"] for step in ctx.scenario["steps"] if step.get("dataTable")), None)
    if table is None or len(table) < 2:
        raise AssertionError("scenario %s carries no `| page | contains |` table" % ctx.scenario["id"])
    header = [cell.strip() for cell in table[0]]
    page_column, contains_column = header.index("page"), header.index("contains")
    return [{"page": row[page_column].strip(), "contains": row[contains_column].strip()} for row in table[1:]]


# endregion 🔖️Fixture


# region 🔖️Scenarios
def declared_pages_carry_their_printed_text(ctx: Context) -> Outcome:
    """🔮️ Every declared page must contain the text the feature says the document prints."""
    pages = reader(ctx).pages
    projection = []
    for claim in claims(ctx):
        text = pages[int(claim["page"]) - 1].extract_text()
        projection.append({"page": int(claim["page"]), "contains": claim["contains"], "found": claim["contains"] in text, "characters": len(text)})
    absent = [row for row in projection if not row["found"]]
    if absent:
        raise AssertionError("%d declared claim(s) are absent from the extracted text: %s" % (len(absent), ", ".join("page %s wants %r" % (row["page"], row["contains"]) for row in absent)))
    return Outcome(projection)


def every_page_yields_text(ctx: Context) -> Outcome:
    """🔮️ A text layer this document is known to carry must survive extraction on every page."""
    document = reader(ctx)
    texts = [page.extract_text() for page in document.pages]
    empty = [index + 1 for index, text in enumerate(texts) if text.strip() == ""]
    if len(texts) == 0:
        raise AssertionError("the reference implementation reported no pages at all")
    if empty:
        raise AssertionError("%d of %d page(s) extracted to nothing: %s" % (len(empty), len(texts), empty))
    return Outcome({"pageCount": len(texts), "emptyPages": empty, "totalCharacters": sum(len(text) for text in texts), "header": document.pdf_header})


# endregion 🔖️Scenarios


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration entry point the host calls."""
    return Adapter("python").oracle("declared-pages-carry-their-printed-text", declared_pages_carry_their_printed_text).oracle("every-page-yields-text", every_page_yields_text)


# endregion 🔖️Registration
