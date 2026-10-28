# Paper notes: is_hypergamy_real

Not an Obsidian note in the vault sense; build and provenance notes for `paper/`.

## Build

- `make pdf`: latexmk (pdflatex) into `build/`, copies `build/main.pdf` to `main.pdf`. BibTeX reads `../bibliography.bib` through `BIBINPUTS`, so there is one bibliography, not a copy.
- `make clean` removes `build/`; `make distclean` also removes `main.pdf`.
- Figures are TikZ/pgfplots sources in `figures/` (`concept_map.tex`, `stated_revealed.tex`, `reversal.tex`, `pipeline.tex`, `aim_up.tex`, `taller.tex`, `paternity.tex`, `goal_weights.tex`), `\input` by `main.tex`. No external images. The colour palette (`hgblue`, `hgverm`, `hgorange`, `hggreen`, `hgsky`, `hgink`) and the `hgshadow` / `hgbadge` styles are defined once in the `main.tex` preamble.
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

App numbers (base 60, all weights in Table IV, region factors 0.6 to 1.5, goal weights 0.5/1.2/1.5, tier thresholds 85/70/55/40/25, examples 80.24 and 22) are read from `../app/src/model.rs` as of 2026-10-28. The 80.24 example matches the unit test `default_profile_scores_as_expected` (60 + 12 + 4 x 1.3 x 1.2 + 2). The 22 example is the test's "worst" profile, computed by hand (60 - 22 - 8 x 1.5 - 2 - 2; looks, build, smarts average = 0, goal weight 1 for Undecided). Gendered bottom memes and their captions are from `../app/src/memes.rs` (`rock_bottom`). Demo names from `../app/src/demo.rs`; recomputed by hand (Python transcription of the weights, not by running the app): top 115.16 clamped to 100, niche 50.9, its-over 12, single-mother-arc 16, all in the tiers `demos_land_in_their_tiers` asserts. The paper does not print the demo scores. Size applies in full (no midpoint) to non-binary users who answer; the paper's Table IV note says so. Fig. 7 (goal weights) plots `goal_weight` exactly.

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

Not used (no notes.md entry with the number): the OkCupid "two-thirds of men's messages to the top third" and "11 times as many messages" figures in literature_review.md 11.6. Fig. 3 (reversal) is a hand-drawn schematic with no data.

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

## Figure log (2026-10-28)

- **Palette.** One Okabe-Ito (colour-blind safe) palette in the preamble: `hgblue` (dark in grey), `hgverm` and `hggreen` (mid), `hgorange` and `hgsky` (light), `hgink` for text and lines. Wherever two series must be told apart there is a second cue: hatching (orange bars), dashed or dotted lines, circle vs square markers, or a direct label. Checked with a greyscale render of pages 3 and 5.
- **Polished (no number, verdict or meaning changed).**
  - Fig. 1 `concept_map`: each claim is a card with a coloured header (D1 blue, D2 orange, D3 green, D4 vermilion) and a tinted body; the hub is a dark ellipse; soft shadows.
  - Fig. 3 `reversal`: blue solid line with circles (hypergamy), vermilion dashed line with squares (hypogamy), green dotted line (graduates); the gap between the two curves is shaded on each side of the crossover; light blue and hatched light vermilion backgrounds labelled "hypergamy more common" and "hypogamy more common"; the ILLUSTRATIVE badge moved above the plot so no curve runs under it.
  - Fig. 4 `pipeline`: numbered stage badges 1 to 6, tinted boxes (input light blue, steps blue, tier orange), a brace over steps 2 to 6 labelled "pure functions" (as the caption already says), men's card blue and women's card vermilion; all texts unchanged.
  - Fig. 7 `goal_weights`: blue solid bars (looks, build) vs hatched orange bars (education, income, smarts); values 0.5/1/1.2/1.5 unchanged.
- **Added (data only, numbers already in the paper and `../notes.md`).**
  - Fig. 2 `stated_revealed` (Section III-B): (a) stated sex gap, 0.30 points (financial prospects, women higher) and 0.27 points (attractiveness, men higher), walter2020; (b) after meeting, r = .40 (attractiveness) and r = .10 (earning prospects) for both sexes, sex difference about .03, eastwick2014 (secondary; the caption says it was read from an author copy).
  - Fig. 5 `aim_up` (Section III-G): men +26%, women +23% (bruch2018); arrow length proportional to the value.
  - Fig. 6 `taller` (Section IV-B): man not taller in 7.5% of 12,502 UK couples vs 10.2% under random pairing (stulp2013plos); stacked bars to 100%, only 7.5% and 10.2% printed.
  - Not added: the IQ-ceiling curve (secondary, press-only evidence) and an evidence-grade ladder (would repeat Table III).
