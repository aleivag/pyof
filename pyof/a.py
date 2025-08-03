from __future__ import annotations


from pydantic import BaseModel
from .c import LESS_THAN


_CallableAttributes = {}


class CallableAttribute(BaseModel):
    type: str = "callable-attribute"
    name: str

    def __init_subclass__(cls) -> None:
        _CallableAttributes[cls.name] = cls
        return super().__init_subclass__()

    def __lt__(self, other):
        return LESS_THAN(attribute=self, value=other)

    @classmethod
    def get_attribute(cls, attribute_name: str) -> CallableAttribute:
        return _CallableAttributes[attribute_name]


class Hostname(CallableAttribute):
    name: str = "socket.hostname"


class SessionRandom(CallableAttribute):
    name: str = "random.session"
