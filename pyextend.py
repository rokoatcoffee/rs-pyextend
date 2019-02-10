#!/usr/bin/python3

# -- dependencies --
import libpyextend as pyext
from itertools import permutations


# -- python implementation --

"""
Pure Python version of the palindrom function.
"""
def py_palindrome(sentence: str) -> bool:
    sentence = list(sentence.replace(" ", "").lower())
    size = len(sentence)

    for index in range(size // 2):
        if sentence[index] != sentence[size - 1 - index]:
            return False

    return True


"""
Python itertools implmentation of permutations.
"""
def py_permutate(word: str) -> list:
    return [''.join(x) for x in permutations(word)]


# -- benchmark test --

sentance = "a" * 5000 + "b" + "a" * 5000
word = "abcdef"

def test_py_palindrome(benchmark):
    benchmark(py_palindrome, sentance)

def test_rs_palindrome(benchmark):
    benchmark(pyext.palindrome, sentance)

def test_py_permutation(benchmark):
    benchmark(py_permutate, word)

def test_rs_permutation(benchmark):
    benchmark(pyext.permutation, word)


if __name__ == "__main__":
    pyext.antigravity()