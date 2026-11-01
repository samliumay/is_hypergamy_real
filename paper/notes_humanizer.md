# Humanizer pass, 2026-10-29

Covers the paper (`paper/main.tex`) and the blog post "Is Hypergamy Real?" (`content` of slug `is-hypergamy-real` in `~/Desktop/personal/umaysamlicom/src/app/config/blog.ts`).

## Ownership decision

The researcher asked for the humanizer on this side project's paper and blog post as a whole (2026-10-29). This is a blog side project, not a P-numbered research paper, and the text was agent-drafted. **Edits are allowed in every section, including the abstract, introduction and conclusion.** Every sentence changed in those three parts is still listed in the report below.

## Before copies

- Paper: `paper/old_versions/main_2026-10-29_before_humanizer.tex` (the project does not use `paper_<v>/` versions).
- Blog: the site's git history (branch `feat/how-turkic-is-anatolian-turkish` before the humanizer commit).

## Glossary (one term per concept)

| Concept | Term | Notes |
|---|---|---|
| the phenomenon | hypergamy | the four meanings stay distinct; do not merge them |
| marrying up in education or status | marrying up (verb: marry up) | the D1 label "partnering up" stays: D1 covers all couples, not only marriages |
| similar people pairing | homogamy; assortative mating where the source uses it | keep both, the paper defines them |
| what people say vs do | stated preference / revealed preference | never "declared", "actual", "real-world" preference |
| choosing a partner | partner choice | "mate preference" only in a cited study's own wording |
| the app | the Hypergamy Meter, then "the app" | not "the tool", "the meter", "the program" |
| apps for dating | dating apps | "online dating" only for the wider phenomenon as a source names it |
| trait cluster | the Dark Triad | |
| informal type | bad boys | |
| paternity | misattributed paternity; "non-paternity" only inside a quoted claim | hyphenated if kept |
| genetics | Y-chromosome (adjective and noun) | |
| anatomy | upper-body (adjective), upper body (noun) | |
| money | income for measured income, earning prospects / earning capacity as in each cited study | do not rotate |

Spelling: British (behaviour, analysed, favour, colour). No em dashes. `--` only for ranges.

## Baseline (before)

Paper build: 13 pages, 0 LaTeX warnings, no undefined references, no overfull boxes.

