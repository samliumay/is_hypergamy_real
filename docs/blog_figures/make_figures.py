"""Builds every diagram of the blog post "Is Hypergamy Real?" as SVG.

Run `python3 make_figures.py` from anywhere: the SVGs land next to this file,
and `render.sh` turns them into the PNGs the site serves. Figures 9 to 12 of
the post are app screenshots and are not built here.

Every number drawn here comes from the blog post (which cites
literature_review.md and notes.md) or, for the Hypergamy Meter, from
app/src/model.rs. Schematic shapes carry an "Illustrative, not data" badge.

All figures share one look: a warm opaque paper card (so they read the same on
the site's light, sepia and dark themes), a serif title, sans-serif body text
and one verdict palette.
"""
from __future__ import annotations

import math
import pathlib
from xml.sax.saxutils import escape

OUT = pathlib.Path(__file__).parent
W = 1200

# ---------------------------------------------------------------- palette --
BG = "#faf8f3"        # paper card
PANEL = "#f0ede4"     # inner panels, zebra rows
PANEL_2 = "#e7e3d7"
RULE = "#d9d5c9"
INK = "#1b1f1c"
MUTED = "#5b6159"
FAINT = "#8e938a"
ACCENT = "#1f6a4a"    # site accent green
ALERT = "#b8322b"

# verdicts (meaning unchanged from the original figures)
V_EXISTS = "#2e8555"
V_PARTLY = "#c8820f"
V_MOSTLY_NOT = "#c4553a"
V_NOT = "#a3282a"
V_UNCLEAR = "#6c706a"
V_NONE = "#a3a69f"
V_BIAS = "#5b4b9a"

# two neutral series colours (site chart-a / chart-b)
SER_A = "#2a74a8"     # blue
SER_B = "#c46a1c"     # orange

SERIF = 'font-family="Noto Serif, DejaVu Serif, serif"'
SANS = 'font-family="Noto Sans, DejaVu Sans, sans-serif"'


# ---------------------------------------------------------------- helpers --
def t(x, y, s, size=19, fill=INK, font=SANS, weight="normal", anchor="start",
      style="normal", spacing=None, extra=""):
    ls = f' letter-spacing="{spacing}"' if spacing is not None else ""
    return (f'<text x="{x:.1f}" y="{y:.1f}" {font} font-size="{size}" '
            f'font-weight="{weight}" font-style="{style}" fill="{fill}" '
            f'text-anchor="{anchor}"{ls} {extra}>{escape(s)}</text>')


def rect(x, y, w, h, fill, rx=0, extra=""):
    return f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" rx="{rx}" fill="{fill}" {extra}/>'


def pill(cx, cy, w, h, fill, label, size=18, color="#fff"):
    return (rect(cx - w / 2, cy - h / 2, w, h, fill, rx=h / 2)
            + t(cx, cy + size * 0.36, label, size, color, SANS, "bold", "middle"))


def header(kicker, title, subtitle=None, badge=False):
    out = [rect(40, 34, 44, 5, ACCENT, rx=2.5),
           t(40, 64, kicker.upper(), 15, ACCENT, SANS, "bold", spacing=2.2),
           t(40, 106, title, 36, INK, SERIF, "bold")]
    if subtitle:
        out.append(t(40, 140, subtitle, 19, MUTED))
    if badge:
        out.append(illustrative_badge(W - 40, 50))
    return out


def illustrative_badge(right, y):
    label = "ILLUSTRATIVE, NOT DATA"
    w = 262
    return (rect(right - w, y - 20, w, 34, "#fbeceb", rx=17,
                 extra=f'stroke="{ALERT}" stroke-width="1.5"')
            + t(right - w / 2, y + 2, label, 14, ALERT, SANS, "bold", "middle", spacing=1.4))


def footer(y, lines):
    out = [f'<line x1="40" y1="{y - 30}" x2="{W - 40}" y2="{y - 30}" stroke="{RULE}" stroke-width="1.5"/>']
    for i, line in enumerate(lines):
        out.append(t(40, y + i * 26, line, 16, MUTED))
    return out


def svg(h, body):
    return "\n".join([
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{h}" viewBox="0 0 {W} {h}">',
        "<defs>",
        '<marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">'
        f'<path d="M0,0 L10,5 L0,10 z" fill="{INK}"/></marker>',
        '<marker id="arrA" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">'
        f'<path d="M0,0 L10,5 L0,10 z" fill="{ACCENT}"/></marker>',
        '<marker id="arrM" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">'
        f'<path d="M0,0 L10,5 L0,10 z" fill="{MUTED}"/></marker>',
        "</defs>",
        rect(0, 0, W, h, BG),
        *body,
        "</svg>",
    ]) + "\n"


def write(name, h, body):
    (OUT / f"{name}.svg").write_text(svg(h, body), encoding="utf-8")
    print(f"{name}.svg  {W}x{h}")


def evidence_bar(x, cy, level, seg=26, gap=5, h=12):
    """Three segments: 3 strong, 2 moderate, 1 weak, 0 none, .5 steps = half."""
    out = []
    for k in range(3):
        sx = x + k * (seg + gap)
        out.append(rect(sx, cy - h / 2, seg, h, PANEL_2, rx=3))
        fill = min(max(level - k, 0), 1)
        if fill > 0:
            out.append(rect(sx, cy - h / 2, seg * fill, h, INK, rx=3))
    return out


