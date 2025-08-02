from enum import Enum
from pydantic import BaseModel
from .c import ClassifierBase
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
    defaut: object | None = None

    def get_bucket_name(self):
        for bucket in self.buckets:
            if bucket.classifier.eval():
                return bucket.name
        return "default"

    def get_value_for_bucket(self, bucket_name) -> object:
        if bucket_name == "default":
            return self.defaut
        return self.values[bucket_name]

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