| Text | words | sentences | ; /sent | : /sent | ( /sent | nominal./sent | longest |
|---|---|---|---|---|---|---|---|
| paper `main.tex` | 6355 | 392 | 0.04 | 0.14 | 0.20 | 0.70 | 103 |
| blog post | 7248 | 480 | 0.01 | 0.41 | 0.39 | 0.53 | 56 |

Pattern hits, paper: contrast tag 9, rhetorical question 5, tier2 2 (key, significant), "rather than" 2, `--` 2, "overall," 1; repeated caveat (photo ratings, not real partner choice) in Beyond Status and Limitations.
Pattern hits, blog: rhetorical question 23, contrast tag 8, tier2 1, "rather than" 1, "overall," 1, "a range of" 1; term drift: upper-body/upper body, Y-chromosome/Y chromosome, non-paternity/nonpaternity, behaviors/behavioral (American minority).

## Report

Coordinator plus four parallel editors (paper: front to Evidence by Claim + Explanations to end; Beyond Status. Blog: Introduction to pairing + Meter to Conclusion; Beyond Money). The texts were split into part files so that no two editors touched the same file, then reassembled; the splits were checked byte for byte against the originals before editing.

### Scan, before -> after

| Text | words | sentences | ; /sent | : /sent | ( /sent | nominal./sent | longest |
|---|---|---|---|---|---|---|---|
| paper `main.tex` | 6355 -> 6337 | 392 -> 422 | 0.04 -> 0.01 | 0.14 -> 0.12 | 0.20 -> 0.18 | 0.70 -> 0.64 | 103 -> 44 |
| blog prose (without the frozen References list) | 6075 -> 6075 | 389 -> 409 | 0.02 -> 0.01 | 0.23 -> 0.14 | 0.27 -> 0.23 | 0.39 -> 0.37 | 56 -> 56 |
| blog, whole post | 7248 -> 7248 | 480 -> 500 | 0.01 -> 0.01 | 0.41 -> 0.34 | 0.39 -> 0.35 | 0.53 -> 0.51 | 56 -> 56 |

Pattern hits, paper: contrast tags 9 -> 8, rhetorical questions 5 -> 3, "key" removed, "rather than" 2 -> 0, repeated-caveat item 1 -> 0. Remaining `--` (2) are in captions/tables; "significant" refers to a test; "overall" is content.
Pattern hits, blog prose: rhetorical questions 5 -> 3 (the rest are bold run-in labels, a site convention). Remaining term-drift items are in the References list (cited titles, frozen).

Paper build after: 13 pages (same), no undefined references, BibTeX clean. Two new `Underfull \vbox while \output is active` warnings on the float-heavy pages after page 4 (full-width Tables II and III with a few lines of text). Not visible at reading size; left as is.

### Fidelity

Facts unchanged in both texts (numbers, citations, refs, labels, math, captions, tables, URLs, headings). Kept review items, one line each:

Paper
- "Why is debated" -> "The cause is debated": same claim, matches the abstract's wording.
- "twelve inputs, most of them self-rated" moved into its own sentence: split, both still present.
- "rather than long-term choice" -> "but not long-term choice": same contrast.
- Beyond Status: "All of this is ratings of photos and figures, not real partner choice." -> "All of these studies rate photos and figures." The full caveat stays in Limitations and at the section opening (this was the repeated-caveat item).
- "art.~16-11" split artefact: the article number is unchanged.
- "The clearest demonstration is the model of Grow and Van Bavel" -> "Grow and Van Bavel show this most clearly with ...": same superlative.
- "from abstracts or secondary summaries rather than full texts" -> "come only from abstracts or secondary summaries": same claim, stated positively.
- Reverted by the coordinator: the abstract's "a desktop joke whose hand-picked weights measure nothing" (stronger than the original) back to "a desktop toy whose hand-picked weights make it a joke, not a measurement".

Blog
- "Both are ratings of photos, not choices of real partners." (Build and strength) removed: it repeats the section's opening warning; removed by the coordinator to keep the total length from growing.
- "I found no study at all" -> "I found no study": same claim.
- "So where does 30% come from? From men who already doubted." -> "The 30% comes from men who already doubted.": question folded into a statement.
- French law articles moved from parentheses into the sentence ("under article 16-11 of the Civil Code"): numbers unchanged.
- "So what caused it? There are three main candidates." -> "There are three main candidates for the cause.": question folded.

### Blind reader check (seed 1)

- Blog: N = 12, after read as more human in 12/12 (100%), 0 meaning flags, 0 unclear. Pass. Cues named for the before version: colon reveals, stacked parentheses, packed list sentences.
- Paper: N = 12, after preferred in 10/12 (83%), 2 ties, 1 meaning flag (P08). The reader was right: the editor had renamed the D1 label "partnering up" to "marrying up", but D1 covers all couples, not only marriages. Reverted, and the glossary corrected. After the revert, the only meaning change is gone.

### Kinds of change, with examples

- Packed sentences split (semicolon chains, the contributions list read as one 103-word sentence).
  - "...women sent four times fewer messages, but their messages were twice as likely to connect, and couple similarity arose mostly from unanswered messages." -> three sentences.
- Asides and definitions moved out of parentheses into sentences.
  - "the Dark Triad: narcissism (grandiose, entitled), Machiavellianism (cynical, manipulative) and psychopathy (callous, impulsive) ([Paulhus and Williams, 2002](...))" -> "the Dark Triad ([Paulhus and Williams, 2002](...)). Narcissism is grandiose and entitled, Machiavellianism cynical and manipulative, and psychopathy callous and impulsive."
- Nominalizations and agentless passives turned into verbs with an actor.
  - "interest in much younger women was rarely turned into actual sex" -> "men rarely turned interest in much younger women into actual sex".
- Term drift fixed: Y-chromosome, British spelling, "the app".

### Changed sentences in the abstract, introduction and conclusion

Paper abstract: "...than men say, but the gap is small..." -> "...than men say. But the gap is small...". (The Hypergamy Meter sentence was reverted to the original.)
Paper introduction: "the master key to modern dating" -> "the hidden rule of modern dating"; "Mate-preference research asks a separate question, namely what people say they want ..., and dating research asks a third: whom people actually contact, like and marry" -> two sentences beginning "Preference research asks" and "Dating research asks"; "This paper sorts the claims and checks each one" -> "We sort the claims and check each one"; "It is a side project and it has a joke app attached" -> "This is a side project with a joke app attached"; the contributions list items became capitalised sentences; the last item became "A description of the Hypergamy Meter and of why its output means nothing."
Paper conclusion: "Is hypergamy real? Partly, and it depends which one you mean." -> "Hypergamy is partly real, depending on which meaning you take."; the effective-size sentence split in two; "Why is debated" -> "The cause is debated".
Blog introduction: the list after "the forums bring up next:" now starts "That meant ..."; "the Y chromosome" -> "the Y-chromosome"; "That came to 153 sources" -> "Altogether I read 153 sources".
Blog conclusion: point 7 split at "The cause is debated."; "viral app numbers" -> "viral dating-app numbers"; "Race on the apps" -> "Race on dating apps".

### Substance problems noticed, not fixed (for the author)

- Explanations: the Grow and Van Bavel model is described as calibrated for cohorts born 1921 to 2012. Cohorts born in 2012 cannot have married yet; check grow2015 (projection or typo).
- Beyond Status: "In 456 Y sequences" sits in the Karmin sentence, but Karmin reports the mtDNA/Y effective-size ratio; check that 456 is Karmin's Y sample.
- anderson2006's "30%" comes from a press release and has not been checked against the paper (already flagged in the caption).
- The paper's keywords still say "mate preferences" and "online dating"; the abstract sentence "Beyond status, women's stronger interest in status ..." reads oddly because "Beyond status" names a section.
- Blog: Figure 9's caption says "Y chromosome" (captions frozen); figure numbers and image file names differ (fig-14-n1c.png is Figure 11).

This pass changes style, not authorship; disclose AI assistance per the venue's policy.
