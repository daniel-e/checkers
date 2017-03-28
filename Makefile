build-module:
	cd rust_engine && cargo build --release --lib

rest: build-module rust_engine/target/release/libengine.so
	cp rust_engine/target/release/libengine.so rest/engine.so
	cd rest && ./rest.py

# ------------------------------------------------------------------------------
# nginx
# ------------------------------------------------------------------------------

HTMLDEP   = $(wildcard html/*.html html/*.js html/*.css html/*.png)
NGINXDST  = /opt/nginx-dame
NGINXTMP := $(shell mktemp -d)

$(NGINXDST): extra/nginx.conf
	rm -rf $(NGINXDST)
	tar xzf extra/nginx-1.10.3.tar.gz -C $(NGINXTMP)
	cd $(NGINXTMP)/nginx-1.10.3 && ./configure --prefix=$(NGINXDST) && make -j4 && make install
	rm -rf $(NGINXTMP)
	cp extra/nginx.conf $(NGINXDST)/conf/

$(NGINXDST)/conf/nginx.conf: extra/nginx.conf
	cp extra/nginx.conf $(NGINXDST)/conf/

html: $(NGINXDST) $(NGINXDST)/conf/nginx.conf $(HTMLDEP)
	@rm -rf $(NGINXDST)/html/
	@mkdir -p $(NGINXDST)/html
	cp html/* $(NGINXDST)/html/

html-run: html
	/opt/nginx-dame/sbin/nginx

html-clean:
	rm -rf $(NGINXDST)
