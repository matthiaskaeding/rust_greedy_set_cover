# Activate venv
venv:
	uv venv && source .venv/bin/activate
# Install requirements
reqs:
	uv pip install -r py-setcover/pyproject.toml --all-extras
# test the python package
py-test: py-install
	uv run pytest py-setcover/tests

# Install python pkg
py-install:
	uv tool run maturin develop -m py-setcover/Cargo.toml --uv

# Copies repo into clipboard, needs reposyn
rsyn:
	reposyn -i rcpp_greedy_set_cover/ -c

# Debug installation
py-debug: py-install
	uv run python -c "import sys; print(sys.path)"
	uv run python -c "import setcover; print('Success!')"

# Clean and reinstall
py-clean:
	rm -rf py-setcover/target/
	rm -rf .venv/lib/python*/site-packages/setcover*
	rm -rf .venv/lib/python*/site-packages/_setcover*

# lint python
py-lint:
	uv tool run ruff format py-setcover
	uv tool run ruff check --fix py-setcover 

# make data for benchmark
make-bench-data:
	Rscript scripts/benchmark/make_data.r