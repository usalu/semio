"""🔴️ A failing oracle fixture of `repo test-file`: it disagrees on purpose."""
import sys


def main(arguments):
    print("the failing oracle disagrees on purpose")
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
