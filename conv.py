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
from pyof.a import Hostname, SessionRandom, CallableAttribute
from pyof.of import Offlinefeature, PythonVersion, FeatureType, Bucket


of = Offlinefeature(
    type=FeatureType.OFFLINE,
    python_versions=[PythonVersion.ALL],
    buckets=[
        Bucket(
            name="holdout",
            classifier=ALL(
                value=[
                    SessionRandom() < 0.1,
                    REGEXMATCH(attribute=Hostname(), value="^len.+"),
                ]
            ),
        ),
        Bucket(
            name="control",
            classifier=ALL(
                value=[
                    SessionRandom() < 0.2,
                    REGEXMATCH(attribute=Hostname(), value="^len.+"),
                ]
            ),
        ),
    ],
    values={"holdout": True, "control": False},
    default=False,
)

of.write(Path("ofs/test.json"), indent=2, only_update=True)

json_of = of.model_dump_json(indent=2)


nof = Offlinefeature.loads(json_of)

nof = rust_of.OfflineFeature.loads(json_of)

bucket = nof.get_bucket_name()

print(f"{bucket=}")
value = nof.get_value_for_bucket(bucket)
print(f"{value=}")
#
# print(
#     "same bucket?",
#     nof.get_bucket_name() == (bucket := rust_of.get_bucket_name(json_of)),
# )
# print(
#     "same random?",
#     rust_of.Attribute(name=rust_of.AttributeType.SessionRandom).eval()
#     == rust_of.Attribute(name=rust_of.AttributeType.SessionRandom).eval(),
# )
# print("bucket?", bucket)
#
#
# ### the other test
#
# print(isinstance(rust_of.Classifier.ALL, rust_of.Classifier))

# RANDOM = rust_of.Attribute(name=rust_of.AttributeType.SessionRandom)
# from pprint import pprint
#
# pprint(json.loads(nof.dumps()))
# pprint(json.loads(json_of))
