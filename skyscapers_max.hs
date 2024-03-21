

skyscrapersMAX :: (Num a, Ord a, Show a) => [a] -> [a] -> a -> [a]
skyscrapersMAX [] zs highest = reverse zs 
skyscrapersMAX (x:xs) zs highest
    | x > highest = skyscrapersMAX xs (x:zs) x 
    | otherwise = skyscrapersMAX xs zs highest


skyscrapersMax :: (Num a, Ord a, Show a) => [a] -> [a]
skyscrapersMax xs = skyscrapersMAX xs [] (minimum xs - 1)