# ------------------------------------------------- verdict table (fig 2, 6) --
def verdict_table(name, kicker, title, subtitle, rows, colours, legend, source):
    row_h, top = 58, 248
    h = top + row_h * len(rows) + 150
    out = header(kicker, title, subtitle)

    # tally strip: how many claims got each verdict
    order = [v for v in colours if any(r == v for _, r, _ in rows)]
    counts = {v: sum(1 for _, r, _ in rows if r == v) for v in order}
    x0, x1, sy = 40, W - 40, 166
    unit = (x1 - x0) / len(rows)
    x = x0
    for v in order:
        w = unit * counts[v]
        out.append(rect(x + 1, sy, w - 2, 12, colours[v], rx=3))
        x += w
    # tally legend underneath, laid out left to right
    x = x0
    for v in order:
        label = f"{counts[v]} × {v.lower()}"
        out.append(f'<circle cx="{x + 6}" cy="{sy + 29}" r="6" fill="{colours[v]}"/>')
        out.append(t(x + 18, sy + 35, label, 15, MUTED, SANS, "bold"))
        x += 18 + len(label) * 8.4 + 26

    out.append(t(64, top - 12, "CLAIM", 13, FAINT, SANS, "bold", spacing=1.6))
    out.append(t(880, top - 12, "VERDICT", 13, FAINT, SANS, "bold", "middle", spacing=1.6))
    out.append(t(1040, top - 12, "EVIDENCE", 13, FAINT, SANS, "bold", spacing=1.6))
    for i, (claim, verdict, ev) in enumerate(rows):
        y = top + i * row_h
        if i % 2 == 0:
            out.append(rect(40, y, W - 80, row_h, PANEL, rx=8))
        col = colours[verdict]
        out.append(rect(40, y + 10, 6, row_h - 20, col, rx=3))
        out.append(t(64, y + row_h / 2 + 7, claim, 20, INK))
        out.append(pill(880, y + row_h / 2, 196, 36, col, verdict, 17))
        out.extend(evidence_bar(1040, y + row_h / 2, ev))
    yb = top + row_h * len(rows)
    # legend
    ly = yb + 46
    lx = 40
    for lvl, lab in legend:
        out.extend(evidence_bar(lx, ly - 5, lvl, seg=16, gap=3, h=9))
        out.append(t(lx + 62, ly, lab, 15, MUTED))
        lx += 62 + len(lab) * 8.2 + 34
    out.extend(footer(yb + 116, [source]))
    write(name, h, out)
    return h


MONEY_ROWS = [
    ("Women say a partner's earnings matter more than men say", "Exists", 3),
    ("That stated gap is large", "Partly", 3),
    ("Both sexes rank kindness and intelligence above money and looks", "Exists", 3),
    ("Stated status preferences predict whom people like in person", "Mostly not", 3),
    ("Women marry up in education", "Partly", 3),
    ("Women marry men who earn more", "Exists", 3),
    ("Women partner with slightly older men", "Exists", 3),
    ("Aiming “up” is a female trait", "Does not exist", 3),
    ("Most women chase the top 20% of men", "Does not exist", 2),
]

BEYOND_ROWS = [
    ("For casual partners, both sexes put looks first", "Exists", 2),
    ("Women's tastes shift around ovulation", "Mostly not", 2),
    ("Women want taller men more than men want shorter women", "Exists", 3),
    ("Real couples follow the male-taller norm", "Partly", 3),
    ("6′3″ men get 65% more messages than 5′7″ men", "Unclear", 1),
    ("Upper-body strength drives ratings of men's bodies", "Exists", 2),
    ("Arm span (“wingspan”) affects attractiveness", "No evidence", 0),
    ("Penis size affects women's ratings of men", "Partly", 1.5),
    ("Men overestimate how much size matters", "Exists", 2),
    ("Intelligence is one of the most wanted traits", "Exists", 3),
    ("Wanting a smarter partner stops near the 90th percentile", "Partly", 1.5),
    ("Partners match on IQ and education", "Exists", 3),
    ("Looks drive messages on dating sites, for both sexes", "Exists", 3),
    ("Men's messages target younger women", "Exists", 2.5),
    ("A measured “Gini of likes” proves 80/20 on apps", "Unclear", 1),
    ("Racial bias shapes who gets messages and replies", "Documented bias", 3),
]

COLOURS = {"Exists": V_EXISTS, "Partly": V_PARTLY, "Mostly not": V_MOSTLY_NOT,
           "Does not exist": V_NOT, "Unclear": V_UNCLEAR, "No evidence": V_NONE,
           "Documented bias": V_BIAS}


