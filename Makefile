all: $(wildcard *_overlay.png)
	git -C repo commit -am "Update overlay"
	git -C repo push

define overlay =
$1_overlay.png: $1.png
	python ./generator.py $$< $$@ $2 $3
	optipng $$@
endef

$(eval $(call overlay,chi,594,399))
$(eval $(call overlay,bucky,250,1552))
