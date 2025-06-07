import pytest
from setcover import setcover


def test_obvious_choice_is_taken():
    """
    Tests a simple case where one set contains all the elements
    and should be the only set chosen.
    """
    # Arrange: Define the input data where 'A' is the clear winner
    sets = {"A": [1, 2, 3, 4, 5], "B": [1, 2], "C": [3, 4]}

    # Act: Call the function we want to test
    result = setcover(sets, algo="greedy-0")

    # Assert: Check if the result is exactly what we expect
    assert result == {"A"}


def test_algorithms_agree_on_simple_case():
    """
    Tests that for a simple case, both the HashSet ('greedy-0') and
    BitVec ('greedy-1') implementations yield the same result.
    """
    # Arrange
    sets = {"S1": [1, 2, 3], "S2": [3, 4, 5], "S3": [5, 6, 7]}

    # Act
    result_0 = setcover(sets, algo="greedy-0")
    result_1 = setcover(sets, algo="greedy-1")

    # Assert
    # The result should be {'S1', 'S2', 'S3'} in this case for the greedy algorithm
    expected_result = {"S1", "S2", "S3"}
    assert result_0 == expected_result
    assert result_1 == expected_result


def test_default_algorithm_runs():
    """
    Tests that calling the function without an 'algo' argument works
    and uses the default ("greedy-1").
    """
    # Arrange
    sets = {"A": [1, 2], "B": [2, 3]}

    # Act
    try:
        result = setcover(sets)  # Call with no algo
    except Exception as e:
        pytest.fail(f"Calling with default algo failed unexpectedly: {e}")

    # Assert
    # For this input, the greedy choice is non-deterministic (A or B first),
    # but the final cover will always have 2 sets.
    assert len(result) == 2
    assert result == {"A", "B"}


# --- Test Cases for Error Handling ---


def test_invalid_algo_string_raises_error():
    """
    Tests that the function correctly raises a ValueError when an
    unrecognized string is passed for the 'algo' parameter.
    """
    # Arrange
    sets = {"A": [1]}

    # Act & Assert
    # We use pytest.raises to check that the specified error is thrown.
    with pytest.raises(ValueError) as excinfo:
        setcover(sets, algo="invalid-algorithm-name")

    # Optionally, check that the error message is helpful
    assert 'must be in ("greedy-0", "greedy-1")' in str(excinfo.value)
