.PHONY: help clean clean-pyc clean-build list test test-all coverage docs release sdist

help:
	@echo "clean-build - remove build artifacts"
	@echo "clean-pyc - remove Python file artifacts"
	@echo "lint - check style with flake8"
	@echo "test - run tests quickly with the default Python"
	@echo "test-all - run tests on every Python version with tox"
	@echo "coverage - check code coverage quickly with the default Python"
	@echo "docs - generate Sphinx HTML documentation, including API docs"
	@echo "release - package and upload a release"
	@echo "sdist - package"

clean: clean-build clean-pyc clean-locks

clean-locks:
	rm -fr *.lock

clean-build:
	rm -fr build/
	rm -fr dist/
	rm -fr *.egg-info

clean-pyc:
	find . -name '*.pyc' -exec rm -f {} +
	find . -name '*.pyo' -exec rm -f {} +
	find . -name '*~' -exec rm -f {} +

lint: clean
	flake8 datagears/core tests
	mypy . --show-error-codes --platform win32 --platform darwin --platform linux

format:
	black .
	isort .

test:
	pytest . -vvv --isort --flake8 --mypy --cache-clear --dist loadfile --max-worker-restart 0 -n auto --cov=datagears tests/

test-all:
	tox

coverage:
	coverage run --source datagears setup.py tests
	coverage report -m
	coverage html
	open htmlcov/index.html

docs:
	rm -f docs/datagears.rst
	rm -f docs/modules.rst
	sphinx-apidoc -o docs/ datagears
	$(MAKE) -C docs clean
	$(MAKE) -C docs html
	open docs/_build/html/index.html

release: clean
	python setup.py sdist upload
	python setup.py bdist_wheel upload

sdist: clean
	python setup.py sdist
	python setup.py bdist_wheel upload
	ls -l dist
