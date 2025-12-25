SOURCE ?= input.txt
TARGET ?= output.txt

run:
	cargo run $(SOURCE) $(TARGET)