

skyscraperCount :: (Num a, Ord a, Show a) => [a] -> a -> a -> a
skyscraperCount [] highest acc = acc
skyscraperCount (x:xs) highest acc 
    | x > highest = skyscraperCount xs x (acc+1)
    | otherwise = skyscraperCount xs highest acc


skyscraperCounter :: (Num a, Ord a, Show a) => [a] -> a
skyscraperCounter xs = skyscraperCount xs (minimum xs -1) 0 