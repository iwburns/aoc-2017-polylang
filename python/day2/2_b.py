import sys
from itertools import permutations

input_file = open(sys.path[0] + "\\input.txt", "r")
input_array = input_file.readlines()

total = 0

for row in input_array:
    
    row_list = map(int, row.replace('\n', '').split('\t'))
    
    for ordered_pair in permutations(row_list, 2):
        if ordered_pair[0] % ordered_pair[1] == 0:
            total += ordered_pair[0] / ordered_pair[1]

print(total)
