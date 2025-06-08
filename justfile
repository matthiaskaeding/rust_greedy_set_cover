# Activate venv
venv:
	uv venv && source .venv/bin/activate

# Cargo test
ctest:
	cargo test

# Install requirements
reqs:
	uv pip install -r py-setcover/pyproject.toml --all-extras
# test the python package
pytest: pyinstall
	uv run pytest py-setcover/tests

# Install python pkg
pyinstall:
	@echo "Installing in development mode"
	uv tool run maturin develop -m py-setcover/Cargo.toml --uv

# Innstall python pkg - release mode
pyinstall-rel:
	@echo "Installing in release mode"
	uv tool run maturin develop --release -m py-setcover/Cargo.toml --uv

# Copies repo into clipboard, needs reposyn
rsyn:
	reposyn -i rcpp_greedy_set_cover/ -c

# Debug installation
pydebug: pyinstall
	uv run python -c "import sys; print(sys.path)"
	uv run python -c "import setcover; print('Success!')"

# Clean and reinstall
clean:
	rm -rf py-setcover/target/
	rm -rf .venv/lib/python*/site-packages/setcover*
	rm -rf .venv/lib/python*/site-packages/_setcover*
	rm scripts/benchmark/data.csv
# lint python
pylint:
	uv tool run ruff format py-setcover
	uv tool run ruff check --fix py-setcover 

# Benchmark stuff

# Make data for benchmark
prep-bench n_sets="1e5" n_elements="2e3" n_rows="1e7":
	@echo "Creating simulation data with:"
	@echo "  Number of sets: {{n_sets}}"
	@echo "  Number of elements: {{n_elements}}"
	@echo "  Number of rows: {{n_rows}}"
	Rscript scripts/benchmark/make_data.r {{n_sets}} {{n_elements}} {{n_rows}}

# Take timing for python. Install in release mode first 
pytime: pyinstall-rel
	uv run --with polars scripts/benchmark/time_py.py
# Take timing for python
rtime:
	Rscript scripts/benchmark/time_r.r	
# Both times
time: pytime rtime

# Run benchmarks
bench: prep-bench pytime rtime 
	@echo "Deleting simulation data"
	rm scripts/benchmark/data.csv