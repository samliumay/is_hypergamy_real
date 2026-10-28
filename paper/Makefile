# Builds main.pdf with latexmk (pdflatex) into build/ and copies it here.
# BibTeX finds ../bibliography.bib (the project's verified bibliography) via BIBINPUTS.
pdf:
	BIBINPUTS=$(abspath ..): latexmk -pdf -interaction=nonstopmode -halt-on-error -outdir=build main.tex
	cp build/main.pdf main.pdf

clean:
	rm -rf build

distclean: clean
	rm -f main.pdf
