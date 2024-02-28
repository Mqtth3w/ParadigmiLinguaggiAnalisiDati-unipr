pista = [1, 2, 3]
--alt + 96 `
media xs = sum ( xs ) `div` fromInteger ( length (xs))

media_cinque lista = media (take 5 lista)

variabile = media_cinque pista