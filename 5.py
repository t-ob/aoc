if __name__ == '__main__':
    i = 0
    with open('5_input', 'r') as fd:
        for line in fd:
            if line:
                vowels = 0
                double_letter = False
                bad_pairs = False
                
                prev = None
                for c in line:
                    if c in 'aeiou':
                        vowels += 1
                    if c == prev:
                        double_letter = True
                    if prev == 'a' and c == 'b' or \
                       prev == 'c' and c == 'd' or \
                       prev == 'p' and c == 'q' or \
                       prev == 'x' and c == 'y':
                        bad_pairs = True

                    prev = c

                if 3 <= vowels and double_letter and not bad_pairs:
                    i += 1

    print i
