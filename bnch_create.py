# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "pydantic",
# ]
# ///

import sys

sys.path.append("../")


def a():
    from pyof.of import Offlinefeature


def b():
    from rust_of import OfflineFeature


if len(sys.argv) < 2:
    raise ValueError("Invalid argument, use 'a' or 'b' not nothing")  # pragma: no cover
if sys.argv[1] == "a":
    a()
elif sys.argv[1] == "b":
    b()
else:
    raise ValueError("Invalid argument, use 'a' or 'b'")  # pragma: no cover
