import sys

input_file = open(sys.path[0] + "\\input.txt", "r")
input_array = input_file.readlines()

total = 0
for row in input_array:
    row_list = sorted(map(int, row.replace('\n', '').split('\t')))
    total += row_list[-1] - row_list[0]

print(total)
