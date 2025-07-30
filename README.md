# PyOF - Python Offline Features

PyOF is a Python library for managing offline feature flags and configurations. It allows developers to define complex feature rollout rules based on various classifiers, such as session data, host properties, or other custom attributes.

These feature definitions can be serialized to and from JSON, making them easy to store, manage, and distribute to different environments.

## Core Concepts

*   **Offlinefeature**: The main object representing a feature or configuration. It contains a list of buckets, their corresponding values, and the Python versions it applies to.
*   **Bucket**: Represents a segment of users or requests that will receive a specific feature value. Each bucket has a `name` and a `classifier`.
*   **Classifier**: A set of rules that are evaluated to determine if the current context (e.g., a user session) belongs to a bucket. Classifiers can be combined using logical operators (`&`, `|`).
*   **Serialization**: Features defined in Python code can be easily dumped to a JSON file. This allows for a clean separation between feature definition and application code. The library also supports loading these JSON definitions back into Python objects.

## Quick Start

Here is a simple example of how to define and use an offline feature.

### 1. Define the Feature

Create a Python script to define your feature. In this example, we create a feature that splits traffic between a "holdout" and a "control" group based on a random session number and the machine's hostname.

`create_feature.py`:
```python
from pathlib import Path
from pyof.c import REGEXMATCH
from pyof.a import SESSION_RANDOM, HOSTNAME
from pyof.of import Offlinefeature, FeatureType, PythonVersion, Bucket

# Define an offline feature
my_feature = Offlinefeature(
    type=FeatureType.OFFLINE,
    python_versions=[PythonVersion.ALL],
    buckets=[
        Bucket(
            name="holdout",
            # Classifier for the "holdout" group:
            # 10% of sessions on hosts starting with "len"
            classifier=(
                (SESSION_RANDOM() < 0.1)
                & (REGEXMATCH(attribute=HOSTNAME(), value="^len.+"))
            ),
        ),
        Bucket(
            name="control",
            # Classifier for the "control" group (same as holdout)
            classifier=(
                (SESSION_RANDOM() < 0.1)
                & (REGEXMATCH(attribute=HOSTNAME(), value="^len.+"))
            ),
        ),
    ],
    # Define the values that each bucket will receive
    values={"holdout": True, "control": False},
    default=None,
)

# Serialize the feature definition to a JSON file
my_feature.write(Path("features/my_feature.json"), indent=2)

print("Feature 'my_feature.json' created successfully.")
```

### 2. Use the Feature in Your Application

In your application, you can load the JSON definition and evaluate it to get the correct value for the current context.

`app.py`:
```python
import json
from pathlib import Path
from pyof.c import ClassifierBase
from pyof.a import CallableAttribute
from pyof.of import Offlinefeature

# Custom JSON object hook to reconstruct the feature object
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

# Load the feature from the JSON file
feature_path = Path("features/my_feature.json")
with feature_path.open() as f:
    json_string = f.read()
    feature = json.loads(json_string, object_hook=obj_hook)

# Evaluate the feature to get the bucket name and value
bucket_name = feature.get_bucket_name()
value = feature.eval()

print(f"The current session is in the '{bucket_name}' bucket.")
print(f"The feature value is: {value}")
```

## How It Works

The library uses Pydantic for data validation and serialization. The core logic revolves around the `ClassifierBase` which evaluates a set of conditions to determine the active `Bucket`. The `Offlinefeature` object then returns the appropriate value for that bucket.

This system allows you to decouple your feature flag logic from your main application code, making it easier to manage and update features without deploying new code.