# ------------------------------------------------ fig 1: four meanings map --
def fig_meanings():
    out = header("One word, four claims", "What does “hypergamy” even mean?",
                 "Online arguments slide between four meanings. Research measures each one separately.")
    rows = [
        ("1", "Marrying up", "\u201cWomen always marry up.\u201d",
         "census data on real couples: education, income, age",
         "Partly", V_PARTLY, ("Education gap flipped;", "income and age gaps remain")),
        ("2", "Saying you want up", "\u201cWomen want a rich partner.\u201d",
         "surveys of how much people say each trait matters",
         "Exists, small", V_EXISTS, ("Earnings gap about 0.30", "on a 1 to 7 scale")),
        ("3", "Choosing up", "\u201cWomen only date up.\u201d",
         "messages, replies, speed dates, who pairs with whom",
         "Everyone does it", "#2f7f86", ("Men and women both message", "about 25% \u201cup\u201d")),
        ("4", "The strong online claim", "\u201c80% of women chase the top 20% of men.\u201d",
         "nothing peer-reviewed; viral numbers trace to blogs",
         "Does not exist", V_NOT, ("Not as stated", "")),
    ]
    top, rh, gap = 196, 122, 16
    hub_x, hub_y = 140, top + (4 * rh + 3 * gap) / 2
    card_x = 300
    # column labels
    out.append(t(card_x + 70, top - 16, "WHAT THE INTERNET SAYS", 13, FAINT, SANS, "bold", spacing=1.6))
    out.append(t(1030, top - 16, "WHAT RESEARCH FINDS", 13, FAINT, SANS, "bold", "middle", spacing=1.6))
    for i, (num, name, online, measure, verdict, col, note) in enumerate(rows):
        y = top + i * (rh + gap)
        cy = y + rh / 2
        # connector from hub
        out.append(f'<path d="M{hub_x + 92},{hub_y} C{hub_x + 150},{hub_y} {card_x - 70},{cy} {card_x - 4},{cy}" '
                   f'fill="none" stroke="{col}" stroke-width="3" marker-end="url(#arrM)"/>')
        out.append(rect(card_x, y, W - 40 - card_x, rh, PANEL, rx=14))
        out.append(rect(card_x, y, 8, rh, col, rx=4))
        out.append(f'<circle cx="{card_x + 38}" cy="{y + 36}" r="18" fill="{col}"/>')
        out.append(t(card_x + 38, y + 43, num, 19, "#fff", SANS, "bold", "middle"))
        out.append(t(card_x + 70, y + 43, name, 23, INK, SERIF, "bold"))
        out.append(t(card_x + 70, y + 76, online, 18, MUTED, SERIF, style="italic"))
        out.append(t(card_x + 70, y + 104, "MEASURED BY", 12, FAINT, SANS, "bold", spacing=1.2))
        out.append(t(card_x + 178, y + 104, measure, 15.5, INK))
        # verdict
        out.append(pill(1030, y + 40, 210, 36, col, verdict, 17))
        for k, line in enumerate(note):
            out.append(t(1030, y + 80 + k * 20, line, 14.5, MUTED, SANS, anchor="middle"))
    # hub
    out.append(f'<circle cx="{hub_x}" cy="{hub_y}" r="92" fill="{INK}"/>')
    out.append(f'<circle cx="{hub_x}" cy="{hub_y}" r="80" fill="none" stroke="{BG}" stroke-width="1.5" stroke-dasharray="4 5"/>')
    out.append(t(hub_x, hub_y - 4, "“Hypergamy”", 21, "#fff", SERIF, "bold", "middle"))
    out.append(t(hub_x, hub_y + 22, "one word", 14, "#cfd3cc", SANS, anchor="middle"))
    yb = top + 4 * rh + 3 * gap
    out.extend(footer(yb + 70, ["Evidence for one meaning does not prove another. Verdicts and numbers from the post and the is_hypergamy_real literature review."]))
    write("fig-01-four-meanings", yb + 100, out)


# --------------------------------------------- fig 3: stated vs revealed --
def fig_stated_revealed():
    out = header("What people say vs whom they like",
                 "On paper the gap is small. In person it almost vanishes.",
                 "Women say earnings matter a bit more. Face to face, earnings barely predict attraction for anyone.")
    py, ph = 176, 470
    # left panel: stated (Walter et al. 2020)
    lx, lw = 40, 545
    out.append(rect(lx, py, lw, ph, PANEL, rx=16))
    out.append(t(lx + 28, py + 46, "1  What people say", 24, INK, SERIF, "bold"))
    out.append(t(lx + 28, py + 76, "Survey ratings, 45 countries, 14,399 people", 16, MUTED))
    out.append(t(lx + 28, py + 98, "Walter et al. (2020)", 16, MUTED))
    sx0, sx1 = lx + 40, lx + lw - 40
    scale = (sx1 - sx0) / 6.0
    items = [("Good financial prospects", "women rate it higher by", 0.30, SER_B),
             ("Physical attractiveness", "men rate it higher by", 0.27, SER_A)]
    for k, (trait, who, gapv, col) in enumerate(items):
        yy = py + 172 + k * 128
        out.append(t(sx0, yy - 18, trait, 19, INK, SANS, "bold"))
        # whole bar = the 6-point range of a 1 to 7 scale; the sliver = the gap, to scale
        out.append(rect(sx0, yy, sx1 - sx0, 26, PANEL_2, rx=6))
        out.append(rect(sx0, yy, gapv * scale, 26, col, rx=6))
        out.append(f'<line x1="{sx1:.1f}" y1="{yy - 6}" x2="{sx1:.1f}" y2="{yy + 32}" stroke="{FAINT}" stroke-width="1.5"/>')
        out.append(t(sx1, yy + 50, "whole 1 to 7 scale", 13.5, FAINT, anchor="end"))
        out.append(t(sx0, yy + 50, who, 16, MUTED))
        out.append(t(sx0 + len(who) * 8.1 + 6, yy + 50, f"{gapv:.2f} points", 16, col, SANS, "bold"))
    out.append(t(lx + 28, py + ph - 48, "The coloured sliver is the sex difference, drawn to", 14.5, MUTED))
    out.append(t(lx + 28, py + ph - 26, "scale against the full width of the rating scale.", 14.5, MUTED))

    # right panel: revealed (Eastwick et al. 2014)
    rx_, rw = 615, 545
    out.append(rect(rx_, py, rw, ph, PANEL, rx=16))
    out.append(t(rx_ + 28, py + 46, "2  Whom they like in person", 24, INK, SERIF, "bold"))
    out.append(t(rx_ + 28, py + 76, "Meta-analysis of 97 studies", 16, MUTED))
    out.append(t(rx_ + 28, py + 98, "Eastwick et al. (2014)", 16, MUTED))
    bx0 = rx_ + 40
    bmax = rw - 80
    rmax = 0.5
    bars = [("Their attractiveness", 0.40, INK), ("Their earning prospects", 0.10, SER_B)]
    out.append(t(bx0, py + 146, "How well a trait predicts romantic interest (r)", 15.5, MUTED))
    for k, (lab, r, col) in enumerate(bars):
        yy = py + 196 + k * 92
        out.append(t(bx0, yy - 12, lab, 19, INK, SANS, "bold"))
        out.append(rect(bx0, yy, bmax, 34, PANEL_2, rx=6))
        out.append(rect(bx0, yy, bmax * r / rmax, 34, col, rx=6))
        out.append(t(bx0 + bmax * r / rmax + 12, yy + 25, f"r ≈ {r:.2f}".replace("0.", "."), 20, col, SANS, "bold"))
    ay = py + 196 + 92 + 34 + 16
    for v in (0, 0.1, 0.2, 0.3, 0.4, 0.5):
        ax = bx0 + bmax * v / rmax
        out.append(f'<line x1="{ax:.1f}" y1="{ay - 6}" x2="{ax:.1f}" y2="{ay}" stroke="{FAINT}" stroke-width="1.5"/>')
        out.append(t(ax, ay + 20, f"{v:.1f}".replace("0.", ".") if v else "0", 14, FAINT, anchor="middle"))
    out.append(rect(bx0, py + ph - 92, bmax, 66, BG, rx=10))
    out.append(t(bx0 + 16, py + ph - 64, "Earning prospects: the same for men and women.", 16, INK, SANS, "bold"))
    out.append(t(bx0 + 16, py + ph - 40, "Sex difference about .03", 16, SER_B, SANS, "bold"))
    yb = py + ph
    out.extend(footer(yb + 64, ["Covers early attraction and existing couples, not long-term choice from a big pool, so the debate is still open."]))
    write("fig-03-stated-vs-revealed", yb + 96, out)


