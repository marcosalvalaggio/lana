from lana import Matrix

zeros = Matrix.zeros((3,3))
print(zeros)
print(f"shape: {zeros.shape}, type: {type(zeros)}")
print(zeros[0], zeros[0][0])
for row in zeros:
    print(row, type(row))


