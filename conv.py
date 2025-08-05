from of import (
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
                    a.SessionRandom() < 0.1,
                    c.REGEXMATCH(attribute=a.Hostname(), value="^len.+"),
                ],
            ),
            value=True,
        ),
        Bucket(
            name="control",
            classifier=c.ALL(
                value=[
                    a.SessionRandom() < 0.2,
                    c.REGEXMATCH(attribute=a.Hostname(), value="^len.+"),
                ],
            ),
            value=False,
        ),
        Bucket(
            name="",
            classifier=c.ALL(
                value=[
                    c.EQ(a.StaticNumber(3), 0.9),
                ],
            ),
            value=None,
        ),
    ],
    default=False,
)
of.write_to_disk("ofs/test.json")
json_of = of.dumps()


nof = OfflineFeature.loads(json_of)


bucket = nof.get_bucket_name()

print(f"{bucket=}")
print("pair", nof.get_bucket_name_and_value())  #
print("pair", nof.get_bucket())  #
