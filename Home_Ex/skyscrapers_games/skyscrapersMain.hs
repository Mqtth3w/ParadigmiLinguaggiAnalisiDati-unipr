
import System.IO (readFile)
import Data.List (transpose)

-- https://www.brainbashers.com/skyscrapershelp.asp

main = do
    putStrLn "1) Every row and column contains the numbers 1 to the size of the grid."
    putStrLn "2) Every row and column contains a number only once."
    putStrLn "3) The clues tell you how many skyscrapers you can see in that direction."
    putStrLn "3) You can't see a shorter skyscraper behind a taller one."
    --data
    putStrLn "---------- skyscrapers-3x3.txt ----------"
    checkRules "skyscrapers-3x3.txt"

    putStrLn "---------- skyscrapers-4x4.txt ----------"
    checkRules "skyscrapers-4x4.txt"

    putStrLn "---------- skyscrapers-5x5.txt ----------"
    checkRules "skyscrapers-5x5.txt"

    putStrLn "---------- skyscrapers-6x6.txt ----------"
    checkRules "skyscrapers-6x6.txt"

    putStrLn "---------- Done at all. ----------"

checkRules file = do 
    contents <- readFile file
    let skyscrapers = map (map parseInt . words) (lines contents)
    let boardgame = map (init . tail) (init $ tail skyscrapers)
    let gridDim = length boardgame
    --rule one
    let checkOneRows = and (ruleOne boardgame gridDim)
    let checkOneCols = and (ruleOne (transpose boardgame) gridDim)
    putStrLn "Is rule 1 respected? [rows,cols]"
    print [checkOneRows,checkOneCols]
    --rule two
    let checkTwoRows = and (ruleTwo boardgame gridDim)
    let checkTwoCols = and (ruleTwo (transpose boardgame) gridDim)
    putStrLn "Is rule 2 respected? [rows,cols]"
    print [checkTwoRows,checkTwoCols]
    --rule three
    let checkThreeRows = and (ruleThree (init $ tail skyscrapers))
    let checkThreeCols = and (ruleThree (init $ tail (transpose skyscrapers)))
    putStrLn "Is rule 3 respected? [rows,cols]"
    print [checkThreeRows,checkThreeCols]
    --print skyscrapers
    --print gridDim
    --print boardgame
    
parseInt :: String -> Int
parseInt str = read str :: Int

ruleOne board gridDim =
    map (foldl (\acc e -> not (e < 1 || e > gridDim) && acc) True) board

ruleTwo board gridDim =
    let res = map (foldl (\acc e -> if e `elem` acc then acc else e:acc) []) board
    in map (\e -> length e == gridDim) res

ruleThree =
    map (\e -> 
        --check left and right for each
        let 
            eReal = init $ tail e
            lres = skyscrapersCounter eReal == head e
            rres = skyscrapersCounter (reverse eReal) == last e
        in lres && rres)
        
skyscrapersCount :: (Num a, Ord a, Show a) => [a] -> a -> a -> a
skyscrapersCount [] highest acc = acc
skyscrapersCount (x:xs) highest acc 
    | x > highest = skyscrapersCount xs x (acc+1)
    | otherwise = skyscrapersCount xs highest acc

skyscrapersCounter :: (Num a, Ord a, Show a) => [a] -> a
skyscrapersCounter xs = skyscrapersCount xs 0 0