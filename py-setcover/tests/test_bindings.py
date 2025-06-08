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
    assert result == ["A"]


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
    # The result should be ['S1', 'S2', 'S3'] in this case for the greedy algorithm
    expected_result = {"S1", "S2", "S3"}
    assert set(result_0) == expected_result
    assert set(result_1) == expected_result


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
    assert set(result) == {"A", "B"}


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


def test_different_key_types():
    """
    Tests that the set cover algorithm works with different key types.
    """
    # Test with integer keys
    sets_int = {1: [1, 2, 3], 2: [3, 4, 5]}
    result_int = setcover(sets_int)
    assert len(result_int) == 2
    assert set(result_int) == {1, 2}


def test_verify_coverage():
    """
    Tests that the selected sets actually cover all elements in the universe.
    """
    # Test case 1: Simple case
    sets1 = {"A": [1, 2, 3], "B": [3, 4, 5], "C": [5, 6, 7]}
    result1 = setcover(sets1)

    # Get all elements from selected sets
    covered_elements = set()
    for set_name in result1:
        covered_elements.update(sets1[set_name])

    # Get all elements in the universe
    universe = set()
    for elements in sets1.values():
        universe.update(elements)

    assert covered_elements == universe

    # Test case 2: More complex case with overlapping elements
    sets2 = {
        "X": [1, 2, 3, 4],
        "Y": [3, 4, 5, 6],
        "Z": [5, 6, 7, 8],
        "W": [1, 8, 9, 10],
    }
    result2 = setcover(sets2)

    # Verify coverage
    covered_elements = set()
    for set_name in result2:
        covered_elements.update(sets2[set_name])

    universe = set()
    for elements in sets2.values():
        universe.update(elements)

    assert covered_elements == universe


def test_string_key_int_values():
    sets = {
        "A": [1, 2, 3],
        "B": [2, 3, 4],
        "C": [3, 4, 5],
    }
    result = setcover(sets)
    assert isinstance(result, list)
    assert all(isinstance(k, str) for k in result)
    assert len(result) > 0


def test_string_key_string_values():
    sets = {
        "A": ["1", "2", "3"],
        "B": ["2", "3", "4"],
        "C": ["3", "4", "5"],
    }
    result = setcover(sets)
    assert isinstance(result, list)
    assert all(isinstance(k, str) for k in result)
    assert len(result) > 0


def test_int_key_int_values():
    sets = {
        1: [1, 2, 3],
        2: [2, 3, 4],
        3: [3, 4, 5],
    }
    result = setcover(sets)
    assert isinstance(result, list)
    assert all(isinstance(k, int) for k in result)
    assert len(result) > 0


def test_int_key_string_values():
    sets = {
        1: ["1", "2", "3"],
        2: ["2", "3", "4"],
        3: ["3", "4", "5"],
    }
    result = setcover(sets)
    assert isinstance(result, list)
    assert all(isinstance(k, int) for k in result)
    assert len(result) > 0


def test_invalid_input():
    with pytest.raises(TypeError):
        setcover("not a dict")

    with pytest.raises(TypeError):
        setcover({"A": "not a list"})

    with pytest.raises(TypeError):
        setcover({1.0: [1, 2, 3]})  # float key

    with pytest.raises(TypeError):
        setcover({"A": [1.0, 2.0, 3.0]})  # float values

    with pytest.raises(ValueError):
        setcover({})  # empty dict

    with pytest.raises(ValueError):
        setcover({"A": [], "B": []})  # empty lists

    with pytest.raises(ValueError):
        setcover({"A": [1, 2, 3]}, algo="invalid")
