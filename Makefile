push: bucky_overlay.png
	git commit -am "Update overlay"
	git push

slice = $(wordlist $1,$(words $2),$2)

OVERLAYS :=

define overlay =
IMGS += $1.png
OVERLAY_CMDS += --template $1.png $2 $3
endef

$(eval $(call overlay,chi,594,399))
$(eval $(call overlay,bucky_face,250,1552))
$(eval $(call overlay,wisco,99,579))

target/release/placegen:
	cargo b -r

bucky_overlay.png: target/release/placegen $(IMGS)
	target/release/placegen $@ $(OVERLAY_CMDS)
	optipng $@

clean:
	rm -f bucky_overlay.png
