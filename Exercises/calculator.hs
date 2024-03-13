

calculate :: (Show z, Fractional z, Eq z) => z -> z -> String -> String
calculate a b "+" = show a ++ " + " ++ show b ++ " = " ++ show (a+b)
calculate a b "-" = show a ++ " - " ++ show b ++ " = " ++ show (a-b)
calculate a b "*" = show a ++ " * " ++ show b ++ " = " ++ show (a*b)
calculate a 0 "/" = "Error!"
calculate a b "/" = show a ++ " / " ++ show b ++ " = " ++ show (a/b)

