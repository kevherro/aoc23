import itertools
import operator
from functools import reduce

import numpy as np


def array_of_chars(file_path):
    with open(file_path, "r") as file:
        return np.array([list(line.strip()) for line in file])


def parts(matrix):
    rows, cols = matrix.shape
    counted = set()
    part_numbers = dict()

    def get_full_number(matrix, i, j, cols):
        number = ""
        while j < cols and matrix[i][j].isdigit():
            number += matrix[i][j]
            j += 1
        return int(number), j - 1

    def is_symbol(char):
        return not char.isdigit() and char != "."

    def has_adjacent_symbol(i, j, end_j):
        for k in range(j, end_j + 1):
            for di in range(-1, 2):
                for dj in range(-1, 2):
                    if di == 0 and dj == 0:
                        continue
                    ni, nj = i + di, k + dj
                    if 0 <= ni < rows and 0 <= nj < cols:
                        if is_symbol(matrix[ni][nj]):
                            return ni, nj
        return False

    for i in range(rows):
        j = 0
        while j < cols:
            if matrix[i][j].isdigit() and (i, j) not in counted:
                number, end_j = get_full_number(matrix, i, j, cols)
                key = has_adjacent_symbol(i, j, end_j)
                if key:
                    part_numbers.setdefault(key, []).append(number)
                    for k in range(j, end_j + 1):
                        counted.add((i, k))
                j = end_j
            j += 1

    return part_numbers


if __name__ == "__main__":
    matrix = array_of_chars("input.txt")

    def part_one():
        nested_part_numbers = parts(matrix).values()
        flattened_part_numbers = list(itertools.chain(*nested_part_numbers))
        print(sum(flattened_part_numbers))

    def part_two():
        def product(iterable):
            return reduce(operator.mul, iterable, 1)

        part_numbers = parts(matrix)
        products = []
        for x, y in part_numbers.keys():
            if matrix[x][y] == "*" and len(part_numbers[(x, y)]) == 2:
                products.append(product(part_numbers[(x, y)]))
        print(sum(products))

    part_one()
    part_two()