# --------------------------------------------- fig 4: everyone aims up --
def person(cx, cy, col, s=1.0):
    return (f'<circle cx="{cx}" cy="{cy - 30 * s}" r="{13 * s}" fill="{col}"/>'
            f'<path d="M{cx - 20 * s},{cy + 16 * s} Q{cx - 20 * s},{cy - 12 * s} {cx},{cy - 12 * s} '
            f'Q{cx + 20 * s},{cy - 12 * s} {cx + 20 * s},{cy + 16 * s} Z" fill="{col}"/>')


def fig_aims_up():
    out = header("Everyone aims up", "Aiming high is not a female trait. Nearly everyone does it.",
                 "Whom people message on a large US dating site, and who gets a reply.", badge=False)
    py, ph = 178, 452
    # left: ladder panel
    lx, lw = 40, 640
    out.append(rect(lx, py, lw, ph, PANEL, rx=16))
    out.append(t(lx + 28, py + 44, "Whom people message", 23, INK, SERIF, "bold"))
    out.append(t(lx + 28, py + 70, "Bruch and Newman (2018)", 16, MUTED))
    # one desirability column per group; "you" sits at 70% of the column and
    # the people messaged sit 25% higher than that (70% x 1.25 = 87.5%)
    ay0, ay1 = py + ph - 64, py + 128
    out.append(f'<defs><linearGradient id="desir" x1="0" y1="1" x2="0" y2="0">'
               f'<stop offset="0" stop-color="{PANEL_2}"/><stop offset="1" stop-color="{INK}" stop-opacity="0.22"/></linearGradient></defs>')
    you_f, tgt_f = 0.70, 0.70 * 1.25
    you_y = ay0 - you_f * (ay0 - ay1)
    tgt_y = ay0 - tgt_f * (ay0 - ay1)
    for name, col, gx in (("Men", SER_A, lx + 200), ("Women", SER_B, lx + 470)):
        out.append(t(gx, py + 112, name, 22, col, SERIF, "bold", "middle"))
        out.append(rect(gx - 22, ay1, 44, ay0 - ay1, "url(#desir)", rx=10))
        # "you"
        out.append(f'<line x1="{gx - 30}" y1="{you_y}" x2="{gx + 30}" y2="{you_y}" stroke="{col}" stroke-width="4" stroke-dasharray="6 4"/>')
        out.append(person(gx - 62, you_y + 14, col, 0.85))
        out.append(t(gx - 92, you_y + 6, "you", 16, col, SANS, "bold", "end"))
        # whom they message
        out.append(f'<line x1="{gx - 30}" y1="{tgt_y}" x2="{gx + 30}" y2="{tgt_y}" stroke="{col}" stroke-width="4"/>')
        out.append(f'<circle cx="{gx}" cy="{tgt_y}" r="9" fill="{col}"/>')
        out.append(t(gx - 40, tgt_y - 8, "whom they", 15, col, SANS, "bold", "end"))
        out.append(t(gx - 40, tgt_y + 11, "message", 15, col, SANS, "bold", "end"))
        out.append(f'<path d="M{gx + 34},{you_y} C{gx + 70},{you_y} {gx + 70},{tgt_y} {gx + 40},{tgt_y}" fill="none" stroke="{INK}" stroke-width="3" marker-end="url(#arr)"/>')
        out.append(t(gx + 76, (you_y + tgt_y) / 2 + 9, "+25%", 26, INK, SANS, "bold"))
    out.append(t(lx + 40, ay1 - 2 + 0, "", 1))
    out.append(t(lx + 28, ay0 + 34, "Columns schematic: desirability from the bottom up. The 25% is the", 14, MUTED))
    out.append(t(lx + 28, ay0 + 52, "measured average for both sexes.", 14, MUTED))

    # right: message stats
    rx_, rw = 700, 460
    out.append(rect(rx_, py, rw, ph, PANEL, rx=16))
    out.append(t(rx_ + 28, py + 44, "Who sends, who gets replies", 23, INK, SERIF, "bold"))
    out.append(t(rx_ + 28, py + 70, "Kreager et al. (2014)", 16, MUTED))

    def envelope(x, y, col):
        return (rect(x, y, 34, 24, col, rx=3)
                + f'<path d="M{x},{y + 1} L{x + 17},{y + 13} L{x + 34},{y + 1}" fill="none" stroke="{PANEL}" stroke-width="2"/>')

    y1 = py + 124
    out.append(t(rx_ + 28, y1, "Messages sent", 18, INK, SANS, "bold"))
    out.append(t(rx_ + 28, y1 + 44, "Men", 16, SER_A, SANS, "bold"))
    for k in range(4):
        out.append(envelope(rx_ + 110 + k * 44, y1 + 26, SER_A))
    out.append(t(rx_ + 28, y1 + 84, "Women", 16, SER_B, SANS, "bold"))
    out.append(envelope(rx_ + 110, y1 + 66, SER_B))
    out.append(t(rx_ + 160, y1 + 84, "about 4 times fewer", 16, MUTED))

    y2 = py + 264
    out.append(t(rx_ + 28, y2, "Chance a message gets a reply", 18, INK, SANS, "bold"))
    bw = 150
    out.append(t(rx_ + 28, y2 + 44, "Men", 16, SER_A, SANS, "bold"))
    out.append(rect(rx_ + 110, y2 + 26, bw, 24, SER_A, rx=4))
    out.append(t(rx_ + 110 + bw + 10, y2 + 44, "1x", 16, SER_A, SANS, "bold"))
    out.append(t(rx_ + 28, y2 + 84, "Women", 16, SER_B, SANS, "bold"))
    out.append(rect(rx_ + 110, y2 + 66, bw * 2, 24, SER_B, rx=4))
    out.append(t(rx_ + 110 + 2 * bw + 10, y2 + 84, "2x", 16, SER_B, SANS, "bold"))
    out.append(t(rx_ + 28, py + ph - 30, "Relative scale; only the ratios are from the study.", 14, MUTED))

    yb = py + ph
    out.extend(footer(yb + 64, ["Messages sent “up” go unanswered more often, so couples end up roughly matched."]))
    write("fig-04-everyone-aims-up", yb + 96, out)


