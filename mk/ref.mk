RF += static/cdn/jquery.min.js
static/cdn/jquery.min.js:
	$(CURL) $@ https://cdnjs.cloudflare.com/ajax/libs/jquery/$(JQUERY_VER)/jquery.min.js

RF += ref/capsule/README.md ref/NetBricks/README.md
ref/capsule/README.md:
	$(GITREF) https://github.com/capsule-rs/capsule.git $(dir $@)
ref/NetBricks/README.md:
	$(GITREF) https://github.com/NetSys/NetBricks.git $(dir $@)
