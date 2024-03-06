from random import shuffle
import math

class Sky:
    ''''''
    def __init__(self, values: list[int]):
        self._values = values
        self._min = -math.inf
        self._index = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self._index >= len(self._values):
            raise StopIteration
        elem = self._values[self._index]
        self._index +=1
        if elem > self._min:
            self._min = elem
            return elem
        else:
            return self.__next__()
    
    
    
values = list(range(1, 6))
shuffle(values)  # e.g., [3, 1, 4, 2, 5]

print(values)

it = Sky(values)
for num in it:
    print(num)
    
    