from ._setcover_lib import greedy_set_cover_py as _greedy_set_cover_py


# 2. Define the new, user-facing Python function.
def setcover(sets: dict, algo: str = "greedy-1") -> set:
    """
    Finds an approximate solution to the set cover problem.

    This is a user-friendly Python wrapper around the core Rust implementation.

    Args:
        sets (dict): A dictionary of lists
        algo (str, optional): The algorithm to use.
                              greedy-0 for HashSet-based, greedy-1 for BitVec-based.
                              Defaults to greedy-1, which for most cases is faster.

    Returns:
        set: A set containing the keys of the chosen sets that form the cover.
    """
    match algo:
        case "greedy-0":
            return _greedy_set_cover_py(sets, 0)
        case "greedy-1":
            return _greedy_set_cover_py(sets, 1)
        case _:
            msg = f"""<algo> must be in ("greedy-0", "greedy-1") but is {algo}"""
            raise ValueError(msg)


__all__ = [
    "setcover",
]
