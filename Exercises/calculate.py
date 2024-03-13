import dataclasses

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


def calculate(op):
    match op:
        case Sum(x=x, y=y):
            print("x + y = " + str(x+y))
        case Product(x=x, y=y):  # idiomatic: y=y
            print("x * y = " + str(x*y))
        case Negate(x=x):  # idiomatic: x=x
            print("x negate = " + str(-x))
        case _ :
            print("Not a stuff")
            

calculate(Sum(1,1))
calculate(Product(1,2))
calculate(Negate(1))
calculate("KO")
