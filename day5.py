day5 = "files/days5.txt"

def part1_valid(rules, pages):
    mid_list = []
    invalid = []
    for pgs in pages:
        temp = set()
        for ix, pg in enumerate(pgs):
            if ix < len(pgs)-1:
                exp = (pg, pgs[ix + 1])
                temp.add(exp)
        if temp.issubset(rules):
            mid = pgs[int((len(pgs) - 1)/2)]
            mid_list.append(int(mid))
        else:
            invalid.append(pgs)
    return mid_list, invalid

def part2_valid(rules, pgs):
    valid = []
    invalid = []
    for r in rules:
        if r[0] in pgs and r[1] in pgs:
            if pgs.index(r[0]) > pgs.index(r[1]):
                pgs[pgs.index(r[0])] = r[1]
                pgs[pgs.index(r[1])] = r[0]
    temp = set()
    for ix, pg in enumerate(pgs):
        if ix < len(pgs)-1:
            exp = (pg, pgs[ix + 1])
            temp.add(exp)
    if temp.issubset(rules):
        return pgs
    else:
        part2_valid(rules, pgs)
    return pgs

word = []
rules = set()
pages = []
with open(day5) as fp:
    temp = fp.read().splitlines()
    for lines in temp:
        if "|" in lines:
            x, y = lines.split("|")
            rules.add((x,y))
        if "," in lines:
            pg = lines.split(",")
            pages.append(pg)
    valid, invalid = part1_valid(rules, pages)
    print("Day 5 Part 1:", sum(valid))

    valid = []
    for pgs in invalid:
        v = part2_valid(rules, pgs)
        valid.append(v)
    mid_list = []
    for v in valid:
        mid = v[int((len(v) - 1)/2)]
        mid_list.append(int(mid))
    print("Day 5 Part 2:", sum(mid_list))
    