# ----------------------------------------------- fig 5: marry up, reversed --
def fig_marry_up():
    out = header("Marry up, reversed", "Same wish, new supply",
                 "Share of couples with an education gap, by who has more education.", badge=True)
    x0, x1, y0, y1 = 130, 1120, 600, 190
    xm = 0.5 * (x0 + x1) + 6

    def curve(a, b):
        pts = []
        for i in range(81):
            u = i / 80
            s = 1 / (1 + math.exp(-9 * (u - 0.58)))
            v = a + (b - a) * s
            pts.append((x0 + u * (x1 - x0), y0 - v * (y0 - y1)))
        return pts
    he = curve(0.88, 0.30)
    she = curve(0.27, 0.80)
    # era shading
    cross_u = 0.58
    cx = x0 + cross_u * (x1 - x0)
    out.append(rect(x0, y1 - 20, cx - x0, y0 - y1 + 20, "#2a74a8", extra='fill-opacity="0.06"'))
    out.append(rect(cx, y1 - 20, x1 - cx, y0 - y1 + 20, "#c46a1c", extra='fill-opacity="0.07"'))
    out.append(t(x0 + 16, y0 - 16, "THEN: MORE MEN HOLD DEGREES", 13, SER_A, SANS, "bold", spacing=1.4))
    out.append(t(x1 - 16, y0 - 16, "NOW: MORE WOMEN HOLD DEGREES", 13, SER_B, SANS, "bold", "end", spacing=1.4))
    # gridlines
    for k in range(1, 4):
        gy = y0 - k * (y0 - y1) / 4
        out.append(f'<line x1="{x0}" y1="{gy}" x2="{x1}" y2="{gy}" stroke="{RULE}" stroke-width="1"/>')

    def path(pts):
        return "M" + " L".join(f"{x:.1f},{y:.1f}" for x, y in pts)

    for pts, col in ((he, SER_A), (she, SER_B)):
        out.append(f'<path d="{path(pts)}" fill="none" stroke="{col}" stroke-width="7" stroke-linecap="round"/>')
    # crossing
    cy = None
    for (xa, ya), (xb, yb) in zip(he, she):
        if xa >= cx:
            cy = (ya + yb) / 2
            break
    out.append(f'<line x1="{cx:.1f}" y1="{y1 - 20}" x2="{cx:.1f}" y2="{y0}" stroke="{MUTED}" stroke-width="2" stroke-dasharray="7 6"/>')
    out.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="13" fill="{BG}" stroke="{INK}" stroke-width="4"/>')
    out.append(rect(cx + 14, y1 - 12, 300, 36, INK, rx=18))
    out.append(t(cx + 164, y1 + 12, "Women overtake men in education", 16, "#fff", SANS, "bold", "middle"))
    # labels
    out.append(t(x0 + 30, he[4][1] - 20, "He has more education (“she married up”)", 21, SER_A, SANS, "bold"))
    out.append(t(x0 + 30, she[4][1] + 40, "She has more education (“she married down”)", 21, SER_B, SANS, "bold"))
    # axes
    out.append(f'<line x1="{x0}" y1="{y0}" x2="{x1}" y2="{y0}" stroke="{INK}" stroke-width="2"/>')
    out.append(f'<line x1="{x0}" y1="{y1 - 20}" x2="{x0}" y2="{y0}" stroke="{INK}" stroke-width="2"/>')
    out.append(t(x0, y0 + 34, "Then (mid 20th century)", 18, MUTED))
    out.append(t(x1, y0 + 34, "Now (2010s)", 18, MUTED, anchor="end"))
    out.append(t(100, (y0 + y1) / 2, "Share of couples with an education gap", 17, MUTED, anchor="middle",
                 extra=f'transform="rotate(-90 100 {(y0 + y1) / 2})"'))
    out.extend(footer(y0 + 108, ["Shape only, based on the global trend in Esteve et al. (2016). Once more women hold degrees, “down” becomes common."]))
    write("fig-05-marry-up-reversed", y0 + 138, out)


