import dataclasses
from typing import Union

@dataclasses.dataclass
class Sum:  # ~ struct, with init, repr… methods
    x: int
    y: int
    
@dataclasses.dataclass
class Product:  # ~ struct, with init, repr… methods
    x: int
    y: int

@dataclasses.dataclass
class Negate:  # ~ struct, with init, repr… methods
    x: int


def calculate(op: Union[Product, Negate, Sum]) -> str:
    result = ""
    match op:
        case Sum(x=x, y=y):
            result = "x + y = " + str(x+y)
        case Product(x=x, y=y):  # idiomatic: y=y
            result = "x * y = " + str(x*y)
        case Negate(x=x):  # idiomatic: x=x
            result = "x negate = " + str(-x)
        case _ :
            result = "Not a stuff"
    return result   

print(calculate(Sum(1,1)))
print(calculate(Product(1,2)))
print(calculate(Negate(1)))
print(calculate("KO"))
