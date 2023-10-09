all: biber

biber: main
	biber main

main:
	pdflatex main

clean:
	rm -fr \
		*.toc \
		*.aux \
		*.nav \
		*.out \
		*.snm \
		*.toc \
		*.log \
		*.bbl \
		*.blg \
		*.bcf \
		*.run.xml
