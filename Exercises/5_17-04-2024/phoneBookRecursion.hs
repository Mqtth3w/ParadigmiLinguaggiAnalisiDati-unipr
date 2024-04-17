

type Name = String
type PhoneNumber = String
type PhoneBook = [(Name,PhoneNumber)]

getPhoneNumber :: Name -> PhoneBook -> PhoneNumber
getPhoneNumber y [] = ""
getPhoneNumber y (x:xs) = if y == fst x then snd x else getPhoneNumber y xs
