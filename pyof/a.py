from __future__ import annotations


from pydantic import BaseModel
from .c import LESS_THAN

import rust_of

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


for name, type_ in rust_of.AttributeType.members().items():
    _, class_name = str(type_).rsplit(".", 1)
    globals()[class_name] = type(
        class_name,
        (CallableAttribute,),
        {"__annotations__": {"name": str}, "name": name, "eval": lambda x: None},
    )
