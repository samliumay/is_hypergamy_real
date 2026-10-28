"""Builds figure-7.svg (beyond-money verdicts) in the style of figure-1.svg."""
from xml.sax.saxutils import escape
import pathlib

F = 'font-family="Noto Serif, DejaVu Serif, serif"'
COL = {"Exists": "#2f855a", "Partly": "#c07d10", "Mostly not": "#b4533a",
       "Unclear": "#6b6b6b", "No evidence": "#8a8a8a", "Documented bias": "#2f855a"}
# evidence: 3 strong, 2 moderate, 1 weak, 0 none; .5 = in between (half dot)
ROWS = [
    ("For casual partners, both sexes put looks first", "Exists", 2),
    ("Women's tastes shift around ovulation", "Mostly not", 2),
    ("Women want taller men more than men want shorter women", "Exists", 3),
    ("Real couples follow the male-taller norm", "Partly", 3),
    ("6\u20323\u2033 men get 65% more messages than 5\u20327\u2033 men", "Unclear", 1),
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
ROW_H, TOP = 62, 110
H = TOP + ROW_H * len(ROWS) + 100
out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="{H}" viewBox="0 0 1200 {H}">',
       f'<rect width="1200" height="{H}" fill="#ffffff"/>',
       f'<text x="40" y="60" {F} font-size="30" font-weight="bold" fill="#222">Beyond money: height, looks, size, IQ and the apps</text>',
       f'<text x="40" y="95" {F} font-size="19" fill="#666">Claim</text>',
       f'<text x="790" y="95" {F} font-size="19" fill="#666">Verdict</text>',
       f'<text x="1040" y="95" {F} font-size="19" fill="#666">Evidence</text>']
for i, (claim, verdict, ev) in enumerate(ROWS):
    y = TOP + i * ROW_H
    if i % 2 == 0:
        out.append(f'<rect x="20" y="{y}" width="1160" height="{ROW_H}" fill="#f4f4f2"/>')
    ty, cy = y + 38, y + 31
    out.append(f'<text x="40" y="{ty}" {F} font-size="21" fill="#222">{escape(claim)}</text>')
    out.append(f'<rect x="780" y="{y + 12}" width="200" height="38" rx="19" fill="{COL[verdict]}"/>')
    out.append(f'<text x="880" y="{ty}" {F} font-size="19" font-weight="bold" fill="#fff" text-anchor="middle">{verdict}</text>')
    for k in range(3):
        cx = 1050 + 28 * k
        if ev >= k + 1:
            fill = "#333"
        else:
            fill = "none"
        out.append(f'<circle cx="{cx}" cy="{cy}" r="9" fill="{fill}" stroke="#333" stroke-width="2"/>')
        if k + 0.5 == ev:  # half-filled dot: left half
            out.append(f'<path d="M {cx} {cy - 9} A 9 9 0 0 0 {cx} {cy + 9} Z" fill="#333"/>')
yb = TOP + ROW_H * len(ROWS)
out.append(f'<text x="40" y="{yb + 45}" {F} font-size="18" fill="#555">Evidence: three dots = strong, two = moderate, one = weak, none = no study found; a half dot sits in between.</text>')
out.append(f'<text x="40" y="{yb + 73}" {F} font-size="18" fill="#555">Section 11 of the literature review in the is_hypergamy_real repository.</text>')
out.append('</svg>')
p = pathlib.Path(__file__).with_name("figure-7.svg")
p.write_text("\n".join(out) + "\n", encoding="utf-8")
print(p, H)
