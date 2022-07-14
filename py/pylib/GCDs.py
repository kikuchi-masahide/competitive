# 最大公約数
# O(log max(a,b))
def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a % b)