# ----------------------------------------------- fig 7: male-taller norm --
def fig_height():
    out = header("The male-taller norm", "Taller is wanted. Couples only partly follow.",
                 "What students say they want, next to who real couples are.")
    py, ph = 176, 540
    # left: stated ideal (Stulp et al. 2013a)
    lx, lw = 40, 545
    out.append(rect(lx, py, lw, ph, PANEL, rx=16))
    out.append(t(lx + 28, py + 46, "1  The ideal height gap", 24, INK, SERIF, "bold"))
    out.append(t(lx + 28, py + 76, "650 Dutch students, Stulp et al. (2013a)", 16, MUTED))
    # axis: how much taller he is, 0..25 cm
    ax0, ax1, ayy = lx + 50, lx + lw - 50, py + 380
    cm = (ax1 - ax0) / 25
    out.append(f'<line x1="{ax0}" y1="{ayy}" x2="{ax1}" y2="{ayy}" stroke="{INK}" stroke-width="2"/>')
    for v in range(0, 26, 5):
        tx = ax0 + v * cm
        out.append(f'<line x1="{tx}" y1="{ayy}" x2="{tx}" y2="{ayy + 8}" stroke="{INK}" stroke-width="2"/>')
        out.append(t(tx, ayy + 30, f"+{v}" if v else "0", 15, MUTED, anchor="middle"))
    out.append(t((ax0 + ax1) / 2, ayy + 58, "How much taller he is than she is (cm)", 15.5, MUTED, anchor="middle"))
    marks = [("Men's ideal", "she is about 8 cm shorter", 8, SER_A, py + 200),
             ("Women's ideal", "he is about 21 cm taller", 21, SER_B, py + 130)]
    for lab, sub, v, col, ly in marks:
        mx = ax0 + v * cm
        out.append(f'<line x1="{mx}" y1="{ly + 18}" x2="{mx}" y2="{ayy - 12}" stroke="{col}" stroke-width="3"/>')
        out.append(f'<circle cx="{mx}" cy="{ayy}" r="11" fill="{col}" stroke="{PANEL}" stroke-width="3"/>')
        anchor = "end" if v > 15 else "start"
        tx = mx + (8 if anchor == "start" else -8)
        if anchor == "end":
            tx = mx + 20
        out.append(t(tx, ly - 8, lab, 19, col, SANS, "bold", anchor))
        out.append(t(tx, ly + 14, sub, 15.5, MUTED, SANS, anchor=anchor))
    # bracket showing asymmetry
    bx0, bx1, by = ax0 + 8 * cm, ax0 + 21 * cm, ayy - 120
    out.append(f'<path d="M{bx0},{by + 10} L{bx0},{by} L{bx1},{by} L{bx1},{by + 10}" fill="none" stroke="{MUTED}" stroke-width="1.8"/>')
    out.append(t((bx0 + bx1) / 2, by - 10, "the two ideals", 15, MUTED, SANS, "bold", "middle"))

    # right: real couples (Stulp et al. 2013b)
    rx_, rw = 615, 545
    out.append(rect(rx_, py, rw, ph, PANEL, rx=16))
    out.append(t(rx_ + 28, py + 46, "2  Real couples", 24, INK, SERIF, "bold"))
    out.append(t(rx_ + 28, py + 76, "12,502 UK couples, Stulp et al. (2013b)", 16, MUTED))
    out.append(t(rx_ + 28, py + 128, "Couples where the man is not taller", 18, INK, SANS, "bold"))
    bx, bw = rx_ + 40, rw - 80
    vmax = 12.0
    bars = [("Observed", 7.5, INK), ("If people paired at random", 10.2, FAINT)]
    for k, (lab, v, col) in enumerate(bars):
        yy = py + 176 + k * 86
        out.append(t(bx, yy - 10, lab, 16.5, MUTED))
        out.append(rect(bx, yy, bw, 30, PANEL_2, rx=6))
        out.append(rect(bx, yy, bw * v / vmax, 30, col, rx=6))
        out.append(t(bx + bw * v / vmax + 12, yy + 23, f"{v}%", 21, col if col != FAINT else MUTED, SANS, "bold"))
    out.append(t(bx, py + 360, "About 1 couple in 13:", 18, INK, SANS, "bold"))
    # 13 couples, one highlighted
    for k in range(13):
        cx = bx + 18 + k * 36
        cy = py + 430
        hi = k == 12
        cm_col = SER_B if hi else FAINT
        # man (A) and woman (B) as two bars; in the highlighted couple she is taller
        hm, hw = (34, 44) if hi else (46, 36)
        out.append(rect(cx - 12, cy - hm, 10, hm, SER_A if hi else "#b9bdb5", rx=3))
        out.append(rect(cx + 2, cy - hw, 10, hw, cm_col if hi else "#cfd2ca", rx=3))
    out.append(f'<line x1="{bx}" y1="{py + 431}" x2="{bx + 13 * 36}" y2="{py + 431}" stroke="{RULE}" stroke-width="2"/>')
    out.append(t(bx, py + 470, "Men are taller on average, so chance alone gives a taller", 15, MUTED))
    out.append(t(bx, py + 492, "man about 90% of the time. Preference adds only a little.", 15, MUTED))
    out.append(t(bx + 13 * 36 - 6, py + 382, "she is taller", 13.5, SER_B, SANS, "bold", "end"))
    yb = py + ph
    out.extend(footer(yb + 64, ["Height on dating profiles is also inflated, mostly by men, so app “height effects” come with a measurement problem."]))
    write("fig-07-male-taller-norm", yb + 96, out)


