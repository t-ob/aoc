import re



def toggle(grid, x, y):
    grid[x][y] = 1 - grid[x][y]

def turn_on(grid, x, y):
    grid[x][y] = 1

def turn_off(grid, x, y):
    grid[x][y] = 0

def parse_line(line):
    dims = [int(n) for n in re.findall(r'\d+', line)]
    x1, y1, x2, y2 = dims

    if line.startswith('toggle'):
        fn = toggle
    elif line.startswith('turn on'):
        fn = turn_on
    elif line.startswith('turn off'):
        fn = turn_off

    return fn, x1, y1, x2, y2

if __name__ == '__main__':
    i = 0
    grid = [[0] * 1000] * 1000
    with open('6_input_test', 'r') as fd:
        for line in fd:
            if line:
                fn, x1, y1, x2, y2 = parse_line(line)
                coords = [(x, y)
                          for x in range(x1, x2 + 1)
                          for y in range(y1, y2 + 1)]
                for x, y in coords:
                    fn(grid, x, y)
    for row in grid:
        i += sum(row)

    print i