- **Text changes in `main.tex`.** Three figure references only: "(Fig. 2)" after the eastwick2014 sentence, "(Fig. 5)" in the 80/20 paragraph, "(Fig. 6)" after the stulp2013plos sentence; three new figure floats with captions. Preamble: TikZ libraries `patterns`, `shapes.multipart`, `shapes.geometric`, `shadows`, `decorations.pathreplacing`; pgfplots libraries `groupplots`, `fillbetween`; palette and two styles.
- **Build.** `make pdf`: **10 pages** (was 9), no undefined references or citations, no overfull or underfull boxes, BibTeX 0 warnings. Every figure page inspected at 150 dpi; fixed during the pass: an 11pt overfull box in Fig. 6, value labels overlapping bars in Fig. 2, the ILLUSTRATIVE badge touching the hypogamy curve in Fig. 3, an in-plot note crossing a bar in Fig. 6. `make clean` run afterwards; `main.pdf` kept.

## Dark Triad, paternity and red-flags update (2026-10-28)

Sources: `../literature_review.md` sections 11.9 (red-flags row), 11.10 and 11.11 and the new verdict rows of section 2; grades from `../notes.md` ("Extension: Dark Triad" and "Extension: paternity"); app numbers from `../app/src/model.rs`. The paper title is unchanged on purpose: the researcher will handle it separately. Pre-update copy: `/tmp/claude-1000/-home-umaysamli-Desktop-research/4405169b-ba96-4543-a2d0-6336caa38f5a/scratchpad/hyper_v3_before/` (scratchpad, not kept).

### What changed

- **New Section IV-E "Personality: Do 'Bad Boys' Win?"** (`sec:darktriad`): definitions and the subclinical caveat, sex differences, short-term mating (two-sided: psychopathy carries the link in women, freyth2023), first impressions vs acquaintance (back2010, dufner2013, jauk2016, holtzman2010/2013; fading in paulhus1998, leckelt2015), the replication problem (carter2014 vs dragostinov2025, plus qureshi2016, rauthmann2013, watts2019, jonason2015birds, richards2024), infidelity (jonason2010poach, ataman2025, brewer2015, kardum2024), verdict "partly" (herold1999).
- **New Section IV-H "Paternity and Y-Chromosome History"** (`sec:paternity`): the 10 to 30% claim does not hold (populations about 1 to 2%; 30% only among doubting men who tested); context effects; the Y bottleneck signal (wilder2004, karmin2015), with female choice vs clan competition (zeng2018) as the open question; "17 women per man" reported as a misreading of effective population size; haplogroup N in Latvia explained by migration and founder effects (pliss2015, rootsi2007).
- **Table II**: 14 new rows in two labelled groups (Dark Triad, 10 rows; paternity and Y history, 4 rows), all from the review's section 2 verdict rows. A new Def. code "--" (outside D1 to D4) is explained in the caption. A dagger marks every row, old and new, with at least one source read only in a secondary summary or metadata. Table II is now a float page (`[p]`). At 37 rows it no longer fits as a top float, and as one it drifted behind the references, taking Tables III and IV with it.
- **New Fig. 7 `figures/paternity.tex`**: the viral 10 to 30% band against measured rates, as a dot-and-range plot in the palette style (claim = hatched vermilion band, population estimates = blue, selected sample = orange square). Values: greeff2015 0.9% (0.4 to 1.5), larmuseau2013 1 to 2%, larmuseau2019 0.4 to 5.9%, dahlen2022 1.7%, anderson2006 1.7 to 1.9% (abstract) and about 30% (press release, stated in the caption).
- **App section and Table IV**: red flags added to the input list; a weights paragraph; a Table IV row (+3 casual, cap +8; -5 long-term, cap -15; -8 marriage, cap -24; 0 undecided; same for every gender; not multiplied by a goal weight, note d); the Fig. 8 caption ("red flags ... weight 1"); the pipeline input box ("red flags"); "eleven inputs" -> "twelve"; "no personality" -> "personality reduced to a joke count of red flags". The marriage note "Red flags win the first date, not the marriage." is quoted from `red_flags_contribution`.
- **Abstract, keywords, contributions, Limitations and Conclusion** updated. Limitations lists the new secondary-read sources.
- **Layout**: Fig. 3 (`reversal.tex`) height changed from 4.9 cm to 4.7 cm to remove an underfull vbox on page 3 (a gap under the figure). Nothing else in that figure changed.

