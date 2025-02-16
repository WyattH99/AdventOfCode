data = open("example.txt").read().strip()
# data = open("input.txt").read().strip()

data = [line for line in data.splitlines()]
# print(data)

h = 0
d = 0
aim = 0
for line in data:
    dir, dis = line.split(' ')
    if dir == 'forward':
        h += int(dis)
        d += (aim * int(dis))
    elif dir == 'down':
        aim += int(dis)
    elif dir == 'up':
        aim -= int(dis)

print(h * d)

