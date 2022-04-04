push: bucky_overlay.png
	git commit -am "Update overlay"
	git push

slice = $(wordlist $1,$(words $2),$2)

OVERLAYS :=

define overlay =
OVERLAYS += build/$1_overlay.png
build/$1_overlay.png: $1.png
	@mkdir -p $$(@D)
	python ./generator.py $$< $$@ $2 $3
	optipng $$@
endef

$(eval $(call overlay,chi,594,399))
$(eval $(call overlay,bucky_face,279,1553))
$(eval $(call overlay,wisco,99,579))

bucky_overlay.png: $(OVERLAYS)
	convert $< $(foreach f,$(call slice,2,$^),$f -composite) $@
