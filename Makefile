push: bucky_overlay.png
	git commit -am "Update overlay"
	git push

slice = $(wordlist $1,$(words $2),$2)

LAYERS := chi bucky_face wisco

bucky_overlay.png: $(foreach l,$(LAYERS),build/$l_overlay.png)
	convert $< $(foreach f,$(call slice,2,$^),$f -composite) $@

define overlay =
build/$1_overlay.png: $1.png
	@mkdir -p $$(@D)
	python ./generator.py $$< $$@ $2 $3
	optipng $$@
endef

$(eval $(call overlay,chi,594,399))
$(eval $(call overlay,bucky_face,250,1552))
$(eval $(call overlay,wisco,99,579))