### Number check

- App: read from `model.rs` (`red_flags_contribution`, `goal_weight`, and the test `red_flags_help_casual_and_hurt_marriage`: Parade casual 8, AFew long-term -5, Parade marriage -24, None marriage 0, goal weight 1). The default profile has `RedFlags::None` (0 points), so 80.24 still holds. The "worst" example is Undecided (0 points), so 22 still holds. All four demos use `RedFlags::None`.
- Mismatch: the review's section 11.9 gives the marriage cap as -0.25 on a 0 to 1 scale (-25 points), but the app uses 3 x -8 = -24. The paper follows the app.
- Dark Triad and paternity numbers, all from `../notes.md`:

| Number(s) | Key | How notes.md read it |
|---|---|---|
| d = .26, 470,846 people | grijalva2015 | abstract |
| 11 world regions, 30,470 people (53 nations in Table II) | schmitt2017narc | abstract |
| N = 495 | freyth2023 | abstract |
| 90 adults, 691 dates | jauk2016 | full text |
| seventh meeting | paulhus1998 | **secondary**; daggered, Limitations |
| three weeks | leckelt2015 | abstract |
| 128 students, mean age 19; 4.44 vs 3.34 on 6 points; d = 0.94 | carter2014 | full text |
| older women prefer low DT | qureshi2016 | **secondary / metadata**; "reportedly", daggered, Limitations |
| N = 475 and 794 | dragostinov2025 | abstract and full text |
| 11 studies; r = .13, .23, .25 | ataman2025 | abstract |
| 205 couples | kardum2024 | abstract |
| 165 Canadian university women; "more than half" | herold1999 | abstract |
| 1 to 2% per generation | larmuseau2013 | abstract |
| 0.9% (95% CI 0.4 to 1.5%), 330 years, 23 families | greeff2015 | abstract |
| 1.7%, about 1% today, nearly two million | dahlen2022 | abstract |
| 67 studies; 1.7 to 1.9% | anderson2006 | abstract |
| about 30% | anderson2006 | **secondary** (press release); stated in text, caption, Limitations |
| 0.8 to 30%, median 3.7% | bellis2005 | abstract |
| 0.4% to 5.9%, 500 years | larmuseau2019 | abstract |
| about twice as deep | wilder2004 | abstract |
| last 10,000 years | karmin2015 | abstract |
| "17 women per man" | karmin2015 | superseded: see the re-check update below (it is in the full text) |
| over 85%, N1c/R1a/I1 | pliss2015 | abstract (the 45% N1c poster figure is not used) |
| 12,000 to 14,000 years ago | rootsi2007 | abstract |

### Build

`make pdf`: **12 pages** (was 10), no undefined references or citations, no overfull or underfull boxes, BibTeX 0 warnings, 122 references. Inspected: page 5 (the Table II float page), pages 7 and 8 (new subsections, Fig. 7), page 10, and the pipeline figure. Fig. 7 first had overlapping two-line row labels and was redrawn with one-line labels. `make clean` was run; `main.pdf` is kept.

Known cosmetic issue, not fixed because it is outside `paper/`: IEEEtran lowercases some words in the new titles, e.g. "dark triad" and "machiavellianism" in paulhus2002 and "uk, fiji" and "dark triad" in richards2024. The fix is to brace those words in `../bibliography.bib`, as was done for kreager2014.

## Humanizer log, Dark Triad and paternity update (2026-10-28)

