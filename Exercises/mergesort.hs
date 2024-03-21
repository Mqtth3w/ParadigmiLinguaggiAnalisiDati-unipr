

merge xs [] = xs
merge [] ys = ys
merge (x:xs) (y:ys) 
    | x <= y = x:merge xs (y:ys)
    | otherwise =  y:merge (x:xs) ys



mergeSort :: Ord a => [a] -> [a]
mergeSort [] = []
mergeSort [x] = [x]
mergeSort zs = 
    let mergesx = mergeSort (take (length zs `div` 2) zs)
        mergedx = mergeSort (drop (length zs `div` 2) zs)
    in merge mergesx mergedx