
skyscrapersCount :: (Num a, Ord a, Show a) => [a] -> a -> a -> a
skyscrapersCount [] highest acc = acc
skyscrapersCount (x:xs) highest acc 
    | x > highest = skyscrapersCount xs x (acc+1)
    | otherwise = skyscrapersCount xs highest acc


skyscrapersCounter :: (Num a, Ord a, Show a) => [a] -> a
skyscrapersCounter xs = skyscrapersCount xs (minimum xs -1) 0 --second par is 0 if list of positive values
