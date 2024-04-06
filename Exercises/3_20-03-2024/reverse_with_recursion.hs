
reverse' :: [a] -> [a] -> [a]
reverse' [] acc = acc
reverse' (x:xs) acc = reverse' xs (x:acc)


rev' :: [a] -> [a]
rev' xs = reverse' xs []