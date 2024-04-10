
multipleAny :: Int -> [Int] -> Bool
multipleAny x ys = or(map (\k -> x `mod` k == 0) ys)

sumSquares0 = takeWhile  (<5000) (filter (`multipleAny` [3,5,7]) (map (^2) [1..]))
sumSquares = sum(takeWhile  (<5000) (filter (`multipleAny` [3,5,7]) (map (^2) [1..])))