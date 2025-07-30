# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "pydantic",
# ]
# ///

import json
from pathlib import Path
import rust_of

#
from pyof.c import ClassifierBase, _classifiers, REGEXMATCH, ALL, LESS_THAN, ANY
from pyof.a import SESSION_RANDOM, HOSTNAME, CallableAttribute
from pyof.of import Offlinefeature, PythonVersion, FeatureType, Bucket


of = Offlinefeature(
    type=FeatureType.OFFLINE,
    python_versions=[PythonVersion.ALL],
    buckets=[
        Bucket(
            name="holdout",
            classifier=ALL(
                value=[
                    LESS_THAN(attribute=SESSION_RANDOM(), value=0.1),
                    REGEXMATCH(attribute=HOSTNAME(), value="^len.+")
                ]
            ),
        ),
        Bucket(
            name="control",
            classifier=ALL(
                value=[
                    LESS_THAN(attribute=SESSION_RANDOM(), value=0.1),
                    REGEXMATCH(attribute=HOSTNAME(), value="^len.+")
                ]
            ),
        ),
    ],
    values={"holdout": True, "control": False},
    default=None,
)

of.write(Path("ofs/test.json"), indent=2, only_update=True)

json_of = of.model_dump_json(indent=2)


def obj_hook(dct):
    if "type" not in dct:
        return dct

    type_ = dct["type"]
    if type_ == "callable-attribute":
        return CallableAttribute.get_attribute(dct["name"])()
    if type_ == "offline-feature":
        return Offlinefeature.model_validate(dct)
    if classifier := ClassifierBase.get_classifier(type_):
        return classifier.model_validate(dct)

    return dct


nof = json.loads(json_of, object_hook=obj_hook)


print(nof.get_bucket_name())
print(rust_of.get_bucket_name(json_of))
