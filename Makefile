check:
	$(MAKE) -C backend check
	$(MAKE) -C client check

dev:
	script/dev
