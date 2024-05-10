

import Data.Array -- :set -package array
import System.Random ( mkStdGen, Random(randomRs) ) 

main :: IO ()
main = do
    putStrLn "Insert the number of cols:"
    c <- getLine
    putStrLn "Insert the number of rows:"
    r <- getLine
    let cols = read c :: Int
    let rows = read r :: Int
    putStrLn $ "Matrix dim (cols*rows): " ++ show cols ++ "*" ++ show rows
    putStrLn "Insert the number of first random moves:"
    mm <- getLine
    let m = read mm :: Int
    let initialMatrix = playM (array (0,cols*rows-1) [(i,False) | i <- [0..cols*rows-1]]) (getRandomMoves m (randomRs (0,cols*rows - 1) (mkStdGen m) :: [Int]) []) (cols,rows)
    playCleanUp initialMatrix (cols,rows) 0

-- Haskell is a scam is not really lazy lol
--getRandomMoves' :: Int -> Int -> [Int]
--getRandomMoves' m dim = take m (foldl (\acc r -> if r `elem` acc then acc else r:acc) [] (randomRs (0,dim) (mkStdGen m) :: [Int]))

getRandomMoves :: Int -> [Int] -> [Int] -> [Int]
getRandomMoves 0 rands acc = acc
getRandomMoves m all@(r:ands) acc = 
    if r `elem` acc then 
        getRandomMoves m ands acc
    else
        getRandomMoves (m-1) all (r:acc)

playM :: [Bool] -> [Int] -> (Int, Int) -> [Bool]
playM matrix [] (cols,rows) = matrix
playM matrix (m:oves) (cols,rows) = 
    let m1 = purePlay (cols,rows) matrix m 
    in playM m1 oves (cols,rows)

playCleanUp :: [Bool] -> (Int, Int) -> Int -> IO ()
playCleanUp matrix (cols,rows) moves = do 
    putStrLn "Matrix:"
    print matrix
    putStrLn $ "Moves: " ++ show moves
    if and matrix then do 
        putStrLn "You won!"
    else do 
        putStrLn "Insert the col index to play:"
        x <- getLine
        putStrLn "Insert the row index to play:"
        y <- getLine
        let i = (read x :: Int) + (read y :: Int) * cols
        let m1 = purePlay (cols,rows) matrix i
        playCleanUp m1 (cols,rows) (moves+1) 

purePlay (cols,rows) matrix i = matrix // [(j,not(matrix ! j)) | j <- [0..cols*rows-1], j `elem` [i-1,i+1,i-cols,i+cols]] -- [w,e,n,s]
