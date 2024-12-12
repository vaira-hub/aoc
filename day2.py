day2 = "files/days2.txt"

def are_levels_in_order(l1):
    dec = l1 == sorted(l1, reverse=True)
    inc = l1 == sorted(l1)
    return inc or dec

def adjacent_level_check(l1):
    for ix, num in enumerate(l1):
        if ix < len(l1)-1:
            diff = abs(num - l1[ix+1])
            if diff < 1 or diff > 3:
                return False
        else:
            return True

with open(day2) as fp:
    temp = fp.read().splitlines()
    safe1 = 0
    safe2 = 0
    for l in temp:
        r_str = l.split(" ")
        report = list(map(int, r_str))
        n_report = report.copy()
        if are_levels_in_order(report):
            if adjacent_level_check(report):
                safe1 += 1
                safe2 += 1
                continue
        for idx, ele in enumerate(report):
            del n_report[idx]
            if are_levels_in_order(n_report):
                if adjacent_level_check(n_report):
                    safe2 += 1
                    break
            n_report.insert(idx, ele)
print("Day2 part 1: ", safe1)
print("Day2 part 2: ", safe2)

        
