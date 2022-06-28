class Modint:
    # instance var.
    # v(int):
    # M(int):法
    def __init__(self, v, M=1000000000 + 7):
        self.v = v % M
        self.M = M

    def __add__(self, other):
        return Modint(self.v + other.v, self.M)

    def __sub__(self, other):
        return Modint(self.v - other.v, self.M)

    def __mul__(self, other):
        self.v %= self.M
        other.v %= other.M
        return Modint(self.v * other.v, self.M)

    def __eq__(self, other):
        return (self.v - other.v) % self.M == 0

    def __ne__(self, other):
        return (self.v - other.v) % self.M != 0

    def __iadd__(self, other):
        self.v += other.v
        return self

    def __isub__(self, other):
        self.v -= other.v
        return self

    def __imul__(self, other):
        self.v %= self.M
        other.v %= other.M
        self.v *= other.v
        return self

    def __pow__(self, power, modulo=None):
        if type(power) == int and power >= 0:
            if power == 0:
                return Modint(1, self.M)
            if power == 1:
                return self
            answer = self ** (power // 2)
            answer *= answer
            if power % 2 == 1:
                answer *= self
            return answer
        return self

    def __pos__(self):
        return Modint(self.v, self.M)

    def __neg__(self):
        return Modint(-self.v, self.M)

    def __rtruediv__(self, other):
        inv = other ** (other.M - 2)
        return self * inv

    def __str__(self):
        return str(self.v % self.M)


