import sys

input_file = open(sys.path[0] + "\\input.txt", "r")
digit_list = input_file.readlines()

sum_array = []
first_digit = digit_list[0][0]
previous_digit = None

for index, digit in enumerate(digit_list[0]):

	if digit == previous_digit:
		sum_array.append(int(digit))

	previous_digit = digit

if first_digit == previous_digit:
	sum_array.append(int(previous_digit))

print(sum(sum_array))