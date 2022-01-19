CARGO ?= cargo
REDIS ?= redis-server

test:
	$(REDIS) & $(CARGO) test && pkill -f redis-server
