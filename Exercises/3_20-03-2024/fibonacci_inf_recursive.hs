

fib :: Num t => t -> t -> [t]
fib a b = a : fib b (a+b)
