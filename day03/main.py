# data = open('example.txt').read().strip()
data = open('input.txt').read().strip()
# print(data)

data = [line for line in data.splitlines()]
# print(data)


# iterate over oxy columns
# find the most common bit left in that column of oxy
# remove all numbers that don't have it
# repeat until len(oxy) == 1

oxy = data.copy()
for i in range(len(oxy[0])):
    if len(oxy) == 1:
        break
    common = 0
    for s in oxy:
        if s[i] == "1":
            common += 1
        else:
            common -= 1
    
    if common >= 0:
        # keep the 1s
        for s in oxy[:]:
            if s[i] == "0":
                oxy.remove(s)
    else:
        # keep the 0s
        for s in oxy[:]:
            if s[i] == "1":
                oxy.remove(s)
print(oxy)
print(int(oxy[0], 2))

co2 = data.copy()
# iterate over the columns
for i in range(len(co2[0])):
    if len(co2) == 1:
        break
    # find the least common value
    c = 0
    for s in co2:
        if s[i] == "1":
            c -= 1
        else:
            c += 1
    # remove all values that don't have this number in that column
    for s in co2[:]:
        if c <= 0:
            # keep the 0s
            if s[i] == "1":
                co2.remove(s)
        else: 
            # keep the 1s
            if s[i] == "0":
                co2.remove(s)
print(co2)
print(int(co2[0], 2))

print(f"result: {int(oxy[0],2) * int(co2[0],2)}")
