from typing import TypedDict, Unpack

__all__ = ['f1', 'f2']

class _Args(TypedDict):
    a: int
    b: int

def f1(**kwargs: Unpack[_Args]):
    ...

def f2(*, a: int, b: int):
    ...
