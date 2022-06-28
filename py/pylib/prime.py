# N(int):素因数分解する自然数
# V(int[]):約数を格納するリスト(重複有り)
def enumerateFactors(N, V):
    p = 2
    while N > 1:
        if p * p > N:
            V.append(N)
            return
        if N % p == 0:
            V.append(p)
            N //= p
        else:
            p += 1
    return

