import unittest
import os
import json
from of import (
    OfflineFeature,
    FeatureType,
    PythonVersion,
    Bucket,
    Classifier as c,
    Attribute as a,
    eval,
    EvalResultType,
)

class TestEval(unittest.TestCase):
    def setUp(self):
        self.features_dir = "features/materialized"
        os.makedirs(self.features_dir, exist_ok=True)
        self.feature_name = "test_feature"
        self.feature_path = os.path.join(self.features_dir, f"{self.feature_name}.json")

        of = OfflineFeature(
            feature_type=FeatureType.Offline,
            python_versions=[PythonVersion.All],
            buckets=[
                Bucket(
                    name="control",
                    classifier=c.ALL(
                        value=[
                            a.StaticNumber(1) == 1
                        ]
                    ),
                    value="feature_value"
                )
            ],
            default="default_value"
        )
        with open(self.feature_path, "w") as f:
            f.write(of.dumps(True))

    def tearDown(self):
        if os.path.exists(self.feature_path):
            os.remove(self.feature_path)

    def test_eval_feature_exists(self):
        result = eval(self.feature_name, "default_value")
        self.assertEqual(result.result_type, EvalResultType.Ok)
        self.assertEqual(result.bucket_name, "control")
        self.assertEqual(result.value, "feature_value")

    def test_eval_feature_does_not_exist(self):
        result = eval("non_existent_feature", "default_value")
        self.assertEqual(result.result_type, EvalResultType.NotExist)
        self.assertIsNone(result.bucket_name)
        self.assertEqual(result.value, "default_value")

    def test_eval_feature_is_invalid(self):
        with open(self.feature_path, "w") as f:
            f.write("invalid json")
        result = eval(self.feature_name, "default_value")
        self.assertEqual(result.result_type, EvalResultType.Error)
        self.assertIsNone(result.bucket_name)
        self.assertEqual(result.value, "default_value")

    def test_eval_default_bucket(self):
        of = OfflineFeature(
            feature_type=FeatureType.Offline,
            python_versions=[PythonVersion.All],
            buckets=[
                Bucket(
                    name="control",
                    classifier=c.ALL(
                        value=[
                            a.StaticNumber(1) == 0
                        ]
                    ),
                    value="feature_value"
                )
            ],
            default="default_value"
        )
        with open(self.feature_path, "w") as f:
            f.write(of.dumps(True))

        result = eval(self.feature_name, "default_value")
        self.assertEqual(result.result_type, EvalResultType.Default)
        self.assertEqual(result.bucket_name, "default")
        self.assertEqual(result.value, "default_value")

if __name__ == '__main__':
    unittest.main()