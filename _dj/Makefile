DJ_PATH := ../_gen
DJ := $(DJ_PATH)/target/release/_gen
DJ_FLAGS :=

rwildcard = $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d))

HEAD ?= ./_head.dj
DJ_TEMPLATES := $(wildcard ./_*.dj)
DJ_SRC := $(filter-out $(DJ_TEMPLATES),$(call rwildcard,.,*.dj))
HTMLS := $(DJ_SRC:%.dj=../%.html)

all: $(HTMLS)
	@echo "ok"

$(DJ):
	cd $(DJ_PATH) && cargo build --release

../%.html: %.dj $(DJ_TEMPLATES) $(DJ)
	$(eval $@_TITLE = $(shell head -n 1 "$<" | sed -e "s/^\\W*#\\W*//"))
	@echo "Generating: '$($@_TITLE)' => $@"
	@mkdir -p $(dir $@)
	@$(DJ) $(DJ_FLAGS) $(HEAD) > $@
	@echo "<title>$($@_TITLE)</title>" >> $@
	@$(DJ) $(DJ_FLAGS) _prebody.dj >> $@
	@$(DJ) $(DJ_FLAGS) $< >> $@
	@$(DJ) $(DJ_FLAGS) _tail.dj >> $@

.PHONY: clean
clean:
	rm -f $(call rwildcard,..,*.html)
