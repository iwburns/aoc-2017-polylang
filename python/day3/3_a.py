n = 1
layer_count = 0
# side_length = 1

while n*n <= 277678:
    n += 2
    layer_count += 1
    # side_length += 2

# print(n*n - 277678)
# print(side_length)

print(abs(layer_count - (n*n - 277678)) + layer_count)
