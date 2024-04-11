from itertools import count

def multiple_any(x: int, ys: list[int]) -> bool:
    return any(map(lambda y: x%y == 0, ys))

print(multiple_any(40, [3, 5, 7]))

#v1 ok
tot = 0
for i in count(1):
    if multiple_any(i**2, [3, 5, 7]) and i**2 < 5000:
        #print(i**2)
        tot += i**2
    elif i**2 >= 5000:
        break
print(tot)

#v2 too heavy to calculate
#print(sum(i**2 for i in count(1) if multiple_any(i**2, [3, 5, 7]) and i**2 < 5000))


#generator
def sum_gen(ys:list[int], lim:int):
    gen = 0
    for x in count(1):
        if multiple_any(x**2, ys) and x**2 < lim:
            yield(x**2)
            gen += x**2
        elif x**2 >= lim:
            break
    
for y in sum_gen([3, 5, 7], 5000):
    print(y)
