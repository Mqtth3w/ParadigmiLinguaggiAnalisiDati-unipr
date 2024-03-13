# [(a,b,c) | a <- [1..10],  b <- [1..10], c <- [1..10], (a^2) == (b^2) + (c^2), a + b + c == 24]

# evita ripetizioni
# [(a,b,c) | a <- [1..10],  b <- [a..10], c <- [1..10], (a^2) == (b^2) + (c^2), a + b + c == 24]
# [(a,b,c) | a <- [1..10],  b <- [a..10], c <- [24 - a - b], (a^2) == (b^2) + (c^2), a + b + c == 24]
# [(a,b,c) | a <- [1..10],  b <- [a..10], let c = 24 - a - b, (a^2) == (b^2) + (c^2), a + b + c == 24]
'''
Which right triangle…

    that has integers for all sides…
    and all sides equal to or smaller than 10…
    has a perimeter of 24?

Solve using a comprehension
'''


print([(a,b,c) for a in range(1, 11) for b in range(1, 11) for c in range(1, 11) if (a+b+c == 24) and (a**2 == b**2 + c**2)])

print([(a,b,c) for a in range(1, 11) for b in range(a, 11) for c in [24 - a - b] if (a**2 == b**2 + c**2)])
