# Paper notes: is_hypergamy_real

Not an Obsidian note in the vault sense; build and provenance notes for `paper/`.

## Build

- `make pdf`: latexmk (pdflatex) into `build/`, copies `build/main.pdf` to `main.pdf`. BibTeX reads `../bibliography.bib` through `BIBINPUTS`, so there is one bibliography, not a copy.
- `make clean` removes `build/`; `make distclean` also removes `main.pdf`.
- Figures are TikZ/pgfplots sources in `figures/` (`concept_map.tex`, `reversal.tex`, `pipeline.tex`, `goal_weights.tex`), `\input` by `main.tex`. No external images.
- `bstcontrol.bib` (paper-local, not a source) holds the IEEEtran control entry `IEEEexample:BSTcontrol`: author lists are cut to 3 names + "et al." to keep the reference list short. `main.tex` loads it with `\bstctlcite` and `\bibliography{bstcontrol,bibliography}`.
- First build (2026-10-28): 6 pages. "Beyond status" update (2026-10-28): **9 pages** (about 7 pages of text, 2 of references, 83 entries), no undefined references or citations, no overfull or underfull boxes, BibTeX 0 warnings. The 8-page target was not reached: the new section added 32 references (about 0.8 page) and two floats; reaching 8 would need cutting about one page of review content or references.
- Table I was 32pt too wide in the first build (missed then); fixed in the update with `\tabcolsep` 4pt and narrower columns. Table IV (weights) was 3.9pt too wide; columns narrowed.
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

App numbers (base 60, all weights in Table IV, region factors 0.6 to 1.5, goal weights 0.5/1.2/1.5, tier thresholds 85/70/55/40/25, examples 80.24 and 22) are read from `../app/src/model.rs` as of 2026-10-28. The 80.24 example matches the unit test `default_profile_scores_as_expected` (60 + 12 + 4 x 1.3 x 1.2 + 2). The 22 example is the test's "worst" profile, computed by hand (60 - 22 - 8 x 1.5 - 2 - 2; looks, build, smarts average = 0, goal weight 1 for Undecided). Gendered bottom memes and their captions are from `../app/src/memes.rs` (`rock_bottom`). Demo names from `../app/src/demo.rs`; recomputed by hand (Python transcription of the weights, not by running the app): top 115.16 clamped to 100, niche 50.9, its-over 12, single-mother-arc 16, all in the tiers `demos_land_in_their_tiers` asserts. The paper does not print the demo scores. Size applies in full (no midpoint) to non-binary users who answer; the paper's Table IV note says so. Fig. 4 (goal weights) plots `goal_weight` exactly.

### Beyond status numbers (Section IV, Tables II and III)

All from `../notes.md` "Extension" part and `../literature_review.md` section 11 (grades A-D as in notes.md):

| Number(s) | Key | How notes.md read it |
|---|---|---|
| 93 students, 24 traits, four commitment levels | kenrick1990 | abstract |
| N = 382, ages 18 to 54; 6% sex, 4% relationship | asendorpf2011 | abstract |
| N = 584 | jones2018 | abstract |
| N = 157 | stern2020 | abstract |
| N = 650; 21 cm taller / 8 cm shorter | stulp2013paid | **secondary** (repository abstract); flagged in Limitations |
| 12,502 couples; r = .18; 7.5% vs 10.2%; about 90% by chance | stulp2013plos | abstract and full text |
| "6'3" 65% more" | 2006 Hitsch draft | **secondary**, grade C; cited via hitsch2010qme as dropped |
| over 70% of variance | sell2017 | abstract |
| legs about 5% longer | sorokowski2008 | **secondary**; flagged |
| figure study details (size about as large as height; torso strongest) | mautz2013 | abstract, partly **secondary**; flagged |
| 75 women; 16.3 vs 16.0 cm | prause2015 | abstract |
| 15,521 men; 13.1 cm (SD 1.7) | veale2015 | abstract (13.12, SD 1.66 rounded) |
| 52,031 adults; 85% vs 55% | lever2006 | abstract |
| 90th percentile, about IQ 120 | gignac2018 | **secondary** (press); flagged |
| 214 young adults | gignac2019 | abstract |
| 199 studies; r = .44 IQ, .55 education | horwitz2023 | full text |
| N = 883, born 1921; OR 0.42 | taylor2005 | **secondary**; flagged |
| 0.6% vs 10% | tyson2016 | secondary, grade C |
| Tinder Gini 0.58; Hinge 0.54 or 0.73 | Medium post; Quartz/Quillette | grade D, not in bib, not cited |
| 30 of 31 race effects; 10%; 12% | hitsch2010qme | full text (author version) |
| 6,070 profiles | robnett2011 | abstract |
| intermarriage tripled 1980 to 2008 | qian2011 | abstract |
| 2009 to 2014 no decline | rudder2014race | **secondary**, grade D |

Not used (no notes.md entry with the number): the OkCupid "two-thirds of men's messages to the top third" and "11 times as many messages" figures in literature_review.md 11.6. Fig. 2 (reversal) is a hand-drawn schematic with no data.

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

## Humanizer log, "Beyond status" update (2026-10-28)

- **Ownership.** Agent-drafted text throughout (first version and this update), so all of it was editable. Scope: the changed text only. Before copy of this pass: `/tmp/claude-1000/-home-umaysamli-Desktop-research/4405169b-ba96-4543-a2d0-6336caa38f5a/scratchpad/hyper_v2_prehum/` (the copy before the whole update is `.../scratchpad/hyper_paper_v2_before/`; scratchpad, not kept).
- **Glossary.** As in the first pass, plus: short-term / long-term partner (casual / committed), goal weight, meme card, blog-grade (grade D), documented bias.
- **Scan before / after (whole paper).** 4477 / 4476 prose words; 276 / 277 sentences; semicolons 0.05 / 0.04, colons 0.11 / 0.10, parentheses 0.20 / 0.20 per sentence; nominalizations 0.71 / 0.71. The draft was already under every target, so this was a light pass.
- **Changes kept (3).** Semicolon split ("found the same [cite]; both are stated ratings" -> two sentences); repeated subject ("The score is ... The score is mapped" -> "This score is mapped"); "Then you press" -> "You press".
- **Changes reverted.** `$S$ is mapped` (added a math token, fidelity exit 1) -> "This score is mapped". Deleting the repeated caveat "All of this is ratings of photos and figures, not real partner choice." was flagged as a scope change by the blind reader and is reverted (the caveat also appears in the section intro and Limitations; kept on purpose).
- **Fidelity.** Final run: exit 0, facts identical, 0 review items.
- **Blind reader check.** Seed 1; only one paragraph pair counted as changed, so N = 1, one fresh reader subagent. After version preferred 1/1, but the dropped caveat was flagged as a meaning change -> formal FAIL; resolved by reverting that deletion. Not repeated, by protocol.
- **Substance problems noticed, not fixed.** The paper is 9 pages, over the 8-page aim. Several Section IV numbers rest on secondary summaries (listed in the table above and in Limitations).

This pass changes style, not authorship; disclose AI assistance per the venue's policy.
