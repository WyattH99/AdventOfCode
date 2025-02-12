# data = open("example.txt").read().strip()
data = open("input.txt").read().strip()

data = [int(line) for line in data.splitlines()]
print(data)

result = 0
prev = None
for curr in data:
    if prev and curr > prev:
        result += 1
    prev = curr

print(result)
