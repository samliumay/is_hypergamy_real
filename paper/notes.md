# Paper notes: is_hypergamy_real

Not an Obsidian note in the vault sense; build and provenance notes for `paper/`.

## Build

- `make pdf`: latexmk (pdflatex) into `build/`, copies `build/main.pdf` to `main.pdf`. BibTeX reads `../bibliography.bib` through `BIBINPUTS`, so there is one bibliography, not a copy.
- `make clean` removes `build/`; `make distclean` also removes `main.pdf`.
- Figures are TikZ/pgfplots sources in `figures/` (`concept_map.tex`, `reversal.tex`, `pipeline.tex`), `\input` by `main.tex`. No external images.
- Final build (2026-10-28): 6 pages, no undefined references or citations, no overfull or underfull boxes, BibTeX 0 warnings.
- One formatting-only change to `../bibliography.bib`: `kreager2014` title now starts `{``Where}` so IEEEtran does not lower-case "Where" inside the quoted title. No content changed.

## Sources of numbers

Every number in the text is taken from `../notes.md` (annotated bibliography, checked 2026-10-28). Grade of each figure as `../notes.md` marks it:

| Number(s) in paper | Key | How notes.md read it |
|---|---|---|
| 37 samples, 33 countries, N = 10,047 | buss1989 | abstract |
| 45 countries, N = 14,399; 0.30 and 0.27 points on 1 to 7; D = 0.73 (5 items); partners 2.4 years older / 2.3 younger | walter2020 | full text |
| D = 2.41 (18 items), 22.8% overlap, 92.2% classification | conroybeam2015 | abstract |
| 97 studies; r = .40 (attractiveness), r = .10 (earning prospects), sex difference about .03 | eastwick2014 | **secondary** (author PDF); flagged in Limitations |
| 24 fake profiles, 3,600 evaluations | neyt2019 | abstract |
| 1960 to 2003 homogamy rise | schwartz2005 | abstract |
| 56 countries | esteve2012 | abstract |
| 420 samples, 120 countries, 1960 to 2011 | esteve2016 | abstract/secondary |
| 1980 vs 2008 to 2012 | qian2017 | abstract |
| 58% higher income; 90% vs 70% employed | lichter2020 | abstract |
| N = 2,655 | antfolk2017 | abstract |
| around 2013 (online overtakes friends) | rosenfeld2019 | abstract |
| about 25% (26% and 23%); desirability peaks near 50 / at 18 | bruch2018 | full text |
| 8,259 men, 6,274 women; 4 times fewer messages; twice as likely | kreager2014 | abstract |
| 0.6% vs 10% match rates | tyson2016 | **secondary**; flagged in text and Limitations |
| "80% of men below medium" | rudder2009 | **secondary** (archived company blog, grade D); flagged |
| 12 countries, cohorts 1921 to 2012 | grow2015 | full-text summary |
| zentner2012 equality result (no number quoted) | zentner2012 | **secondary** (press release); flagged in Limitations |

Grey "Gini of likes" figures (Medium post, Hinge via Quartz) are mentioned without numbers and not cited: they are not in `bibliography.bib`.

App numbers (base 60, weights, region factors 0.6 to 1.5, tier thresholds 85/70/55/40/25, examples 79.2 and 22) are read from `../app/src/model.rs` (the 79.2 example matches its unit test `default_profile_scores_as_expected`; the 22 example is the test's "worst" profile, computed by hand: 60 - 22 - 8 x 1.5 - 2 - 2). Fig. 2 (reversal) is a hand-drawn schematic with no data.

## Humanizer log (2026-10-28)

- **Ownership.** The whole paper was drafted by an agent in this session; there was no researcher-written text, so every section was editable. Pre-humanizer copy: `/tmp/claude-1000/-home-umaysamli-Desktop-research/4405169b-ba96-4543-a2d0-6336caa38f5a/scratchpad/hyper_paper_before/` (scratchpad, not kept in the repo).
- **Glossary.** hypergamy (he ranks higher), hypogamy (she ranks higher), homogamy / like with like, stated preference (D2), revealed choice (D3), supply, the Hypergamy Meter, contribution, tier, meme card.
- **Scan before / after.** 3000 / 2997 prose words; 180 / 185 sentences; semicolons 0.09 / 0.06, colons 0.13 / 0.12, parentheses 0.23 / 0.22 per sentence; nominalizations 0.78 / 0.76 per sentence; participial tails 1 / 0; summary closers 1 / 0; term drift (out-number / outnumber) 1 / 0. Remaining flags kept on purpose: two "X, not Y" jokes ("a joke, not a measurement", "a friendly message, not a crash"), two real contrasts with "rather than", two rhetorical questions that frame sections.
- **Kinds of change (4 edits, light pass; the draft was already near the targets).**
  - Packed semicolon sentence split, participial tail removed: "..., echoing a stated difference that is real but small; for women the education curve ... (fewer ``equal or higher'' partners ...); and the age curve ..." became three sentences.
  - Summary closer: "So the difference shows up in every country studied, and whether equality shrinks it is still open." -> "The difference appears in every country studied. Whether equality shrinks it is still open."
  - Colon/semicolon list unpacked: "Our reading: a ...; its size may depend on context; and actual pairing ..." -> two sentences, hedge "may" kept.
  - Term drift: "out-number" -> "outnumber".
- **Fidelity.** Facts identical (numbers, citations, refs, labels, math, captions, tables). One review item kept: "So ... shows up" -> "appears" (connective dropped, same claim).
- **Blind reader check.** Seed 1, N = 12, one fresh reader subagent. Score 3/12 = 25%, formally FAIL, because only 4 paragraphs changed and the 8 identical pairs count as ties. On the changed pairs the revision was preferred 3 of 3 (the fourth changed pair differs only by the spelling outnumber/out-number, a tie). No meaning change flagged; the before version of the weights paragraph was flagged unclear. Reader cues: semicolon list after a colon, participial tail, mid-sentence parenthesis, "So ..., and ..." wrap-up. Reader remark kept: in the split "our reading" sentence the last sentence is slightly less clearly part of our reading; left as is to avoid adding a hedge. Light pass by design, not repeated.
- **Substance problems noticed, not fixed.** None found beyond the secondary-source caveats listed above.

This pass changes style, not authorship; disclose AI assistance per the venue's policy.
