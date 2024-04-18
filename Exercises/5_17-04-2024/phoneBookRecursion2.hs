import Data.Maybe
--import Text.Read

type Name = String
type PhoneNumber = String 
type PhoneBook = [(Name,PhoneNumber)]


--second 
getPhoneNumber2 :: Name -> PhoneBook -> Maybe PhoneNumber
getPhoneNumber2 y [] = Nothing
getPhoneNumber2 y all@(x:xs) = if y == fst x then Just (snd x)  else getPhoneNumber2 y xs