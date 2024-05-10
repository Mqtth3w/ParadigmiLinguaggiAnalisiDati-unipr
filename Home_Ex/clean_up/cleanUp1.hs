

main = do
    let cols = 4
    let rows = 3
    putStrLn $ "Matrix dim (cols*rows): " ++ show cols ++ "*" ++ show rows
    let initialMatrix = [False | i <- [1..cols*rows]]
    playCleanUp initialMatrix cols rows

playCleanUp matrix cols rows = do 
    putStrLn "Matrix:"
    print matrix
    if and matrix then do 
        putStrLn "You won!"
    else do 
        putStrLn "Insert the col index to play:"
        x <- getLine
        putStrLn "Insert the row index to play:"
        y <- getLine
        let i = (read x :: Int) + (read y :: Int) * cols
        let m1 = purePlay (cols,rows) matrix i
        playCleanUp m1 cols rows     

purePlay :: (Int, Int) -> [Bool] -> Int -> [Bool]
purePlay (cols,rows) matrix i = 
    let delta = [i-1,i+1,i-cols,i+cols] -- [w,e,n,s]
        in [if i `elem` delta then not z else z  | (i,z) <- zip [0..(cols*rows - 1)] matrix]