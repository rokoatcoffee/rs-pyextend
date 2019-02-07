#!/usr/bin/python3

# -- dependencies --
import libpyextend as pyext

"""
Pure Python version of the palindrom function.
"""
def py_palindrom(sentence: str) -> bool:
    sentence = list(sentence.replace(" ", "").lower())
    size = len(sentence)

    for index in range(size // 2):
        if sentence[index] != sentence[size - 1 - index]:
            return False

    return True


sentance = "a" * 5000 + "b" + "a" * 5000

def test_py(benchmark):
    benchmark(py_palindrom, sentance)

def test_rs(benchmark):
    benchmark(pyext.palindrom, sentance)


if __name__ == "__main__":
    pyext.antigravity()