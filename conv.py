# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "pydantic",
# ]
# ///

from pathlib import Path
import rust_of

from rust_of import (
    OfflineFeature,
    FeatureType,
    PythonVersion,
    Bucket,
    Classifier as c,
    Attribute as a,
)


of = OfflineFeature(
    feature_type=FeatureType.Offline,
    python_versions=[PythonVersion.All],
    buckets=[
        Bucket(
            name="holdout",
            classifier=c.ALL(
                value=[
                    c.LT(a.SessionRandom(), 0.1),
                    c.REGEXMATCH(attribute=a.Hostname(), value="^len.+"),
                ],
            ),
        ),
        Bucket(
            name="control",
            classifier=c.ALL(
                value=[
                    c.LT(a.SessionRandom(), 0.9),
                    c.REGEXMATCH(attribute=a.Hostname(), value="^len.+"),
                ],
            ),
        ),
        Bucket(
            name="",
            classifier=c.ALL(
                value=[
                    c.EQ(a.StaticNumber(3), 0.9),
                ],
            ),
        ),
    ],
    values={"holdout": True, "control": 42},
    default=False,
)
Path("ofs/test.json").write_text(of.dumps(indent=True))
# of.write(Path("ofs/test.json"), indent=2, only_update=True)

json_of = of.dumps()


# nof = Offlinefeature.loads(json_of)

nof = rust_of.OfflineFeature.loads(json_of)

bucket = nof.get_bucket_name()

print(f"{bucket=}")
value = nof.get_value_for_bucket(bucket)
print(f"{value=}")
print("pair", nof.get_bucket_and_value())  #
