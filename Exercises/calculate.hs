

calculate :: (Show z, Fractional z, Eq z) => z -> z -> String -> String
calculate a b "+" = show a ++ " + " ++ show b ++ " = " ++ show (a+b)
calculate a b "-" = show a ++ " - " ++ show b ++ " = " ++ show (a-b)
calculate a b "*" = show a ++ " * " ++ show b ++ " = " ++ show (a*b)
calculate a 0 "/" = "Error!" -- Haskell is able to handle it alone
calculate a b "/" = show a ++ " / " ++ show b ++ " = " ++ show (a/b)

