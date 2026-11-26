"""🟢️ A passing oracle fixture of `repo test-file`.

    python 🐍️.py check <path to 🧫️fixtures/🔣️.json>
"""
import sys
from pathlib import Path


def main(arguments):
    if len(arguments) != 2 or arguments[0] != "check" or not Path(arguments[1]).is_file():
        print("the declared check arguments did not arrive: %r" % (arguments,))
        return 1
    print("oracle agrees on %s" % Path(arguments[1]).name)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
