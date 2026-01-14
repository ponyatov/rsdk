RTARGET = x86_64-unknown-linux-gnu

$(RUSTUP) $(CARGO):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup target add x86_64-unknown-linux-gnu
$(RUSTUP) $(CARGO):
# PROXY = -x 10.110.1.12:8888
	curl $(PROXY) --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	. $HOME/.cargo/env
	rustup target add x86_64-unknown-linux-gnu
# rustup self update ; rustup update
