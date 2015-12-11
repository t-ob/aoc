import md5

if __name__ == '__main__':
    secret = 'bgvyzdsv'
    i = 0
    while True:
        key = secret + str(i)
        digest = md5.md5(key).digest()
        leading_zeroes = 0
        for x in digest[:3].encode('hex'):
            if not x == '0':
                break
            else:
                leading_zeroes += 1
        if 5 <= leading_zeroes:
            break

        i += 1

    print i
