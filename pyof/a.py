from __future__ import annotations

from abc import ABC, abstractmethod

from pydantic import BaseModel
from .c import LESS_THAN


_CallableAttributes = {}


class CallableAttribute(ABC, BaseModel):
    type: str = "callable-attribute"
    name: str

    @classmethod
    @abstractmethod
    def eval(cls): ...

    def __init_subclass__(cls) -> None:
        _CallableAttributes[cls.name] = cls
        return super().__init_subclass__()

    def __lt__(self, other):
        return LESS_THAN(attribute=self, value=other)

    @classmethod
    def get_attribute(cls, attribute_name: str) -> CallableAttribute:
        return _CallableAttributes[attribute_name]


class HOSTNAME(CallableAttribute):
    name: str = "socket.hostname"

    @classmethod
    def eval(cls) -> str:
        import socket

        return socket.gethostname()


class SESSION_RANDOM(CallableAttribute):
    name: str = "random.session"

    @classmethod
    def eval(cls) -> float:
        import random

        return random.random()
