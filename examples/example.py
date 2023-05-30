from lana import Matrix

zeros = Matrix.zeros((3,3))
print(zeros)
print(f"shape: {zeros.shape}, type: {type(zeros)}")
print(zeros[0], zeros[0][0])
for row in zeros:
    print(row, type(row))

a = Matrix.ones((2,3))
print(a)
b = Matrix.matrix([[2,2,2],[2,2,2]])
print(b)
c = a + b
print(c)