- **Ownership.** Agent-drafted text in every version, so all of it was editable. Scope: only the text changed in this update. Before copy: `.../scratchpad/hyper_v3_prehum/main.tex`.
- **Glossary.** As before, plus: Dark Triad, high scorer, short-term / casual, misattributed paternity, effective population size, haplogroup, red flags.
- **Scan before / after (whole paper).** 5960 / 5960 prose words; 371 / 376 sentences; semicolons 0.04 / 0.04, colons 0.11 / 0.10, parentheses 0.20 / 0.20 per sentence; nominalizations 0.69 / 0.68. The remaining scan flags are in older text and were kept on purpose (see the earlier logs).
- **Changes (5).**
  - Abstract: a long compound sentence split ("..., and predict more infidelity later, so" -> "... They also predict more infidelity later, so").
  - First impressions: ", and narcissism raised" -> ". Narcissism also raised".
  - Verdict: a three-clause comma splice with a parenthesis became two sentences ("It runs both ways, since narcissistic women are chosen more often too").
  - Paternity: the mid-sentence press-release aside moved into its own sentence ("That 30% comes from a press release, unchecked against the paper.").
  - Bottleneck: a semicolon antithesis split into two sentences.
- **Fidelity.** Exit 2, facts identical. Three review items were kept. "Since" marks the same evidence the parenthesis gave before, so the claim is unchanged. In the other two items the 30% moved to the next sentence and still describes the same figure.
- **Blind reader check.** Seed 1, N = 3 (only three paragraph pairs changed), one fresh reader subagent. The after version was preferred in 3/3 pairs (100%), with no meaning change and no after version flagged unclear. PASS.
- **Substance problems noticed, not fixed.** Several Dark Triad and paternity numbers rest on secondary summaries (daggered in Table II, listed in Limitations). The "17 women" ratio and the 30% figure were not read in the original papers.

This pass changes style, not authorship; disclose AI assistance per the venue's policy.

## Re-check update: bad boys split, 17:1, N1c, France (2026-10-28)

The researcher pushed back that the old wording was too dismissive ("do not be protective against women, be natural"). The review's sections 11.10 and 11.11, its verdict rows and summary callout, and `../notes.md` were re-checked; the paper now follows them.

