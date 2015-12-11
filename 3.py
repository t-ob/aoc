if __name__ == '__main__':
    position = (0,0)
    houses = {position}
    
    with open('3_input', 'r') as fd:
        directions = fd.read().strip()
        for direction in directions:
            x, y = position
            if direction == '^':
                position = (x, y+1)
            elif direction == '>':
                position = (x+1, y)
            elif direction == 'v':
                position = (x, y-1)
            elif direction == '<':
                position = (x-1, y)

            houses.add(position)

    print len(houses)
