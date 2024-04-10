
multipleAny :: Int -> [Int] -> Bool
multipleAny x ys = 0 `elem` (map (x `rem`) ys)
