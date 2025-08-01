from __future__ import annotations
from pydantic import BaseModel
from typing import Type

_classifiers: dict[str, Type[ClassifierBase]] = {}


class ClassifierBase(BaseModel):
    type: str
    attribute: object
    value: object

    def __and__(self, with_what):
        return AND(value=[self, with_what])

    def eval(self) -> bool:
        return _classifiers[self.type].eval(self)

    def __init_subclass__(cls) -> None:
        type_ = getattr(cls, "type", None)
        if type_ not in _classifiers:
            _classifiers[type_] = cls

        return super().__init_subclass__()

    @classmethod
    def get_classifier(cls, classifier_name: str) -> ClassifierBase:
        return _classifiers.get(classifier_name)


class LESS_THAN(ClassifierBase):
    type: str = "comparison.lt"

    def eval(self) -> bool:
        return self.attribute.eval() < self.value


class ALL(ClassifierBase):
    type: str = "bool.all"
    attribute: None = None

    def eval(self) -> bool:
        for value in self.value:
            if not value.eval():
                return False
        return True


class ANY(ClassifierBase):
    type: str = "bool.any"
    attribute: None = None

    def eval(self) -> bool:
        for value in self.value:
            if not value.eval():
                return False
        return True


class REGEXMATCH(ClassifierBase):
    type: str = "re.match"
    value: str

    def eval(self) -> bool:
        import re

        return bool(re.match(self.value, self.attribute.eval()))


_classifiers["comparison.lt"] = LESS_THAN
_classifiers["bool.all"] = ALL
_classifiers["bool.any"] = ANY
_classifiers["re.match"] = REGEXMATCH
