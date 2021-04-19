import importlib

CORE_FIXTURES = [
    "tests.core.fixtures.features",
    "tests.core.fixtures.package",
    "tests.core.fixtures.engine",
]


INCLUDE_FIXTURES = CORE_FIXTURES


for _fixture in INCLUDE_FIXTURES:
    module = importlib.import_module(_fixture)
    globals().update(
        {n: getattr(module, n) for n in module.__all__}  # type: ignore
        if hasattr(module, "__all__")
        else {k: v for (k, v) in module.__dict__.items() if not k.startswith("_")}
    )
