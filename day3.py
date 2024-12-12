import re
day3 = "files/days3.txt"

def mul(a,b):
    return a*b

with open(day3) as fp:
    temp = fp.read()
    temp = temp.replace("\n", "...")
    instructions = re.findall("mul\([0-9]+,[0-9]++\)", temp)
    sum_ins = 0
    for ins in instructions:
        sum_ins += eval(ins)
    print("Day3 part 1: ", sum_ins)
    
    instructions = re.finditer(r"don't\(\).*?do\(\)|don't\(\).*?$", temp)
    while(re.search(r"don't\(\).*?do\(\)|don't\(\).*?$", temp) is not None):
        for ins in instructions:
            x,y = ins.span()
            temp = temp[:x] + temp[y:]
            break
        instructions = re.finditer(r"don't\(\).*?do\(\)|don't\(\).*?$", temp)

    instructions = re.findall("mul\([0-9]+,[0-9]++\)", temp)
    sum_ins = 0
    for ins in instructions:
        sum_ins += eval(ins)
    print("Day3 part 2: ", sum_ins)
