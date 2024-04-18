import Data.Maybe

type Name = String
type PhoneNumber = String 
type PhoneBook = [(Name,PhoneNumber)]

--fold
getPhoneNumberFold :: Name -> PhoneBook -> Maybe PhoneNumber
getPhoneNumberFold y xs = foldl (\acc (k,v) -> if (y == k && not (isJust acc)) then Just v else acc) Nothing xs
