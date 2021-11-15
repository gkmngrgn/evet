import asyncio
import typing as t
from functools import wraps


def click_coroutine(f: t.Callable[..., t.Any]) -> t.Callable[..., t.Any]:
    @wraps(f)
    def wrapper(*args: t.Any, **kwargs: t.Any) -> t.Any:
        return asyncio.run(f(*args, **kwargs))

    return wrapper
