day1 = "files/days4.txt"

def find_xmas(ix, jx, word):
    temp = 0
    for d in range(1,9):
        n_ix, n_jx = ix, jx
        xmas = word[ix][jx]
        for x in range(1,4):
            try:
                if d == 1:
                    n_jx += 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[ix][n_jx]
                if d == 2:
                    n_jx += 1
                    n_ix += 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][n_jx]
                if d == 3:
                    n_ix += 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][jx]
                if d == 4:
                    n_jx -= 1
                    n_ix += 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][n_jx]
                if d == 5:
                    n_jx -= 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[ix][n_jx]
                if d == 6:
                    n_jx -= 1
                    n_ix -= 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][n_jx]
                if d == 7:
                    n_ix -= 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][jx]
                if d == 8:
                    n_jx += 1
                    n_ix -= 1
                    if n_jx < 0 or n_ix < 0: continue
                    xmas += word[n_ix][n_jx]
            except IndexError:
                continue
        if xmas == "XMAS":
            temp += 1
    return temp

def find_mas(ix, jx, word):
    temp = 0
    try:
        mas1 = word[ix][jx]
        n_ix, n_jx = ix + 1, jx + 1
        p_ix, p_jx = ix - 1, jx - 1
        if n_jx < 0 or n_ix < 0 or p_ix < 0 or p_jx < 0:
            return 0
        mas1 = ''.join((word[p_ix][p_jx], mas1, word[n_ix][n_jx]))
        mas2 = word[ix][jx]
        n_ix, n_jx = ix - 1, jx + 1
        p_ix, p_jx = ix + 1, jx - 1
        if n_jx < 0 or n_ix < 0 or p_ix < 0 or p_jx < 0:
            return 0
        mas2 = ''.join((word[p_ix][p_jx], mas2, word[n_ix][n_jx]))
        if (mas1 == "MAS" or mas1 == "SAM") and (mas2 == "MAS" or mas2 == "SAM"):
            temp += 1
    except IndexError:
        return 0
    return temp

word = []
xmas_count = 0
mas_count = 0
with open(day1) as fp:
    temp = fp.read().splitlines()
    for i in temp:
        word.append(list(i))
    for ix, i in enumerate(word):
        for jx, j in enumerate(i):
            if j == "X":
                temp = find_xmas(ix, jx, word)
                xmas_count += temp                
    print("Day4 part 1:", xmas_count)

    for ix, i in enumerate(word):
        for jx, j in enumerate(i):
            if j == "A":
                temp = find_mas(ix, jx, word)
                mas_count += temp
    print("Day4 part 2:", mas_count)
