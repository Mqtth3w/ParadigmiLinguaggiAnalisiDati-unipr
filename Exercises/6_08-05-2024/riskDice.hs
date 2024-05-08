
import System.Random
import Data.List

main = do
    putStrLn "Risk Dice"
    let n = 10
    let r = playN n
    let awin = filter id r
    let len = length awin
    putStrLn $ "A won: " ++ show len
    putStrLn $ "D won: " ++ show (n - len)
    putStrLn "Risk Dice end"

launchDices seed numDiece = sort $ take numDiece $ randomRs (1,6) (mkStdGen seed) :: [Int]


playN n = 
    let numDice = 3
    in 
    map (\iter ->
        let a = launchDices 100 numDice
            d = launchDices 109 numDice
            aWin =  a > d
        in aWin) [1..n]
    
--prof 
