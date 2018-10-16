import sys

input_file = open(sys.path[0] + "\\input.txt", "r")
digit_list = input_file.readlines()[0]

sum_array = []
previous_digit = None
halfway_index = int(len(digit_list) / 2)

for index, digit in enumerate(digit_list):

	halfway_digit_index = (index + halfway_index) % len(digit_list)

	if digit == digit_list[halfway_digit_index]:
		sum_array.append(int(digit))

	previous_digit = digit

print(sum(sum_array))