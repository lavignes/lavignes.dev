MD := comrak
MD_FLAGS := -e strikethrough,table,superscript,footnotes --unsafe --syntax-highlighting none

rwildcard = $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

MD_TEMPLATES := ./_head.md ./_prebody.md ./_tail.md
MD_SRC := $(filter-out $(MD_TEMPLATES),$(call rwildcard,.,*.md))
HTMLS := $(MD_SRC:%.md=../%.html)

all: $(HTMLS)
	@echo "ok"

../%.html: %.md $(MD_TEMPLATES)
	$(eval $@_TITLE = $(shell head -n 1 "$<" | sed -e "s/^\\W*#\\W*//"))
	@echo "Generating: '$($@_TITLE)' => $@"
	@mkdir -p $(dir $@)
	@$(MD) $(MD_FLAGS) _head.md > $@
	@echo "<title>$($@_TITLE)</title>" >> $@
	@$(MD) $(MD_FLAGS) _prebody.md >> $@
	@$(MD) $(MD_FLAGS) $< >> $@
	@$(MD) $(MD_FLAGS) _tail.md >> $@

clean:
	rm -f $(call rwildcard,..,*.html)
