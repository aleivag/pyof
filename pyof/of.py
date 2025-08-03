from enum import Enum
from pydantic import BaseModel
from .c import ClassifierBase
from .a import _CallableAttributes
from pathlib import Path
import json


class FeatureType(Enum):
    OFFLINE = "offline-feature"


class PythonVersion(Enum):
    ALL = "all"
    PY310 = "py310"
    PY312 = "py312"
    PY314 = "py314"


class Bucket(BaseModel):
    name: str
    classifier: ClassifierBase


class Offlinefeature(BaseModel):
    type: FeatureType
    python_versions: list[PythonVersion]
    buckets: list[Bucket]
    values: dict[str, object] = {}
    default: object | None = None

    def get_bucket_name(self):
        for bucket in self.buckets:
            if bucket.classifier.eval():
                return bucket.name
        return "default"

    def get_value_for_bucket(self, bucket_name) -> object:
        if bucket_name == "default":
            return self.defaut
        return self.values[bucket_name]

    @classmethod
    def loads(self, json_string):
        def obj_hook(dct):
            if "type" not in dct:
                return dct

            type_ = dct["type"]
            if type_ == "callable-attribute":
                # name = rust_of.AttributeType.members()[dct["name"]]
                return _CallableAttributes[dct["name"]]()
                # return rust_of.Attribute(attribute_type=type_, name=name)
            if type_ == "offline-feature":
                return Offlinefeature.model_validate(dct)
            if classifier := ClassifierBase.get_classifier(type_):
                return classifier.model_validate(dct)

            return dct

        return json.loads(json_string, object_hook=obj_hook)

    def eval(self) -> object:
        return self.get_value_for_bucket(self.get_bucket_name())

    def write(self, path: Path, indent: int = 2, only_update: bool = False) -> None:
        model = self.model_dump_json(indent=indent)
        if only_update and path.exists():
            with path.open("rb") as f:
                if json.load(f) == json.loads(model):
                    return

        with open(path, "w") as f:
            f.write(model)
