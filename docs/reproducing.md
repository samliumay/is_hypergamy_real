# Reproducing

## The Hypergamy Meter

Needs a recent stable Rust toolchain (edition 2024, so Rust 1.85 or newer). The only dependency is
`eframe` 0.37.0, pinned in `app/Cargo.toml`.

```sh
cd app
cargo run --release                     # open the app
cargo run --release -- --demo top       # open it already scored with a preset
```

| `--demo` name | Lands in |
|---------------|----------|
| `top` | Top of the market |
| `niche` | Niche, but with a loyal fan base |
| `its-over` | Your life is over, men's meme card ("It's over") |
| `single-mother-arc` | Your life is over, women's meme card ("Single mother arc") |

An unknown name prints the valid names and exits with status 2.

### Checks

```sh
cd app
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`cargo test` runs the unit tests of the scoring model in `model.rs` and a test in `demo.rs` that each preset
lands in the tier its name promises.

## The paper

Needs a TeX Live with `latexmk`, `pdflatex`, BibTeX and the IEEEtran class.

```sh
cd paper
make pdf        # latexmk (pdflatex) into build/, copies build/main.pdf to main.pdf
make clean      # removes build/
make distclean  # also removes main.pdf
```

BibTeX reads `../bibliography.bib` through `BIBINPUTS`, so there is one bibliography for the review and
the paper. The figures are TikZ/pgfplots sources in `paper/figures/`; there are no external images. More
build notes are in [`paper/notes.md`](../paper/notes.md).

## Blog figures

`docs/blog_figures/make_figures.py` draws the blog post's diagrams as SVG (standard library only).
`render.sh` reruns it and turns every `fig-*.svg` into a 1200 px wide PNG with `rsvg-convert` (from librsvg).

```sh
python3 docs/blog_figures/make_figures.py          # SVGs next to the script
sh docs/blog_figures/render.sh [output_dir]        # SVGs + PNGs (default: docs/blog_figures/)
```

Every number in the figures comes from the blog post (which cites the review and `notes.md`) or, for the
Meter, from `app/src/model.rs`. Schematic shapes carry an "Illustrative, not data" badge.
