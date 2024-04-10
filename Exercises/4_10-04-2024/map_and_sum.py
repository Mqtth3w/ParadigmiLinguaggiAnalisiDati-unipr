from itertools import count

def multiple_any(x: int, ys: list[int]) -> bool:
    return any(map(lambda y: x%y == 0, ys))

print(multiple_any(40, [3, 5, 7]))

#v1 ok
tot = 0
for x in count(1):
    if multiple_any(x**2, [3, 5, 7]) and x**2 < 5000:
        #print(x**2)
        tot += x**2
    elif x**2 >= 5000:
        break
print(tot)

#v2 too weight to calculate
print(sum(x**2 for x in count(1) if multiple_any(x**2, [3, 5, 7]) and x**2 < 5000))
