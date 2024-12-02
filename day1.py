# 2024 day1

day1 = "files/days1.txt"
side1 = []
side2 = []
with open(day1) as fp:
    temp = fp.read().splitlines()
    for l in temp:
        x, y = l.split("   ")
        side1.append(int(x))
        side2.append(int(y))

def day1_1(side1, side2):
    sum_of_dist = 0
    while(side1):
        m1 = min(side1)
        m2 = min(side2)
        dist = abs(m1 - m2)
        sum_of_dist += dist
        side1.remove(m1)
        side2.remove(m2)
    return sum_of_dist
print("Day1 part 1: ", day1_1(side1.copy(), side2.copy()))

def day1_2(side1, side2):
    similarity = 0
    for i in side1:
        score = side2.count(i)
        similarity += score * i
    return similarity
print("Day1 part 1: ", day1_2(side1.copy(), side2.copy()))
