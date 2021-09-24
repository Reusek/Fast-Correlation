import pytest
from fast_correlation import covariance
import numpy as np

@pytest.mark.parametrize("function", [
    # rust_sleep,
    covariance
])
def test_example(function):
    assert callable(function)

def test_covariation():
    q = np.array([1, 2, 3], dtype="float64")
    c = covariance(q)
    assert type(c) == np.ndarray
    assert all(a == b for a, b in zip(c, q))