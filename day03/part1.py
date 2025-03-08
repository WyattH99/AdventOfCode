# data = open('example.txt').read().strip()
data = open('input.txt').read().strip()
# print(data)

data = [line for line in data.splitlines()]
# print(data)

gamma = [0] * len(data[0])
epsilon = [0] * len(data[0])

for string in data:
    for index,char in enumerate(string):
        if char == '0':
            gamma[index] -= 1
            epsilon[index] += 1
        else:
            gamma[index] += 1
            epsilon[index] -= 1

for index,num in enumerate(gamma):
    if num > 0:
        gamma[index] = 1
    else:
        gamma[index] = 0
for index,num in enumerate(epsilon):
    if num > 0:
        epsilon[index] = 1
    else:
        epsilon[index] = 0

gamma = int(''.join(map(str, gamma)), 2)
epsilon = int(''.join(map(str, epsilon)), 2)

print(gamma * epsilon)
# 3429254
