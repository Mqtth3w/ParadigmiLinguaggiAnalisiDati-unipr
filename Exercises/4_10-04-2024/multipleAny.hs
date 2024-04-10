
multipleAny :: Int -> [Int] -> Bool
multipleAny x ys = or(map (\k -> x `mod` k == 0) ys)