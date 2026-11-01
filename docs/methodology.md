# Methodology

> [!NOTE]
> This is a literature review, not a new study. There is no data set and no statistical analysis in the
> repo: every number comes from published sources.

## Question

Is *hypergamy* (partnering with someone of higher status, income or education) a real, measurable
pattern?

The review's answer is "it depends on which hypergamy you mean", because the word is used for four
different claims. The review keeps them apart:

| # | Meaning |
|---|---------|
| D1 | **Partnering up in status**: the woman's partner ranks above her on education, income, occupation, class or age |
| D2 | **Stated preferences**: women say they weigh status or earnings more than men do |
| D3 | **Revealed preferences and actual matching**: what people do when choosing |
| D4 | **The strong online claim**: most women pursue a small top fraction of men ("80/20") |

What would count as evidence for each, and which data it needs, is in section 1 of
[`literature_review.md`](../literature_review.md#1-definitions).

## How sources were collected and checked

- [`notes.md`](../notes.md) is the annotated bibliography: one entry per source, with claim, method and
  sample, finding, and relevance. Keys match [`bibliography.bib`](../bibliography.bib).
- Every citation was checked on 2026-10-28 against Crossref, Semantic Scholar, RePEc or Open Library.
- Each number is marked by where it was read: *(abstract)*, *(full text)*, or *(secondary)* when it was
  taken from a summary and not checked in the original.

## Evidence grades

Each source gets a grade in `notes.md`:

| Grade | Meaning |
|-------|---------|
| A | Peer-reviewed, large sample or meta-analysis, replicated |
| B | Peer-reviewed, single study or contested |
| C | Conference paper, working paper or book |
| D | Company blog or anonymous post (cited only to trace where an online claim came from) |

Each verdict in the review also gets a strength: **Strong**, **Moderate** or **Weak** (defined at the top
of section 2 of the review).

## Where the results live

| What | Where |
|------|-------|
| Short answer | The summary callout at the top of [`literature_review.md`](../literature_review.md) |
| Verdict per claim, with strength and main sources | Section 2, [verdict table](../literature_review.md#2-verdict-table) |
| Methodological pitfalls in the studies | Section 7 of the review |
| The paper | [`paper/main.pdf`](../paper/main.pdf) (IEEE format; source `paper/main.tex`) |
| Which source and grade each number in the paper comes from | [`paper/notes.md`](../paper/notes.md), "Sources of numbers" |

## The Hypergamy Meter is not a result

The app's weights are hand-picked and **illustrative, not estimates**. The comment block at the top of
`app/src/model.rs` says which findings they loosely echo, in direction only. Section 11.9 of the review
lists the parameters for the joke model.
