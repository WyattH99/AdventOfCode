# data = open("example.txt").read().strip()
data = open("input.txt").read().strip()

data = [line for line in data.splitlines()]
print(data)

h = 0
d = 0
for line in data:
    dir, dis = line.split(' ')
    if dir == 'forward':
        h += int(dis)
    elif dir == 'down':
        d += int(dis)
    elif dir == 'up':
        d -= int(dis)

print(h * d)

