

def merge(v1: list, b1, e1,  # beg, end
          v2: list, b2, e2) -> list:
    i = 0
    result = []  # required extra memory
    while b1 < e1 or b2 < e2:
        if b1 < e1 and (b2 == e2 or v1[b1] <= v2[b2]):
            result.append(v1[b1]); b1 += 1
            
        else:
            result.append(v2[b2]); b2 += 1
            
        yield result[i]
        i += 1
            
pista = [1, 3, 5, 8]
busta = [2, 4, 6, 7]

sium = merge(pista,0,len(pista),busta,0,len(busta))
for elem in sium: 
    print(elem )

