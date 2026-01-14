.PHONY: ai tmp/$(APP).ai.md
ai: tmp/$(APP).ai.md
tmp/$(APP).ai.md: doc
	cat \
		doc/$(APP)/*.md \
		doc/ai.md doc/$(APP)/bib.md README.md \
	> $@ ; touch $@
