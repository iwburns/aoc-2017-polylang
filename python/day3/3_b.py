x = 0
y = 0
dx = 0
dy = -1
spiral = {}
checking = True
coord_check = [(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1)]

while checking == True:
    sum_value = 0

    if (x == y == 0):
        sum_value = 1

    for check in coord_check:
        check_x, check_y = check

        if (x + check_x, y + check_y) in spiral:
            sum_value += spiral[(x + check_x, y + check_y)]
    
    spiral[(x,y)] = sum_value

    #from another solution
    if (x == y) or (x < 0 and x == -y) or (x > 0 and x == 1 - y):
        dx, dy = -dy, dx
    x = x + dx
    y = y + dy

    if (sum_value > 277833):
        checking = False

print(sum_value)
    