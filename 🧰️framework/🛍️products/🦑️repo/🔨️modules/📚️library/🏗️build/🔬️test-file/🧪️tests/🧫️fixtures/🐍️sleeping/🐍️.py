"""😴️ A sleeping oracle fixture of `repo test-file`: it waits to be cancelled."""
import sys
import time


def main(arguments):
    time.sleep(120)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
