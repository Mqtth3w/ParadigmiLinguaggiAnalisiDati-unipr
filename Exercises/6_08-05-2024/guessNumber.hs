
import System.Random
import Control.Monad ( when ) 

main = do 
    gen <- getStdGen
    let (secret, newGen) = randomR (1,90) gen :: (Int, StdGen)
    askForNumber 1 secret


askForNumber :: Int -> Int -> IO ()
askForNumber trie secret = do
    putStrLn $ "Which number (1-90) am I thinking of? (max 10 attemps) Current attemp: " ++ show trie
    guess <- getLine
    when (not $ null guess) $
        if guess == show secret then 
            putStrLn "You are correct!"
        else 
            if trie < 10 then do
                putStrLn $ "Sorry, it isn't correct." ++ if (read guess :: Int) > secret then " Your is GT" else " Your is LT" 
                askForNumber (trie+1) secret
            else do 
                putStrLn "Sorry, too much attemps."
    