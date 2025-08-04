# PyOF - Python Offline Features

PyOF is a Python library for managing offline feature flags and configurations. It allows developers to define complex feature rollout rules based on various classifiers, such as session data, host properties, or other custom attributes.

These feature definitions can be serialized to and from JSON, making them easy to store, manage, and distribute to different environments.

## Core Concepts

* **Offlinefeature**: The main object representing a feature or configuration. It contains a list of buckets, their corresponding values, and the Python versions it applies to.
* **Bucket**: Represents a segment of users or requests that will receive a specific feature value. Each bucket has a `name` and a `classifier`.
* **Classifier**: A set of rules that are evaluated to determine if the current context (e.g., a user session) belongs to a bucket. Classifiers can be combined using logical operators (`&`, `|`).
* **Serialization**: Features defined in Python code can be easily dumped to a JSON file. This allows for a clean separation between feature definition and application code. The library also supports loading these JSON definitions back into Python objects.

## Quick Start

Here is a simple example of how to define and use an offline feature.

### 1. Define the Feature

Create a Python script to define your feature. In this example, we create a feature that splits traffic between a "holdout" and a "control" group based on a random session number and the machine's hostname.

`create_feature.py`:

```python
from pathlib import Path
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
of.write_to_disk("ofs/test.json", True)

```

### 2. Use the Feature in Your Application

In your application, you can load the JSON definition and evaluate it to get the correct value for the current context.

`app.py`:

```python
from of import OfflineFeature

json_of = of.dumps()

nof = OfflineFeature.loads(json_of)

bucket = nof.get_bucket_name()

print(f"{bucket=}")
value = nof.get_value_for_bucket(bucket)
print(f"{value=}")
print("pair", nof.get_bucket_and_value())
```

## How It Works

The library uses Pydantic for data validation and serialization. The core logic revolves around the `ClassifierBase` which evaluates a set of conditions to determine the active `Bucket`. The `Offlinefeature` object then returns the appropriate value for that bucket.

This system allows you to decouple your feature flag logic from your main application code, making it easier to manage and update features without deploying new code.