build:
	$(MAKE) -C backend build
	$(MAKE) -C client build

check:
	$(MAKE) -C backend check
	$(MAKE) -C client check

dev:
	script/start dev

prod:
	script/start prod
