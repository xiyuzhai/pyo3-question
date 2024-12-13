.PHONY: run venv clean

run:
	cargo run

venv:
	python -m venv .venv
	. .venv/bin/activate && pip install spacy

clean:
	rm -rf .venv
