from random import shuffle
import math

class Sky:
    ''''''
    def __init__(self, values: list[int]):
        self._values = values
        self._min = -math.inf

    def __iter__(self):
        return self

    def __next__(self):
        for elem in self._values:
            if elem > self._min:
                self._min = elem
                return elem
        raise StopIteration
    
    
    
values = list(range(1, 6))
shuffle(values)  # e.g., [3, 1, 4, 2, 5]

print(values)

it = Sky(values)
for num in it:
    print(num)
    
    