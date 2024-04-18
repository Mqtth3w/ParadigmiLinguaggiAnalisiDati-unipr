
skyscrapersCountFold xs = length (foldl (\all@(a:cc) x -> if x > a then x:all else all) [0] xs) - 1 

--prof
countTops xs = foldl (\ (m, c) x -> max (x, c + 1) (m, c)) (0, 0) xs