# --------------------------------------------------- fig 8: the IQ ceiling --
def fig_iq():
    out = header("The IQ ceiling", "Smart is wanted, up to a point",
                 "How desirable a partner at each IQ level is rated, in shape only.", badge=True)
    x0, x1, y0, y1 = 120, 800, 620, 270
    iq0, iq1 = 70, 145

    def X(iq):
        return x0 + (iq - iq0) / (iq1 - iq0) * (x1 - x0)

    def f(iq):  # illustrative: rises, peaks at 120, falls by 135
        if iq <= 120:
            return 0.12 + 0.83 * (1 - ((120 - iq) / 50) ** 2) if iq >= 70 else 0.12
        return 0.95 - 0.55 * ((iq - 120) / 25) ** 2

    pts = [(X(q), y0 - f(q) * (y0 - y1)) for q in [iq0 + i * 0.5 for i in range(int((iq1 - iq0) / 0.5) + 1)]]
    d = "M" + " L".join(f"{x:.1f},{y:.1f}" for x, y in pts)
    out.append(f'<defs><linearGradient id="iqfill" x1="0" y1="0" x2="0" y2="1">'
               f'<stop offset="0" stop-color="{ACCENT}" stop-opacity="0.28"/><stop offset="1" stop-color="{ACCENT}" stop-opacity="0.02"/></linearGradient></defs>')
    for k in range(1, 4):
        gy = y0 - k * (y0 - y1) / 4
        out.append(f'<line x1="{x0}" y1="{gy}" x2="{x1}" y2="{gy}" stroke="{RULE}" stroke-width="1"/>')
    out.append(f'<path d="{d} L{x1},{y0} L{x0},{y0} Z" fill="url(#iqfill)"/>')
    out.append(f'<path d="{d}" fill="none" stroke="{ACCENT}" stroke-width="7" stroke-linecap="round"/>')
    px, py_ = X(120), y0 - f(120) * (y0 - y1)
    out.append(f'<line x1="{px}" y1="{py_}" x2="{px}" y2="{y0}" stroke="{ACCENT}" stroke-width="2" stroke-dasharray="6 6"/>')
    out.append(f'<circle cx="{px}" cy="{py_}" r="12" fill="{BG}" stroke="{ACCENT}" stroke-width="5"/>')
    out.append(rect(px - 150, py_ - 78, 300, 52, INK, rx=12))
    out.append(t(px, py_ - 56, "Peak: 90th percentile", 17, "#fff", SANS, "bold", "middle"))
    out.append(t(px, py_ - 35, "about IQ 120", 15, "#cfd3cc", SANS, anchor="middle"))
    qx, qy = X(135), y0 - f(135) * (y0 - y1)
    out.append(f'<circle cx="{qx}" cy="{qy}" r="9" fill="{ALERT}"/>')
    out.append(t(qx - 18, qy + 34, "Lower again", 16, ALERT, SANS, "bold", "end"))
    out.append(t(qx - 18, qy + 54, "at the 99th", 16, ALERT, SANS, "bold", "end"))
    # axes
    out.append(f'<line x1="{x0}" y1="{y0}" x2="{x1}" y2="{y0}" stroke="{INK}" stroke-width="2"/>')
    out.append(f'<line x1="{x0}" y1="{y1 - 20}" x2="{x0}" y2="{y0}" stroke="{INK}" stroke-width="2"/>')
    for q, lab in ((100, "50th"), (120, "90th"), (135, "99th")):
        out.append(f'<line x1="{X(q)}" y1="{y0}" x2="{X(q)}" y2="{y0 + 8}" stroke="{INK}" stroke-width="2"/>')
        out.append(t(X(q), y0 + 30, lab, 16, MUTED, anchor="middle"))
    out.append(t((x0 + x1) / 2, y0 + 62, "Partner's IQ percentile", 17, MUTED, anchor="middle"))
    out.append(t(92, (y0 + y1) / 2, "Rated desirability", 17, MUTED, anchor="middle",
                 extra=f'transform="rotate(-90 92 {(y0 + y1) / 2})"'))

    # side panel: real couples
    sx, sw, sy, sh = 850, 310, 196, 450
    out.append(rect(sx, sy, sw, sh, PANEL, rx=16))
    out.append(t(sx + 24, sy + 42, "Meanwhile, in", 21, INK, SERIF, "bold"))
    out.append(t(sx + 24, sy + 70, "real couples", 21, INK, SERIF, "bold"))
    out.append(t(sx + 24, sy + 96, "199 studies, Horwitz et al. (2023)", 14.5, MUTED))
    for k, (lab, r) in enumerate((("IQ", 0.44), ("Education", 0.55))):
        yy = sy + 150 + k * 100
        out.append(t(sx + 24, yy, f"Partners' {lab.lower() if lab != 'IQ' else lab}", 17, INK, SANS, "bold"))
        out.append(rect(sx + 24, yy + 14, sw - 48, 26, PANEL_2, rx=5))
        out.append(rect(sx + 24, yy + 14, (sw - 48) * r, 26, SER_A, rx=5))
        out.append(t(sx + 24 + (sw - 48) * r + 10, yy + 34, f"r = {r:.2f}".replace("0.", "."), 17, SER_A, SANS, "bold"))
    out.append(t(sx + 24, sy + 384, "Like marries like.", 17, INK, SANS, "bold"))
    out.append(t(sx + 24, sy + 410, "Correlation scale 0 to 1.", 14, MUTED))
    out.extend(footer(y0 + 130, ["Shape only, after Gignac and Starbuck (2019): 214 young adults, one lab, stated ratings. A hint, not a law."]))
    write("fig-08-iq-ceiling", y0 + 160, out)


