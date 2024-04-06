

maxOfTwo :: (Show z, Ord z) => z -> z -> String
maxOfTwo a b 
    | a < b = "The maximum is " ++ show b
    | a > b = "The maximum is " ++ show a
    | otherwise  = "???"
    
