#!/usr/bin/env python3

import subprocess
from pathlib import Path


def main():
    thisdir = Path(__file__).parent
    cmd = ["cargo", "run"]
    p = subprocess.run(cmd, cwd=thisdir / "generator", capture_output=True)

    if p.returncode != 0:
        print("error running `cargo run` in generator/")
        return 1

    stdout = p.stdout.decode("utf-8")

    with open(thisdir / "src" / "lib.rs", "w") as f:
        f.write(stdout)

    subprocess.run(["cargo", "fmt"])


if __name__ == "__main__":
    raise SystemExit(main())