# ------------------------------------------- fig 13: the meter's pipeline --
def fig_pipeline():
    out = header("How the Hypergamy Meter works", "A joke model, drawn like a serious one",
                 "Hand-picked weights, not estimated from data. Ethnicity is deliberately not an input.")
    # flow of six stages
    stages = [
        ("1", "Ten inputs", ["age, gender,", "education, income,", "height, looks, build,", "smarts, region, goal", "+ size (optional)"]),
        ("2", "Points", ["each input adds or", "subtracts points;", "looks weigh most,", "smarts: +6, then +3"]),
        ("3", "Goal weights", ["the goal stretches", "or shrinks those", "points (see below)"]),
        ("4", "Score", ["add it all up,", "clamp to 0 to 100"]),
        ("5", "Tier", ["six tiers, from", "\u201cTop of the market\u201d", "to \u201cIt's over\u201d"]),
        ("6", "Meme card", ["huge captions,", "then the retraction:", "\u201cgo touch grass :D\u201d"]),
    ]
    top, bh, bw, gap = 180, 206, 160, 32
    for i, (num, title, lines) in enumerate(stages):
        x = 40 + i * (bw + gap)
        hl = num == "3"
        out.append(rect(x, top, bw, bh, ACCENT if hl else PANEL, rx=14))
        fg = "#fff" if hl else INK
        sub = "#d6eadf" if hl else MUTED
        out.append(f'<circle cx="{x + 28}" cy="{top + 30}" r="15" fill="{"#fff" if hl else INK}"/>')
        out.append(t(x + 28, top + 36, num, 16, ACCENT if hl else "#fff", SANS, "bold", "middle"))
        out.append(t(x + 14, top + 80, title, 19, fg, SERIF, "bold"))
        for k, line in enumerate(lines):
            out.append(t(x + 14, top + 108 + k * 20, line, 13.5, sub))
        if i < len(stages) - 1:
            ax = x + bw + 5
            out.append(f'<line x1="{ax}" y1="{top + bh / 2}" x2="{ax + gap - 10}" y2="{top + bh / 2}" '
                       f'stroke="{INK}" stroke-width="3" marker-end="url(#arr)"/>')

    # meter bar with tiers (thresholds from model.rs: 25, 40, 55, 70, 85)
    my = top + bh + 70
    mx0, mx1 = 40, W - 40
    out.append(t(40, my - 16, "THE METER: TIERS BY SCORE", 13, FAINT, SANS, "bold", spacing=1.6))
    tiers = [(0, 25, "It's over*", "#b8322b"), (25, 40, "Clearance aisle", "#d0663a"),
             (40, 55, "Acquired taste", "#d69a2a"), (55, 70, "Not bad", "#a7a83a"),
             (70, 85, "Strong buy", "#5e9f4a"), (85, 100, "Top of the market", "#2e8555")]
    sc = (mx1 - mx0) / 100
    for lo, hi, lab, col in tiers:
        out.append(rect(mx0 + lo * sc + 1, my, (hi - lo) * sc - 2, 40, col, rx=6))
        out.append(t(mx0 + (lo + hi) / 2 * sc, my + 26, lab, 15, "#fff", SANS, "bold", "middle"))
    for v in (0, 25, 40, 55, 70, 85, 100):
        out.append(t(mx0 + v * sc, my + 64, str(v), 14, MUTED, anchor="middle"))
    out.append(t(mx1, my + 90, "*followed by: “...just kidding, go touch grass :D”", 14, MUTED, anchor="end"))

    # goal weights
    gy = my + 140
    out.append(t(40, gy, "STEP 3: GOAL WEIGHTS", 13, FAINT, SANS, "bold", spacing=1.6))
    factors = ["Looks", "Build", "Education", "Income", "Smarts", "Age, height, size, goal"]
    goals = [("Casual", [1.5, 1.5, 0.5, 0.5, 0.5, 1.0]),
             ("Long-term", [1.0, 1.0, 1.2, 1.2, 1.2, 1.0]),
             ("Marriage", [1.0, 1.0, 1.2, 1.2, 1.2, 1.0]),
             ("No idea, honestly", [1.0, 1.0, 1.0, 1.0, 1.0, 1.0])]
    lab_w = 230
    col_w = (W - 80 - lab_w) / len(goals)
    hy = gy + 40
    for j, (g, _) in enumerate(goals):
        out.append(t(40 + lab_w + j * col_w + col_w / 2, hy, g, 17, INK, SERIF, "bold", "middle"))
    rh = 44
    for i, fac in enumerate(factors):
        y = hy + 18 + i * rh
        if i % 2 == 0:
            out.append(rect(40, y, W - 80, rh, PANEL, rx=6))
        out.append(t(56, y + 28, fac, 16.5, INK))
        for j, (g, ws) in enumerate(goals):
            w = ws[i]
            cx0 = 40 + lab_w + j * col_w + 20
            full = col_w - 90
            col = V_EXISTS if w > 1 else (V_MOSTLY_NOT if w < 1 else "#b9bdb5")
            # bar length proportional to the weight, 1.5 = full
            out.append(rect(cx0, y + 13, full * w / 1.5, 18, col, rx=4))
            out.append(f'<line x1="{cx0 + full / 1.5:.1f}" y1="{y + 8}" x2="{cx0 + full / 1.5:.1f}" y2="{y + 36}" stroke="{INK}" stroke-width="1.2" stroke-dasharray="3 3"/>')
            out.append(t(cx0 + full + 12, y + 28, f"{w:g}x", 16, col if w != 1 else MUTED, SANS, "bold"))
    yb = hy + 18 + len(factors) * rh
    out.append(t(40, yb + 30, "Dashed line = 1x. Example: a man's high income, worth +10, becomes +12 for long-term or marriage, and +5 for casual.", 15, MUTED))
    out.extend(footer(yb + 100, ["From app/src/model.rs in the is_hypergamy_real repository. For fun only. Not science, not advice."]))
    write("fig-13-meter-pipeline", yb + 130, out)


if __name__ == "__main__":
    fig_meanings()
    verdict_table("fig-02-verdict-money", "The verdict, claim by claim", "Is hypergamy real?",
                  "Nine common claims about money and status, checked against the research.",
                  MONEY_ROWS, COLOURS,
                  [(3, "strong: several large studies or meta-analyses agree"), (2, "moderate")],
                  "Summary of the literature review in the is_hypergamy_real repository.")
    fig_stated_revealed()
    fig_aims_up()
    fig_marry_up()
    verdict_table("fig-06-verdict-beyond-money", "Beyond money", "Height, looks, size, IQ and the apps",
                  "Sixteen claims beyond money, checked against the research.",
                  BEYOND_ROWS, COLOURS,
                  [(3, "strong"), (2, "moderate"), (1, "weak"), (0, "no study found"), (1.5, "in between")],
                  "Section 11 of the literature review in the is_hypergamy_real repository.")
    fig_height()
    fig_iq()
    fig_pipeline()
