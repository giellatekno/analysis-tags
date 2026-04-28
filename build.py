#!/usr/bin/env python3

import subprocess
from pathlib import Path
from shlex import split


def run(cmd, **args):

    extra = ""
    if directory := args.get("cwd"):
        extra = f" (cwd={directory})"

    print(cmd + extra)

    cmd = split(cmd)
    p = subprocess.run(cmd, **args)
    if p.returncode != 0:
        print(f"Error (returncode: {p.returncode})")
        if "capture_output" in args:
            print("stderr:")
            print(p.stderr)
        else:
            print("(stderr was not captured")

    return p


def main():
    thisdir = Path(__file__).parent
    cmd = f"cargo run -- {thisdir / 'generator' / 'template.rs'}"
    p = run(cmd, cwd=thisdir / "generator", capture_output=True)

    if p.returncode != 0:
        return 1

    stdout = p.stdout.decode("utf-8")

    with open(thisdir / "src" / "lib.rs", "w") as f:
        f.write(stdout)

    p = run("cargo fmt")
    if p.returncode != 0:
        return 1

    p = run("cargo check")
    if p.returncode != 0:
        return 1

    p = run("cargo test")
    if p.returncode != 0:
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
