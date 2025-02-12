# data = open("example.txt").read().strip()
data = open("input.txt").read().strip()

nums = [int(line) for line in data.splitlines()]

result = 0
prev_sum = None
prev = None
prev2 = None
for curr in nums:
    if prev2 and prev and curr:
        if prev_sum and prev + prev2 + curr > prev_sum:
            result += 1
        prev_sum = prev2 + prev + curr
    prev2 = prev
    prev = curr

print(result)
