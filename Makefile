UID := $(shell id -u)
GID := $(shell id -g)
USER := $(UID):$(GID)
DOCKER_COMPOSE := user=$(USER) docker-compose


up:
	$(DOCKER_COMPOSE) up -d

shRust:
	$(DOCKER_COMPOSE) exec api bash

shDb:
	$(DOCKER_COMPOSE) exec postgis bash