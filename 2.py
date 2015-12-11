def parse_line(line):
    return [int(s) for s in line.split('x')]

if __name__ == '__main__':
    i = 0
    with open('2_input', 'r') as fd:
        for line in fd:
            if line:
                dims = parse_line(line)
                dims.sort()
                a, b, c = dims
                i += 2*a*b + 2*b*c + 2*a*c + a* b

    print i
