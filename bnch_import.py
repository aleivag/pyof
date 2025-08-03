# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "pydantic",
# ]
# ///

import sys

sys.path.append("../")


def a():
    from pyof.of import Offlinefeature, FeatureType, PythonVersion, Bucket
    from pyof.c import ALL

    of = Offlinefeature(
        type=FeatureType.OFFLINE,
        python_versions=[PythonVersion.ALL],
        buckets=[
            Bucket(
                name="holdout",
                classifier=ALL(value=[]),
            ),
            Bucket(
                name="control",
                classifier=ALL(value=[]),
            ),
        ],
        values={"holdout": True, "control": False},
        default=False,
    )


def b():
    from rust_of import OfflineFeature, FeatureType, PythonVersion, Bucket, Classifier

    of = OfflineFeature(
        feature_type=FeatureType.Offline,
        python_versions=[PythonVersion.All],
        buckets=[
            Bucket(
                name="holdout",
                classifier=Classifier.ALL(attribute=None, value=[]),
            ),
            Bucket(
                name="control",
                classifier=Classifier.ALL(attribute=None, value=[]),
            ),
        ],
        values={"holdout": True, "control": False},
        default=False,
    )


if len(sys.argv) < 2:
    raise ValueError("Invalid argument, use 'a' or 'b' not nothing")  # pragma: no cover
if sys.argv[1] == "a":
    a()
elif sys.argv[1] == "b":
    b()
else:
    raise ValueError("Invalid argument, use 'a' or 'b'")  # pragma: no cover
