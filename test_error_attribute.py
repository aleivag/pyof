import unittest
from of import (
    OfflineFeature,
    FeatureType,
    PythonVersion,
    Bucket,
    Classifier as c,
    Attribute as a,
)

class TestErrorAttribute(unittest.TestCase):
    def test_error_attribute(self):
        error_of = OfflineFeature(
            feature_type=FeatureType.Offline,
            python_versions=[PythonVersion.All],
            buckets=[
                Bucket(
                    name="error_bucket",
                    classifier=c.ALL(
                        value=[
                            a.ErrorAttribute() == 1,
                        ],
                    ),
                    value=True,
                ),
            ],
            default=False,
        )

        error_bucket_name = error_of.get_bucket_name()
        self.assertEqual(error_bucket_name, "default", f"Expected 'default' but got '{error_bucket_name}'")

if __name__ == '__main__':
    unittest.main()