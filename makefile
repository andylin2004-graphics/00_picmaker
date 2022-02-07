all: main
	./main
	magick display imageFile.ppm

main:
	rustc hw-00/src/main.rs