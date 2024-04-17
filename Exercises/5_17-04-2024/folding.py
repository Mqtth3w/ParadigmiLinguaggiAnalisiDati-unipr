from functools import reduce

l = [1,5,8,3,9]
#lambda parameters : code_for_if if (condition) else code_for_else
print(reduce(lambda a, b: a + 1 if b > a else a, l, 0)) 