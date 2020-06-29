.PHONY: docker-run
docker-run:
	docker build -t rust-playground-book .

.PHONY: docker-build
docker-build:
	docker run -it -v ${PWD}:/usr/src/myapp --rm --name rust-book rust-playground-book


