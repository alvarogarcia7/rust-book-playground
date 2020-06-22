.PHONY: docker-build
docker-build:
	docker build -t rust-playground-book .
	docker run -it -v ${PWD}:/usr/src/myapp --rm --name rust-book rust-playground-book


