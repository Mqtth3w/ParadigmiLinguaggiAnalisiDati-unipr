import Data.Maybe

type Name = String
type PhoneNumber = String 
data Maybe PhoneNumber = Nothing | Just PhoneNumber
type PhoneBook = [(Name,PhoneNumber)]


--second
getPhoneNumber2 :: Name -> PhoneBook -> Maybe PhoneNumber
getPhoneNumber2 z [] = Nothing
getPhoneNumber2 z all@(x:xs) = fromMaybe Nothing all