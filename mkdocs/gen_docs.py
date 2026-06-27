"""Copy source files from their canonical locations into the mkdocs docs directory.

This script is used by the mkdocs-gen-files plugin so that no symlinks are
needed — files are materialised at build time (mkdocs serve or mkdocs build).

Uses mkdocs_gen_files.open() so that generated files are properly tracked by
MkDocs (visible to link checker, search index, etc.).
"""

from pathlib import Path

import mkdocs_gen_files

REPO_ROOT = Path(__file__).resolve().parent.parent

# Destination (relative to docs dir) → source (relative to repo root)
FILE_MAP: dict[str, str] = {
    "specification.md": "docsdog-spec/specification.md",
    "scan.schema.json": "docsdog-spec/scan.schema.json",
    "relationship.schema.json": "docsdog-spec/relationship.schema.json",
    "scan-example.json": "docsdog-spec/scan-example.json",
}

# Assets directory — copy every file under it
ASSETS_SRC = REPO_ROOT / "assets"
ASSETS_DST_PREFIX = "assets"


def copy_assets() -> None:
    """Copy all files from the assets/ directory tree into the docs dir."""
    for src in sorted(ASSETS_SRC.rglob("*")):
        if not src.is_file():
            continue
        rel = src.relative_to(ASSETS_SRC)
        dst = f"{ASSETS_DST_PREFIX}/{rel}"
        with mkdocs_gen_files.open(dst, "wb") as df:
            df.write(src.read_bytes())
        print(f"  ✓  {dst}")


def main() -> None:
    for dest_rel, src_rel in FILE_MAP.items():
        src = REPO_ROOT / src_rel
        with mkdocs_gen_files.open(dest_rel, "wb") as df:
            df.write(src.read_bytes())
        print(f"  ✓  {dest_rel}")

    copy_assets()


main()
