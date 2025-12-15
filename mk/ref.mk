RF += static/cdn/jquery.min.js
static/cdn/jquery.min.js:
	$(CURL) $@ https://cdnjs.cloudflare.com/ajax/libs/jquery/$(JQUERY_VER)/jquery.min.js