- **Section IV-E (Dark Triad)** rewritten by sub-claim, plain and two-sided: partner counts *exists* (borrazleon2021, schmitt2017narc, lee2013sex, jonason2009, freyth2023); real first meetings *exists* (dufner2013 incl. the street study, jauk2016, back2010; mostly looks and boldness); fictional profiles *mixed* (for: carter2014, aitken2013, haslam2015; against: brewer2018faces, lyons2016faces, dragostinov2025, qureshi2016); dominance *partly* (sadalla1987, jensencampbell1995, burger1999, snyder2008); Machiavellianism/psychopathy as such *mostly does not exist* except for raters high themselves (jonason2015birds, watts2019, marcinkowska2015); long term *does not exist*; nice guys finish last *partly* (herold1999, urbaniak2003, mcdaniel2005, jensencampbell1995); verdict *split*; men show the same pattern toward narcissistic women. jonason2012choices is no longer cited (dropped for space; its point is carried by jonason2009 and lee2013sex).
- **Section IV-H (paternity)**: intro now says the 10 to 30% claim fails and the "fewer men reproduced" claim holds with a debated cause. New paragraphs: France (no peer-reviewed rate; Civil Code art. 16-11 and Penal Code art. 226-28 cited as text, no bib entry; nearest data larmuseau2013, larmuseau2019; French rate stated as an inference), the 2:1 and 17-fold results quoted from karmin2015 with what Ne means, corroboration (poznik2016, batini2015), Baumeister 2:1 measured vs 80/40 illustration (baumeister2010), causes (karmin2015, zeng2018, batini2015), and N1c (rootsi2007 Supplementary Table 1, preussner2025, ilumae2016, lappalainen2008, pliss2015). All "misreading" and "known only from press" wording for 17:1 removed (text, Limitations, notes table above).
- **Tables**: the personality and paternity rows moved out of Table II into a new **Table III** (`tab:darkpat`) because Table II no longer fitted one page; the old Tables III and IV are now IV and V. New rows: first meetings, fictional profiles (mixed, for/against sources), dominance, nice guys, France (unclear), fewer men reproduced (exists, strong), 17:1 (exists as an effective-size ratio), cause (unclear), Baumeister 2:1 vs 80/40 (partly), N1c common (exists, strong), founder success (exists), few living men father most children (does not exist). Tables I to III are now `\scriptsize` (Table II arraystretch 1.07, I and III 1.1) to stay at 13 pages.
- **Abstract, contributions item, Conclusion, Limitations** updated. Limitations adds the secondary-read sources (burger1999, urbaniak2003, mcdaniel2005, marcinkowska2015, aitken2013, lyons2016faces title only, haslam2015 and dufner2013 street-study sample sizes, Baumeister's 80/40) and states that the French texts are cited as law.
- **Layout**: the Table II/III source block now sits before "Short-Term Versus Long-Term Partners" so page 5 no longer has a near-empty column. `\usepackage{eurosym}` added for the euro sign.
- **Figure**: an N1c bar chart was drafted (`figures/n1c.tex`) but removed, because it pushed the paper to 14 pages. The numbers are in the text and Table III. Fig. 7 (paternity) unchanged.

| Number(s) | Key | How notes.md read it |
|---|---|---|
| 61 men, street study (not printed) | dufner2013 | **secondary**; daggered, Limitations |
| N = 146 (not printed) | haslam2015 | **secondary**; Limitations |
| cads vs dads | aitken2013 | abstract via search summary; daggered, Limitations |
| N = 794 | dragostinov2025 | full text |
| 1,962 Finnish women | marcinkowska2015 | **secondary** (repository abstract); daggered |
| mtDNA Ne more than twice Y Ne; 8,000 to 4,000 years ago; "up to 17-fold" (quoted) | karmin2015 | full text |
| 456 sequences | karmin2015 | abstract |
| 64% of a 334-man sample; Bronze Age | batini2015 | abstract |
| 80% / 40% | baumeister2010 | **secondary** (quotations); Limitations |
| 58.2%, 41.9%, 36.7%, 30.6% | rootsi2007 | Supplementary Table 1 (full text) |
| 64% of 1,802 Finnish men | preussner2025 | abstract |
| VL29 over a third; split about 5,000 years ago | ilumae2016 | full text |
| over 85% (N1c, R1a, I1) | pliss2015 | abstract |
| 1 year prison, 15,000 euro fine; art. 16-11, 226-28 | French Civil and Penal Code | legal texts via the review; not in bibliography.bib |

### Build

`make pdf`: **13 pages**, 140 references, no undefined references or citations, no overfull boxes, BibTeX 0 warnings. One underfull vbox remains on page 5 (a float-heavy page: Fig. 4, Table II, Figs. 5 and 6 and three text lines); inspected, no visible gap. IEEEtran now prints "Dark Triad", "Machiavellianism", "UK" and "Fiji" capitalised (the titles in `../bibliography.bib` are brace-protected). Inspected pages 5 to 8. `make clean` run; `main.pdf` kept.

## Humanizer log, re-check update (2026-10-28)

- **Ownership.** Agent-drafted text, all editable. Scope: the text changed in this update. Before copy: `.../scratchpad/hum_before/main.tex` (the drafted version); the pre-update copy is `.../scratchpad/hyper_v4_prehum/`.
- **Glossary.** As before, plus: partner counts, first meetings, fictional profiles, effective population size ($N_e$), haplogroup N1c, founding male lines.
- **Scan.** 6348 / 6348 prose words; 386 / 392 sentences; semicolons 0.04 / 0.04, colons 0.15 / 0.14, parentheses 0.20 / 0.20 per sentence; nominalizations 0.71 / 0.70.
- **Changes (6).** Sentence splits only: abstract ("..., but no study finds" -> ". No study finds"); fictional profiles (a four-part "Against" list split in two); nice guys (semicolon split); causes (a colon-after-citation list became its own sentence, "...all raise that variance"); N1c (colon split); conclusion (semicolon split).
- **Fidelity.** Exit 2, facts identical; the one review item is a sentence split with the same claims.
- **Blind reader check.** Seed 1, N = 12 sampled paragraphs, only 2 of them changed. The after version was preferred in both changed pairs; the 10 unchanged pairs were ties, so the formal score is 2/12 (17%, FAIL by the 70% rule). No meaning change and nothing flagged unclear. Read as a light pass on already plain text, not a regression.
- **Substance problems noticed, not fixed.** Several Dark Triad numbers still rest on secondary summaries (daggered, listed in Limitations). The France row is an inference from neighbours. The 30% of anderson2006 is still from a press release.

This pass changes style, not authorship; disclose AI assistance per the venue's policy.
