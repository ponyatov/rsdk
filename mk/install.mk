.PHONY : install update ref gz
install: $(WS)_install doc ref gz $(RUSTUP)
	$(MAKE) update
update : $(WS)_update
ref    : $(RF)
gz     : $(GZ)

Debian_install:
Debian_update: apt.$(WS) $(RUSTUP)
	sudo apt update
	sudo apt install -uy `cat apt.$(WS)` $(APT)
	$(RUSTUP) self update ; $(RUSTUP) update

Ubuntu_install:
Ubuntu_update: apt.$(WS)
	sudo apt update
	sudo apt install -uy `cat apt.$(WS)` $(APT)
