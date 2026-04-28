#!/usr/bin/env python3

"""
Simple helper to write out the unicode names of every character in
argument 1
"""

import sys
import unicodedata


def main():
    if len(sys.argv) != 2:
        print(f"usage: python {__file__} <string>")
        return 1

    for c in sys.argv[1]:
        name = unicodedata.name(c)
        print(c, ord(c), name)


if __name__ == "__main__":
    raise SystemExit(main())
