from lana import Matrix

zeros = Matrix.zeros((3,3))
print(zeros)
print(f"shape: {zeros.shape}, type: {type(zeros)}")

print("\n")
print(zeros[0], zeros[0][0])
for row in zeros:
     print(row, type(row))

print("\n")
a = Matrix.ones((2,3))
b = Matrix.matrix([[2,2,2],[2,2,2]])
c = a + b
print(c)
d = c - a
print(d)
e = -d 
print(e)
print(e.shape, type(e))

print(zeros[0], type(zeros[0]))
