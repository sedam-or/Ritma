#!/usr/bin/env python3
"""Generate tasks/DECISIONS-INDEX.md from the TD entries in tasks/DECISIONS.md.

Usage: decisions-index.py [--check]
--check exits 1 when the committed index differs from the generated one.
"""
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SOURCE = ROOT / "tasks" / "DECISIONS.md"
INDEX = ROOT / "tasks" / "DECISIONS-INDEX.md"
HEADER = re.compile(r"^### (TD-\d+) [—-] (.+)$")


def entries(text):
    current = None
    for line in text.splitlines():
        match = HEADER.match(line)
        if match:
            current = {"id": match.group(1), "title": match.group(2).strip(), "ratify": "not stated"}
            yield current
        elif current and line.startswith("- **Ratify:**"):
            current["ratify"] = line.split(":**", 1)[1].strip().rstrip(".")


def render(items):
    lines = [
        "# Technical decisions — index",
        "",
        "Generated from `tasks/DECISIONS.md` by `scripts/decisions-index.py`. Do not edit by hand; CI runs `--check`.",
        "",
        f"Total: {len(items)} decisions.",
        "",
        "| ID | Decision | Ratify |",
        "|---|---|---|",
    ]
    lines += [f"| {e['id']} | {e['title']} | {e['ratify']} |" for e in items]
    return "\n".join(lines) + "\n"


def main():
    generated = render(list(entries(SOURCE.read_text(encoding="utf-8"))))
    if "--check" in sys.argv[1:]:
        current = INDEX.read_text(encoding="utf-8") if INDEX.exists() else ""
        if current != generated:
            print("tasks/DECISIONS-INDEX.md is stale: run scripts/decisions-index.py", file=sys.stderr)
            return 1
        print("DECISIONS-INDEX.md up to date")
        return 0
    INDEX.write_text(generated, encoding="utf-8")
    print(f"wrote {INDEX.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
