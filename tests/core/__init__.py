from datagears.core.network import Depends


def add(a, b=10) -> int:
    return a + b


def reduce(c: int, sum: int = Depends(add)) -> int:
    return sum - c


def add_one() -> int:
    return 1


def my_out(reduced: int = Depends(reduce), add_one: int = Depends(add_one)) -> float:
    return reduced / 2
