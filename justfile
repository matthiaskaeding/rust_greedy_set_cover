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
	maturin develop -m py-setcover/Cargo.toml --uv

# Copies repo into clipboard, needs reposyn
rsyn:
	reposyn -i rcpp_greedy_set_cover/ -c
	