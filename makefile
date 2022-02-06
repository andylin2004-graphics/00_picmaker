all:
	rustc hw-00/src/main.rs
	./main
	magick display imageFile.ppm