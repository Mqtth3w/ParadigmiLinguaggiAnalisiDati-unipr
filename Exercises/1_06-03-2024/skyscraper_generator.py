from random import shuffle
import math


def prod_max(values: list[int]) -> int:
    
    min_now = -math.inf
    
    for elem in values:
        
        if elem > min_now:
            
            min_now = elem
            yield elem
            
    
values = list(range(1, 6))
shuffle(values)  # e.g., [3, 1, 4, 2, 5]

print(values)

pm = prod_max(values)
for num in pm:
    print(num)
    