from random import shuffle
import math


def count_tops(values: list[int]) -> int:
    
    min_ora = -math.inf
    tops = 0
    
    for elem in values:
        
        if elem > min_ora:
            
            min_ora = elem
            tops += 1
            
    return tops


lista = [-3, -2, -1, 0, 1, 2, 3]
values = list(range(1, 6))
shuffle(values)  # e.g., [3, 1, 4, 2, 5]

print(values)
print("test1 " + str(count_tops(values)) + "\n")
print("test1 " + str(count_tops(lista)) + "\n")
